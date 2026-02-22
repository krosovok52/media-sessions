# MediaSessions

The main class for controlling media sessions.

## Creating an Instance

### Basic Creation

```rust
use media_sessions::MediaSessions;

// Simple creation
let sessions = MediaSessions::new()?;
```

### Creating with Configuration

```rust
use media_sessions::MediaSessions;
use std::time::Duration;

let sessions = MediaSessions::builder()
    .debounce_duration(Duration::from_millis(500))  // Event filtering
    .operation_timeout(Duration::from_secs(10))     // Operation timeout
    .enable_artwork(true)                           // Load artwork
    .build()?;
```

## Control Methods

### Query Information

#### `current()`

Get information about the current track.

```rust
if let Some(info) = sessions.current().await? {
    println!("🎵 {} - {}", info.artist(), info.title());
    println!("💿 {}", info.album());
    println!("⏱ {}/{}", info.position_secs(), info.duration_secs());
}
```

**Returns:** `Result<Option<MediaInfo>, MediaError>`

#### `active_app()`

Get the name of the active media application.

```rust
if let Some(app) = sessions.active_app().await? {
    println!("▶️ Application: {}", app);
}
```

**Returns:** `Result<Option<String>, MediaError>`

### Playback Control

#### `play()`

Start playback.

```rust
sessions.play().await?;
```

#### `pause()`

Pause playback.

```rust
sessions.pause().await?;
```

#### `play_pause()`

Toggle Play/Pause state.

```rust
sessions.play_pause().await?;
```

#### `stop()`

Stop playback.

```rust
sessions.stop().await?;
```

#### `next()`

Next track.

```rust
sessions.next().await?;
```

#### `previous()`

Previous track.

```rust
sessions.previous().await?;
```

#### `seek(duration)`

Seek to the specified position.

```rust
use std::time::Duration;

// Seek to 30 seconds
sessions.seek(Duration::from_secs(30)).await?;

// Seek to 1 minute
sessions.seek(Duration::from_secs(60)).await?;
```

### Extended Control

#### `set_volume(level)`

Set volume (Linux only).

```rust
// 50% volume
sessions.set_volume(0.5).await?;

// 75% volume
sessions.set_volume(0.75).await?;
```

**Parameters:** `level: f64` (from 0.0 to 1.0)

**Returns:** `Result<(), MediaError>`

**Note:** This method may not be supported on Windows and macOS.

#### `set_repeat_mode(mode)`

Set repeat mode.

```rust
use media_sessions::RepeatMode;

// Repeat off
sessions.set_repeat_mode(RepeatMode::None).await?;

// Repeat one track
sessions.set_repeat_mode(RepeatMode::One).await?;

// Repeat all
sessions.set_repeat_mode(RepeatMode::All).await?;
```

**Parameters:** `mode: RepeatMode`

#### `set_shuffle(enabled)`

Enable/disable shuffle.

```rust
// Enable shuffle
sessions.set_shuffle(true).await?;

// Disable shuffle
sessions.set_shuffle(false).await?;
```

**Parameters:** `enabled: bool`

### Event Stream

#### `watch()`

Create a media session event stream.

```rust
use media_sessions::{MediaSessions, MediaSessionEvent};
use futures::StreamExt;

let mut stream = sessions.watch().await?;

while let Some(event) = stream.next().await {
    match event? {
        MediaSessionEvent::MetadataChanged(info) => {
            println!("🎵 Now: {}", info.display_string());
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
```

**Returns:** `Result<impl Stream<Item = MediaSessionEvent>, MediaError>`

## Builder Pattern

### `MediaSessionsBuilder`

| Method | Description | Default |
|--------|-------------|---------|
| `debounce_duration(Duration)` | Event filtering | 800ms |
| `operation_timeout(Duration)` | Operation timeout | 5s |
| `enable_artwork(bool)` | Load artwork | true |

### Examples

**Minimal configuration:**

```rust
let sessions = MediaSessions::builder()
    .debounce_duration(Duration::from_millis(300))
    .build()?;
```

**For high-load applications:**

```rust
let sessions = MediaSessions::builder()
    .debounce_duration(Duration::from_millis(100))  // Minimal delay
    .operation_timeout(Duration::from_secs(3))      // Fast timeout
    .enable_artwork(false)                          // No artwork
    .build()?;
```

## Usage Examples

### 1. Simple Player Controller

```rust
use media_sessions::MediaSessions;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sessions = MediaSessions::new()?;

    // Play/Pause cycle
    sessions.play().await?;
    tokio::time::sleep(Duration::from_secs(5)).await;
    sessions.pause().await?;
    tokio::time::sleep(Duration::from_secs(2)).await;

    // Next track
    sessions.next().await?;

    Ok(())
}
```

### 2. Current Track Monitor

```rust
use media_sessions::MediaSessions;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sessions = MediaSessions::new()?;

    loop {
        if let Some(info) = sessions.current().await? {
            println!("╔════════════════════════════════════════╗");
            println!("║         Now Playing                    ║");
            println!("╠════════════════════════════════════════╣");
            println!("║ {} - {}", info.artist(), info.title());
            if let Some(album) = &info.album {
                println!("║ 💿 {}", album);
            }
            println!("╚════════════════════════════════════════╝");
        }

        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }
}
```

### 3. Integration with tokio::select

```rust
use media_sessions::{MediaSessions, MediaSessionEvent};
use futures::StreamExt;
use tokio::sync::broadcast;

struct MediaMonitor {
    sessions: MediaSessions,
    event_tx: broadcast::Sender<String>,
}

impl MediaMonitor {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let sessions = MediaSessions::builder()
            .debounce_duration(std::time::Duration::from_millis(500))
            .build()?;

        let (event_tx, _) = broadcast::channel(100);
        Ok(Self { sessions, event_tx })
    }

    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut stream = self.sessions.watch().await?;
        let mut rx = self.event_tx.subscribe();

        loop {
            tokio::select! {
                event = stream.next() => {
                    if let Some(Ok(MediaSessionEvent::MetadataChanged(info))) = event {
                        let msg = format!("Now playing: {}", info.display_string());
                        let _ = self.event_tx.send(msg);
                    }
                }
                msg = rx.recv() => {
                    println!("Received: {}", msg?);
                }
            }
        }
    }
}
```

### 4. CLI Utility

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
            _ => "⏳",
        };

        println!("{} {}", icon, info.display_string());

        if let Some(album) = &info.album {
            println!("💿 {}", album);
        }

        let progress = info.progress_percent();
        println!("📊 {:.1}%", progress);
    }

    Ok(())
}
```

## Error Handling

```rust
use media_sessions::{MediaSessions, MediaError};

match sessions.play().await {
    Ok(()) => println!("✅ Play successful"),
    Err(MediaError::NoSession) => println!("❌ No active session"),
    Err(MediaError::NotSupported(_)) => println!("❌ Not supported"),
    Err(MediaError::Timeout(d)) => println!("❌ Timeout after {:?}", d),
    Err(e) => println!("❌ Error: {}", e),
}
```

## See Also

- **[MediaInfo](media-info.md)** — Track metadata
- **[PlaybackStatus](playback-status.md)** — Playback statuses
- **[Events](events.md)** — Event stream
- **[Error Handling](../guides/error-handling.md)** — Error handling guide
