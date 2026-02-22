//! Core backend trait for platform-specific implementations.
//!
//! This module defines the `MediaSessionBackend` trait that all platform
//! backends must implement, along with the factory function for creating
//! the appropriate backend at runtime.

use std::time::Duration;

use tokio::sync::mpsc;

use crate::error::{MediaError, MediaResult};
use crate::media_info::MediaInfo;
use crate::media_sessions::{MediaSessionEvent, RepeatMode};

/// Trait defining the interface for platform-specific media session backends.
///
/// This trait is implemented by each platform's backend module and provides
/// the abstraction layer that allows the public API to be platform-agnostic.
///
/// # Implementation Notes
///
/// - All methods are async to support non-blocking I/O
/// - Implementations should be `Send + Sync` for thread safety
/// - Errors should be converted to [`MediaError`] variants
///
/// # Safety
///
/// Implementations may use unsafe FFI code internally, but the trait
/// interface itself is safe Rust.
#[async_trait::async_trait]
pub trait MediaSessionBackend: Send + Sync {
    /// Returns the platform name for this backend.
    fn platform_name(&self) -> &'static str;

    /// Gets the current media session information.
    ///
    /// # Errors
    ///
    /// Returns [`MediaError::NoSession`] if no session is active.
    /// Returns [`MediaError::Backend`] if the query fails.
    async fn get_current(&self) -> MediaResult<MediaInfo>;

    /// Gets the artwork for the current session.
    ///
    /// Returns `Ok(None)` if artwork is not available.
    ///
    /// # Errors
    ///
    /// Returns [`MediaError::Backend`] if fetching fails.
    async fn get_artwork(&self) -> MediaResult<Option<Vec<u8>>>;

    /// Gets the active application name.
    ///
    /// # Errors
    ///
    /// Returns [`MediaError::Backend`] if the query fails.
    fn get_active_app(&self) -> MediaResult<Option<String>>;

    /// Starts playback.
    ///
    /// # Errors
    ///
    /// Returns [`MediaError::NoSession`] if no session exists.
    /// Returns [`MediaError::Backend`] if the command fails.
    async fn play(&self) -> MediaResult<()>;

    /// Pauses playback.
    ///
    /// # Errors
    ///
    /// Returns [`MediaError::NoSession`] if no session exists.
    /// Returns [`MediaError::Backend`] if the command fails.
    async fn pause(&self) -> MediaResult<()>;

    /// Toggles play/pause state.
    ///
    /// # Errors
    ///
    /// Returns [`MediaError::NoSession`] if no session exists.
    /// Returns [`MediaError::Backend`] if the command fails.
    async fn play_pause(&self) -> MediaResult<()>;

    /// Stops playback.
    ///
    /// # Errors
    ///
    /// Returns [`MediaError::NoSession`] if no session exists.
    /// Returns [`MediaError::Backend`] if the command fails.
    async fn stop(&self) -> MediaResult<()>;

    /// Skips to next track.
    ///
    /// # Errors
    ///
    /// Returns [`MediaError::NoSession`] if no session exists.
    /// Returns [`MediaError::Backend`] if the command fails.
    async fn next(&self) -> MediaResult<()>;

    /// Skips to previous track.
    ///
    /// # Errors
    ///
    /// Returns [`MediaError::NoSession`] if no session exists.
    /// Returns [`MediaError::Backend`] if the command fails.
    async fn previous(&self) -> MediaResult<()>;

    /// Seeks to the specified position.
    ///
    /// # Errors
    ///
    /// Returns [`MediaError::NoSession`] if no session exists.
    /// Returns [`MediaError::Backend`] if the command fails.
    async fn seek(&self, position: Duration) -> MediaResult<()>;

    /// Sets the volume level (0.0 to 1.0).
    ///
    /// # Errors
    ///
    /// Returns [`MediaError::Backend`] if the command fails.
    async fn set_volume(&self, volume: f64) -> MediaResult<()>;

    /// Sets the repeat mode.
    ///
    /// # Errors
    ///
    /// Returns [`MediaError::Backend`] if the command fails.
    async fn set_repeat_mode(&self, mode: RepeatMode) -> MediaResult<()>;

    /// Sets shuffle mode.
    ///
    /// # Errors
    ///
    /// Returns [`MediaError::Backend`] if the command fails.
    async fn set_shuffle(&self, enabled: bool) -> MediaResult<()>;

    /// Starts listening for media session events.
    ///
    /// # Errors
    ///
    /// Returns [`MediaError::Backend`] if event listening cannot be started.
    async fn start_listening(
        &self,
        tx: mpsc::Sender<MediaResult<MediaSessionEvent>>,
        debounce_duration: Duration,
    ) -> MediaResult<()>;
}

/// Creates the appropriate backend for the current platform.
///
/// This factory function selects and instantiates the correct
/// platform-specific backend implementation at runtime.
///
/// # Errors
///
/// Returns [`MediaError::NotSupported`] if the current platform
/// is not supported by this crate.
pub fn create_backend() -> MediaResult<Box<dyn MediaSessionBackend>> {
    #[cfg(target_os = "windows")]
    {
        return Ok(Box::new(
            crate::platform::windows_backend::WindowsBackend::new()?,
        ));
    }

    #[cfg(target_os = "macos")]
    {
        return Ok(Box::new(
            crate::platform::macos_backend::MacOSBackend::new()?,
        ));
    }

    #[cfg(target_os = "linux")]
    {
        return Ok(Box::new(
            crate::platform::linux_backend::LinuxBackend::new()?,
        ));
    }

    #[allow(unreachable_code)]
    Err(MediaError::NotSupported(
        std::env::consts::OS.to_string(),
    ))
}

/// Helper for debouncing rapid events.
#[derive(Clone)]
pub struct Debouncer {
    duration: Duration,
    last_emit: std::sync::Arc<tokio::sync::Mutex<Option<tokio::time::Instant>>>,
}

impl Debouncer {
    /// Creates a new debouncer with the specified duration.
    #[must_use]
    pub fn new(duration: Duration) -> Self {
        Self {
            duration,
            last_emit: std::sync::Arc::new(tokio::sync::Mutex::new(None)),
        }
    }

    /// Attempts to emit an event, returning false if debounced.
    pub async fn should_emit(&self) -> bool {
        let now = tokio::time::Instant::now();
        let mut last = self.last_emit.lock().await;

        match *last {
            Some(prev) if now.duration_since(prev) < self.duration => false,
            _ => {
                *last = Some(now);
                true
            }
        }
    }
}
