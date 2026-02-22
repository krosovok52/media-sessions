# Документация Media Sessions

> Кроссплатформенное управление медиа-сессиями для Rust

<div align="center">

**[🇷🇺 Русская версия](index.html)** &nbsp;|&nbsp; **[🇬🇧 English](../index.html)**

</div>

---

## Быстрый старт

### Установка

```toml
[dependencies]
media-sessions = "0.2"
tokio = { version = "1", features = ["full"] }
```

### Пример

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

## Документация

### API Reference

- **[MediaSessions](rust-api/media-sessions.md)** — Главный класс
- **[MediaInfo](rust-api/media-info.md)** — Метаданные трека
- **[PlaybackStatus](rust-api/playback-status.md)** — Статус
- **[События](rust-api/events.md)** — Поток событий

### C API (FFI)

- **[C API Reference](c-api.md)** — Использование из других языков
- **[Python](languages/python.md)** — ctypes binding
- **[C# (.NET)](languages/csharp.md)** — P/Invoke
- **[C/C++](languages/c-cpp.md)** — Нативный API

### Платформы

- **[Windows](platforms/windows.md)** — SMTC API ✅
- **[macOS](platforms/macos.md)** — MediaRemote ⚠️
- **[Linux](platforms/linux.md)** — MPRIS ✅

### Гайды

- **[Обработка ошибок](guides/error-handling.md)**
- **[Производительность](guides/performance.md)**
- **[FAQ](faq.md)**

---

## Ресурсы

- **GitHub:** https://github.com/krosovok52/media-sessions
- **Crates.io:** https://crates.io/crates/media-sessions
- **Docs.rs:** https://docs.rs/media-sessions

---

<div align="center">

**Версия:** 0.2.0 | **Лицензия:** MIT OR Apache-2.0

</div>
