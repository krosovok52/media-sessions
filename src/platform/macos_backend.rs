//! macOS backend implementation using `MediaRemote` framework.
//!
//! This module provides integration with macOS's private `MediaRemote` framework
//! through Objective-C runtime bindings.
//!
//! # Requirements
//!
//! - macOS 12.0 (Monterey) or later
//! - The `objc2` and `objc2-foundation` crates
//!
//! # Notes
//!
//! `MediaRemote` is a private framework. This implementation uses FFI to access
//! its symbols. Some features may require Accessibility permissions.

use std::sync::Arc;
use std::time::Duration;

use tokio::sync::{RwLock, mpsc};

use super::backend::MediaSessionBackend;
use crate::error::{MediaError, MediaResult};
use crate::media_info::{MediaInfo, PlaybackStatus};
use crate::media_sessions::{MediaSessionEvent, RepeatMode};

/// macOS `MediaRemote` backend.
#[derive(Clone, Debug)]
pub struct MacOSBackend {
    #[allow(dead_code)]
    initialized: bool,
    last_info: Arc<RwLock<Option<MediaInfo>>>,
}

impl MacOSBackend {
    /// Creates a new macOS backend instance.
    pub fn new() -> MediaResult<Self> {
        Ok(Self {
            initialized: true,
            last_info: Arc::new(RwLock::new(None)),
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
        let mut last_emit_time = tokio::time::Instant::now();

        loop {
            tokio::time::sleep(Duration::from_millis(500)).await;

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
impl MediaSessionBackend for MacOSBackend {
    fn platform_name(&self) -> &'static str {
        "macos"
    }

    async fn get_current(&self) -> MediaResult<Option<MediaInfo>> {
        // Note: Full MediaRemote integration requires FFI bindings to private symbols.
        // This is a placeholder implementation.
        Ok(None)
    }

    async fn get_artwork(&self) -> MediaResult<Option<Vec<u8>>> {
        Ok(None)
    }

    fn get_active_app(&self) -> MediaResult<Option<String>> {
        Ok(None)
    }

    async fn play(&self) -> MediaResult<()> {
        Err(MediaError::NoSession)
    }

    async fn pause(&self) -> MediaResult<()> {
        Err(MediaError::NoSession)
    }

    async fn play_pause(&self) -> MediaResult<()> {
        Err(MediaError::NoSession)
    }

    async fn stop(&self) -> MediaResult<()> {
        Err(MediaError::NoSession)
    }

    async fn next(&self) -> MediaResult<()> {
        Err(MediaError::NoSession)
    }

    async fn previous(&self) -> MediaResult<()> {
        Err(MediaError::NoSession)
    }

    async fn seek(&self, _position: Duration) -> MediaResult<()> {
        Err(MediaError::NoSession)
    }

    async fn set_volume(&self, _volume: f64) -> MediaResult<()> {
        Err(MediaError::Backend {
            platform: "macos".to_string(),
            message: "Volume control requires MediaRemote FFI".to_string(),
        })
    }

    async fn set_repeat_mode(&self, _mode: RepeatMode) -> MediaResult<()> {
        Err(MediaError::Backend {
            platform: "macos".to_string(),
            message: "Repeat mode requires MediaRemote FFI".to_string(),
        })
    }

    async fn set_shuffle(&self, _enabled: bool) -> MediaResult<()> {
        Err(MediaError::Backend {
            platform: "macos".to_string(),
            message: "Shuffle control requires MediaRemote FFI".to_string(),
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
    fn test_backend_creation() {
        let result = MacOSBackend::new();
        assert!(result.is_ok());
    }

    #[test]
    fn test_platform_name() {
        let backend = MacOSBackend::new().unwrap();
        assert_eq!(backend.platform_name(), "macos");
    }
}
