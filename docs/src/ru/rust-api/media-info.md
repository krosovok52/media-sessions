# MediaInfo

Структура с метаданными текущего трека.

## Получение MediaInfo

```rust
use media_sessions::MediaSessions;

let sessions = MediaSessions::new()?;

if let Some(info) = sessions.current().await? {
    // Работа с info
}
```

## Поля структуры

| Поле | Тип | Описание |
|------|-----|----------|
| `title` | `Option<String>` | Название трека |
| `artist` | `Option<String>` | Исполнитель |
| `album` | `Option<String>` | Название альбома |
| `duration` | `Option<Duration>` | Общая длительность |
| `position` | `Option<Duration>` | Текущая позиция |
| `playback_status` | `PlaybackStatus` | Статус воспроизведения |
| `artwork` | `Option<Vec<u8>>` | Обложка (сырые байты PNG/JPEG) |
| `genre` | `Option<String>` | Жанр |
| `year` | `Option<i32>` | Год выпуска |
| `track_number` | `Option<u32>` | Номер трека в альбоме |
| `disc_number` | `Option<u32>` | Номер диска |
| `url` | `Option<String>` | URL источника |
| `thumbnail_url` | `Option<String>` | URL миниатюры обложки |

## Методы

### Базовые аксессоры

```rust
impl MediaInfo {
    /// Название трека (пустая строка если None)
    pub fn title(&self) -> &str
    
    /// Исполнитель (пустая строка если None)
    pub fn artist(&self) -> &str
    
    /// Альбом (пустая строка если None)
    pub fn album(&self) -> &str
    
    /// Жанр (пустая строка если None)
    pub fn genre(&self) -> &str
}
```

**Пример:**

```rust
println!("Title: {}", info.title());    // "Song Title"
println!("Artist: {}", info.artist());  // "Artist Name"
println!("Album: {}", info.album());    // "Album Name"
println!("Genre: {}", info.genre());    // "Rock"
```

### Длительность и позиция

```rust
impl MediaInfo {
    /// Длительность в секундах
    pub fn duration_secs(&self) -> u64
    
    /// Позиция в секундах
    pub fn position_secs(&self) -> u64
    
    /// Прогресс от 0.0 до 1.0
    pub fn progress(&self) -> f64
    
    /// Прогресс в процентах (0 to 100)
    pub fn progress_percent(&self) -> f64
}
```

**Пример:**

```rust
println!("Duration: {}s", info.duration_secs());  // 240
println!("Position: {}s", info.position_secs());  // 60
println!("Progress: {:.1}%", info.progress_percent());  // 25.0%
```

### Форматирование

```rust
impl MediaInfo {
    /// "Artist - Title"
    pub fn display_string(&self) -> String
    
    /// Проверка на Playing
    pub fn is_playing(&self) -> bool
    
    /// Проверка на Paused
    pub fn is_paused(&self) -> bool
    
    /// Формат обложки (PNG/JPEG)
    pub fn artwork_format(&self) -> Option<&str>
}
```

**Пример:**

```rust
println!("🎵 {}", info.display_string());  // "Artist - Song"
println!("Status: {}", if info.is_playing() { "▶️" } else { "⏸️" });
println!("Artwork: {:?}", info.artwork_format());  // Some("PNG")
```

## Примеры использования

### 1. Базовый вывод информации

```rust
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
```

### 2. Сохранение обложки

```rust
use std::fs;

if let Some(info) = sessions.current().await? {
    if let Some(artwork) = &info.artwork {
        let format = info.artwork_format().unwrap_or("png");
        let filename = format!("cover.{}", format);
        fs::write(&filename, artwork)?;
        println!("✅ Обложка сохранена в {}", filename);
    }
}
```

### 3. Форматирование времени

```rust
fn format_time(secs: u64) -> String {
    let mins = secs / 60;
    let secs = secs % 60;
    format!("{}:{:02}", mins, secs)
}

if let Some(info) = sessions.current().await? {
    println!("⏱ {} / {}", 
        format_time(info.position_secs()),
        format_time(info.duration_secs())
    );
}
```

### 4. Проверка статуса

```rust
use media_sessions::PlaybackStatus;

if let Some(info) = sessions.current().await? {
    match info.playback_status {
        PlaybackStatus::Playing => println!("▶️ Playing"),
        PlaybackStatus::Paused => println!("⏸️ Paused"),
        PlaybackStatus::Stopped => println!("⏹️ Stopped"),
        PlaybackStatus::Transitioning => println!("⏳ Transitioning"),
    }
    
    // Или через helper методы
    if info.is_playing() {
        println!("▶️");
    } else if info.is_paused() {
        println!("⏸️");
    }
}
```

### 5. Расширенная информация

```rust
if let Some(info) = sessions.current().await? {
    // Основная информация
    println!("Title: {}", info.title());
    println!("Artist: {}", info.artist());
    println!("Album: {}", info.album());
    
    // Дополнительная информация
    if let Some(genre) = &info.genre {
        println!("Genre: {}", genre);
    }
    if let Some(year) = info.year() {
        println!("Year: {}", year);
    }
    if let Some(track) = info.track_number() {
        println!("Track: {}", track);
    }
    if let Some(disc) = info.disc_number() {
        println!("Disc: {}", disc);
    }
    
    // URL
    if let Some(url) = &info.url {
        println!("URL: {}", url);
    }
    if let Some(thumb) = &info.thumbnail_url {
        println!("Thumbnail: {}", thumb);
    }
}
```

## Работа с Option полями

Большинство полей `MediaInfo` — опциональные. Используйте паттерн matching:

```rust
// Pattern matching
if let Some(album) = &info.album {
    println!("Album: {}", album);
}

// unwrap_or
let year = info.year().unwrap_or(0);
println!("Year: {}", year);

// map
let track_str = info.track_number()
    .map(|t| format!("Track {}", t))
    .unwrap_or_default();

// is_some
if info.artwork.is_some() {
    println!("Есть обложка");
}
```

## Полная структура

```rust
pub struct MediaInfo {
    pub title: Option<String>,
    pub artist: Option<String>,
    pub album: Option<String>,
    pub duration: Option<std::time::Duration>,
    pub position: Option<std::time::Duration>,
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

## См. также

- **[MediaSessions](media-sessions.md)** — Главный класс
- **[PlaybackStatus](playback-status.md)** — Статусы воспроизведения
- **[События](events.md)** — MediaSessionEvent содержит MediaInfo
