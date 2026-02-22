# PlaybackStatus

Playback status enumeration.

## Overview

`PlaybackStatus` represents the current state of media playback.

## Definition

```rust
pub enum PlaybackStatus {
    Playing,    // ▶️ Currently playing
    Paused,     // ⏸️ Currently paused
    Stopped,    // ⏹️ Currently stopped
    Transitioning, // ⏳ Changing state
}
```

## Variants

| Variant | Icon | Description |
|---------|------|-------------|
| **Playing** | ▶️ | Media is currently playing |
| **Paused** | ⏸️ | Media is currently paused |
| **Stopped** | ⏹️ | Media is currently stopped |
| **Transitioning** | ⏳ | Media is changing state |

## Usage Examples

### 1. Display Status

```rust
use media_sessions::{MediaSessions, PlaybackStatus};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sessions = MediaSessions::new()?;

    if let Some(info) = sessions.current().await? {
        let icon = match info.playback_status {
            PlaybackStatus::Playing => "▶️",
            PlaybackStatus::Paused => "⏸️",
            PlaybackStatus::Stopped => "⏹️",
            PlaybackStatus::Transitioning => "⏳",
        };

        println!("{} {:?}", icon, info.playback_status);
    }

    Ok(())
}
```

### 2. Check Status

```rust
if info.is_playing() {
    println!("Currently playing");
}

if info.is_paused() {
    println!("Currently paused");
}
```

### 3. Format Status

```rust
fn format_status(status: &PlaybackStatus) -> &'static str {
    match status {
        PlaybackStatus::Playing => "Playing",
        PlaybackStatus::Paused => "Paused",
        PlaybackStatus::Stopped => "Stopped",
        PlaybackStatus::Transitioning => "Loading...",
    }
}
```

### 4. Event Handling

```rust
use media_sessions::{MediaSessionEvent, PlaybackStatus};

match event? {
    MediaSessionEvent::PlaybackStatusChanged(status) => {
        match status {
            PlaybackStatus::Playing => println!("▶️ Playback started"),
            PlaybackStatus::Paused => println!("⏸️ Playback paused"),
            PlaybackStatus::Stopped => println!("⏹️ Playback stopped"),
            PlaybackStatus::Transitioning => println!("⏳ Loading..."),
        }
    }
    _ => {}
}
```

## See Also

- **[MediaInfo](media-info.md)** — Track metadata
- **[Events](events.md)** — PlaybackStatusChanged event
