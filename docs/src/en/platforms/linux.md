# Linux

Media session control on Linux via D-Bus / MPRIS 2.0.

## Overview

On Linux, the library uses D-Bus to communicate with MPRIS 2.0-compatible media players.

## Requirements

| Requirement | Version | Description |
|-------------|---------|-------------|
| **D-Bus** | Any | Session bus required |
| **Rust** | 1.80+ | Minimum Supported Rust Version |
| **MPRIS Player** | Any | Spotify, Firefox, VLC, etc. |

## Installation

### Install Dependencies

```bash
# Debian/Ubuntu
sudo apt install libdbus-1-dev pkg-config

# Fedora
sudo dnf install dbus-devel

# Arch
sudo pacman -S dbus
```

### Add Dependency

```toml
[dependencies]
media-sessions = { version = "0.2", features = ["linux"] }
tokio = { version = "1", features = ["full"] }
```

Or Linux only:

```toml
[dependencies]
media-sessions = { version = "0.2", default-features = false, features = ["linux"] }
```

## Supported Players

### ✅ Full Support

| Application | Notes |
|-------------|-------|
| **Spotify** | Full MPRIS support |
| **Firefox** | Enable in about:config |
| **VLC** | Full support |
| **mpv** | With `--input-mpremote-command` |
| **Rhythmbox** | Full support |
| **Clementine** | Full support |
| **Chromium** | Full support |

### ⚠️ Configuration Required

| Application | Configuration |
|-------------|---------------|
| **Firefox** | Enable `media.hardwaremediakeys.enabled` |
| **mpv** | Add `--input-mpremote-command` flag |

## Player Setup

### Firefox

1. Open `about:config`
2. Search for `media.hardwaremediakeys.enabled`
3. Set to `true`
4. Restart Firefox

### mpv

Add to `~/.config/mpv/mpv.conf`:

```
input-mpremote-command=yes
```

Or run with flag:

```bash
mpv --input-mpremote-command=yes music.mp3
```

### VLC

VLC supports MPRIS by default. No configuration needed.

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

### Check Available Players

```rust
use std::process::Command;

fn check_mpris_players() {
    let output = Command::new("dbus-send")
        .args(&[
            "--session",
            "--dest=org.freedesktop.DBus",
            "--type=method_call",
            "--print-reply",
            "/org/freedesktop/Bus",
            "org.freedesktop.DBus.ListNames",
        ])
        .output()
        .expect("Failed to run dbus-send");

    let stdout = String::from_utf8_lossy(&output.stdout);
    
    println!("MPRIS Players:");
    for line in stdout.lines() {
        if line.contains("mpris") {
            println!("  ✅ {}", line.trim());
        }
    }
}
```

### Full Player Controller

```rust
use media_sessions::{MediaSessions, PlaybackStatus};
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sessions = MediaSessions::new()?;

    // Get current track
    if let Some(info) = sessions.current().await? {
        let icon = match info.playback_status {
            PlaybackStatus::Playing => "▶️",
            PlaybackStatus::Paused => "⏸️",
            PlaybackStatus::Stopped => "⏹️",
            _ => "⏳",
        };

        println!("╔════════════════════════════════════════╗");
        println!("║         Now Playing                    ║");
        println!("╠════════════════════════════════════════╣");
        println!("║ {} {}", icon, info.display_string());
        
        if !info.album().is_empty() {
            println!("║ 💿 {}", info.album());
        }
        
        if let Some(duration) = info.duration {
            let position = info.position.unwrap_or_default();
            let progress = info.progress_percent();
            
            println!("║ [{}{}] {:.1}%",
                "█".repeat((progress / 100.0 * 30.0) as usize),
                "░".repeat(30 - (progress / 100.0 * 30.0) as usize),
                progress
            );
            println!("║ {}/{}", 
                format_duration(position),
                format_duration(duration)
            );
        }
        
        println!("╚════════════════════════════════════════╝");
    }

    Ok(())
}

fn format_duration(duration: Duration) -> String {
    let mins = duration.as_secs() / 60;
    let secs = duration.as_secs() % 60;
    format!("{}:{:02}", mins, secs)
}
```

## MPRIS Features

### Supported Properties

| Property | Support |
|----------|---------|
| **Title** | ✅ |
| **Artist** | ✅ |
| **Album** | ✅ |
| **Duration** | ✅ |
| **Position** | ✅ |
| **Playback Status** | ✅ |
| **Artwork** | ✅ |
| **Volume** | ✅ |
| **Repeat Mode** | ✅ |
| **Shuffle** | ✅ |

### Supported Methods

| Method | Support |
|--------|---------|
| **Play** | ✅ |
| **Pause** | ✅ |
| **Play/Pause** | ✅ |
| **Stop** | ✅ |
| **Next** | ✅ |
| **Previous** | ✅ |
| **Seek** | ✅ |
| **Set Volume** | ✅ |
| **Set Repeat Mode** | ✅ |
| **Set Shuffle** | ✅ |

## Troubleshooting

### D-Bus Connection Error

**Error:**

```
Error: Backend { platform: "linux", message: "Failed to connect to session bus" }
```

**Solution:**

```bash
# Check D-Bus
echo $DBUS_SESSION_BUS_ADDRESS

# If empty, start D-Bus
eval $(dbus-launch)

# Install D-Bus if missing
sudo apt install dbus
```

### Player Not Detected

**Check:**

```bash
dbus-send --session --dest=org.freedesktop.DBus \
  --type=method_call --print-reply \
  /org/freedesktop/Bus org.freedesktop.DBus.ListNames | grep mpris
```

**Solution:**

1. Start media player
2. Begin playback
3. Check player MPRIS support

### Volume Control Not Working

**Issue:** Some players don't support volume via MPRIS.

**Solution:** Use system volume control (PulseAudio/PipeWire):

```rust
// Use libpulsebinding for PulseAudio
// Or pipewire bindings for PipeWire
```

## Performance

| Operation | Time | Notes |
|-----------|------|-------|
| `current()` | ~2 ms | D-Bus call |
| `watch()` first event | ~5 ms | Initial connection |
| Event throughput | ~200/sec | D-Bus signals |

## See Also

- **[Windows](windows.md)** — Windows implementation
- **[macOS](macos.md)** — macOS implementation
- **[Error Handling](../guides/error-handling.md)** — Error handling guide
