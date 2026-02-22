# Media Sessions Documentation

<div align="center">

**Cross-platform media session control for Rust**

[🇷🇺 Русская версия](../ru/index.html) &nbsp;|&nbsp; [🇬🇧 English](index.html)

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
    }

    sessions.play().await?;

    Ok(())
}
```

---

## 📚 Documentation

### Introduction

- **[Introduction](introduction.md)** — What is Media Sessions
- **[Installation](installation.md)** — Installation and setup
- **[Quick Start](quickstart.md)** — Get started in 5 minutes

### Rust API

- **[MediaSessions](rust-api/media-sessions.md)** — Main class
- **[MediaInfo](rust-api/media-info.md)** — Track metadata
- **[PlaybackStatus](rust-api/playback-status.md)** — Playback statuses
- **[RepeatMode](rust-api/repeat-mode.md)** — Repeat modes
- **[Events](rust-api/events.md)** — Event stream

### C API (FFI)

- **[C API Reference](c-api.md)** — Using from other languages
- **[Python](languages/python.md)** — ctypes binding
- **[C# (.NET)](languages/csharp.md)** — P/Invoke
- **[C/C++](languages/c-cpp.md)** — Native API
- **[Node.js](languages/nodejs.md)** — ffi-napi

### Platforms

- **[Windows](platforms/windows.md)** — SMTC API ✅
- **[macOS](platforms/macos.md)** — MediaRemote ⚠️
- **[Linux](platforms/linux.md)** — MPRIS ✅

### Guides

- **[Error Handling](guides/error-handling.md)**
- **[Performance](guides/performance.md)**
- **[Integration](guides/integration.md)**
- **[Troubleshooting](guides/troubleshooting.md)**

---

## 🎯 Features

| Feature | Description |
|---------|-------------|
| **🎯 Single API** | One interface for all platforms |
| **⚡ Async-first** | Built on Tokio for non-blocking operations |
| **🔒 Safety** | 100% safe Rust |
| **📊 Debounce** | Event spam filtering |
| **🖼️ Artwork** | Album art extraction support |
| **🔌 C API** | Use from Python, C#, Node.js |

---

## 📊 Performance

Benchmarks on Windows 11 (Ryzen 9 7950X):

| Operation | media-sessions | playerctl |
|-----------|---------------|-----------|
| `current()` | **~350 ns** | ~2.3 ms |
| `watch()` first event | **~600 ns** | N/A |
| Event throughput | **~850/sec** | ~100/sec |

---

## 🖥️ Platform Support

| Platform | Min. Version | Backend | Status |
|----------|--------------|---------|--------|
| Windows 10/11 | 1803+ | WinRT SMTC | ✅ Stable |
| macOS | 12.0+ (Monterey) | MediaRemote | 🟡 In development |
| Linux | Any with D-Bus | MPRIS 2.0 | ✅ Stable |

---

## 📦 Resources

| Resource | Link |
|----------|------|
| **GitHub** | https://github.com/krosovok52/media-sessions |
| **Crates.io** | https://crates.io/crates/media-sessions |
| **Docs.rs** | https://docs.rs/media-sessions |
| **Examples** | https://github.com/krosovok52/media-sessions/tree/main/examples |

---

## 📄 License

Dual-licensed under:

- **MIT License** ([LICENSE-MIT](../LICENSE-MIT))
- **Apache License 2.0** ([LICENSE-APACHE](../LICENSE-APACHE))

---

## 📬 Contact

- **Author:** krosov_ok
- **Telegram:** [@programsKrosovok](https://t.me/programsKrosovok)
- **GitHub:** [@krosovok52](https://github.com/krosovok52)

---

<div align="center">

**Version:** 0.2.0 | **MSRV:** 1.80+ | **Last Updated:** February 2026

</div>
