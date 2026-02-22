# RepeatMode

Режим повтора для медиа-плеера.

## Определение

```rust
pub enum RepeatMode {
    None,  // Повтор выключен
    One,   // Повтор одного трека
    All    // Повтор всех треков
}
```

## Варианты

| Вариант | Значение | Иконка | Описание |
|---------|----------|--------|----------|
| `None` | 0 | 🔁❌ | Повтор выключен |
| `One` | 1 | 🔂 | Повтор одного трека |
| `All` | 2 | 🔁 | Повтор всех треков |

## Установка режима повтора

```rust
use media_sessions::{MediaSessions, RepeatMode};

let sessions = MediaSessions::new()?;

// Выключить повтор
sessions.set_repeat_mode(RepeatMode::None).await?;

// Повтор одного трека
sessions.set_repeat_mode(RepeatMode::One).await?;

// Повтор всех треков
sessions.set_repeat_mode(RepeatMode::All).await?;
```

## Перемешивание (Shuffle)

```rust
// Включить shuffle
sessions.set_shuffle(true).await?;

// Выключить shuffle
sessions.set_shuffle(false).await?;
```

## Примеры использования

### 1. Циклическое переключение режима

```rust
use media_sessions::{MediaSessions, RepeatMode};

async fn cycle_repeat_mode(sessions: &MediaSessions) -> Result<(), Box<dyn std::error::Error>> {
    // Получаем текущий режим (если поддерживается)
    // Переключаем: None -> One -> All -> None
    let current = RepeatMode::None; // Заглушка, т.к. нет getter'а
    
    let next_mode = match current {
        RepeatMode::None => RepeatMode::One,
        RepeatMode::One => RepeatMode::All,
        RepeatMode::All => RepeatMode::None,
    };
    
    sessions.set_repeat_mode(next_mode).await?;
    println!("Режим повтора: {:?}", next_mode);
    
    Ok(())
}
```

### 2. CLI переключатель

```rust
use media_sessions::{MediaSessions, RepeatMode};

fn repeat_icon(mode: RepeatMode) -> &'static str {
    match mode {
        RepeatMode::None => "🔁❌",
        RepeatMode::One => "🔂",
        RepeatMode::All => "🔁",
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sessions = MediaSessions::new()?;
    
    // Установить повтор одного трека
    sessions.set_repeat_mode(RepeatMode::One).await?;
    println!("Repeat: {}", repeat_icon(RepeatMode::One));
    
    Ok(())
}
```

### 3. Комбинация с Shuffle

```rust
use media_sessions::{MediaSessions, RepeatMode};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sessions = MediaSessions::new()?;
    
    // Вечеринка: shuffle + repeat all
    sessions.set_shuffle(true).await?;
    sessions.set_repeat_mode(RepeatMode::All).await?;
    
    println!("🎉 Party mode enabled!");
    
    Ok(())
}
```

### 4. Пресеты режимов

```rust
use media_sessions::{MediaSessions, RepeatMode};

struct PlaybackPresets;

impl PlaybackPresets {
    // Режим "Фокус" - без повтора, без shuffle
    async fn focus_mode(sessions: &MediaSessions) -> Result<(), Box<dyn std::error::Error>> {
        sessions.set_shuffle(false).await?;
        sessions.set_repeat_mode(RepeatMode::None).await?;
        Ok(())
    }
    
    // Режим "Тренировка" - repeat one для мотивации
    async fn workout_mode(sessions: &MediaSessions) -> Result<(), Box<dyn std::error::Error>> {
        sessions.set_shuffle(false).await?;
        sessions.set_repeat_mode(RepeatMode::One).await?;
        Ok(())
    }
    
    // Режим "Вечеринка" - shuffle + repeat all
    async fn party_mode(sessions: &MediaSessions) -> Result<(), Box<dyn std::error::Error>> {
        sessions.set_shuffle(true).await?;
        sessions.set_repeat_mode(RepeatMode::All).await?;
        Ok(())
    }
}
```

### 5. Мониторинг изменений

```rust
use media_sessions::{MediaSessions, MediaSessionEvent, RepeatMode};
use futures::StreamExt;

let mut stream = sessions.watch().await?;

while let Some(event) = stream.next().await {
    if let MediaSessionEvent::RepeatModeChanged { repeat, shuffle } = event? {
        println!("Repeat: {:?}, Shuffle: {}", repeat, shuffle);
    }
}
```

## Событие RepeatModeChanged

```rust
pub enum MediaSessionEvent {
    // ...
    RepeatModeChanged {
        repeat: RepeatMode,
        shuffle: bool,
    },
}
```

**Пример обработки:**

```rust
use media_sessions::{MediaSessions, MediaSessionEvent};
use futures::StreamExt;

let mut stream = sessions.watch().await?;

while let Some(event) = stream.next().await {
    match event? {
        MediaSessionEvent::RepeatModeChanged { repeat, shuffle } => {
            let repeat_icon = match repeat {
                RepeatMode::None => "🔁❌",
                RepeatMode::One => "🔂",
                RepeatMode::All => "🔁",
            };
            let shuffle_icon = if shuffle { "🔀" } else { "▶️" };
            
            println!("{} {} Режим обновлён", repeat_icon, shuffle_icon);
        }
        _ => {}
    }
}
```

## Платформенная поддержка

| Платформа | RepeatMode | Shuffle |
|-----------|------------|---------|
| Windows | ❌ Не поддерживается | ❌ Не поддерживается |
| macOS | 🟡 Частично | 🟡 Частично |
| Linux | ✅ Поддерживается | ✅ Поддерживается |

**Примечание:** На Windows SMTC API не предоставляет управление повтором и shuffle.

## Сериализация (с feature `serde`)

```toml
[dependencies]
media-sessions = { version = "0.2", features = ["serde"] }
serde = { version = "1.0", features = ["derive"] }
```

```rust
use media_sessions::RepeatMode;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct PlayerState {
    repeat_mode: RepeatMode,
    shuffle: bool,
}

let state = PlayerState {
    repeat_mode: RepeatMode::All,
    shuffle: true,
};

let json = serde_json::to_string(&state)?;
```

## См. также

- **[PlaybackStatus](playback-status.md)** — Статусы воспроизведения
- **[MediaSessions](media-sessions.md)** — Метод set_repeat_mode
- **[События](events.md)** — RepeatModeChanged event
