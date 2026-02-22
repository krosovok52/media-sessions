//! Linux backend implementation using D-Bus and MPRIS 2.0.
//!
//! This module provides integration with Linux media players through
//! the Media Player Remote Interfacing Specification (MPRIS) 2.0.
//!
//! # Requirements
//!
//! - D-Bus session bus running
//! - MPRIS-compatible media player (Spotify, Firefox, mpv, etc.)
//! - The `zbus` crate for async D-Bus communication

use std::sync::Arc;
use std::time::Duration;

use tokio::sync::{RwLock, mpsc};

use super::backend::MediaSessionBackend;
use crate::error::{MediaError, MediaResult};
use crate::media_info::{MediaInfo, PlaybackStatus};
use crate::media_sessions::{MediaSessionEvent, RepeatMode};

/// MPRIS service name prefix.
const MPRIS_SERVICE_PREFIX: &str = "org.mpris.MediaPlayer2.";

/// MPRIS object path.
const MPRIS_PATH: &str = "/org/mpris/MediaPlayer2";

/// MPRIS player interface.
const MPRIS_PLAYER_INTERFACE: &str = "org.mpris.MediaPlayer2.Player";

/// Linux MPRIS backend.
#[derive(Clone, Debug)]
pub struct LinuxBackend {
    connection: Option<zbus::Connection>,
    player_name: Arc<RwLock<Option<String>>>,
}

impl LinuxBackend {
    /// Creates a new Linux backend instance.
    pub fn new() -> MediaResult<Self> {
        match tokio::runtime::Handle::try_current() {
            Ok(handle) => handle.block_on(Self::new_async()),
            Err(_) => {
                let rt = tokio::runtime::Runtime::new().map_err(|e| MediaError::Backend {
                    platform: "linux".to_string(),
                    message: format!("Failed to create runtime: {e}"),
                })?;
                rt.block_on(Self::new_async())
            }
        }
    }

    /// Async constructor.
    async fn new_async() -> MediaResult<Self> {
        let connection = zbus::Connection::session()
            .await
            .map_err(|e| MediaError::DBusError(format!("Failed to connect to session bus: {e}")))?;

        let player_name = Self::find_player(&connection).await?;

        Ok(Self {
            connection: Some(connection),
            player_name: Arc::new(RwLock::new(player_name)),
        })
    }

    /// Finds an active MPRIS player.
    async fn find_player(connection: &zbus::Connection) -> MediaResult<Option<String>> {
        let proxy = zbus::fdo::DBusProxy::new(connection)
            .await
            .map_err(|e| MediaError::DBusError(format!("Failed to create DBus proxy: {e}")))?;

        let names = proxy
            .list_names()
            .await
            .map_err(|e| MediaError::DBusError(format!("Failed to list names: {e}")))?;

        for name in names {
            if name.starts_with(MPRIS_SERVICE_PREFIX) {
                return Ok(Some(name.to_string()));
            }
        }

        Ok(None)
    }

    /// Gets the player proxy.
    async fn get_proxy(&self) -> MediaResult<zbus::Proxy<'_>> {
        let connection = self.connection.as_ref().ok_or(MediaError::NoSession)?;
        let player_guard = self.player_name.read().await;
        let player_name = player_guard.as_ref().ok_or(MediaError::NoSession)?;

        zbus::Proxy::new(connection, player_name, MPRIS_PATH, MPRIS_PLAYER_INTERFACE)
            .await
            .map_err(|e| MediaError::DBusError(format!("Failed to create proxy: {e}")))
    }

    /// Converts MPRIS playback state.
    fn convert_playback_state(state: &str) -> PlaybackStatus {
        match state {
            "Playing" => PlaybackStatus::Playing,
            "Paused" => PlaybackStatus::Paused,
            "Stopped" => PlaybackStatus::Stopped,
            _ => PlaybackStatus::Transitioning,
        }
    }

    /// Polls for media session changes and emits events.
    async fn poll_events(
        &self,
        tx: mpsc::Sender<MediaResult<MediaSessionEvent>>,
        debounce_duration: Duration,
    ) {
        let mut last_status: Option<PlaybackStatus> = None;
        let mut last_title: Option<String> = None;
        let mut last_emit_time = tokio::time::Instant::now();

        loop {
            tokio::time::sleep(Duration::from_millis(500)).await;

            if tx.is_closed() {
                break;
            }

            // Check for player changes
            if let Ok(conn) = self.connection.as_ref().ok_or(MediaError::NoSession) {
                let player = Self::find_player(conn).await.unwrap_or(None);
                let mut current_player = self.player_name.write().await;
                if player != *current_player {
                    *current_player = player.clone();
                    if player.is_none() && last_status.is_some() {
                        let _ = tx.send(Ok(MediaSessionEvent::SessionClosed)).await;
                        last_status = None;
                        last_title = None;
                    }
                }
            }

            let info = match self.get_current().await {
                Ok(Some(i)) => i,
                Ok(None) => {
                    if last_status.is_some() {
                        last_status = None;
                        let _ = tx.send(Ok(MediaSessionEvent::SessionClosed)).await;
                    }
                    continue;
                }
                Err(e) => {
                    let _ = tx.send(Err(e)).await;
                    continue;
                }
            };

            // Debounce check
            let now = tokio::time::Instant::now();
            if now.duration_since(last_emit_time) < debounce_duration {
                continue;
            }

            // Check for metadata changes
            let title_changed = info.title != last_title;
            if title_changed {
                last_title = info.title.clone();
                let _ = tx
                    .send(Ok(MediaSessionEvent::MetadataChanged(info.clone())))
                    .await;
                last_emit_time = now;
                continue;
            }

            // Check for playback status changes
            if Some(info.playback_status) != last_status {
                last_status = Some(info.playback_status);
                let _ = tx
                    .send(Ok(MediaSessionEvent::PlaybackStatusChanged(
                        info.playback_status,
                    )))
                    .await;
                last_emit_time = now;
            }
        }
    }
}

#[async_trait::async_trait]
impl MediaSessionBackend for LinuxBackend {
    fn platform_name(&self) -> &'static str {
        "linux"
    }

    async fn get_current(&self) -> MediaResult<Option<MediaInfo>> {
        let proxy = match self.get_proxy().await {
            Ok(p) => p,
            Err(MediaError::NoSession) => return Ok(None),
            Err(e) => return Err(e),
        };

        let metadata: zbus::zvariant::Value<'_> = proxy
            .get_property("Metadata")
            .await
            .map_err(|e| MediaError::DBusError(format!("Failed to get metadata: {e}")))?;

        let status: String = proxy
            .get_property("PlaybackStatus")
            .await
            .map_err(|e| MediaError::DBusError(format!("Failed to get status: {e}")))?;

        let position: i64 = proxy
            .get_property("Position")
            .await
            .map_err(|e| MediaError::DBusError(format!("Failed to get position: {e}")))?;

        let mut info = MediaInfo {
            playback_status: Self::convert_playback_state(&status),
            position: Some(Duration::from_micros(position as u64)),
            ..Default::default()
        };

        if let zbus::zvariant::Value::Dict(dict) = metadata {
            for (key, value) in dict.iter() {
                let key_str = match key.as_str() {
                    Some(s) => s,
                    None => continue,
                };

                match key_str {
                    "xesam:title" => {
                        if let zbus::zvariant::Value::Str(s) = value {
                            info.title = Some(s.to_string());
                        }
                    }
                    "xesam:artist" => {
                        if let zbus::zvariant::Value::Array(arr) = value {
                            let artists: Vec<String> = arr
                                .iter()
                                .filter_map(|v| {
                                    if let zbus::zvariant::Value::Str(s) = v {
                                        Some(s.to_string())
                                    } else {
                                        None
                                    }
                                })
                                .collect();
                            info.artist = Some(artists.join(", "));
                        }
                    }
                    "xesam:album" => {
                        if let zbus::zvariant::Value::Str(s) = value {
                            info.album = Some(s.to_string());
                        }
                    }
                    "mpris:length" => {
                        if let zbus::zvariant::Value::I64(n) = value {
                            info.duration = Some(Duration::from_micros(n as u64));
                        }
                    }
                    _ => {}
                }
            }
        }

        Ok(Some(info))
    }

    async fn get_artwork(&self) -> MediaResult<Option<Vec<u8>>> {
        // Artwork would require fetching the artwork URL and downloading it
        // This is a simplified implementation
        Ok(None)
    }

    fn get_active_app(&self) -> MediaResult<Option<String>> {
        let player_guard = futures::executor::block_on(self.player_name.read());
        Ok(player_guard.as_ref().map(|name| {
            name.strip_prefix(MPRIS_SERVICE_PREFIX)
                .unwrap_or(name)
                .to_string()
        }))
    }

    async fn play(&self) -> MediaResult<()> {
        let proxy = self.get_proxy().await?;
        proxy
            .call("Play", &())
            .await
            .map_err(|e| MediaError::DBusError(format!("Failed to call Play: {e}")))
    }

    async fn pause(&self) -> MediaResult<()> {
        let proxy = self.get_proxy().await?;
        proxy
            .call("Pause", &())
            .await
            .map_err(|e| MediaError::DBusError(format!("Failed to call Pause: {e}")))
    }

    async fn play_pause(&self) -> MediaResult<()> {
        let proxy = self.get_proxy().await?;
        proxy
            .call("PlayPause", &())
            .await
            .map_err(|e| MediaError::DBusError(format!("Failed to call PlayPause: {e}")))
    }

    async fn stop(&self) -> MediaResult<()> {
        let proxy = self.get_proxy().await?;
        proxy
            .call("Stop", &())
            .await
            .map_err(|e| MediaError::DBusError(format!("Failed to call Stop: {e}")))
    }

    async fn next(&self) -> MediaResult<()> {
        let proxy = self.get_proxy().await?;
        proxy
            .call("Next", &())
            .await
            .map_err(|e| MediaError::DBusError(format!("Failed to call Next: {e}")))
    }

    async fn previous(&self) -> MediaResult<()> {
        let proxy = self.get_proxy().await?;
        proxy
            .call("Previous", &())
            .await
            .map_err(|e| MediaError::DBusError(format!("Failed to call Previous: {e}")))
    }

    async fn seek(&self, position: Duration) -> MediaResult<()> {
        let proxy = self.get_proxy().await?;
        let position_us = position.as_micros() as i64;

        proxy
            .call(
                "SetPosition",
                &(
                    zbus::zvariant::ObjectPath::from_static_str_unchecked(MPRIS_PATH),
                    position_us,
                ),
            )
            .await
            .map_err(|e| MediaError::DBusError(format!("Failed to call SetPosition: {e}")))
    }

    async fn set_volume(&self, volume: f64) -> MediaResult<()> {
        let proxy = self.get_proxy().await?;
        proxy
            .set_property("Volume", &volume)
            .await
            .map_err(|e| MediaError::DBusError(format!("Failed to set volume: {e}")))
    }

    async fn set_repeat_mode(&self, mode: RepeatMode) -> MediaResult<()> {
        let proxy = self.get_proxy().await?;
        let loop_status = match mode {
            RepeatMode::None => "None",
            RepeatMode::One => "Track",
            RepeatMode::All => "Playlist",
        };
        proxy
            .set_property("LoopStatus", &loop_status)
            .await
            .map_err(|e| MediaError::DBusError(format!("Failed to set loop status: {e}")))
    }

    async fn set_shuffle(&self, enabled: bool) -> MediaResult<()> {
        let proxy = self.get_proxy().await?;
        proxy
            .set_property("Shuffle", &enabled)
            .await
            .map_err(|e| MediaError::DBusError(format!("Failed to set shuffle: {e}")))
    }

    async fn start_listening(
        &self,
        tx: mpsc::Sender<MediaResult<MediaSessionEvent>>,
        debounce_duration: Duration,
    ) -> MediaResult<()> {
        let this = self.clone();
        tokio::spawn(async move {
            this.poll_events(tx, debounce_duration).await;
        });
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_playback_state_conversion() {
        assert_eq!(
            LinuxBackend::convert_playback_state("Playing"),
            PlaybackStatus::Playing
        );
        assert_eq!(
            LinuxBackend::convert_playback_state("Paused"),
            PlaybackStatus::Paused
        );
        assert_eq!(
            LinuxBackend::convert_playback_state("Stopped"),
            PlaybackStatus::Stopped
        );
    }

    #[test]
    #[ignore]
    fn test_backend_creation() {
        let result = LinuxBackend::new();
        println!("Linux backend result: {result:?}");
    }
}
