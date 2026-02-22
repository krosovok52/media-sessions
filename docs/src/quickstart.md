# Quick Start

5-minute guide to get started with Media Sessions.

---

## 1. Installation (1 minute)

### Add dependencies

```bash
cargo add media-sessions tokio futures
```

Or manually in `Cargo.toml`:

```toml
[dependencies]
media-sessions = "0.2"
tokio = { version = "1", features = ["full"] }
futures = "0.3"
```

### Check installation

```bash
cargo check
```

---

## 2. First Run (2 minutes)

### Create main.rs

```rust
use media_sessions::MediaSessions;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create instance
    let sessions = MediaSessions::new()?;
    println!("✅ Media Sessions initialized!");

    // Get current track
    if let Some(info) = sessions.current().await? {
        println!("🎵 {} - {}", info.artist(), info.title());
        println!("💿 {}", info.album());
    } else {
        println!("ℹ️ No active media session");
        println!("💡 Start Spotify, YouTube in browser, or another player");
    }

    Ok(())
}
```

### Run

```bash
cargo run
```

### Expected output

```
✅ Media Sessions initialized!
🎵 Artist - Song Title
💿 Album Name
```

---

## 3. Playback Control (1 minute)

### Add control

```rust
use media_sessions::MediaSessions;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sessions = MediaSessions::new()?;

    // Play
    sessions.play().await?;
    println!("▶️ Playing");

    // Wait 5 seconds
    tokio::time::sleep(Duration::from_secs(5)).await;

    // Pause
    sessions.pause().await?;
    println!("⏸️ Paused");

    // Next track
    sessions.next().await?;
    println!("⏭️ Next track");

    // Seek 30 seconds
    sessions.seek(Duration::from_secs(30)).await?;
    println!("⏱ Seeked to 30s");

    Ok(())
}
```

---

## 4. Event Monitoring (1 minute)

### Subscribe to events

```rust
use media_sessions::{MediaSessions, MediaSessionEvent};
use futures::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sessions = MediaSessions::new()?;
    let mut stream = sessions.watch().await?;

    println!("🎵 Media Sessions Monitor");
    println!("{}", "=".repeat(40));

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

---

## 📚 What's Next?

### Explore API

- **[MediaSessions](rust-api/media-sessions.md)** — Main class
- **[MediaInfo](rust-api/media-info.md)** — Track metadata
- **[Events](rust-api/events.md)** — Event stream

### Guides

- **[Installation](installation.md)** — Detailed installation
- **[Error Handling](guides/error-handling.md)** — Error handling
- **[Performance](guides/performance.md)** — Optimization

### Examples

More examples on GitHub:

- [basic_usage.rs](https://github.com/krosovok52/media-sessions/tree/main/examples/basic_usage.rs)
- [simple_player.rs](https://github.com/krosovok52/media-sessions/tree/main/examples/simple_player.rs)
- [event_watcher.rs](https://github.com/krosovok52/media-sessions/tree/main/examples/event_watcher.rs)

---

## ❓ Troubleshooting?

### "No active media session"

**Solution:** Start a media player:

- Spotify (UWP or Desktop)
- YouTube in browser (Edge, Chrome, Firefox)
- VLC
- Any MPRIS player (Linux)

### Compilation error

**Solution:** Check dependencies:

```bash
cargo update
cargo build
```

### Player not detected

See **[Troubleshooting](guides/troubleshooting.md)**.

---

**Version:** 0.2.0 | **Read time:** 5 minutes
