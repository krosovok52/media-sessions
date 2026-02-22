//! Error types for media-sessions crate.

/// Result type alias for media-sessions operations.
pub type MediaResult<T> = Result<T, MediaError>;

/// Comprehensive error type for all media-sessions operations.
///
/// This enum represents all possible error conditions that can occur
/// when interacting with platform-specific media session backends.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum MediaError {
    /// The current platform is not supported by this library.
    #[error("platform not supported: {0}")]
    NotSupported(String),

    /// No active media session was found.
    #[error("no active media session found")]
    NoSession,

    /// Backend-specific error from the native OS integration layer.
    #[error("backend error on {platform}: {message}")]
    Backend {
        /// The platform where the error occurred.
        platform: String,
        /// The underlying error message from the native backend.
        message: String,
    },

    /// D-Bus communication error (Linux only).
    #[error("D-Bus error: {0}")]
    DBusError(String),

    /// COM/WinRT initialization error (Windows only).
    #[error("COM/WinRT initialization failed: HRESULT 0x{0:08X}")]
    ComError(u32),

    /// Objective-C runtime error (macOS only).
    #[error("Objective-C runtime error: {0}")]
    ObjCError(String),

    /// Async operation timeout.
    #[error("operation timed out after {0:?}")]
    Timeout(std::time::Duration),

    /// Invalid artwork data.
    #[error("invalid artwork data: {0}")]
    InvalidArtwork(String),

    /// Seek position out of range.
    #[error("seek position {requested:?} is out of range (track duration: {duration:?})")]
    SeekOutOfRange {
        /// The requested seek position.
        requested: std::time::Duration,
        /// The total duration of the track.
        duration: std::time::Duration,
    },

    /// Permission denied by the operating system.
    #[error("permission denied: {0}")]
    PermissionDenied(String),
}

impl MediaError {
    /// Returns the platform name associated with this error, if applicable.
    #[must_use]
    pub fn platform(&self) -> &str {
        match self {
            Self::NotSupported(p) => p,
            Self::Backend { platform, .. } => platform,
            Self::DBusError(_) => "linux",
            Self::ComError(_) => "windows",
            Self::ObjCError(_) => "macos",
            _ => "unknown",
        }
    }

    /// Returns the HRESULT code if this is a Windows COM error.
    #[must_use]
    pub const fn hresult(&self) -> Option<u32> {
        match self {
            Self::ComError(code) => Some(*code),
            _ => None,
        }
    }

    /// Returns true if this error indicates a transient condition.
    #[must_use]
    pub const fn is_retryable(&self) -> bool {
        matches!(
            self,
            Self::Timeout(_) | Self::DBusError(_) | Self::ComError(_) | Self::Backend { .. }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = MediaError::NotSupported("freebsd".to_string());
        assert_eq!(format!("{err}"), "platform not supported: freebsd");
    }

    #[test]
    fn test_error_platform() {
        let err = MediaError::ComError(0x8000_4005);
        assert_eq!(err.platform(), "windows");
    }

    #[test]
    fn test_error_hresult() {
        let err = MediaError::ComError(0x8001_010E);
        assert_eq!(err.hresult(), Some(0x8001_010E));
    }

    #[test]
    fn test_error_retryable() {
        let err = MediaError::Timeout(std::time::Duration::from_secs(5));
        assert!(err.is_retryable());

        let err = MediaError::NotSupported("test".to_string());
        assert!(!err.is_retryable());
    }
}
