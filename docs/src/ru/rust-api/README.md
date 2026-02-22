# Rust API Reference

Полное описание Rust API библиотеки Media Sessions.

## Основные типы

| Тип | Описание |
|-----|----------|
| [`MediaSessions`](media-sessions.md) | Главная точка входа для управления медиа-сессиями |
| [`MediaInfo`](media-info.md) | Метаданные текущего трека |
| [`PlaybackStatus`](playback-status.md) | Статус воспроизведения (Playing, Paused, Stopped) |
| [`RepeatMode`](repeat-mode.md) | Режим повтора (None, One, All) |
| [`MediaSessionEvent`](events.md) | События потока `watch()` |

## Быстрый пример

```rust
use media_sessions::{MediaSessions, PlaybackStatus};
use futures::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Создание экземпляра
    let sessions = MediaSessions::new()?;

    // Получить текущий трек
    if let Some(info) = sessions.current().await? {
        println!("🎵 {} - {}", info.artist(), info.title());
        println!("💿 {}", info.album());
        println!("⏱ Статус: {:?}", info.playback_status);
    }

    // Управление воспроизведением
    sessions.play().await?;
    sessions.seek(std::time::Duration::from_secs(30)).await?;

    // Подписаться на события
    let mut stream = sessions.watch().await?;
    while let Some(event) = stream.next().await {
        println!("📡 Событие: {:?}", event?);
    }

    Ok(())
}
```

## Структура API

```
MediaSessions
├── new()                          # Создать экземпляр
├── builder()                      # Builder для настройки
├── current().await                # Получить текущий трек
├── active_app().await             # Получить имя приложения
├── play().await                   # Play
├── pause().await                  # Pause
├── play_pause().await             # Toggle Play/Pause
├── stop().await                   # Stop
├── next().await                   # Следующий трек
├── previous().await               # Предыдущий трек
├── seek(duration).await           # Перемотка
├── set_volume(level).await        # Громкость (Linux)
├── set_repeat_mode(mode).await    # Режим повтора
├── set_shuffle(enabled).await     # Перемешивание
└── watch().await                  # Поток событий
```

## Конфигурация через Builder

```rust
use media_sessions::MediaSessions;
use std::time::Duration;

let sessions = MediaSessions::builder()
    .debounce_duration(Duration::from_millis(500))  // Фильтрация событий
    .operation_timeout(Duration::from_secs(10))     // Таймаут операций
    .enable_artwork(true)                           // Загрузка обложек
    .build()?;
```

## Обработка ошибок

```rust
use media_sessions::{MediaSessions, MediaError};

match MediaSessions::new() {
    Ok(sessions) => { /* OK */ }
    Err(MediaError::NotSupported(platform)) => {
        eprintln!("Платформа {} не поддерживается", platform);
    }
    Err(MediaError::NoSession) => {
        eprintln!("Нет активной медиа-сессии");
    }
    Err(MediaError::Backend { platform, message }) => {
        eprintln!("Ошибка бэкенда на {}: {}", platform, message);
    }
    Err(MediaError::Timeout(duration)) => {
        eprintln!("Таймаут операции после {:?}", duration);
    }
    Err(e) => {
        eprintln!("Ошибка: {}", e);
    }
}
```

## Подразделы

- **[MediaSessions](media-sessions.md)** — Главный класс, методы управления
- **[MediaInfo](media-info.md)** — Метаданные трека, поля, методы
- **[PlaybackStatus](playback-status.md)** — Статусы воспроизведения
- **[RepeatMode](repeat-mode.md)** — Режимы повтора
- **[События](events.md)** — Поток событий, debounce

---

**См. также:**
- [Quick Start](../quickstart.md)
- [C API](../c-api.md)
- [Примеры на GitHub](https://github.com/krosovok52/media-sessions/tree/main/examples)
