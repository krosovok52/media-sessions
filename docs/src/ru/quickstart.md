# Quick Start (Быстрый старт)

5-минутное руководство по началу работы с Media Sessions.

---

## 1. Установка (1 минута)

### Добавить зависимости

```bash
cargo add media-sessions tokio futures
```

Или вручную в `Cargo.toml`:

```toml
[dependencies]
media-sessions = "0.2"
tokio = { version = "1", features = ["full"] }
futures = "0.3"
```

### Проверка установки

```bash
cargo check
```

---

## 2. Первый запуск (2 минуты)

### Создать main.rs

```rust
use media_sessions::MediaSessions;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Создать экземпляр
    let sessions = MediaSessions::new()?;
    println!("✅ Media Sessions initialized!");

    // Получить текущий трек
    if let Some(info) = sessions.current().await? {
        println!("🎵 {} - {}", info.artist(), info.title());
        println!("💿 {}", info.album());
    } else {
        println!("ℹ️ Нет активной медиа-сессии");
        println!("💡 Запустите Spotify, YouTube в браузере, или другой плеер");
    }

    Ok(())
}
```

### Запустить

```bash
cargo run
```

### Ожидаемый вывод

```
✅ Media Sessions initialized!
🎵 Artist - Song Title
💿 Album Name
```

---

## 3. Управление воспроизведением (1 минута)

### Добавить управление

```rust
use media_sessions::MediaSessions;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sessions = MediaSessions::new()?;

    // Play
    sessions.play().await?;
    println!("▶️ Playing");

    // Ждать 5 секунд
    tokio::time::sleep(Duration::from_secs(5)).await;

    // Pause
    sessions.pause().await?;
    println!("⏸️ Paused");

    // Следующий трек
    sessions.next().await?;
    println!("⏭️ Next track");

    // Перемотка на 30 секунд
    sessions.seek(Duration::from_secs(30)).await?;
    println!("⏱ Seeked to 30s");

    Ok(())
}
```

---

## 4. Мониторинг событий (1 минута)

### Подписаться на события

```rust
use media_sessions::{MediaSessions, MediaSessionEvent};
use futures::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sessions = MediaSessions::new()?;
    let mut stream = sessions.watch().await?;

    println!("🎵 Media Sessions Monitor");
    println!("=".repeat(40));

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

---

## 📚 Что дальше?

### Изучить API

- **[MediaSessions](rust-api/media-sessions.md)** — Главный класс
- **[MediaInfo](rust-api/media-info.md)** — Метаданные трека
- **[События](rust-api/events.md)** — Поток событий

### Гайды

- **[Установка](installation.md)** — Подробная установка
- **[Обработка ошибок](guides/error-handling.md)** — Error handling
- **[Производительность](guides/performance.md)** — Оптимизация

### Примеры

Больше примеров на GitHub:

- [basic_usage.rs](https://github.com/krosovok52/media-sessions/tree/main/examples/basic_usage.rs)
- [simple_player.rs](https://github.com/krosovok52/media-sessions/tree/main/examples/simple_player.rs)
- [event_watcher.rs](https://github.com/krosovok52/media-sessions/tree/main/examples/event_watcher.rs)

---

## ❓ Проблемы?

### "No active media session"

**Решение:** Запустите медиа-плеер:

- Spotify (UWP или Desktop)
- YouTube в браузере (Edge, Chrome, Firefox)
- VLC
- Любой MPRIS плеер (Linux)

### Ошибка компиляции

**Решение:** Проверьте зависимости:

```bash
cargo update
cargo build
```

### Плеер не обнаруживается

Смотрите **[Troubleshooting](guides/troubleshooting.md)**.

---

**Версия:** 0.2.0 | **Время чтения:** 5 минут
