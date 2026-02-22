# Rust API Overview

The Rust API for Media Sessions provides a type-safe, async-first interface for controlling media players.

## Core Types

| Type | Description |
|------|-------------|
| **[MediaSessions](media-sessions.md)** | Main class for controlling media sessions |
| **[MediaInfo](media-info.md)** | Track metadata (title, artist, album, etc.) |
| **[PlaybackStatus](playback-status.md)** | Playback state (Playing, Paused, Stopped) |
| **[RepeatMode](repeat-mode.md)** | Repeat mode (None, One, All) |
| **[MediaSessionEvent](events.md)** | Event types for the event stream |

## Quick Example

```rust
use media_sessions::{MediaSessions, PlaybackStatus};
use futures::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sessions = MediaSessions::new()?;

    // Get current track
    if let Some(info) = sessions.current().await? {
        println!("🎵 {} - {}", info.artist(), info.title());
        println!("💿 {}", info.album());
        println!("▶️ Status: {:?}", info.playback_status);
    }

    // Control playback
    sessions.play().await?;
    sessions.seek(std::time::Duration::from_secs(30)).await?;

    // Subscribe to events
    let mut stream = sessions.watch().await?;
    while let Some(event) = stream.next().await {
        println!("📡 Event: {:?}", event?);
    }

    Ok(())
}
```

## Installation

```toml
[dependencies]
media-sessions = "0.2"
tokio = { version = "1", features = ["full"] }
futures = "0.3"
```

## Feature Flags

| Feature | Description |
|---------|-------------|
| `default` | All platforms |
| `windows` | Windows only |
| `macos` | macOS only |
| `linux` | Linux only |
| `tracing` | Tracing support |
| `serde` | Serialization |
| `c-api` | C FFI |

## Platform Support

| Platform | Status | Backend |
|----------|--------|---------|
| **Windows** | ✅ Stable | WinRT SMTC |
| **macOS** | 🟡 Beta | MediaRemote |
| **Linux** | ✅ Stable | MPRIS/D-Bus |

## See Also

- **[MediaSessions](media-sessions.md)** — Main class
- **[C API](../c-api.md)** — Using from other languages
- **[Platforms](../platforms/windows.md)** — Platform-specific details
