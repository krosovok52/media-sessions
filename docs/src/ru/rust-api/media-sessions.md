# MediaSessions

Главный класс для управления медиа-сессиями.

## Создание экземпляра

### Базовое создание

```rust
use media_sessions::MediaSessions;

// Простое создание
let sessions = MediaSessions::new()?;
```

### Создание с конфигурацией

```rust
use media_sessions::MediaSessions;
use std::time::Duration;

let sessions = MediaSessions::builder()
    .debounce_duration(Duration::from_millis(500))  // Фильтрация событий
    .operation_timeout(Duration::from_secs(10))     // Таймаут операций
    .enable_artwork(true)                           // Загрузка обложек
    .build()?;
```

## Методы управления

### Запрос информации

#### `current()`

Получить информацию о текущем треке.

```rust
if let Some(info) = sessions.current().await? {
    println!("🎵 {} - {}", info.artist(), info.title());
    println!("💿 {}", info.album());
    println!("⏱ {}/{}", info.position_secs(), info.duration_secs());
}
```

**Возвращает:** `Result<Option<MediaInfo>, MediaError>`

#### `active_app()`

Получить имя активного медиа-приложения.

```rust
if let Some(app) = sessions.active_app().await? {
    println!("▶️ Приложение: {}", app);
}
```

**Возвращает:** `Result<Option<String>, MediaError>`

### Управление воспроизведением

#### `play()`

Запустить воспроизведение.

```rust
sessions.play().await?;
```

#### `pause()`

Приостановить воспроизведение.

```rust
sessions.pause().await?;
```

#### `play_pause()`

Переключить состояние Play/Pause.

```rust
sessions.play_pause().await?;
```

#### `stop()`

Остановить воспроизведение.

```rust
sessions.stop().await?;
```

#### `next()`

Следующий трек.

```rust
sessions.next().await?;
```

#### `previous()`

Предыдущий трек.

```rust
sessions.previous().await?;
```

#### `seek(duration)`

Перемотать на указанную позицию.

```rust
use std::time::Duration;

// Перемотать на 30 секунд
sessions.seek(Duration::from_secs(30)).await?;

// Перемотать на 1 минуту
sessions.seek(Duration::from_secs(60)).await?;
```

### Расширенное управление

#### `set_volume(level)`

Установить громкость (только Linux).

```rust
// Громкость 50%
sessions.set_volume(0.5).await?;

// Громкость 75%
sessions.set_volume(0.75).await?;
```

**Параметры:** `level: f64` (от 0.0 до 1.0)

**Возвращает:** `Result<(), MediaError>`

**Примечание:** На Windows и macOS этот метод может не поддерживаться.

#### `set_repeat_mode(mode)`

Установить режим повтора.

```rust
use media_sessions::RepeatMode;

// Повтор выключен
sessions.set_repeat_mode(RepeatMode::None).await?;

// Повтор одного трека
sessions.set_repeat_mode(RepeatMode::One).await?;

// Повтор всех
sessions.set_repeat_mode(RepeatMode::All).await?;
```

**Параметры:** `mode: RepeatMode`

#### `set_shuffle(enabled)`

Включить/выключить перемешивание.

```rust
// Включить shuffle
sessions.set_shuffle(true).await?;

// Выключить shuffle
sessions.set_shuffle(false).await?;
```

**Параметры:** `enabled: bool`

### Поток событий

#### `watch()`

Создать поток событий медиа-сессии.

```rust
use media_sessions::{MediaSessions, MediaSessionEvent};
use futures::StreamExt;

let mut stream = sessions.watch().await?;

while let Some(event) = stream.next().await {
    match event? {
        MediaSessionEvent::MetadataChanged(info) => {
            println!("🎵 Теперь: {}", info.display_string());
        }
        MediaSessionEvent::PlaybackStatusChanged(status) => {
            println!("▶️ Статус: {:?}", status);
        }
        MediaSessionEvent::PositionChanged { position, .. } => {
            println!("⏱ Позиция: {}s", position.as_secs());
        }
        _ => {}
    }
}
```

**Возвращает:** `Result<impl Stream<Item = MediaSessionEvent>, MediaError>`

## Builder паттерн

### `MediaSessionsBuilder`

| Метод | Описание | По умолчанию |
|-------|----------|--------------|
| `debounce_duration(Duration)` | Фильтрация событий | 800ms |
| `operation_timeout(Duration)` | Таймаут операций | 5s |
| `enable_artwork(bool)` | Загрузка обложек | true |

### Примеры

**Минимальная конфигурация:**

```rust
let sessions = MediaSessions::builder()
    .debounce_duration(Duration::from_millis(300))
    .build()?;
```

**Для highload приложений:**

```rust
let sessions = MediaSessions::builder()
    .debounce_duration(Duration::from_millis(100))  // Минимальная задержка
    .operation_timeout(Duration::from_secs(3))      // Быстрый таймаут
    .enable_artwork(false)                          // Без обложек
    .build()?;
```

## Примеры использования

### 1. Простой плеер контроллер

```rust
use media_sessions::MediaSessions;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sessions = MediaSessions::new()?;

    // Play/Pause цикл
    sessions.play().await?;
    tokio::time::sleep(Duration::from_secs(5)).await;
    sessions.pause().await?;
    tokio::time::sleep(Duration::from_secs(2)).await;
    
    // Следующий трек
    sessions.next().await?;

    Ok(())
}
```

### 2. Монитор текущего трека

```rust
use media_sessions::MediaSessions;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sessions = MediaSessions::new()?;

    loop {
        if let Some(info) = sessions.current().await? {
            println!("╔════════════════════════════════════════╗");
            println!("║         Now Playing                    ║");
            println!("╠════════════════════════════════════════╣");
            println!("║ {} - {}", info.artist(), info.title());
            if let Some(album) = &info.album {
                println!("║ 💿 {}", album);
            }
            println!("╚════════════════════════════════════════╝");
        }
        
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }
}
```

### 3. Интеграция с tokio::select

```rust
use media_sessions::{MediaSessions, MediaSessionEvent};
use futures::StreamExt;
use tokio::sync::broadcast;

struct MediaMonitor {
    sessions: MediaSessions,
    event_tx: broadcast::Sender<String>,
}

impl MediaMonitor {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let sessions = MediaSessions::builder()
            .debounce_duration(std::time::Duration::from_millis(500))
            .build()?;

        let (event_tx, _) = broadcast::channel(100);
        Ok(Self { sessions, event_tx })
    }

    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut stream = self.sessions.watch().await?;
        let mut rx = self.event_tx.subscribe();

        loop {
            tokio::select! {
                event = stream.next() => {
                    if let Some(Ok(MediaSessionEvent::MetadataChanged(info))) = event {
                        let msg = format!("Now playing: {}", info.display_string());
                        let _ = self.event_tx.send(msg);
                    }
                }
                msg = rx.recv() => {
                    println!("Received: {}", msg?);
                }
            }
        }
    }
}
```

### 4. CLI утилита

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

        println!("{} {}", icon, info.display_string());
        
        if let Some(album) = &info.album {
            println!("💿 {}", album);
        }
        
        let progress = info.progress_percent();
        println!("📊 {:.1}%", progress);
    }

    Ok(())
}
```

## Обработка ошибок

```rust
use media_sessions::{MediaSessions, MediaError};

match sessions.play().await {
    Ok(()) => println!("✅ Play successful"),
    Err(MediaError::NoSession) => println!("❌ Нет активной сессии"),
    Err(MediaError::NotSupported(_)) => println!("❌ Не поддерживается"),
    Err(MediaError::Timeout(d)) => println!("❌ Таймаут после {:?}", d),
    Err(e) => println!("❌ Ошибка: {}", e),
}
```

## См. также

- **[MediaInfo](media-info.md)** — Метаданные трека
- **[PlaybackStatus](playback-status.md)** — Статусы воспроизведения
- **[События](events.md)** — Поток событий
- **[Обработка ошибок](../guides/error-handling.md)** — Гайд по ошибкам
