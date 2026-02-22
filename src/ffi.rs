//! C API for media-sessions crate.
//!
//! This module provides a C-compatible FFI interface for using media-sessions
//! from other programming languages (Python, C#, Node.js, etc.).
//!
//! # Building
//!
//! ```bash
//! cargo build --release --features c-api
//! ```
//!
//! This produces a dynamic library:
//! - Windows: `media_sessions_c.dll`
//! - Linux: `libmedia_sessions_c.so`
//! - macOS: `libmedia_sessions_c.dylib`
//!
//! # Usage
//!
//! See the `c-api/` directory for examples in various languages.

use std::ffi::{CString, c_char, c_void};
use std::ptr;
use std::time::Duration;

use tokio::runtime::Runtime;

use crate::media_info::{MediaInfo, PlaybackStatus};
use crate::media_sessions::{MediaSessions, RepeatMode};

/// Opaque handle to a MediaSessions instance.
pub struct MediaSessionsHandle {
    sessions: MediaSessions,
    runtime: Runtime,
}

/// Playback status enum (C-compatible).
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CPlaybackStatus {
    /// Media is currently playing.
    Playing = 0,
    /// Media is currently paused.
    Paused = 1,
    /// Media is stopped.
    Stopped = 2,
    /// Media is in a transitional state.
    Transitioning = 3,
}

impl From<PlaybackStatus> for CPlaybackStatus {
    fn from(status: PlaybackStatus) -> Self {
        match status {
            PlaybackStatus::Playing => Self::Playing,
            PlaybackStatus::Paused => Self::Paused,
            PlaybackStatus::Stopped => Self::Stopped,
            PlaybackStatus::Transitioning => Self::Transitioning,
        }
    }
}

impl From<CPlaybackStatus> for PlaybackStatus {
    fn from(status: CPlaybackStatus) -> Self {
        match status {
            CPlaybackStatus::Playing => PlaybackStatus::Playing,
            CPlaybackStatus::Paused => PlaybackStatus::Paused,
            CPlaybackStatus::Stopped => PlaybackStatus::Stopped,
            CPlaybackStatus::Transitioning => PlaybackStatus::Transitioning,
        }
    }
}

/// Repeat mode enum (C-compatible).
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CRepeatMode {
    /// Repeat is disabled.
    None = 0,
    /// Repeat current track.
    One = 1,
    /// Repeat entire playlist/album.
    All = 2,
}

impl From<RepeatMode> for CRepeatMode {
    fn from(mode: RepeatMode) -> Self {
        match mode {
            RepeatMode::None => Self::None,
            RepeatMode::One => Self::One,
            RepeatMode::All => Self::All,
        }
    }
}

impl From<CRepeatMode> for RepeatMode {
    fn from(mode: CRepeatMode) -> Self {
        match mode {
            CRepeatMode::None => RepeatMode::None,
            CRepeatMode::One => RepeatMode::One,
            CRepeatMode::All => RepeatMode::All,
        }
    }
}

/// Result codes for C API functions.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CResult {
    /// Success.
    Ok = 0,
    /// General error.
    Error = 1,
    /// No active session.
    NoSession = 2,
    /// Platform not supported.
    NotSupported = 3,
    /// Operation timed out.
    Timeout = 4,
    /// Invalid argument.
    InvalidArg = 5,
}

/// Media info struct for C API.
#[repr(C)]
pub struct CMediaInfo {
    /// Track title.
    pub title: *mut c_char,
    /// Track artist.
    pub artist: *mut c_char,
    /// Album name.
    pub album: *mut c_char,
    /// Total duration in seconds.
    pub duration_secs: u64,
    /// Current position in seconds.
    pub position_secs: u64,
    /// Current playback status.
    pub playback_status: CPlaybackStatus,
    /// Whether artwork is available.
    pub has_artwork: bool,
    /// Artwork size in bytes.
    pub artwork_len: usize,
    /// Raw artwork image bytes.
    pub artwork: *mut u8,
    /// Track number within the album.
    pub track_number: u32,
    /// Disc number for multi-disc albums.
    pub disc_number: u32,
    /// Genre classification.
    pub genre: *mut c_char,
    /// Release year.
    pub year: i32,
    /// Source URL.
    pub url: *mut c_char,
    /// Thumbnail URL.
    pub thumbnail_url: *mut c_char,
}

impl Default for CMediaInfo {
    fn default() -> Self {
        Self {
            title: ptr::null_mut(),
            artist: ptr::null_mut(),
            album: ptr::null_mut(),
            duration_secs: 0,
            position_secs: 0,
            playback_status: CPlaybackStatus::Stopped,
            has_artwork: false,
            artwork_len: 0,
            artwork: ptr::null_mut(),
            track_number: 0,
            disc_number: 0,
            genre: ptr::null_mut(),
            year: 0,
            url: ptr::null_mut(),
            thumbnail_url: ptr::null_mut(),
        }
    }
}

/// Convert MediaInfo to CMediaInfo.
fn media_info_to_c(info: MediaInfo) -> CMediaInfo {
    let mut c_info = CMediaInfo::default();

    c_info.title = rust_string_to_c(info.title.unwrap_or_default());
    c_info.artist = rust_string_to_c(info.artist.unwrap_or_default());
    c_info.album = rust_string_to_c(info.album.unwrap_or_default());
    c_info.duration_secs = info.duration.map_or(0, |d| d.as_secs());
    c_info.position_secs = info.position.map_or(0, |p| p.as_secs());
    c_info.playback_status = info.playback_status.into();

    if let Some(artwork) = info.artwork {
        c_info.has_artwork = true;
        c_info.artwork_len = artwork.len();
        c_info.artwork = Box::into_raw(artwork.into_boxed_slice()) as *mut u8;
    }

    c_info.track_number = info.track_number.unwrap_or(0);
    c_info.disc_number = info.disc_number.unwrap_or(0);
    c_info.genre = rust_string_to_c(info.genre.unwrap_or_default());
    c_info.year = info.year.unwrap_or(0);
    c_info.url = rust_string_to_c(info.url.unwrap_or_default());
    c_info.thumbnail_url = rust_string_to_c(info.thumbnail_url.unwrap_or_default());

    c_info
}

/// Helper to convert Rust String to C string.
fn rust_string_to_c(s: String) -> *mut c_char {
    match CString::new(s) {
        Ok(c_string) => c_string.into_raw(),
        Err(_) => ptr::null_mut(),
    }
}

/// Free a C string allocated by this library.
///
/// # Safety
/// The pointer must have been allocated by this library.
#[no_mangle]
pub unsafe extern "C" fn media_sessions_c_free_string(s: *mut c_char) {
    if !s.is_null() {
        drop(CString::from_raw(s));
    }
}

/// Free artwork data allocated by this library.
///
/// # Safety
/// The pointer must have been allocated by this library with matching length.
#[no_mangle]
pub unsafe extern "C" fn media_sessions_c_free_artwork(data: *mut u8, len: usize) {
    if !data.is_null() && len > 0 {
        drop(Box::from_raw(std::slice::from_raw_parts_mut(data, len)));
    }
}

/// Free a CMediaInfo struct and all its allocated fields.
///
/// # Safety
/// The pointer must have been allocated by this library.
#[no_mangle]
pub unsafe extern "C" fn media_sessions_c_free_info(info: *mut CMediaInfo) {
    if info.is_null() {
        return;
    }

    let info = Box::from_raw(info);
    media_sessions_c_free_string(info.title);
    media_sessions_c_free_string(info.artist);
    media_sessions_c_free_string(info.album);
    media_sessions_c_free_string(info.genre);
    media_sessions_c_free_string(info.url);
    media_sessions_c_free_string(info.thumbnail_url);
    media_sessions_c_free_artwork(info.artwork, info.artwork_len);
}

/// Create a new MediaSessions instance.
///
/// Returns a handle that must be freed with `media_sessions_c_free`.
///
/// # Safety
/// The returned handle must be freed when no longer needed.
#[no_mangle]
pub unsafe extern "C" fn media_sessions_c_new() -> *mut MediaSessionsHandle {
    match MediaSessions::new() {
        Ok(sessions) => {
            let handle = Box::new(MediaSessionsHandle {
                sessions,
                runtime: Runtime::new().unwrap(),
            });
            Box::into_raw(handle)
        }
        Err(_) => ptr::null_mut(),
    }
}

/// Create a new MediaSessions instance with custom debounce duration (ms).
///
/// # Safety
/// The returned handle must be freed when no longer needed.
#[no_mangle]
pub unsafe extern "C" fn media_sessions_c_new_with_debounce(
    debounce_ms: u64,
) -> *mut MediaSessionsHandle {
    match MediaSessions::builder()
        .debounce_duration(Duration::from_millis(debounce_ms))
        .build()
    {
        Ok(sessions) => {
            let handle = Box::new(MediaSessionsHandle {
                sessions,
                runtime: Runtime::new().unwrap(),
            });
            Box::into_raw(handle)
        }
        Err(_) => ptr::null_mut(),
    }
}

/// Free a MediaSessions handle.
///
/// # Safety
/// The handle must have been created by this library and not freed already.
#[no_mangle]
pub unsafe extern "C" fn media_sessions_c_free(handle: *mut MediaSessionsHandle) {
    if !handle.is_null() {
        drop(Box::from_raw(handle));
    }
}

/// Get current media information.
///
/// Returns a pointer to CMediaInfo which must be freed with `media_sessions_c_free_info`.
/// Returns NULL if no session is active or on error.
///
/// # Safety
/// The returned info must be freed when no longer needed.
#[no_mangle]
pub unsafe extern "C" fn media_sessions_c_current(
    handle: *mut MediaSessionsHandle,
) -> *mut CMediaInfo {
    if handle.is_null() {
        return ptr::null_mut();
    }

    let handle = &*handle;
    match handle.runtime.block_on(handle.sessions.current()) {
        Ok(Some(info)) => Box::into_raw(Box::new(media_info_to_c(info))),
        _ => ptr::null_mut(),
    }
}

/// Get the active application name.
///
/// Returns a C string that must be freed with `media_sessions_c_free_string`.
///
/// # Safety
/// The returned string must be freed when no longer needed.
#[no_mangle]
pub unsafe extern "C" fn media_sessions_c_active_app(
    handle: *mut MediaSessionsHandle,
) -> *mut c_char {
    if handle.is_null() {
        return ptr::null_mut();
    }

    let handle = &*handle;
    match handle.runtime.block_on(handle.sessions.active_app()) {
        Ok(Some(app)) => rust_string_to_c(app),
        _ => ptr::null_mut(),
    }
}

/// Start or resume playback.
///
/// Returns CResult::Ok on success.
#[no_mangle]
pub extern "C" fn media_sessions_c_play(handle: *mut MediaSessionsHandle) -> CResult {
    if handle.is_null() {
        return CResult::InvalidArg;
    }

    let handle = unsafe { &*handle };
    match handle.runtime.block_on(handle.sessions.play()) {
        Ok(_) => CResult::Ok,
        Err(crate::error::MediaError::NoSession) => CResult::NoSession,
        Err(_) => CResult::Error,
    }
}

/// Pause playback.
///
/// Returns CResult::Ok on success.
#[no_mangle]
pub extern "C" fn media_sessions_c_pause(handle: *mut MediaSessionsHandle) -> CResult {
    if handle.is_null() {
        return CResult::InvalidArg;
    }

    let handle = unsafe { &*handle };
    match handle.runtime.block_on(handle.sessions.pause()) {
        Ok(_) => CResult::Ok,
        Err(crate::error::MediaError::NoSession) => CResult::NoSession,
        Err(_) => CResult::Error,
    }
}

/// Toggle play/pause state.
///
/// Returns CResult::Ok on success.
#[no_mangle]
pub extern "C" fn media_sessions_c_play_pause(handle: *mut MediaSessionsHandle) -> CResult {
    if handle.is_null() {
        return CResult::InvalidArg;
    }

    let handle = unsafe { &*handle };
    match handle.runtime.block_on(handle.sessions.play_pause()) {
        Ok(_) => CResult::Ok,
        Err(crate::error::MediaError::NoSession) => CResult::NoSession,
        Err(_) => CResult::Error,
    }
}

/// Stop playback.
///
/// Returns CResult::Ok on success.
#[no_mangle]
pub extern "C" fn media_sessions_c_stop(handle: *mut MediaSessionsHandle) -> CResult {
    if handle.is_null() {
        return CResult::InvalidArg;
    }

    let handle = unsafe { &*handle };
    match handle.runtime.block_on(handle.sessions.stop()) {
        Ok(_) => CResult::Ok,
        Err(crate::error::MediaError::NoSession) => CResult::NoSession,
        Err(_) => CResult::Error,
    }
}

/// Skip to next track.
///
/// Returns CResult::Ok on success.
#[no_mangle]
pub extern "C" fn media_sessions_c_next(handle: *mut MediaSessionsHandle) -> CResult {
    if handle.is_null() {
        return CResult::InvalidArg;
    }

    let handle = unsafe { &*handle };
    match handle.runtime.block_on(handle.sessions.next()) {
        Ok(_) => CResult::Ok,
        Err(crate::error::MediaError::NoSession) => CResult::NoSession,
        Err(_) => CResult::Error,
    }
}

/// Skip to previous track.
///
/// Returns CResult::Ok on success.
#[no_mangle]
pub extern "C" fn media_sessions_c_previous(handle: *mut MediaSessionsHandle) -> CResult {
    if handle.is_null() {
        return CResult::InvalidArg;
    }

    let handle = unsafe { &*handle };
    match handle.runtime.block_on(handle.sessions.previous()) {
        Ok(_) => CResult::Ok,
        Err(crate::error::MediaError::NoSession) => CResult::NoSession,
        Err(_) => CResult::Error,
    }
}

/// Seek to the specified position (in seconds).
///
/// Returns CResult::Ok on success.
#[no_mangle]
pub extern "C" fn media_sessions_c_seek(
    handle: *mut MediaSessionsHandle,
    position_secs: u64,
) -> CResult {
    if handle.is_null() {
        return CResult::InvalidArg;
    }

    let handle = unsafe { &*handle };
    match handle
        .runtime
        .block_on(handle.sessions.seek(Duration::from_secs(position_secs)))
    {
        Ok(_) => CResult::Ok,
        Err(crate::error::MediaError::NoSession) => CResult::NoSession,
        Err(_) => CResult::Error,
    }
}

/// Set volume level (0.0 to 1.0).
///
/// Returns CResult::Ok on success.
#[no_mangle]
pub extern "C" fn media_sessions_c_set_volume(
    handle: *mut MediaSessionsHandle,
    volume: f64,
) -> CResult {
    if handle.is_null() {
        return CResult::InvalidArg;
    }

    if volume < 0.0 || volume > 1.0 {
        return CResult::InvalidArg;
    }

    let handle = unsafe { &*handle };
    match handle.runtime.block_on(handle.sessions.set_volume(volume)) {
        Ok(_) => CResult::Ok,
        Err(_) => CResult::Error,
    }
}

/// Set repeat mode.
///
/// Returns CResult::Ok on success.
#[no_mangle]
pub extern "C" fn media_sessions_c_set_repeat_mode(
    handle: *mut MediaSessionsHandle,
    mode: CRepeatMode,
) -> CResult {
    if handle.is_null() {
        return CResult::InvalidArg;
    }

    let handle = unsafe { &*handle };
    match handle
        .runtime
        .block_on(handle.sessions.set_repeat_mode(mode.into()))
    {
        Ok(_) => CResult::Ok,
        Err(_) => CResult::Error,
    }
}

/// Set shuffle mode.
///
/// Returns CResult::Ok on success.
#[no_mangle]
pub extern "C" fn media_sessions_c_set_shuffle(
    handle: *mut MediaSessionsHandle,
    enabled: bool,
) -> CResult {
    if handle.is_null() {
        return CResult::InvalidArg;
    }

    let handle = unsafe { &*handle };
    match handle
        .runtime
        .block_on(handle.sessions.set_shuffle(enabled))
    {
        Ok(_) => CResult::Ok,
        Err(_) => CResult::Error,
    }
}

/// Get the library version string.
///
/// Returns a static C string (does not need to be freed).
#[no_mangle]
pub extern "C" fn media_sessions_c_version() -> *const c_char {
    c"0.2.0".as_ptr()
}

/// Callback type for event notifications.
pub type CEventCallback = unsafe extern "C" fn(event_type: i32, data: *const c_void, user_data: *mut c_void);

/// Event types for callbacks.
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CEventType {
    /// Metadata (title, artist, album) has changed.
    MetadataChanged = 0,
    /// Playback status (playing, paused, stopped) has changed.
    PlaybackStatusChanged = 1,
    /// Playback position has changed.
    PositionChanged = 2,
    /// A new media session has become active.
    SessionOpened = 3,
    /// The current media session has closed.
    SessionClosed = 4,
    /// Artwork has been updated.
    ArtworkChanged = 5,
    /// Volume level has changed.
    VolumeChanged = 6,
    /// Repeat/shuffle mode has changed.
    RepeatModeChanged = 7,
}

/// Event callback handle.
pub struct EventCallbackHandle {
    _callback: CEventCallback,
    _user_data: *mut c_void,
}

/// Free an event callback handle.
///
/// # Safety
/// The handle must have been created by this library.
#[no_mangle]
pub unsafe extern "C" fn media_sessions_c_free_callback(handle: *mut EventCallbackHandle) {
    if !handle.is_null() {
        drop(Box::from_raw(handle));
    }
}

/// Get the current platform name.
///
/// Returns a static C string (does not need to be freed).
#[no_mangle]
pub extern "C" fn media_sessions_c_platform() -> *const c_char {
    #[cfg(target_os = "windows")]
    {
        c"windows".as_ptr()
    }

    #[cfg(target_os = "linux")]
    {
        c"linux".as_ptr()
    }

    #[cfg(target_os = "macos")]
    {
        c"macos".as_ptr()
    }

    #[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
    {
        c"unknown".as_ptr()
    }
}

/// Register event callback (placeholder for future implementation).
///
/// Currently returns NULL as event streaming requires async callback support.
///
/// # Safety
/// The callback must be a valid C function pointer.
#[no_mangle]
pub unsafe extern "C" fn media_sessions_c_register_callback(
    _handle: *mut MediaSessionsHandle,
    _callback: CEventCallback,
    user_data: *mut c_void,
) -> *mut EventCallbackHandle {
    // Placeholder - full event callback implementation requires async runtime integration
    // For now, return NULL to indicate this feature is not yet implemented
    let _ = user_data;
    ptr::null_mut()
}

/// Get current platform as a Rust string (for testing).
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_string() {
        unsafe {
            let version = media_sessions_c_version();
            assert!(!version.is_null());
        }
    }

    #[test]
    fn test_platform_string() {
        unsafe {
            let platform = media_sessions_c_platform();
            assert!(!platform.is_null());
        }
    }

    #[test]
    fn test_playback_status_conversion() {
        assert_eq!(
            CPlaybackStatus::from(PlaybackStatus::Playing),
            CPlaybackStatus::Playing
        );
        assert_eq!(
            PlaybackStatus::from(CPlaybackStatus::Paused),
            PlaybackStatus::Paused
        );
    }

    #[test]
    fn test_repeat_mode_conversion() {
        assert_eq!(CRepeatMode::from(RepeatMode::All), CRepeatMode::All);
        assert_eq!(
            RepeatMode::from(CRepeatMode::One),
            RepeatMode::One
        );
    }
}
