# События (Events)

Поток событий медиа-сессий для реактивного программирования.

## MediaSessionEvent

Перечисление всех возможных событий:

```rust
pub enum MediaSessionEvent {
    /// Метаданные трека изменились
    MetadataChanged(MediaInfo),
    
    /// Статус воспроизведения изменился
    PlaybackStatusChanged(PlaybackStatus),
    
    /// Позиция воспроизведения изменилась
    PositionChanged {
        position: Duration,
        old_position: Option<Duration>,
    },
    
    /// Открыта новая сессия
    SessionOpened {
        app_name: String,
    },
    
    /// Сессия закрыта
    SessionClosed,
    
    /// Обложка изменилась
    ArtworkChanged,
    
    /// Громкость изменилась (Linux)
    VolumeChanged {
        volume: f64,
    },
    
    /// Режим повтора изменился
    RepeatModeChanged {
        repeat: RepeatMode,
        shuffle: bool,
    },
}
```

## Получение потока событий

### Базовое использование

```rust
use media_sessions::{MediaSessions, MediaSessionEvent};
use futures::StreamExt;

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
```

### С debounce

```rust
use media_sessions::MediaSessions;
use std::time::Duration;

// Фильтрация событий (800ms по умолчанию)
let sessions = MediaSessions::builder()
    .debounce_duration(Duration::from_millis(500))
    .build()?;

let mut stream = sessions.watch().await?;
```

## Обработка событий

### 1. Все события

```rust
use media_sessions::{MediaSessions, MediaSessionEvent};
use futures::StreamExt;

let mut stream = sessions.watch().await?;

while let Some(event) = stream.next().await {
    match event? {
        MediaSessionEvent::MetadataChanged(info) => {
            println!("🎵 Metadata: {} - {}", info.artist(), info.title());
        }
        MediaSessionEvent::PlaybackStatusChanged(status) => {
            println!("▶️ Status: {:?}", status);
        }
        MediaSessionEvent::PositionChanged { position, old_position } => {
            println!("⏱ Position: {}s (was: {:?})", 
                position.as_secs(), 
                old_position.map(|d| d.as_secs())
            );
        }
        MediaSessionEvent::SessionOpened { app_name } => {
            println!("📱 Session opened: {}", app_name);
        }
        MediaSessionEvent::SessionClosed => {
            println!("📴 Session closed");
        }
        MediaSessionEvent::ArtworkChanged => {
            println!("🖼️ Artwork changed");
        }
        MediaSessionEvent::VolumeChanged { volume } => {
            println!("🔊 Volume: {:.0}%", volume * 100.0);
        }
        MediaSessionEvent::RepeatModeChanged { repeat, shuffle } => {
            println!("🔁 Repeat: {:?}, Shuffle: {}", repeat, shuffle);
        }
    }
}
```

### 2. Фильтрация событий

```rust
use media_sessions::{MediaSessions, MediaSessionEvent};
use futures::StreamExt;

let mut stream = sessions.watch().await?;

while let Some(event) = stream.next().await {
    match event? {
        // Только метаданные
        MediaSessionEvent::MetadataChanged(info) => {
            println!("🎵 {}", info.display_string());
        }
        // Игнорируем остальные
        _ => {}
    }
}
```

### 3. С tokio::select

```rust
use media_sessions::{MediaSessions, MediaSessionEvent};
use futures::StreamExt;
use tokio::sync::broadcast;
use tokio::time::{sleep, Duration};

let sessions = MediaSessions::new()?;
let mut stream = sessions.watch().await?;
let (tx, mut rx) = broadcast::channel(100);

loop {
    tokio::select! {
        // События медиа-сессии
        event = stream.next() => {
            if let Some(Ok(MediaSessionEvent::MetadataChanged(info))) = event {
                let msg = format!("Now playing: {}", info.display_string());
                let _ = tx.send(msg);
            }
        }
        
        // Другие задачи
        msg = rx.recv() => {
            println!("Received: {}", msg?);
        }
        
        // Таймер
        _ = sleep(Duration::from_secs(60)) => {
            println!("⏰ Minute passed");
        }
    }
}
```

### 4. С futures::stream

```rust
use media_sessions::{MediaSessions, MediaSessionEvent};
use futures::{StreamExt, stream};

let sessions = MediaSessions::new()?;
let mut stream = sessions
    .watch()
    .await?
    .filter_map(|event| async move {
        match event {
            Ok(MediaSessionEvent::MetadataChanged(info)) => Some(info),
            _ => None,
        }
    });

while let Some(info) = stream.next().await {
    println!("🎵 {}", info.display_string());
}
```

### 5. Логирование с tracing

```rust
use media_sessions::{MediaSessions, MediaSessionEvent};
use futures::StreamExt;
use tracing::{info, debug, warn};

let mut stream = sessions.watch().await?;

while let Some(event) = stream.next().await {
    match event? {
        MediaSessionEvent::MetadataChanged(info) => {
            info!(
                title = info.title(),
                artist = info.artist(),
                album = info.album(),
                "Metadata changed"
            );
        }
        MediaSessionEvent::PlaybackStatusChanged(status) => {
            debug!(status = ?status, "Playback status changed");
        }
        MediaSessionEvent::SessionClosed => {
            warn!("Session closed");
        }
        _ => {}
    }
}
```

## Debounce

Debounce фильтрует быстрые повторяющиеся события.

### Настройка debounce

```rust
use media_sessions::MediaSessions;
use std::time::Duration;

// 500ms debounce
let sessions = MediaSessions::builder()
    .debounce_duration(Duration::from_millis(500))
    .build()?;

// 100ms debounce для минимальной задержки
let sessions = MediaSessions::builder()
    .debounce_duration(Duration::from_millis(100))
    .build()?;

// Отключить debounce
let sessions = MediaSessions::builder()
    .debounce_duration(Duration::from_millis(0))
    .build()?;
```

### Когда использовать debounce

| Сценарий | Рекомендуемый debounce |
|----------|------------------------|
| UI обновление | 300-500ms |
| Логирование | 500-800ms |
| Highload мониторинг | 100-200ms |
| Real-time синхронизация | 0-50ms |

## Примеры использования

### 1. Discord Rich Presence

```rust
use media_sessions::{MediaSessions, MediaSessionEvent};
use futures::StreamExt;

async fn update_discord_presence(info: &MediaInfo) {
    // Обновление Discord Rich Presence
    println!("🎮 Discord: {} - {}", info.artist(), info.title());
}

let sessions = MediaSessions::new()?;
let mut stream = sessions.watch().await?;

while let Some(event) = stream.next().await {
    if let MediaSessionEvent::MetadataChanged(info) = event? {
        update_discord_presence(&info).await;
    }
}
```

### 2. OSD уведомление

```rust
use media_sessions::{MediaSessions, MediaSessionEvent, PlaybackStatus};
use futures::StreamExt;

let sessions = MediaSessions::new()?;
let mut stream = sessions.watch().await?;

while let Some(event) = stream.next().await {
    match event? {
        MediaSessionEvent::MetadataChanged(info) => {
            show_osd_notification(&format!(
                "🎵 {} - {}",
                info.artist(),
                info.title()
            ));
        }
        MediaSessionEvent::PlaybackStatusChanged(PlaybackStatus::Paused) => {
            show_osd_notification("⏸️ Paused");
        }
        _ => {}
    }
}

fn show_osd_notification(message: &str) {
    // Интеграция с libnotify (Linux) или аналогом
    println!("🔔 OSD: {}", message);
}
```

### 3. Веб-сервер статус

```rust
use media_sessions::{MediaSessions, MediaSessionEvent};
use axum::{Json, routing::get, Router};
use tokio::sync::RwLock;
use std::sync::Arc;

struct AppState {
    current_track: RwLock<Option<String>>,
}

async fn status(state: Arc<AppState>) -> Json<Option<String>> {
    Json(state.current_track.read().await.clone())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sessions = MediaSessions::new()?;
    let state = Arc::new(AppState {
        current_track: RwLock::new(None),
    });
    
    // Фоновая задача для событий
    let state_clone = state.clone();
    tokio::spawn(async move {
        let mut stream = sessions.watch().await.unwrap();
        while let Some(event) = stream.next().await {
            if let MediaSessionEvent::MetadataChanged(info) = event.unwrap() {
                *state_clone.current_track.write().await = Some(info.display_string());
            }
        }
    });
    
    // Веб-сервер
    let app = Router::new()
        .route("/status", get(status))
        .with_state(state);
    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;
    
    Ok(())
}
```

### 4. Запись истории

```rust
use media_sessions::{MediaSessions, MediaSessionEvent};
use futures::StreamExt;
use std::fs::OpenOptions;
use std::io::Write;

let sessions = MediaSessions::new()?;
let mut stream = sessions.watch().await?;
let mut file = OpenOptions::new()
    .create(true)
    .append(true)
    .open("music_history.log")?;

while let Some(event) = stream.next().await {
    if let MediaSessionEvent::MetadataChanged(info) = event? {
        let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S");
        writeln!(file, "[{}] {}", timestamp, info.display_string())?;
    }
}
```

## Производительность

### Пропускная способность

- **Без debounce:** ~850 событий/сек
- **С debounce 500ms:** ~2 события/сек
- **С debounce 800ms:** ~1 событие/сек

### Рекомендации

```rust
// Для UI - баланс между отзывчивостью и производительностью
let sessions = MediaSessions::builder()
    .debounce_duration(Duration::from_millis(300))
    .build()?;

// Для логирования - минимизация записи
let sessions = MediaSessions::builder()
    .debounce_duration(Duration::from_millis(1000))
    .build()?;

// Для real-time - минимальная задержка
let sessions = MediaSessions::builder()
    .debounce_duration(Duration::from_millis(50))
    .build()?;
```

## См. также

- **[MediaSessions](media-sessions.md)** — Метод watch()
- **[MediaInfo](media-info.md)** — Метаданные в событиях
- **[Производительность](../guides/performance.md)** — Оптимизация
