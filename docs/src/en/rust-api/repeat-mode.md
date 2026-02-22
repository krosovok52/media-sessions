# RepeatMode

Repeat mode enumeration.

## Overview

`RepeatMode` represents the current repeat mode of the media player.

## Definition

```rust
pub enum RepeatMode {
    None,  // Repeat off
    One,   // Repeat one track
    All,   // Repeat all tracks
}
```

## Variants

| Variant | Description | Icon |
|---------|-------------|------|
| **None** | Repeat disabled | 🔁❌ |
| **One** | Repeat current track | 🔂 |
| **All** | Repeat all tracks | 🔁 |

## Usage Examples

### 1. Set Repeat Mode

```rust
use media_sessions::{MediaSessions, RepeatMode};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sessions = MediaSessions::new()?;

    // Repeat one track
    sessions.set_repeat_mode(RepeatMode::One).await?;

    // Repeat all
    sessions.set_repeat_mode(RepeatMode::All).await?;

    // Repeat off
    sessions.set_repeat_mode(RepeatMode::None).await?;

    Ok(())
}
```

### 2. Display Mode

```rust
fn format_mode(mode: &RepeatMode) -> &'static str {
    match mode {
        RepeatMode::None => "Repeat Off",
        RepeatMode::One => "Repeat One",
        RepeatMode::All => "Repeat All",
    }
}
```

### 3. Event Handling

```rust
use media_sessions::{MediaSessionEvent, RepeatMode};

match event? {
    MediaSessionEvent::RepeatModeChanged { repeat, shuffle } => {
        let icon = match repeat {
            RepeatMode::None => "🔁❌",
            RepeatMode::One => "🔂",
            RepeatMode::All => "🔁",
        };
        println!("{} Repeat: {:?}, Shuffle: {}", icon, repeat, shuffle);
    }
    _ => {}
}
```

## Platform Support

| Platform | Support | Notes |
|----------|---------|-------|
| **Windows** | ❌ | Not supported by SMTC |
| **macOS** | ⚠️ | Limited support |
| **Linux** | ✅ | Full MPRIS support |

## See Also

- **[MediaSessions](media-sessions.md)** — set_repeat_mode method
- **[Events](events.md)** — RepeatModeChanged event
