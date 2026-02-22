//! Platform-specific backend implementations.
//!
//! This module contains the abstraction layer for platform-specific
//! media session implementations. Each supported OS has its own backend
//! that implements the common `MediaSessionBackend` trait.
//!
//! # Architecture
//!
//! The backend system uses dynamic dispatch to select the appropriate
//! implementation at runtime based on the target OS:
//!
//! - **Windows:** `windows_backend::WindowsBackend` using `WinRT`
//! - **macOS:** `macos_backend::MacOSBackend` using `MediaRemote` framework
//! - **Linux:** `linux_backend::LinuxBackend` using D-Bus/MPRIS
//!
//! # Safety
//!
//! All unsafe code is isolated within these backend modules. The public
//! API exposed by this crate is 100% safe Rust.

#[cfg(target_os = "windows")]
#[cfg_attr(docsrs, doc(cfg(target_os = "windows")))]
pub mod windows_backend;

#[cfg(target_os = "macos")]
#[cfg_attr(docsrs, doc(cfg(target_os = "macos")))]
pub mod macos_backend;

#[cfg(target_os = "linux")]
#[cfg_attr(docsrs, doc(cfg(target_os = "linux")))]
pub mod linux_backend;

pub mod backend;

pub use backend::{MediaSessionBackend, create_backend};

/// Get the list of available platform backends.
///
/// This returns the platforms that are compiled in (based on feature flags),
/// not necessarily the platforms that are available at runtime.
#[must_use]
pub fn available_platforms() -> Vec<&'static str> {
    let mut platforms = Vec::with_capacity(3);

    #[cfg(target_os = "windows")]
    platforms.push("windows");

    #[cfg(target_os = "linux")]
    platforms.push("linux");

    #[cfg(target_os = "macos")]
    platforms.push("macos");

    platforms
}
