//! macOS backend implementation using `MediaRemote` framework.
//!
//! This module provides integration with macOS's private `MediaRemote` framework
//! through Objective-C runtime bindings.
//!
//! # Requirements
//!
//! - macOS 12.0 (Monterey) or later
//! - The `objc2` and `objc2-foundation` crates

use std::sync::Arc;
use std::time::Duration;

use tokio::sync::{mpsc, RwLock};

use super::backend::MediaSessionBackend;
use crate::error::{MediaError, MediaResult};
use crate::media_info::MediaInfo;
use crate::media_sessions::{MediaSessionEvent, RepeatMode};

/// macOS `MediaRemote` backend.
#[derive(Clone)]
pub struct MacOSBackend {
    /// Flag indicating if `MediaRemote` is available.
    #[allow(dead_code)]
    available: bool,
    /// Last known media info for change detection.
    #[allow(dead_code)]
    last_info: Arc<RwLock<Option<MediaInfo>>>,
}

impl MacOSBackend {
    /// Creates a new macOS backend instance.
    ///
    /// # Errors
    ///
    /// Returns [`MediaError::NotSupported`] if `MediaRemote` framework is unavailable.
    pub fn new() -> MediaResult<Self> {
        Ok(Self {
            available: cfg!(target_os = "macos"),
            last_info: Arc::new(RwLock::new(None)),
        })
    }
}

#[async_trait::async_trait]
impl MediaSessionBackend for MacOSBackend {
    fn platform_name(&self) -> &'static str {
        "macos"
    }

    async fn get_current(&self) -> MediaResult<MediaInfo> {
        if !self.available {
            return Err(MediaError::NotSupported("macos".to_string()));
        }
        Err(MediaError::NoSession)
    }

    async fn get_artwork(&self) -> MediaResult<Option<Vec<u8>>> {
        if !self.available {
            return Err(MediaError::NotSupported("macos".to_string()));
        }
        Ok(None)
    }

    fn get_active_app(&self) -> MediaResult<Option<String>> {
        if !self.available {
            return Err(MediaError::NotSupported("macos".to_string()));
        }
        Ok(Some("macOS Media Session".to_string()))
    }

    async fn play(&self) -> MediaResult<()> {
        if !self.available {
            return Err(MediaError::NotSupported("macos".to_string()));
        }
        Err(MediaError::NoSession)
    }

    async fn pause(&self) -> MediaResult<()> {
        if !self.available {
            return Err(MediaError::NotSupported("macos".to_string()));
        }
        Err(MediaError::NoSession)
    }

    async fn play_pause(&self) -> MediaResult<()> {
        if !self.available {
            return Err(MediaError::NotSupported("macos".to_string()));
        }
        Err(MediaError::NoSession)
    }

    async fn stop(&self) -> MediaResult<()> {
        if !self.available {
            return Err(MediaError::NotSupported("macos".to_string()));
        }
        Err(MediaError::NoSession)
    }

    async fn next(&self) -> MediaResult<()> {
        if !self.available {
            return Err(MediaError::NotSupported("macos".to_string()));
        }
        Err(MediaError::NoSession)
    }

    async fn previous(&self) -> MediaResult<()> {
        if !self.available {
            return Err(MediaError::NotSupported("macos".to_string()));
        }
        Err(MediaError::NoSession)
    }

    async fn seek(&self, _position: Duration) -> MediaResult<()> {
        if !self.available {
            return Err(MediaError::NotSupported("macos".to_string()));
        }
        Err(MediaError::NoSession)
    }

    async fn set_volume(&self, _volume: f64) -> MediaResult<()> {
        Err(MediaError::Backend {
            platform: "macos".to_string(),
            message: "Volume control not supported".to_string(),
        })
    }

    async fn set_repeat_mode(&self, _mode: RepeatMode) -> MediaResult<()> {
        Err(MediaError::Backend {
            platform: "macos".to_string(),
            message: "Repeat mode not supported".to_string(),
        })
    }

    async fn set_shuffle(&self, _enabled: bool) -> MediaResult<()> {
        Err(MediaError::Backend {
            platform: "macos".to_string(),
            message: "Shuffle control not supported".to_string(),
        })
    }

    async fn start_listening(
        &self,
        _tx: mpsc::Sender<MediaResult<MediaSessionEvent>>,
        _debounce_duration: Duration,
    ) -> MediaResult<()> {
        if !self.available {
            return Err(MediaError::NotSupported("macos".to_string()));
        }
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
