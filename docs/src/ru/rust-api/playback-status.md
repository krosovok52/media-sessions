# PlaybackStatus

Статус воспроизведения медиа-сессии.

## Определение

```rust
pub enum PlaybackStatus {
    Playing,      // ▶️ Воспроизведение
    Paused,       // ⏸️ Пауза
    Stopped,      // ⏹️ Остановлено
    Transitioning // ⏳ Переходное состояние
}
```

## Варианты

| Вариант | Значение | Иконка | Описание |
|---------|----------|--------|----------|
| `Playing` | 0 | ▶️ | Активное воспроизведение |
| `Paused` | 1 | ⏸️ | Приостановлено |
| `Stopped` | 2 | ⏹️ | Остановлено |
| `Transitioning` | 3 | ⏳ | Переход между треками |

## Получение статуса

### Из MediaInfo

```rust
use media_sessions::{MediaSessions, PlaybackStatus};

let sessions = MediaSessions::new()?;

if let Some(info) = sessions.current().await? {
    match info.playback_status {
        PlaybackStatus::Playing => println!("▶️ Playing"),
        PlaybackStatus::Paused => println!("⏸️ Paused"),
        PlaybackStatus::Stopped => println!("⏹️ Stopped"),
        PlaybackStatus::Transitioning => println!("⏳ Transitioning"),
    }
}
```

### Helper методы

```rust
if let Some(info) = sessions.current().await? {
    if info.is_playing() {
        println!("▶️ Сейчас играет");
    } else if info.is_paused() {
        println!("⏸️ На паузе");
    }
}
```

## Примеры использования

### 1. Форматирование статуса

```rust
fn status_icon(status: PlaybackStatus) -> &'static str {
    match status {
        PlaybackStatus::Playing => "▶️",
        PlaybackStatus::Paused => "⏸️",
        PlaybackStatus::Stopped => "⏹️",
        PlaybackStatus::Transitioning => "⏳",
    }
}

if let Some(info) = sessions.current().await? {
    println!("{} {}", status_icon(info.playback_status), info.display_string());
}
```

### 2. Мониторинг изменений

```rust
use media_sessions::{MediaSessions, MediaSessionEvent, PlaybackStatus};
use futures::StreamExt;

let mut stream = sessions.watch().await?;

while let Some(event) = stream.next().await {
    if let MediaSessionEvent::PlaybackStatusChanged(status) = event? {
        println!("Статус изменился: {:?}", status);
    }
}
```

### 3. CLI вывод

```rust
use media_sessions::{MediaSessions, PlaybackStatus};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sessions = MediaSessions::new()?;

    if let Some(info) = sessions.current().await? {
        let (icon, status_text) = match info.playback_status {
            PlaybackStatus::Playing => ("▶️", "Playing"),
            PlaybackStatus::Paused => ("⏸️", "Paused"),
            PlaybackStatus::Stopped => ("⏹️", "Stopped"),
            PlaybackStatus::Transitioning => ("⏳", "Transitioning"),
        };

        println!("╔════════════════════════════════════════╗");
        println!("║  {} {:<26} ║", icon, status_text);
        println!("╠════════════════════════════════════════╣");
        println!("║  {} - {:<20} ║", info.artist(), info.title());
        println!("╚════════════════════════════════════════╝");
    }

    Ok(())
}
```

### 4. Логирование

```rust
use tracing::info;

if let Some(info) = sessions.current().await? {
    info!(
        status = ?info.playback_status,
        title = info.title(),
        artist = info.artist(),
        "Media session updated"
    );
}
```

### 5. Condition check

```rust
// Действие только если играет
if let Some(info) = sessions.current().await? {
    if info.is_playing() {
        sessions.pause().await?;
    } else {
        sessions.play().await?;
    }
}
```

## События изменения статуса

```rust
use media_sessions::{MediaSessions, MediaSessionEvent, PlaybackStatus};
use futures::StreamExt;

let mut stream = sessions.watch().await?;

while let Some(event) = stream.next().await {
    match event? {
        MediaSessionEvent::PlaybackStatusChanged(status) => {
            match status {
                PlaybackStatus::Playing => {
                    println!("▶️ Началось воспроизведение");
                }
                PlaybackStatus::Paused => {
                    println!("⏸️ Пауза");
                }
                PlaybackStatus::Stopped => {
                    println!("⏹️ Остановлено");
                }
                PlaybackStatus::Transitioning => {
                    println!("⏳ Переход...");
                }
            }
        }
        _ => {}
    }
}
```

## Реализация трейтов

```rust
// Clone, Copy
let status = PlaybackStatus::Playing;
let copy = status;  // Копирование

// PartialEq, Eq
if status == PlaybackStatus::Playing {
    // ...
}

// Debug
println!("{:?}", status);  // Playing

// Display (через fmt)
println!("{}", status);  // Playing
```

## Сериализация (с feature `serde`)

```toml
[dependencies]
media-sessions = { version = "0.2", features = ["serde"] }
serde = { version = "1.0", features = ["derive"] }
```

```rust
use media_sessions::PlaybackStatus;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct State {
    status: PlaybackStatus,
}

let state = State {
    status: PlaybackStatus::Playing,
};

let json = serde_json::to_string(&state)?;
```

## См. также

- **[MediaInfo](media-info.md)** — Содержит playback_status
- **[MediaSessions](media-sessions.md)** — Управление воспроизведением
- **[RepeatMode](repeat-mode.md)** — Режимы повтора
