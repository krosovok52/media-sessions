# MediaInfo

Метаданные медиа-трека.

## Обзор

`MediaInfo` содержит полную информацию о текущем воспроизводимом треке, включая название, исполнителя, альбом, обложку и многое другое.

## Структура

```rust
pub struct MediaInfo {
    pub title: Option<String>,
    pub artist: Option<String>,
    pub album: Option<String>,
    pub duration: Option<Duration>,
    pub position: Option<Duration>,
    pub playback_status: PlaybackStatus,
    pub artwork: Option<Vec<u8>>,
    pub genre: Option<String>,
    pub year: Option<i32>,
    pub track_number: Option<u32>,
    pub disc_number: Option<u32>,
    pub url: Option<String>,
    pub thumbnail_url: Option<String>,
}
```

## Поля

| Поле | Тип | Описание | Пример |
|------|-----|----------|--------|
| `title` | `Option<String>` | Название трека | `"Bohemian Rhapsody"` |
| `artist` | `Option<String>` | Исполнитель | `"Queen"` |
| `album` | `Option<String>` | Название альбома | `"A Night at the Opera"` |
| `duration` | `Option<Duration>` | Общая длительность | `354 секунды` |
| `position` | `Option<Duration>` | Текущая позиция | `120 секунд` |
| `playback_status` | `PlaybackStatus` | Статус воспроизведения | `Playing`, `Paused` |
| `artwork` | `Option<Vec<u8>>` | Обложка (сырые байты) | PNG/JPEG данные |
| `genre` | `Option<String>` | Жанр | `"Rock"` |
| `year` | `Option<i32>` | Год выпуска | `1975` |
| `track_number` | `Option<u32>` | Номер трека в альбоме | `11` |
| `disc_number` | `Option<u32>` | Номер диска | `1` |
| `url` | `Option<String>` | URL источника | `"https://..."` |
| `thumbnail_url` | `Option<String>` | URL миниатюры | `"https://..."` |

## Методы

### `title()`

Возвращает название трека или пустую строку.

```rust
impl MediaInfo {
    pub fn title(&self) -> &str
}
```

**Пример:**

```rust
if let Some(info) = sessions.current().await? {
    println!("Название: {}", info.title());
    // "Bohemian Rhapsody"
}
```

### `artist()`

Возвращает имя исполнителя или пустую строку.

```rust
impl MediaInfo {
    pub fn artist(&self) -> &str
}
```

**Пример:**

```rust
println!("Исполнитель: {}", info.artist());
// "Queen"
```

### `album()`

Возвращает название альбома или пустую строку.

```rust
impl MediaInfo {
    pub fn album(&self) -> &str
}
```

### `display_string()`

Возвращает отформатированную строку "Artist - Title".

```rust
impl MediaInfo {
    pub fn display_string(&self) -> String
}
```

**Пример:**

```rust
println!("🎵 {}", info.display_string());
// "🎵 Queen - Bohemian Rhapsody"
```

### `duration_secs()`

Возвращает длительность в секундах.

```rust
impl MediaInfo {
    pub fn duration_secs(&self) -> u64
}
```

### `position_secs()`

Возвращает текущую позицию в секундах.

```rust
impl MediaInfo {
    pub fn position_secs(&self) -> u64
}
```

### `progress()`

Возвращает прогресс воспроизведения от 0.0 до 1.0.

```rust
impl MediaInfo {
    pub fn progress(&self) -> f64
}
```

**Пример:**

```rust
println!("Прогресс: {:.1}%", info.progress() * 100.0);
// "Прогресс: 33.9%"
```

### `progress_percent()`

Возвращает прогресс в процентах (от 0 до 100).

```rust
impl MediaInfo {
    pub fn progress_percent(&self) -> f64
}
```

### `is_playing()`

Проверяет, воспроизводится ли трек.

```rust
impl MediaInfo {
    pub fn is_playing(&self) -> bool
}
```

### `is_paused()`

Проверяет, находится ли трек на паузе.

```rust
impl MediaInfo {
    pub fn is_paused(&self) -> bool
}
```

### `artwork_format()`

Возвращает формат обложки (PNG или JPEG).

```rust
impl MediaInfo {
    pub fn artwork_format(&self) -> Option<&str>
}
```

## Примеры использования

### 1. Базовое получение информации

```rust
use media_sessions::MediaSessions;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sessions = MediaSessions::new()?;
    
    if let Some(info) = sessions.current().await? {
        println!("╔════════════════════════════════════════╗");
        println!("║         Now Playing                    ║");
        println!("╠════════════════════════════════════════╣");
        println!("║ 🎵 {} - {}", info.artist(), info.title());
        println!("║ 💿 {}", info.album());
        println!("║ 🎷 {}", info.genre());
        println!("║ 📅 {}", info.year().unwrap_or(0));
        println!("╠════════════════════════════════════════╣");
        println!("║ ⏱ {}/{} ({:.1}%)", 
            info.position_secs(),
            info.duration_secs(),
            info.progress_percent()
        );
        println!("╚════════════════════════════════════════╝");
    }
    
    Ok(())
}
```

### 2. Сохранение обложки

```rust
use std::fs;
use media_sessions::MediaSessions;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sessions = MediaSessions::new()?;
    
    if let Some(info) = sessions.current().await? {
        if let Some(artwork) = &info.artwork {
            let format = info.artwork_format().unwrap_or("png");
            let filename = format!("cover.{}", format);
            
            fs::write(&filename, artwork)?;
            println!("✅ Обложка сохранена в {}", filename);
        } else {
            println!("ℹ️ Обложка недоступна");
        }
    }
    
    Ok(())
}
```

### 3. Форматирование времени

```rust
fn format_time(secs: u64) -> String {
    let mins = secs / 60;
    let secs = secs % 60;
    format!("{}:{:02}", mins, secs)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sessions = MediaSessions::new()?;
    
    if let Some(info) = sessions.current().await? {
        println!("⏱ {} / {}", 
            format_time(info.position_secs()),
            format_time(info.duration_secs())
        );
    }
    
    Ok(())
}
```

### 4. Визуализация прогресс-бара

```rust
fn progress_bar(progress: f64, width: usize) -> String {
    let filled = (progress * width as f64) as usize;
    let empty = width - filled;
    
    format!(
        "[{}{}] {:.1}%",
        "█".repeat(filled),
        "░".repeat(empty),
        progress * 100.0
    )
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sessions = MediaSessions::new()?;
    
    if let Some(info) = sessions.current().await? {
        println!("🎵 {}", info.display_string());
        println!("{}", progress_bar(info.progress(), 30));
    }
    
    Ok(())
}
```

### 5. Discord Rich Presence

```rust
use discord_rich_presence::{DiscordIpc, DiscordIpcClient, activity::Activity};
use media_sessions::MediaSessions;

struct DiscordPresence {
    client: DiscordIpcClient,
}

impl DiscordPresence {
    fn update(&mut self, info: &MediaInfo) -> Result<(), Box<dyn std::error::Error>> {
        let activity = Activity::new()
            .state(info.title())
            .details(info.artist())
            .large_image("cover")
            .large_text(info.album());
        
        self.client.set_activity(activity)?;
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sessions = MediaSessions::new()?;
    let mut presence = DiscordPresence::new("YOUR_CLIENT_ID")?;
    
    if let Some(info) = sessions.current().await? {
        presence.update(&info)?;
    }
    
    Ok(())
}
```

### 6. Веб-сервер статус

```rust
use axum::{Json, routing::get, Router};
use media_sessions::{MediaSessions, MediaInfo};
use std::sync::Arc;
use tokio::sync::RwLock;

struct AppState {
    sessions: MediaSessions,
    current_track: RwLock<Option<MediaInfo>>,
}

async fn get_status(
    state: Arc<AppState>,
) -> Json<Option<serde_json::Value>> {
    let track = state.current_track.read().await;
    
    Json(track.as_ref().map(|info| {
        serde_json::json!({
            "title": info.title(),
            "artist": info.artist(),
            "album": info.album(),
            "duration_secs": info.duration_secs(),
            "position_secs": info.position_secs(),
            "progress_percent": info.progress_percent(),
            "is_playing": info.is_playing(),
        })
    }))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sessions = MediaSessions::new()?;
    
    let state = Arc::new(AppState {
        sessions,
        current_track: RwLock::new(None),
    });
    
    // Фоновая задача для обновления
    let state_clone = state.clone();
    tokio::spawn(async move {
        loop {
            if let Ok(Some(info)) = state_clone.sessions.current().await {
                *state_clone.current_track.write().await = Some(info);
            }
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        }
    });
    
    let app = Router::new()
        .route("/api/status", get(get_status))
        .with_state(state);
    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;
    
    Ok(())
}
```

## Обработка опциональных значений

Большинство полей `MediaInfo` — опциональные (`Option<T>`). Используйте следующие паттерны:

### Pattern matching

```rust
if let Some(album) = &info.album {
    println!("Альбом: {}", album);
} else {
    println!("Альбом неизвестен");
}
```

### unwrap_or

```rust
let year = info.year().unwrap_or(0);
println!("Год: {}", year);
```

### map

```rust
let track_str = info.track_number()
    .map(|t| format!("Трек {}", t))
    .unwrap_or_default();
```

### is_some / is_none

```rust
if info.artwork.is_some() {
    println!("Есть обложка");
}

if info.genre.is_none() {
    println!("Жанр не указан");
}
```

## См. также

- **[MediaSessions](media-sessions.md)** — Главный класс для получения MediaInfo
- **[PlaybackStatus](playback-status.md)** — Статусы воспроизведения
- **[События](events.md)** — MediaSessionEvent содержит MediaInfo
