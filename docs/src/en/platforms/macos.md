# macOS

Media session control on macOS via MediaRemote.framework.

## Overview

On macOS, the library uses the private `MediaRemote` framework for media control.

## Requirements

| Requirement | Version | Description |
|-------------|---------|-------------|
| **macOS** | 12.0+ (Monterey) | Minimum supported version |
| **Rust** | 1.80+ | Minimum Supported Rust Version |
| **Accessibility** | Required | For some features |

## Installation

```toml
[dependencies]
media-sessions = { version = "0.2", features = ["macos"] }
tokio = { version = "1", features = ["full"] }
```

Or macOS only:

```toml
[dependencies]
media-sessions = { version = "0.2", default-features = false, features = ["macos"] }
```

## Accessibility Permissions

MediaRemote requires Accessibility access for full functionality.

### Grant Permissions

1. **System Preferences** → **Privacy & Security** → **Accessibility**
2. Click the **+** button
3. Add your terminal or IDE
4. Restart the application

```
┌────────────────────────────────────────────────────────┐
│  Privacy & Security > Accessibility                    │
│  ┌─────────────────────────────────────────────────┐  │
│  │ ☑ Terminal.app                                   │  │
│  │ ☑ Visual Studio Code.app                         │  │
│  │ ☐ Your App                                       │  │
│  └─────────────────────────────────────────────────┘  │
│                                                        │
│  [+]  [-]                                             │
└────────────────────────────────────────────────────────┘
```

## Supported Players

### ✅ Supported

| Application | Notes |
|-------------|-------|
| **Apple Music** | Full support |
| **Spotify** | Desktop app |
| **YouTube** | Safari, Chrome |
| **VLC** | Full support |
| **QuickTime** | Full support |
| **Podcasts** | Full support |

### ⚠️ Limited Support

| Application | Limitations |
|-------------|-------------|
| **Firefox** | May require permissions |
| **Third-party players** | Varies |

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

### Check Permissions

```rust
use media_sessions::MediaSessions;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    match MediaSessions::new() {
        Ok(sessions) => {
            println!("✅ MediaRemote initialized");
            
            if let Some(info) = sessions.current().await? {
                println!("🎵 {}", info.display_string());
            }
        }
        Err(e) => {
            eprintln!("❌ Error: {}", e);
            eprintln!("💡 Check Accessibility permissions:");
            eprintln!("   System Preferences → Privacy & Security → Accessibility");
        }
    }

    Ok(())
}
```

## MediaRemote Limitations

### Known Issues

| Issue | Status |
|-------|--------|
| **Private framework** | May break in future macOS |
| **Permissions required** | Accessibility access needed |
| **Limited metadata** | Some players don't expose all info |
| **No volume control** | Not available via MediaRemote |

## Troubleshooting

### Accessibility Error

**Error:**

```
Error: Backend { platform: "macos", message: "Access denied" }
```

**Solution:**

1. Open **System Preferences** → **Privacy & Security** → **Accessibility**
2. Unlock with password (click 🔓)
3. Click **+** and add your terminal/IDE
4. Restart the application

### Player Not Detected

**Check:**

```bash
# Check active sessions
log show --predicate 'eventMessage contains "NowPlaying"' --last 1m
```

**Solution:**

1. Make sure player is running
2. Start playback
3. Check Accessibility permissions
4. Restart player

### macOS Version Too Old

**Error:**

```
Error: NotSupported("macos")
```

**Solution:**

1. Check macOS version:
   ```bash
   sw_vers
   ```
2. Requires macOS 12.0+ (Monterey)
3. Update macOS if needed

## Performance

| Operation | Time | Notes |
|-----------|------|-------|
| `current()` | ~500 ns | Async call |
| `watch()` first event | ~800 ns | First event |
| Event throughput | ~600/sec | Throughput |

## See Also

- **[Windows](windows.md)** — Windows implementation
- **[Linux](linux.md)** — Linux implementation
- **[Error Handling](../guides/error-handling.md)** — Error handling guide
