# MediaInfo

Track metadata structure.

## Overview

`MediaInfo` contains complete information about the currently playing track, including title, artist, album, artwork, and more.

## Structure

```rust
pub struct MediaInfo {
    pub title: Option<String>,
    pub artist: Option<String>,
    pub album: Option<String>,
    pub duration: Option<Duration>,
    pub position: Option<Duration>,
    pub playback_status: PlaybackStatus,
    pub artwork: Option<Vec<u8>>,
    pub genre: Option<String>,
    pub year: Option<i32>,
    pub track_number: Option<u32>,
    pub disc_number: Option<u32>,
    pub url: Option<String>,
    pub thumbnail_url: Option<String>,
}
```

## Fields

| Field | Type | Description | Example |
|-------|------|-------------|---------|
| `title` | `Option<String>` | Track title | `"Bohemian Rhapsody"` |
| `artist` | `Option<String>` | Artist name | `"Queen"` |
| `album` | `Option<String>` | Album name | `"A Night at the Opera"` |
| `duration` | `Option<Duration>` | Total duration | `354 seconds` |
| `position` | `Option<Duration>` | Current position | `120 seconds` |
| `playback_status` | `PlaybackStatus` | Playback status | `Playing`, `Paused` |
| `artwork` | `Option<Vec<u8>>` | Album art (raw bytes) | PNG/JPEG data |
| `genre` | `Option<String>` | Genre | `"Rock"` |
| `year` | `Option<i32>` | Release year | `1975` |
| `track_number` | `Option<u32>` | Track number | `11` |
| `disc_number` | `Option<u32>` | Disc number | `1` |
| `url` | `Option<String>` | Source URL | `"https://..."` |
| `thumbnail_url` | `Option<String>` | Thumbnail URL | `"https://..."` |

## Methods

### `title()`

Returns the track title or empty string.

```rust
println!("Title: {}", info.title());
```

### `artist()`

Returns the artist name or empty string.

```rust
println!("Artist: {}", info.artist());
```

### `album()`

Returns the album name or empty string.

```rust
println!("Album: {}", info.album());
```

### `display_string()`

Returns formatted string "Artist - Title".

```rust
println!("🎵 {}", info.display_string());
// "🎵 Queen - Bohemian Rhapsody"
```

### `duration_secs()`

Returns duration in seconds.

```rust
println!("Duration: {}s", info.duration_secs());
```

### `position_secs()`

Returns current position in seconds.

```rust
println!("Position: {}s", info.position_secs());
```

### `progress()`

Returns progress from 0.0 to 1.0.

```rust
println!("Progress: {:.1}%", info.progress() * 100.0);
```

### `progress_percent()`

Returns progress in percent (0 to 100).

```rust
println!("Progress: {:.1}%", info.progress_percent());
```

### `is_playing()`

Checks if track is playing.

```rust
if info.is_playing() {
    println!("Currently playing");
}
```

### `is_paused()`

Checks if track is paused.

```rust
if info.is_paused() {
    println!("Currently paused");
}
```

### `artwork_format()`

Returns artwork format (PNG or JPEG).

```rust
if let Some(format) = info.artwork_format() {
    println!("Artwork format: {}", format);
}
```

## Examples

### 1. Basic Information Display

```rust
use media_sessions::MediaSessions;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sessions = MediaSessions::new()?;

    if let Some(info) = sessions.current().await? {
        println!("╔════════════════════════════════════════╗");
        println!("║         Now Playing                    ║");
        println!("╠════════════════════════════════════════╣");
        println!("║ 🎵 {} - {}", info.artist(), info.title());
        println!("║ 💿 {}", info.album());
        println!("║ ⏱ {}/{} ({:.1}%)",
            info.position_secs(),
            info.duration_secs(),
            info.progress_percent()
        );
        println!("╚════════════════════════════════════════╝");
    }

    Ok(())
}
```

### 2. Save Artwork

```rust
use media_sessions::MediaSessions;
use std::fs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sessions = MediaSessions::new()?;

    if let Some(info) = sessions.current().await? {
        if let Some(artwork) = &info.artwork {
            let format = info.artwork_format().unwrap_or("png");
            let filename = format!("cover.{}", format);

            fs::write(&filename, artwork)?;
            println!("✅ Artwork saved to {}", filename);
        }
    }

    Ok(())
}
```

### 3. Format Time

```rust
fn format_time(secs: u64) -> String {
    let mins = secs / 60;
    let secs = secs % 60;
    format!("{}:{:02}", mins, secs)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sessions = MediaSessions::new()?;

    if let Some(info) = sessions.current().await? {
        println!("⏱ {} / {}",
            format_time(info.position_secs()),
            format_time(info.duration_secs())
        );
    }

    Ok(())
}
```

### 4. Progress Bar

```rust
fn progress_bar(progress: f64, width: usize) -> String {
    let filled = (progress * width as f64) as usize;
    let empty = width - filled;

    format!(
        "[{}{}] {:.1}%",
        "█".repeat(filled),
        "░".repeat(empty),
        progress * 100.0
    )
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sessions = MediaSessions::new()?;

    if let Some(info) = sessions.current().await? {
        println!("🎵 {}", info.display_string());
        println!("{}", progress_bar(info.progress(), 30));
    }

    Ok(())
}
```

## Handling Optional Values

Most `MediaInfo` fields are optional (`Option<T>`). Use these patterns:

### Pattern matching

```rust
if let Some(album) = &info.album {
    println!("Album: {}", album);
} else {
    println!("Album unknown");
}
```

### unwrap_or

```rust
let year = info.year().unwrap_or(0);
println!("Year: {}", year);
```

### map

```rust
let track_str = info.track_number()
    .map(|t| format!("Track {}", t))
    .unwrap_or_default();
```

## See Also

- **[MediaSessions](media-sessions.md)** — Main class for getting MediaInfo
- **[PlaybackStatus](playback-status.md)** — Playback statuses
- **[Events](events.md)** — MediaSessionEvent contains MediaInfo
