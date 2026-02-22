# Media Sessions Documentation

<div align="center">

**Кроссплатформенное управление медиа-сессиями для Rust**

[🇷🇺 Русская версия](ru/index.html) &nbsp;|&nbsp; [🇬🇧 English](../index.html)

</div>

---

## 🚀 Быстрый старт

### Установка

```toml
[dependencies]
media-sessions = "0.2"
tokio = { version = "1", features = ["full"] }
futures = "0.3"
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

## 📚 Документация

### Введение

- **[Введение](ru/introduction.md)** — Что такое Media Sessions
- **[Установка](ru/installation.md)** — Установка и настройка
- **[Quick Start](quickstart.md)** — Быстрый старт за 5 минут

### Rust API

- **[MediaSessions](ru/rust-api/media-sessions.md)** — Главный класс
- **[MediaInfo](ru/rust-api/media-info.md)** — Метаданные трека
- **[PlaybackStatus](ru/rust-api/playback-status.md)** — Статусы воспроизведения
- **[RepeatMode](ru/rust-api/repeat-mode.md)** — Режимы повтора
- **[События](ru/rust-api/events.md)** — Поток событий

### C API (FFI)

- **[C API Reference](ru/c-api.md)** — Использование из других языков
- **[Python](ru/languages/python.md)** — ctypes binding
- **[C# (.NET)](ru/languages/csharp.md)** — P/Invoke
- **[C/C++](ru/languages/c-cpp.md)** — Нативный API
- **[Node.js](ru/languages/nodejs.md)** — ffi-napi

### Платформы

- **[Windows](ru/platforms/windows.md)** — SMTC API ✅
- **[macOS](ru/platforms/macos.md)** — MediaRemote ⚠️
- **[Linux](ru/platforms/linux.md)** — MPRIS ✅

### Гайды

- **[Обработка ошибок](ru/guides/error-handling.md)**
- **[Производительность](ru/guides/performance.md)**
- **[Интеграция](ru/guides/integration.md)**
- **[Troubleshooting](ru/guides/troubleshooting.md)**

---

## 🎯 Возможности

| Возможность | Описание |
|-------------|----------|
| **🎯 Единый API** | Один интерфейс для всех платформ |
| **⚡ Async-first** | Построена на Tokio для неблокирующих операций |
| **🔒 Безопасность** | 100% безопасный Rust |
| **📊 Debounce** | Фильтрация спама событий |
| **🖼️ Обложки** | Поддержка извлечения обложек |
| **🔌 C API** | Использование из Python, C#, Node.js |

---

## 📊 Производительность

Бенчмарки на Windows 11 (Ryzen 9 7950X):

| Операция | media-sessions | playerctl |
|----------|---------------|-----------|
| `current()` | **~350 ns** | ~2.3 ms |
| `watch()` first event | **~600 ns** | N/A |
| Event throughput | **~850/sec** | ~100/sec |

---

## 🖥️ Поддержка платформ

| Платформа | Мин. версия | Бэкенд | Статус |
|-----------|-------------|--------|--------|
| Windows 10/11 | 1803+ | WinRT SMTC | ✅ Стабильно |
| macOS | 12.0+ (Monterey) | MediaRemote | 🟡 В разработке |
| Linux | Любой с D-Bus | MPRIS 2.0 | ✅ Стабильно |

---

## 📦 Ресурсы

| Ресурс | Ссылка |
|--------|--------|
| **GitHub** | https://github.com/krosovok52/media-sessions |
| **Crates.io** | https://crates.io/crates/media-sessions |
| **Docs.rs** | https://docs.rs/media-sessions |
| **Примеры** | https://github.com/krosovok52/media-sessions/tree/main/examples |

---

## 📄 Лицензия

Dual-licensed под:

- **MIT License** ([LICENSE-MIT](../LICENSE-MIT))
- **Apache License 2.0** ([LICENSE-APACHE](../LICENSE-APACHE))

---

## 📬 Контакты

- **Автор:** krosov_ok
- **Telegram:** [@programsKrosovok](https://t.me/programsKrosovok)
- **GitHub:** [@krosovok52](https://github.com/krosovok52)

---

<div align="center">

**Версия:** 0.2.0 | **MSRV:** 1.80+ | **Последнее обновление:** Февраль 2026

</div>
