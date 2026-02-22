# Media Session Events

Event stream for monitoring media session changes.

## Overview

`MediaSessionEvent` is an enum representing different types of events that can occur in a media session.

## Event Types

```rust
pub enum MediaSessionEvent {
    /// Track metadata changed
    MetadataChanged(MediaInfo),
    
    /// Playback status changed (Playing, Paused, etc.)
    PlaybackStatusChanged(PlaybackStatus),
    
    /// Position changed
    PositionChanged { 
        position: Duration, 
        old_position: Option<Duration> 
    },
    
    /// New session opened
    SessionOpened { app_name: String },
    
    /// Session closed
    SessionClosed,
    
    /// Artwork changed
    ArtworkChanged,
    
    /// Volume changed
    VolumeChanged { volume: f64 },
    
    /// Repeat mode or shuffle changed
    RepeatModeChanged { 
        repeat: RepeatMode, 
        shuffle: bool 
    },
}
```

## Event Descriptions

| Event | Description | Data |
|-------|-------------|------|
| **MetadataChanged** | Track info updated | `MediaInfo` |
| **PlaybackStatusChanged** | Play/pause/stop status | `PlaybackStatus` |
| **PositionChanged** | Seek or progress update | `position`, `old_position` |
| **SessionOpened** | New player started | `app_name` |
| **SessionClosed** | Player stopped | — |
| **ArtworkChanged** | New album art | — |
| **VolumeChanged** | Volume adjusted | `volume` (0.0-1.0) |
| **RepeatModeChanged** | Repeat/shuffle toggle | `repeat`, `shuffle` |

## Usage Examples

### 1. Basic Event Monitoring

```rust
use media_sessions::{MediaSessions, MediaSessionEvent};
use futures::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sessions = MediaSessions::new()?;
    let mut stream = sessions.watch().await?;

    println!("🎵 Media Sessions Event Watcher");
    println!("{}", "=".repeat(40));

    while let Some(event) = stream.next().await {
        match event? {
            MediaSessionEvent::MetadataChanged(info) => {
                println!("🎵 Now: {} - {}", info.artist(), info.title());
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

### 2. Filtering Events

```rust
use media_sessions::{MediaSessions, MediaSessionEvent};
use futures::{StreamExt, stream};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sessions = MediaSessions::new()?;
    let mut stream = sessions
        .watch()
        .await?
        .filter(|event| {
            futures::future::ready(matches!(
                event,
                Ok(MediaSessionEvent::MetadataChanged(_))
            ))
        });

    while let Some(event) = stream.next().await {
        if let Ok(MediaSessionEvent::MetadataChanged(info)) = event {
            println!("🎵 Track: {}", info.display_string());
        }
    }

    Ok(())
}
```

### 3. Event Debouncing

```rust
use media_sessions::{MediaSessions, MediaSessionEvent};
use futures::StreamExt;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 500ms debounce to filter rapid changes
    let sessions = MediaSessions::builder()
        .debounce_duration(Duration::from_millis(500))
        .build()?;

    let mut stream = sessions.watch().await?;

    while let Some(event) = stream.next().await {
        match event? {
            MediaSessionEvent::MetadataChanged(info) => {
                println!("🎵 Now playing: {}", info.display_string());
            }
            MediaSessionEvent::PlaybackStatusChanged(status) => {
                println!("▶️ Status: {:?}", status);
            }
            MediaSessionEvent::PositionChanged { position, .. } => {
                println!("⏱ Position: {}s", position.as_secs());
            }
            _ => {}
        }
    }

    Ok(())
}
```

### 4. Multiple Event Types

```rust
use media_sessions::{MediaSessions, MediaSessionEvent, PlaybackStatus, RepeatMode};
use futures::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sessions = MediaSessions::new()?;
    let mut stream = sessions.watch().await?;

    while let Some(event) = stream.next().await {
        match event? {
            MediaSessionEvent::MetadataChanged(info) => {
                println!("╔════════════════════════════════════════╗");
                println!("║         Now Playing                    ║");
                println!("╠════════════════════════════════════════╣");
                println!("║ 🎵 {} - {}", info.artist(), info.title());
                if !info.album().is_empty() {
                    println!("║ 💿 {}", info.album());
                }
                println!("╚════════════════════════════════════════╝");
            }
            MediaSessionEvent::PlaybackStatusChanged(status) => {
                let icon = match status {
                    PlaybackStatus::Playing => "▶️",
                    PlaybackStatus::Paused => "⏸️",
                    PlaybackStatus::Stopped => "⏹️",
                    _ => "⏳",
                };
                println!("{} Status: {:?}", icon, status);
            }
            MediaSessionEvent::PositionChanged { position, old_position } => {
                println!("⏱ Position: {:?} → {:?}", old_position, position);
            }
            MediaSessionEvent::SessionOpened { app_name } => {
                println!("📱 Session opened: {}", app_name);
            }
            MediaSessionEvent::SessionClosed => {
                println!("📱 Session closed");
            }
            MediaSessionEvent::ArtworkChanged => {
                println!("🖼️  Artwork changed");
            }
            MediaSessionEvent::VolumeChanged { volume } => {
                println!("🔊 Volume: {:.0}%", volume * 100.0);
            }
            MediaSessionEvent::RepeatModeChanged { repeat, shuffle } => {
                println!("🔁 Repeat: {:?}, Shuffle: {}", repeat, shuffle);
            }
        }
    }

    Ok(())
}
```

### 5. Integration with Tokio Select

```rust
use media_sessions::{MediaSessions, MediaSessionEvent};
use futures::StreamExt;
use tokio::sync::broadcast;
use tokio::time::{interval, Duration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sessions = MediaSessions::new()?;
    let (tx, mut rx) = broadcast::channel::<String>(100);

    let mut stream = sessions.watch().await?;
    let mut tick = interval(Duration::from_secs(10));

    loop {
        tokio::select! {
            event = stream.next() => {
                if let Some(Ok(MediaSessionEvent::MetadataChanged(info))) = event {
                    let msg = format!("Now playing: {}", info.display_string());
                    let _ = tx.send(msg);
                }
            }
            msg = rx.recv() => {
                println!("Received: {}", msg?);
            }
            _ = tick.tick() => {
                println!("Heartbeat: still listening...");
            }
        }
    }
}
```

### 6. Event Logging

```rust
use media_sessions::{MediaSessions, MediaSessionEvent};
use futures::StreamExt;
use std::io::Write;
use std::fs::OpenOptions;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sessions = MediaSessions::new()?;
    let mut stream = sessions.watch().await?;
    let mut log_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("media_events.log")?;

    while let Some(event) = stream.next().await {
        if let Ok(event) = event {
            let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S");
            let log_line = format!("[{}] {:?}\n", timestamp, event);
            log_file.write_all(log_line.as_bytes())?;
        }
    }

    Ok(())
}
```

## Best Practices

### 1. Handle All Event Types

```rust
match event? {
    MediaSessionEvent::MetadataChanged(info) => { /* ... */ }
    MediaSessionEvent::PlaybackStatusChanged(status) => { /* ... */ }
    MediaSessionEvent::PositionChanged { .. } => { /* ... */ }
    MediaSessionEvent::SessionOpened { .. } => { /* ... */ }
    MediaSessionEvent::SessionClosed => { /* ... */ }
    MediaSessionEvent::ArtworkChanged => { /* ... */ }
    MediaSessionEvent::VolumeChanged { .. } => { /* ... */ }
    MediaSessionEvent::RepeatModeChanged { .. } => { /* ... */ }
}
```

### 2. Use Debounce for Rapid Events

```rust
let sessions = MediaSessions::builder()
    .debounce_duration(Duration::from_millis(500))
    .build()?;
```

### 3. Buffer Events for Performance

```rust
let mut stream = sessions
    .watch()
    .await?
    .buffered(10);  // Buffer 10 events
```

### 4. Graceful Error Handling

```rust
while let Some(event) = stream.next().await {
    match event {
        Ok(event) => {
            // Process event
        }
        Err(e) => {
            eprintln!("Stream error: {}", e);
            break;  // Or continue based on error type
        }
        None => {
            eprintln!("Stream ended");
            break;
        }
    }
}
```

## See Also

- **[MediaInfo](media-info.md)** — Track metadata
- **[MediaSessions](media-sessions.md)** — Main class
- **[Error Handling](../guides/error-handling.md)** — Error handling guide
