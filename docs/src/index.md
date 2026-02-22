# Media Sessions Documentation

> Cross-platform media session control for Rust

<div align="center">

**[🇷🇺 Русская версия](ru/index.html)** &nbsp;|&nbsp; **[🇬🇧 English](index.html)**

</div>

---

## Quick Start

### Installation

```toml
[dependencies]
media-sessions = "0.2"
tokio = { version = "1", features = ["full"] }
```

### Example

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

---

## Documentation

### API Reference

- **[MediaSessions](rust-api/media-sessions.md)** — Main control class
- **[MediaInfo](rust-api/media-info.md)** — Track metadata
- **[PlaybackStatus](rust-api/playback-status.md)** — Status enum
- **[Events](rust-api/events.md)** — Event streaming

### C API (FFI)

- **[C API Reference](c-api.md)** — Use from other languages
- **[Python](languages/python.md)** — ctypes binding
- **[C# (.NET)](languages/csharp.md)** — P/Invoke
- **[C/C++](languages/c-cpp.md)** — Native API

### Platforms

- **[Windows](platforms/windows.md)** — SMTC API ✅
- **[macOS](platforms/macos.md)** — MediaRemote ⚠️
- **[Linux](platforms/linux.md)** — MPRIS ✅

### Guides

- **[Error Handling](guides/error-handling.md)**
- **[Performance](guides/performance.md)**
- **[FAQ](faq.md)**

---

## Resources

- **GitHub:** https://github.com/krosovok52/media-sessions
- **Crates.io:** https://crates.io/crates/media-sessions
- **Docs.rs:** https://docs.rs/media-sessions

---

<div align="center">

**Version:** 0.2.0 | **License:** MIT OR Apache-2.0

</div>
