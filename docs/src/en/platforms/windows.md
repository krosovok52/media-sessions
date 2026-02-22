# Windows

Media session control on Windows via WinRT SMTC API.

## Overview

On Windows, the library uses `Windows.Media.Control` WinRT API to interact with system media sessions.

## Requirements

| Requirement | Version | Description |
|-------------|---------|-------------|
| **Windows** | 10 1803+ | Minimum version with SMTC API |
| **Rust** | 1.80+ | Minimum Supported Rust Version |
| **CMake** | 3.1+ | For building windows-rs dependencies |

## Installation

```toml
[dependencies]
media-sessions = { version = "0.2", features = ["windows"] }
tokio = { version = "1", features = ["full"] }
```

Or Windows only:

```toml
[dependencies]
media-sessions = { version = "0.2", default-features = false, features = ["windows"] }
```

## Supported Players

### ✅ Full Support

| Application | Version | Notes |
|-------------|---------|-------|
| **Spotify UWP** | Any | From Microsoft Store |
| **YouTube** | Any browser | Edge, Chrome, Firefox |
| **Yandex.Music** | Any browser | In browser |
| **SoundCloud** | Any browser | In browser |
| **Twitch** | Any browser | In browser |

### ⚠️ Partial Support

| Application | Version | Notes |
|-------------|---------|-------|
| **Spotify Desktop** | Any | Need to enable in settings |
| **VLC** | 3.0+ | Enable "Show media info" |
| **foobar2000** | 1.6+ | Requires component |
| **AIMP** | 4.70+ | Enable in settings |

### ❌ Not Supported

| Application | Reason |
|-------------|--------|
| **Winamp** | Doesn't use SMTC |
| **Old players** | Pre-Windows 10 1803 |

## Player Setup

### Spotify Desktop

1. Open Spotify settings
2. Scroll to "Show Advanced Settings"
3. Enable "Show media information when playing"

### VLC

1. Tools → Preferences
2. Player tab
3. Enable "Show media info on taskbar"

### foobar2000

1. Install `foo_uie_albumlist` component
2. Preferences → Advanced → Tools → Enable media keys

## Examples

### Basic Usage

```rust
use media_sessions::MediaSessions;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sessions = MediaSessions::new()?;

    if let Some(info) = sessions.current().await? {
        println!("🎵 {} - {}", info.artist(), info.title());
    }

    Ok(())
}
```

### Event Monitoring

```rust
use media_sessions::{MediaSessions, MediaSessionEvent};
use futures::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sessions = MediaSessions::new()?;
    let mut stream = sessions.watch().await?;

    while let Some(event) = stream.next().await {
        match event? {
            MediaSessionEvent::MetadataChanged(info) => {
                println!("🎵 Now: {}", info.display_string());
            }
            MediaSessionEvent::PlaybackStatusChanged(status) => {
                println!("▶️ Status: {:?}", status);
            }
            _ => {}
        }
    }

    Ok(())
}
```

## SMTC API Limitations

### Not Supported

| Feature | Reason |
|---------|--------|
| **Volume control** | SMTC doesn't provide access |
| **Repeat mode** | Not available via API |
| **Shuffle** | Not available via API |
| **Artwork** | Limited access |

### Workarounds

For extended control, use player-specific APIs:

```rust
// For Spotify, use Spotify Web API
// For VLC, use HTTP interface
// For foobar2000, use COM interface
```

## Troubleshooting

### Player Not Detected

**Check:**

```rust
let sessions = MediaSessions::new()?;
let info = sessions.current().await?;

match info {
    Some(info) => println!("Found: {}", info.display_string()),
    None => println!("No active session"),
}
```

**Solution:**

1. Make sure player is running
2. Check player settings (see above)
3. Try different browser for web players

### Initialization Error

```rust
match MediaSessions::new() {
    Ok(sessions) => { /* OK */ }
    Err(MediaError::NotSupported(platform)) => {
        eprintln!("Platform {} not supported", platform);
    }
    Err(e) => {
        eprintln!("Error: {}", e);
    }
}
```

**Solution:**

1. Make sure Windows 10 1803+
2. Check permissions
3. Run as administrator (if needed)

### CMake Build Errors

**Error:**

```
error: failed to run custom build command for `windows-rs`
```

**Solution:**

```bash
# Install CMake
winget install Kitware.CMake

# Or download from https://cmake.org/download/

# Clean and rebuild
cargo clean
cargo build
```

## Performance

| Operation | Time | Notes |
|-----------|------|-------|
| `current()` | ~350 ns | Async call |
| `watch()` first event | ~600 ns | First event |
| Event throughput | ~850/sec | Throughput |

## See Also

- **[macOS](macos.md)** — macOS implementation
- **[Linux](linux.md)** — Linux implementation
- **[Error Handling](../guides/error-handling.md)** — Error handling guide
