# Introduction

Welcome to **Media Sessions** documentation — a cross-platform library for controlling system media players on Windows, macOS, and Linux.

## What is a Media Session?

A media session is an interface that the operating system provides for controlling media content playback. When you open YouTube in your browser, launch Spotify, or play music in VLC — all these applications create media sessions.

## Quick Example

```rust
use media_sessions::MediaSessions;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sessions = MediaSessions::new()?;
    
    if let Some(info) = sessions.current().await? {
        println!("🎵 {} - {}", info.artist(), info.title());
    }
    
    sessions.play().await?;
    
    Ok(())
}
```

## Next Steps

- **[Installation](installation.md)** — Install the library
- **[Quick Start](quickstart.md)** — Get started in 5 minutes
- **[API Reference](rust-api/media-sessions.md)** — Full API documentation
