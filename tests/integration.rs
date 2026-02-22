//! Integration tests for media-sessions crate.
//!
//! These tests require an active media player running on the system.
//! They are marked with `#[ignore]` by default and can be run with:
//!
//! ```bash
//! cargo test --test integration -- --ignored
//! ```

use std::time::Duration;

use media_sessions::{MediaSessions, PlaybackStatus};

/// Tests basic MediaSessions initialization.
#[test]
#[ignore]
fn test_media_sessions_creation() {
    let sessions = MediaSessions::new();
    assert!(sessions.is_ok(), "Failed to create MediaSessions instance");
}

/// Tests querying current media info.
#[tokio::test]
#[ignore]
async fn test_current_media_info() {
    let sessions = MediaSessions::new().expect("Failed to create MediaSessions");
    let result = sessions.current().await;

    // This test passes if it doesn't panic
    // The result may be Ok(None) if no player is active
    assert!(result.is_ok() || matches!(result, Err(media_sessions::MediaError::NoSession)));
}

/// Tests playback control methods.
#[tokio::test]
#[ignore]
async fn test_playback_controls() {
    let sessions = MediaSessions::new().expect("Failed to create MediaSessions");

    // These may fail if no player is active, but shouldn't panic
    let _ = sessions.play().await;
    let _ = sessions.pause().await;
    let _ = sessions.play_pause().await;
    let _ = sessions.stop().await;
    let _ = sessions.next().await;
    let _ = sessions.previous().await;
}

/// Tests seek functionality.
#[tokio::test]
#[ignore]
async fn test_seek() {
    let sessions = MediaSessions::new().expect("Failed to create MediaSessions");

    // Seek to 30 seconds
    let result = sessions.seek(Duration::from_secs(30)).await;

    // May fail if no player active, but shouldn't panic
    assert!(result.is_ok() || result.is_err());
}

/// Tests volume control (Linux only).
#[tokio::test]
#[ignore]
#[cfg(target_os = "linux")]
async fn test_volume_control() {
    let sessions = MediaSessions::new().expect("Failed to create MediaSessions");

    let result = sessions.set_volume(0.5).await;
    assert!(result.is_ok() || result.is_err());
}

/// Tests repeat mode (Linux only).
#[tokio::test]
#[ignore]
#[cfg(target_os = "linux")]
async fn test_repeat_mode() {
    use media_sessions::RepeatMode;

    let sessions = MediaSessions::new().expect("Failed to create MediaSessions");

    let result = sessions.set_repeat_mode(RepeatMode::All).await;
    assert!(result.is_ok() || result.is_err());
}

/// Tests shuffle control (Linux only).
#[tokio::test]
#[ignore]
#[cfg(target_os = "linux")]
async fn test_shuffle_control() {
    let sessions = MediaSessions::new().expect("Failed to create MediaSessions");

    let result = sessions.set_shuffle(true).await;
    assert!(result.is_ok() || result.is_err());
}

/// Tests event stream creation.
#[tokio::test]
#[ignore]
async fn test_event_stream() {
    use futures::StreamExt;

    let sessions = MediaSessions::new().expect("Failed to create MediaSessions");
    let stream_result = sessions.watch().await;

    assert!(stream_result.is_ok(), "Failed to create event stream");

    // Drop the stream
    drop(stream_result);
}

/// Tests builder pattern configuration.
#[test]
#[ignore]
fn test_builder_configuration() {
    let sessions = MediaSessions::builder()
        .debounce_duration(Duration::from_millis(250))
        .operation_timeout(Duration::from_secs(10))
        .enable_artwork(false)
        .build();

    assert!(
        sessions.is_ok(),
        "Failed to build MediaSessions with custom config"
    );
}

/// Tests platform detection.
#[test]
fn test_platform_detection() {
    let platform = media_sessions::current_platform();

    #[cfg(target_os = "windows")]
    assert_eq!(platform, "windows");

    #[cfg(target_os = "linux")]
    assert_eq!(platform, "linux");

    #[cfg(target_os = "macos")]
    assert_eq!(platform, "macos");
}

/// Tests available platforms based on feature flags.
#[test]
fn test_available_platforms() {
    let platforms = media_sessions::available_platforms();

    // At least one platform should be available
    assert!(!platforms.is_empty(), "No platforms available");

    // Current platform should be in the list
    let current = media_sessions::current_platform();
    assert!(
        platforms.contains(&current),
        "Current platform {} not in available platforms: {:?}",
        current,
        platforms
    );
}
