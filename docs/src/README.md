# Media Sessions Documentation

> Cross-platform media session control for Rust

<div align="center">

**[🇷🇺 Русская версия](ru/README.md)** &nbsp;|&nbsp; **[🇬🇧 English](README.md)**

</div>

---

## 🚀 Quick Start

### Installation

```toml
[dependencies]
media-sessions = "0.2"
tokio = { version = "1", features = ["full"] }
futures = "0.3"
```

### Example

```rust
use media_sessions::MediaSessions;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sessions = MediaSessions::new()?;

    if let Some(info) = sessions.current().await? {
        println!("🎵 {} - {}", info.artist(), info.title());
        println!("💿 {}", info.album());
    }

    sessions.play().await?;

    Ok(())
}
```

---

## 📚 Documentation Sections

### Rust API

| Type | Description |
|------|-------------|
| **[MediaSessions](rust-api/media-sessions.md)** | Main control class |
| **[MediaInfo](rust-api/media-info.md)** | Track metadata |
| **[PlaybackStatus](rust-api/playback-status.md)** | Playing, Paused, Stopped |
| **[RepeatMode](rust-api/repeat-mode.md)** | Repeat modes |
| **[Events](rust-api/events.md)** | Event streaming |

### C API (FFI)

| Language | Guide |
|----------|-------|
| **[C API](c-api.md)** | Reference |
| **[Python](languages/python.md)** | ctypes binding |
| **[C#](languages/csharp.md)** | P/Invoke |
| **[C/C++](languages/c-cpp.md)** | Native API |
| **[Node.js](languages/nodejs.md)** | ffi-napi |

### Platforms

| Platform | Backend | Status |
|----------|---------|--------|
| **[Windows](platforms/windows.md)** | SMTC | ✅ Stable |
| **[macOS](platforms/macos.md)** | MediaRemote | 🟡 Beta |
| **[Linux](platforms/linux.md)** | MPRIS | ✅ Stable |

### Guides

- **[Error Handling](guides/error-handling.md)** — Pattern matching, logging, retry
- **[Performance](guides/performance.md)** — Benchmarks, optimization
- **[Integration](guides/integration.md)** — Web servers, desktop apps
- **[Troubleshooting](guides/troubleshooting.md)** — Common issues

---

## 📊 Benchmarks

| Operation | media-sessions | playerctl |
|-----------|---------------|-----------|
| `current()` | **~350 ns** | ~2.3 ms |
| Event throughput | **~850/sec** | ~100/sec |

---

## 🔗 Resources

- **GitHub:** https://github.com/krosovok52/media-sessions
- **Crates.io:** https://crates.io/crates/media-sessions
- **Docs.rs:** https://docs.rs/media-sessions

---

<div align="center">

**Version:** 0.2.0 | **License:** MIT OR Apache-2.0

</div>
