//! Windows backend implementation using WinRT Media Control API.
//!
//! This module provides integration with Windows 10/11's System Media
//! Transport Controls (SMTC) through the WinRT `Windows.Media.Control`
//! namespace.
//!
//! # Requirements
//!
//! - Windows 10 version 1803 or later

use std::sync::Arc;
use std::time::Duration;

use tokio::sync::{RwLock, mpsc};
use tokio::task::spawn_blocking;
use windows::{
    Foundation::TimeSpan,
    Media::Control::{
        GlobalSystemMediaTransportControlsSession,
        GlobalSystemMediaTransportControlsSessionManager,
        GlobalSystemMediaTransportControlsSessionPlaybackInfo,
        GlobalSystemMediaTransportControlsSessionPlaybackStatus,
        GlobalSystemMediaTransportControlsSessionTimelineProperties,
    },
};

use super::backend::MediaSessionBackend;
use crate::error::{MediaError, MediaResult};
use crate::media_info::{MediaInfo, PlaybackStatus};
use crate::media_sessions::{MediaSessionEvent, RepeatMode};

/// Windows Media Control backend.
#[derive(Clone, Debug)]
pub struct WindowsBackend {
    manager: Arc<RwLock<Option<GlobalSystemMediaTransportControlsSessionManager>>>,
    session: Arc<RwLock<Option<GlobalSystemMediaTransportControlsSession>>>,
}

impl WindowsBackend {
    /// Creates a new Windows backend instance.
    pub fn new() -> MediaResult<Self> {
        let request_result = GlobalSystemMediaTransportControlsSessionManager::RequestAsync();

        match request_result {
            Ok(op) => match op.get() {
                Ok(manager) => Ok(Self {
                    manager: Arc::new(RwLock::new(Some(manager))),
                    session: Arc::new(RwLock::new(None)),
                }),
                Err(e) => Err(MediaError::Backend {
                    platform: "windows".to_string(),
                    message: format!("Failed to get session manager: {e:?}"),
                }),
            },
            Err(e) => Err(MediaError::Backend {
                platform: "windows".to_string(),
                message: format!("RequestAsync failed: {e:?}"),
            }),
        }
    }

    /// Gets the current session (blocking).
    fn get_session_blocking(
        &self,
    ) -> MediaResult<Option<GlobalSystemMediaTransportControlsSession>> {
        // Get fresh session from manager
        let manager_guard = self.manager.try_read().map_err(|_| MediaError::Backend {
            platform: "windows".to_string(),
            message: "Manager lock poisoned".to_string(),
        })?;

        let manager = manager_guard.as_ref().ok_or_else(|| MediaError::Backend {
            platform: "windows".to_string(),
            message: "Session manager not initialized".to_string(),
        })?;

        let sessions = manager.GetSessions().map_err(|e| MediaError::Backend {
            platform: "windows".to_string(),
            message: format!("GetSessions failed: {e:?}"),
        })?;

        let first = sessions.First().map_err(|e| MediaError::Backend {
            platform: "windows".to_string(),
            message: format!("First failed: {e:?}"),
        })?;

        let has_current = first.HasCurrent().unwrap_or(false);
        if !has_current {
            return Ok(None);
        }

        let session = first.Current().map_err(|e| MediaError::Backend {
            platform: "windows".to_string(),
            message: format!("Current failed: {e:?}"),
        })?;

        // Update cache
        if let Ok(mut session_guard) = self.session.try_write() {
            *session_guard = Some(session.clone());
        }

        Ok(Some(session))
    }

    /// Converts WinRT playback status.
    fn convert_playback_status(
        status: GlobalSystemMediaTransportControlsSessionPlaybackStatus,
    ) -> PlaybackStatus {
        match status {
            GlobalSystemMediaTransportControlsSessionPlaybackStatus::Playing => {
                PlaybackStatus::Playing
            }
            GlobalSystemMediaTransportControlsSessionPlaybackStatus::Paused => {
                PlaybackStatus::Paused
            }
            GlobalSystemMediaTransportControlsSessionPlaybackStatus::Stopped => {
                PlaybackStatus::Stopped
            }
            _ => PlaybackStatus::Transitioning,
        }
    }

    /// Extracts media info from a WinRT session.
    fn extract_info(session: &GlobalSystemMediaTransportControlsSession) -> MediaResult<MediaInfo> {
        let media_props = session
            .TryGetMediaPropertiesAsync()
            .map_err(|e| MediaError::Backend {
                platform: "windows".to_string(),
                message: format!("TryGetMediaPropertiesAsync failed: {e:?}"),
            })?
            .get()
            .map_err(|e| MediaError::Backend {
                platform: "windows".to_string(),
                message: format!("get media properties failed: {e:?}"),
            })?;

        let title = media_props.Title().ok().and_then(|s| {
            let string: String = s.to_string();
            if string.is_empty() {
                None
            } else {
                Some(string)
            }
        });

        let artist = media_props.Artist().ok().and_then(|s| {
            let string: String = s.to_string();
            if string.is_empty() {
                None
            } else {
                Some(string)
            }
        });

        let album = media_props.AlbumTitle().ok().and_then(|s| {
            let string: String = s.to_string();
            if string.is_empty() {
                None
            } else {
                Some(string)
            }
        });

        let playback_info: GlobalSystemMediaTransportControlsSessionPlaybackInfo =
            session.GetPlaybackInfo().map_err(|e| MediaError::Backend {
                platform: "windows".to_string(),
                message: format!("GetPlaybackInfo failed: {e:?}"),
            })?;

        let playback_status = Self::convert_playback_status(
            playback_info
                .PlaybackStatus()
                .unwrap_or(GlobalSystemMediaTransportControlsSessionPlaybackStatus::Stopped),
        );

        let timeline: GlobalSystemMediaTransportControlsSessionTimelineProperties = session
            .GetTimelineProperties()
            .map_err(|e| MediaError::Backend {
                platform: "windows".to_string(),
                message: format!("GetTimelineProperties failed: {e:?}"),
            })?;

        let position = timeline.Position().ok().and_then(|ts: TimeSpan| {
            let ticks = ts.Duration;
            if ticks >= 0 {
                Some(Duration::from_secs(ticks as u64 / 10_000_000))
            } else {
                None
            }
        });

        let duration = timeline.EndTime().ok().and_then(|ts: TimeSpan| {
            let ticks = ts.Duration;
            if ticks >= 0 {
                Some(Duration::from_secs(ticks as u64 / 10_000_000))
            } else {
                None
            }
        });

        Ok(MediaInfo {
            title,
            artist,
            album,
            duration,
            position,
            playback_status,
            artwork: None,
            track_number: None,
            disc_number: None,
            genre: None,
            year: None,
            url: None,
            thumbnail_url: None,
            media_type: None,
        })
    }

    /// Polls for media session changes and emits events.
    async fn poll_events(
        &self,
        tx: mpsc::Sender<MediaResult<MediaSessionEvent>>,
        debounce_duration: Duration,
    ) {
        let mut last_status: Option<PlaybackStatus> = None;
        let mut last_title: Option<String> = None;
        let mut last_position: Option<Duration> = None;
        let mut last_emit_time = tokio::time::Instant::now();

        loop {
            tokio::time::sleep(Duration::from_millis(250)).await;

            // Check if channel is still open
            if tx.is_closed() {
                break;
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

            // Check for session opened
            if last_status.is_none() {
                let _ = tx
                    .send(Ok(MediaSessionEvent::SessionOpened {
                        app_name: "Windows Media Session".to_string(),
                    }))
                    .await;
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
                continue;
            }

            // Check for position changes
            if let Some(pos) = info.position {
                let should_emit = last_position.map_or(true, |lp| {
                    (pos.as_secs() as i64 - lp.as_secs() as i64).abs() > 1
                });
                if should_emit {
                    let old_position = last_position;
                    last_position = Some(pos);
                    let _ = tx
                        .send(Ok(MediaSessionEvent::PositionChanged {
                            position: pos,
                            old_position,
                        }))
                        .await;
                    last_emit_time = now;
                }
            }
        }
    }
}

#[async_trait::async_trait]
impl MediaSessionBackend for WindowsBackend {
    fn platform_name(&self) -> &'static str {
        "windows"
    }

    async fn get_current(&self) -> MediaResult<Option<MediaInfo>> {
        let this = self.clone();
        spawn_blocking(move || match this.get_session_blocking()? {
            Some(session) => Ok(Some(Self::extract_info(&session)?)),
            None => Ok(None),
        })
        .await
        .map_err(|e| MediaError::Backend {
            platform: "windows".to_string(),
            message: format!("spawn_blocking failed: {e:?}"),
        })?
    }

    async fn get_artwork(&self) -> MediaResult<Option<Vec<u8>>> {
        // Artwork not directly available via WinRT SMTC API
        Ok(None)
    }

    fn get_active_app(&self) -> MediaResult<Option<String>> {
        Ok(Some("Windows Media Session".to_string()))
    }

    async fn play(&self) -> MediaResult<()> {
        let this = self.clone();
        spawn_blocking(move || {
            let session = this.get_session_blocking()?.ok_or(MediaError::NoSession)?;
            session
                .TryPlayAsync()
                .map_err(|e| MediaError::Backend {
                    platform: "windows".to_string(),
                    message: format!("Play failed: {e:?}"),
                })?
                .get()
                .map_err(|e| MediaError::Backend {
                    platform: "windows".to_string(),
                    message: format!("Play await failed: {e:?}"),
                })?;
            Ok(())
        })
        .await
        .map_err(|e| MediaError::Backend {
            platform: "windows".to_string(),
            message: format!("spawn_blocking failed: {e:?}"),
        })??;
        Ok(())
    }

    async fn pause(&self) -> MediaResult<()> {
        let this = self.clone();
        spawn_blocking(move || {
            let session = this.get_session_blocking()?.ok_or(MediaError::NoSession)?;
            session
                .TryPauseAsync()
                .map_err(|e| MediaError::Backend {
                    platform: "windows".to_string(),
                    message: format!("Pause failed: {e:?}"),
                })?
                .get()
                .map_err(|e| MediaError::Backend {
                    platform: "windows".to_string(),
                    message: format!("Pause await failed: {e:?}"),
                })?;
            Ok(())
        })
        .await
        .map_err(|e| MediaError::Backend {
            platform: "windows".to_string(),
            message: format!("spawn_blocking failed: {e:?}"),
        })??;
        Ok(())
    }

    async fn play_pause(&self) -> MediaResult<()> {
        let this = self.clone();
        spawn_blocking(move || {
            let session = this.get_session_blocking()?.ok_or(MediaError::NoSession)?;
            session
                .TryTogglePlayPauseAsync()
                .map_err(|e| MediaError::Backend {
                    platform: "windows".to_string(),
                    message: format!("TogglePlayPause failed: {e:?}"),
                })?
                .get()
                .map_err(|e| MediaError::Backend {
                    platform: "windows".to_string(),
                    message: format!("TogglePlayPause await failed: {e:?}"),
                })?;
            Ok(())
        })
        .await
        .map_err(|e| MediaError::Backend {
            platform: "windows".to_string(),
            message: format!("spawn_blocking failed: {e:?}"),
        })??;
        Ok(())
    }

    async fn stop(&self) -> MediaResult<()> {
        let this = self.clone();
        spawn_blocking(move || {
            let session = this.get_session_blocking()?.ok_or(MediaError::NoSession)?;
            session
                .TryStopAsync()
                .map_err(|e| MediaError::Backend {
                    platform: "windows".to_string(),
                    message: format!("Stop failed: {e:?}"),
                })?
                .get()
                .map_err(|e| MediaError::Backend {
                    platform: "windows".to_string(),
                    message: format!("Stop await failed: {e:?}"),
                })?;
            Ok(())
        })
        .await
        .map_err(|e| MediaError::Backend {
            platform: "windows".to_string(),
            message: format!("spawn_blocking failed: {e:?}"),
        })??;
        Ok(())
    }

    async fn next(&self) -> MediaResult<()> {
        let this = self.clone();
        spawn_blocking(move || {
            let session = this.get_session_blocking()?.ok_or(MediaError::NoSession)?;
            session
                .TrySkipNextAsync()
                .map_err(|e| MediaError::Backend {
                    platform: "windows".to_string(),
                    message: format!("SkipNext failed: {e:?}"),
                })?
                .get()
                .map_err(|e| MediaError::Backend {
                    platform: "windows".to_string(),
                    message: format!("SkipNext await failed: {e:?}"),
                })?;
            Ok(())
        })
        .await
        .map_err(|e| MediaError::Backend {
            platform: "windows".to_string(),
            message: format!("spawn_blocking failed: {e:?}"),
        })??;
        Ok(())
    }

    async fn previous(&self) -> MediaResult<()> {
        let this = self.clone();
        spawn_blocking(move || {
            let session = this.get_session_blocking()?.ok_or(MediaError::NoSession)?;
            session
                .TrySkipPreviousAsync()
                .map_err(|e| MediaError::Backend {
                    platform: "windows".to_string(),
                    message: format!("SkipPrevious failed: {e:?}"),
                })?
                .get()
                .map_err(|e| MediaError::Backend {
                    platform: "windows".to_string(),
                    message: format!("SkipPrevious await failed: {e:?}"),
                })?;
            Ok(())
        })
        .await
        .map_err(|e| MediaError::Backend {
            platform: "windows".to_string(),
            message: format!("spawn_blocking failed: {e:?}"),
        })??;
        Ok(())
    }

    async fn seek(&self, position: Duration) -> MediaResult<()> {
        let this = self.clone();
        let ticks = (position.as_secs() * 10_000_000) as i64;
        spawn_blocking(move || {
            let session = this.get_session_blocking()?.ok_or(MediaError::NoSession)?;
            session
                .TryChangePlaybackPositionAsync(ticks)
                .map_err(|e| MediaError::Backend {
                    platform: "windows".to_string(),
                    message: format!("Seek failed: {e:?}"),
                })?
                .get()
                .map_err(|e| MediaError::Backend {
                    platform: "windows".to_string(),
                    message: format!("Seek await failed: {e:?}"),
                })?;
            Ok(())
        })
        .await
        .map_err(|e| MediaError::Backend {
            platform: "windows".to_string(),
            message: format!("spawn_blocking failed: {e:?}"),
        })??;
        Ok(())
    }

    async fn set_volume(&self, _volume: f64) -> MediaResult<()> {
        Err(MediaError::Backend {
            platform: "windows".to_string(),
            message: "Volume control not supported on Windows SMTC".to_string(),
        })
    }

    async fn set_repeat_mode(&self, _mode: RepeatMode) -> MediaResult<()> {
        Err(MediaError::Backend {
            platform: "windows".to_string(),
            message: "Repeat mode not supported on Windows SMTC".to_string(),
        })
    }

    async fn set_shuffle(&self, _enabled: bool) -> MediaResult<()> {
        Err(MediaError::Backend {
            platform: "windows".to_string(),
            message: "Shuffle control not supported on Windows SMTC".to_string(),
        })
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
    #[ignore]
    fn test_backend_creation() {
        let result = WindowsBackend::new();
        println!("Windows backend: {result:?}");
    }

    #[test]
    fn test_platform_name() {
        let backend = WindowsBackend {
            manager: Arc::new(RwLock::new(None)),
            session: Arc::new(RwLock::new(None)),
        };
        assert_eq!(backend.platform_name(), "windows");
    }
}
