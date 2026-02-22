# Документация Media Sessions

**Кроссплатформенное управление медиа-сессиями для Rust** — Контроль медиаплееров на Windows, macOS и Linux через единый API.

[🇷🇺 Русский](index.html) | [🇬🇧 English](../index.html)

---

## 🚀 Быстрый старт

### Установка (Rust)

```toml
[dependencies]
media-sessions = "0.2"
tokio = { version = "1", features = ["full"] }
```

### Базовое использование

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

## 📚 Содержание

### Начало работы

- **[Quick Start (5 мин)](quickstart.md)** — Быстрое введение
- **[Что такое Media Sessions?](introduction.md)** — Обзор проекта
- **[Установка](installation.md)** — Настройка и установка

### Rust API

- **[MediaSessions](rust-api/media-sessions.md)** — Главный класс
  - [Создание](rust-api/media-sessions.md#создание)
  - [Управление воспроизведением](rust-api/media-sessions.md#управление)
  - [Поток событий](rust-api/media-sessions.md#события)
- **[MediaInfo](rust-api/media-info.md)** — Метаданные трека
  - [Поля](rust-api/media-info.md#поля)
  - [Методы](rust-api/media-info.md#методы)
- **[PlaybackStatus](rust-api/playback-status.md)** — Перечисление статусов
- **[События](rust-api/events.md)** — Поток событий

### C API (FFI)

- **[C API Reference](c-api.md)** — Использование из других языков
  - [Функции](c-api.md#функции)
  - [Типы данных](c-api.md#типы-данных)
  - [Управление памятью](c-api.md#память)

### Языки

- **[Python](languages/python.md)** — ctypes binding
- **[C# (.NET)](languages/csharp.md)** — P/Invoke
- **[C/C++](languages/c-cpp.md)** — Нативный API
- **[Node.js](languages/nodejs.md)** — ffi-napi

### Платформы

- **[Windows](platforms/windows.md)** — SMTC API
  - [Поддерживаемые плееры](platforms/windows.md#плееры)
  - [Ограничения](platforms/windows.md#ограничения)
- **[macOS](platforms/macos.md)** — MediaRemote
  - [Разрешения](platforms/macos.md#разрешения)
- **[Linux](platforms/linux.md)** — MPRIS/D-Bus
  - [Настройка](platforms/linux.md#настройка)

### Гайды и уроки

- **[Обработка ошибок](guides/error-handling.md)** — Правильная обработка
- **[Производительность](guides/performance.md)** — Советы по оптимизации
- **[Интеграция в проект](guides/integration.md)** — Примеры из практики
- **[Тестирование](guides/testing.md)** — Написание тестов
- **[Отладка](guides/debugging.md)** — Решение проблем

### Справка

- **[FAQ](faq.md)** — Частые вопросы
- **[Troubleshooting](troubleshooting.md)** — Типовые проблемы
- **[Changelog](../CHANGELOG.md)** — История версий

---

## 🎯 Частые задачи

### Получить текущий трек

```rust
if let Some(info) = sessions.current().await? {
    println!("Название: {}", info.title());
    println!("Исполнитель: {}", info.artist());
    println!("Альбом: {}", info.album());
}
```

### Управление воспроизведением

```rust
sessions.play().await?;
sessions.pause().await?;
sessions.next().await?;
sessions.seek(Duration::from_secs(30)).await?;
```

### Слушать события

```rust
use futures::StreamExt;

let mut stream = sessions.watch().await?;
while let Some(event) = stream.next().await {
    println!("Событие: {:?}", event?);
}
```

---

## 📦 Ресурсы

- **GitHub:** https://github.com/krosovok52/media-sessions
- **Crates.io:** https://crates.io/crates/media-sessions
- **Docs.rs:** https://docs.rs/media-sessions
- **Telegram:** https://t.me/programsKrosovok

---

**Версия:** 0.2.0 | **Последнее обновление:** Февраль 2026 | **Лицензия:** MIT OR Apache-2.0
