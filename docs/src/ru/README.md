# Media Sessions — Документация

**Выбор языка:** [🇷🇺 Русский](SUMMARY.md) | [🇬🇧 English](../en/SUMMARY.md)

---

# Введение

Media Sessions — это кроссплатформенная библиотека для управления медиа-сессиями на Windows, macOS и Linux.

## Что вы найдёте в этой документации

### 📚 Основы

- [Quick Start (5 мин)](quickstart.md) — быстрое введение
- [Что такое Media Sessions?](introduction.md) — обзор проекта  
- [Установка](installation.md) — как установить и настроить

### 🦀 Rust API

- [MediaSessions](rust-api/media-sessions.md) — главный класс
- [MediaInfo](rust-api/media-info.md) — метаданные трека
- [PlaybackStatus](rust-api/playback-status.md) — статусы воспроизведения
- [События](rust-api/events.md) — поток событий

### 🔌 C API

- [C API Reference](c-api.md) — FFI для других языков

### 🌐 Языки

- [Python](languages/python.md) — ctypes binding
- [C# (.NET)](languages/csharp.md) — P/Invoke
- [C/C++](languages/c-cpp.md) — нативный API
- [Node.js](languages/nodejs.md) — ffi-napi

### 🖥️ Платформы

- [Windows](platforms/windows.md) — SMTC API
- [macOS](platforms/macos.md) — MediaRemote
- [Linux](platforms/linux.md) — MPRIS/D-Bus

### 📖 Гайды

- [Обработка ошибок](guides/error-handling.md)
- [Производительность](guides/performance.md)
- [Интеграция в проект](guides/integration.md)

### ❓ Справка

- [FAQ](faq.md) — частые вопросы
- [Troubleshooting](troubleshooting.md) — решение проблем

---

## 🚀 Быстрый старт

### Rust

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

### Python

```python
import ctypes

lib = ctypes.CDLL('./media_sessions_c.dll')
handle = lib.media_sessions_c_new()
# ... использование
```

### C#

```csharp
using var sessions = new MediaSessionsWrapper();
var info = sessions.Current();
```

---

## 📦 Ресурсы

- **GitHub:** https://github.com/krosovok52/media-sessions
- **Crates.io:** https://crates.io/crates/media-sessions
- **Docs.rs:** https://docs.rs/media-sessions

---

**Версия:** 0.2.0 | **Обновлено:** Февраль 2026
