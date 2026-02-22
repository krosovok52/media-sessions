# 📚 Документация Media Sessions

<div align="center">

![Media Sessions](https://img.shields.io/badge/Media-Sessions-blue?style=for-the-badge)
![Version](https://img.shields.io/badge/version-0.2.0-green?style=for-the-badge)
![Rust](https://img.shields.io/badge/Rust-1.80+-orange?style=for-the-badge&logo=rust)
![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue?style=for-the-badge)

**Кроссплатформенное управление медиа-сессиями для Rust**

[🇬🇧 English](../README.md) &nbsp;|&nbsp; [🇷🇺 Русская версия](README.md)

</div>

---

## 🎯 Что такое Media Sessions?

**Media Sessions** — это высокопроизводительная библиотека для управления системными медиаплеерами на Windows, macOS и Linux через единый API.

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

## 📖 Быстрый старт

### 1. Установка

```toml
[dependencies]
media-sessions = "0.2"
tokio = { version = "1", features = ["full"] }
futures = "0.3"
```

### 2. Первый запуск

```bash
cargo run
```

### 3. Результат

```
✅ Media Sessions initialized!
🎵 Queen - Bohemian Rhapsody
💿 A Night at the Opera
```

---

## 📚 Разделы документации

<div class="card-grid">

### 🔧 Rust API

| Раздел | Описание | Статус |
|--------|----------|--------|
| **[MediaSessions](rust-api/media-sessions.md)** | Главный класс управления | ✅ Готово |
| **[MediaInfo](rust-api/media-info.md)** | Метаданные трека | ✅ Готово |
| **[PlaybackStatus](rust-api/playback-status.md)** | Статусы воспроизведения | ✅ Готово |
| **[RepeatMode](rust-api/repeat-mode.md)** | Режимы повтора | ✅ Готово |
| **[События](rust-api/events.md)** | Поток событий | ✅ Готово |

### 🔌 C API (FFI)

| Язык | Руководство | Статус |
|------|-------------|--------|
| **[C API](c-api.md)** | Reference | ✅ Готово |
| **[Python](languages/python.md)** | ctypes binding | ✅ Готово |
| **[C#](languages/csharp.md)** | P/Invoke | ✅ Готово |
| **[C/C++](languages/c-cpp.md)** | Нативный API | ✅ Готово |
| **[Node.js](languages/nodejs.md)** | ffi-napi | ✅ Готово |

### 🖥️ Платформы

| Платформа | Бэкенд | Мин. версия | Статус |
|-----------|--------|-------------|--------|
| **[Windows](platforms/windows.md)** | SMTC | 10 1803+ | ✅ Стабильно |
| **[macOS](platforms/macos.md)** | MediaRemote | 12.0+ | 🟡 Beta |
| **[Linux](platforms/linux.md)** | MPRIS | D-Bus | ✅ Стабильно |

### 📖 Гайды

| Гайд | Описание | Статус |
|------|----------|--------|
| **[Обработка ошибок](guides/error-handling.md)** | Pattern matching, логирование | ✅ Готово |
| **[Производительность](guides/performance.md)** | Бенчмарки, оптимизация | ✅ Готово |
| **[Интеграция](guides/integration.md)** | Web, Desktop, CLI | ✅ Готово |
| **[Troubleshooting](guides/troubleshooting.md)** | Решение проблем | ✅ Готово |

</div>

---

## 🚀 Возможности

<div class="features">

| Возможность | Описание |
|-------------|----------|
| **🎯 Единый API** | Один интерфейс для всех платформ — никаких платформенных условных компиляций |
| **⚡ Async-first** | Построена на Tokio для неблокирующих операций |
| **🔒 Безопасность** | 100% безопасный Rust, unsafe только в изолированных FFI модулях |
| **📊 Debounce** | Фильтрация спама событий (800ms по умолчанию) |
| **🖼️ Обложки** | Поддержка извлечения обложек альбомов (PNG/JPEG) |
| **🔌 C API** | Использование из Python, C#, Node.js, C++ |
| **📈 Бенчмарки** | Встроенные бенчмарки на Criterion.rs |
| **🎯 Zero-cost** | Минимальный оверхед над нативными OS API |

</div>

---

## 📊 Производительность

Бенчмарки на Windows 11 (Ryzen 9 7950X, 32GB RAM):

| Операция | media-sessions | playerctl | Улучшение |
|----------|---------------|-----------|-----------|
| `current()` latency | **~350 ns** | ~2.3 ms | **6.5x быстрее** |
| `watch()` first event | **~600 ns** | N/A | — |
| Event throughput | **~850/sec** | ~100/sec | **8.5x больше** |

### Запуск бенчмарков

```bash
# Все бенчмарки
cargo bench --bench media_sessions

# Конкретный бенчмарк
cargo bench --bench media_sessions -- current_latency

# HTML отчёт
cargo bench --bench media_sessions -- --report
```

---

## 🎓 Примеры использования

### 1. Простой контроллер

```rust
use media_sessions::MediaSessions;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sessions = MediaSessions::new()?;
    
    sessions.play_pause().await?;
    sessions.next().await?;
    sessions.seek(std::time::Duration::from_secs(30)).await?;
    
    Ok(())
}
```

### 2. Монитор событий

```rust
use media_sessions::{MediaSessions, MediaSessionEvent};
use futures::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sessions = MediaSessions::new()?;
    let mut stream = sessions.watch().await?;
    
    while let Some(event) = stream.next().await {
        match event? {
            MediaSessionEvent::MetadataChanged(info) => {
                println!("🎵 Теперь: {}", info.display_string());
            }
            MediaSessionEvent::PlaybackStatusChanged(status) => {
                println!("▶️ Статус: {:?}", status);
            }
            _ => {}
        }
    }
    
    Ok(())
}
```

### 3. CLI утилита

```rust
use media_sessions::{MediaSessions, PlaybackStatus};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sessions = MediaSessions::new()?;
    
    if let Some(info) = sessions.current().await? {
        let icon = match info.playback_status {
            PlaybackStatus::Playing => "▶️",
            PlaybackStatus::Paused => "⏸️",
            PlaybackStatus::Stopped => "⏹️",
            _ => "⏳",
        };
        
        println!("╔════════════════════════════════════════╗");
        println!("║         Now Playing                    ║");
        println!("╠════════════════════════════════════════╣");
        println!("║ {} {}", icon, info.display_string());
        println!("║ 💿 {}", info.album());
        println!("╚════════════════════════════════════════╝");
    }
    
    Ok(())
}
```

Больше примеров в разделе **[Примеры](guides/integration.md)**.

---

## 🔗 Ресурсы

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

на ваш выбор.

---

## 📬 Контакты

| Платформа | Ссылка |
|-----------|--------|
| **Telegram канал** | [@programsKrosovok](https://t.me/programsKrosovok) |
| **GitHub** | [@krosovok52](https://github.com/krosovok52) |

---

<div align="center">

**Версия:** 0.2.0 &nbsp;|&nbsp; **MSRV:** 1.80+ &nbsp;|&nbsp; **Последнее обновление:** Февраль 2026

*Сделано с ❤️ используя Rust*

</div>
