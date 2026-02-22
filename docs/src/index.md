# Media Sessions Documentation

**Cross-platform media session control for Rust** — Control media players on Windows, macOS, and Linux with a single API.

[🇷🇺 Russian](ru/index.html) | [🇬🇧 English](index.html)

---

## 🚀 Quick Start

### Install (Rust)

```toml
[dependencies]
media-sessions = "0.2"
tokio = { version = "1", features = ["full"] }
```

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

---

## 📚 Table of Contents

### Getting Started

- **[Quick Start (5 min)](quickstart.md)** — Get up and running fast
- **[What is Media Sessions?](introduction.md)** — Project overview
- **[Installation](installation.md)** — Setup and configuration

### Rust API

- **[MediaSessions](rust-api/media-sessions.md)** — Main control class
  - [Creation](rust-api/media-sessions.md#creation)
  - [Playback Control](rust-api/media-sessions.md#playback-control)
  - [Event Streaming](rust-api/media-sessions.md#events)
- **[MediaInfo](rust-api/media-info.md)** — Track metadata
  - [Fields](rust-api/media-info.md#fields)
  - [Methods](rust-api/media-info.md#methods)
- **[PlaybackStatus](rust-api/playback-status.md)** — Status enum
- **[Events](rust-api/events.md)** — Event stream

### C API (FFI)

- **[C API Reference](c-api.md)** — Use from other languages
  - [Functions](c-api.md#functions)
  - [Data Types](c-api.md#data-types)
  - [Memory Management](c-api.md#memory)

### Language Bindings

- **[Python](languages/python.md)** — ctypes binding
- **[C# (.NET)](languages/csharp.md)** — P/Invoke
- **[C/C++](languages/c-cpp.md)** — Native API
- **[Node.js](languages/nodejs.md)** — ffi-napi

### Platforms

- **[Windows](platforms/windows.md)** — SMTC API
  - [Supported Players](platforms/windows.md#players)
  - [Limitations](platforms/windows.md#limitations)
- **[macOS](platforms/macos.md)** — MediaRemote
  - [Permissions](platforms/macos.md#permissions)
- **[Linux](platforms/linux.md)** — MPRIS/D-Bus
  - [Setup](platforms/linux.md#setup)

### Guides & Tutorials

- **[Error Handling](guides/error-handling.md)** — Handle errors properly
- **[Performance](guides/performance.md)** — Optimization tips
- **[Project Integration](guides/integration.md)** — Real-world examples
- **[Testing](guides/testing.md)** — Write tests
- **[Debugging](guides/debugging.md)** — Debug issues

### Reference

- **[FAQ](faq.md)** — Frequently asked questions
- **[Troubleshooting](troubleshooting.md)** — Common problems
- **[Changelog](../CHANGELOG.md)** — Version history

---

## 🎯 Common Tasks

### Get Current Track

```rust
if let Some(info) = sessions.current().await? {
    println!("Title: {}", info.title());
    println!("Artist: {}", info.artist());
    println!("Album: {}", info.album());
}
```

### Control Playback

```rust
sessions.play().await?;
sessions.pause().await?;
sessions.next().await?;
sessions.seek(Duration::from_secs(30)).await?;
```

### Listen for Events

```rust
use futures::StreamExt;

let mut stream = sessions.watch().await?;
while let Some(event) = stream.next().await {
    println!("Event: {:?}", event?);
}
```

---

## 📦 Resources

- **GitHub:** https://github.com/krosovok52/media-sessions
- **Crates.io:** https://crates.io/crates/media-sessions
- **Docs.rs:** https://docs.rs/media-sessions
- **Telegram:** https://t.me/programsKrosovok

---

**Version:** 0.2.0 | **Last Updated:** February 2026 | **License:** MIT OR Apache-2.0
