# Как использовать media-sessions

## Установка

### Способ 1: Из crates.io (рекомендуется)

В файле `Cargo.toml` вашего проекта добавьте:

```toml
[dependencies]
media-sessions = "0.2"
tokio = { version = "1", features = ["full"] }
```

### Способ 2: Из GitHub

```toml
[dependencies]
media-sessions = { git = "https://github.com/krosovok52/media-sessions" }
tokio = { version = "1", features = ["full"] }
```

### Способ 3: Локально

```toml
[dependencies]
media-sessions = { path = "путь/к/папке/MediaSession" }
tokio = { version = "1", features = ["full"] }
```

## Быстрый старт

### 1. Получить текущий трек

```rust
use media_sessions::MediaSessions;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Создаём сессию
    let sessions = MediaSessions::new()?;
    
    // Получаем информацию о текущем треке
    if let Some(info) = sessions.current().await? {
        println!("🎵 Играет: {} - {}", info.artist(), info.title());
        println!("💿 Альбом: {}", info.album());
        println!("▶️ Статус: {}", info.playback_status);
    } else {
        println!("Ничего не играет");
    }
    
    Ok(())
}
```

### 2. Управление воспроизведением

```rust
use media_sessions::MediaSessions;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sessions = MediaSessions::new()?;
    
    // Play/Pause
    sessions.play().await?;
    // или
    sessions.pause().await?;
    // или
    sessions.play_pause().await?;
    
    // Следующий/предыдущий трек
    sessions.next().await?;
    sessions.previous().await?;
    
    // Перемотка
    sessions.seek(Duration::from_secs(30)).await?; // на 30 секунд
    
    // Громкость (0.0 - 1.0)
    sessions.set_volume(0.5).await?;
    
    Ok(())
}
```

### 3. Слушать события

```rust
use media_sessions::{MediaSessions, MediaSessionEvent};
use futures::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sessions = MediaSessions::new()?;
    
    // Подписываемся на события
    let mut stream = sessions.watch().await?;
    
    println!("Слушаем события...");
    
    while let Some(event) = stream.next().await {
        match event? {
            MediaSessionEvent::MetadataChanged(info) => {
                println!("🎵 Теперь играет: {}", info.display_string());
            }
            MediaSessionEvent::PlaybackStatusChanged(status) => {
                println!("▶️ Статус: {}", status);
            }
            MediaSessionEvent::PositionChanged { position, .. } => {
                println!("⏱️ Позиция: {} сек", position.as_secs());
            }
            _ => {}
        }
    }
    
    Ok(())
}
```

### 4. Полное приложение

```rust
use media_sessions::{MediaSessions, PlaybackStatus};
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Инициализация
    let sessions = MediaSessions::builder()
        .debounce_duration(Duration::from_millis(500))
        .operation_timeout(Duration::from_secs(5))
        .build()?;
    
    println!("🎵 Media Sessions v0.2.0\n");
    
    // Получаем текущий трек
    match sessions.current().await? {
        Some(info) => {
            println!("╔════════════════════════════════════════════╗");
            println!("║         Сейчас играет                      ║");
            println!("╠════════════════════════════════════════════╣");
            println!("║ Название: {:<30} ║", info.title());
            println!("║ Артист:   {:<30} ║", info.artist());
            println!("║ Альбом:   {:<30} ║", info.album());
            println!("║ Статус:   {:<30} ║", info.playback_status);
            println!("╚════════════════════════════════════════════╝");
            
            // Если пауза - запускаем
            if info.is_paused() {
                println!("\n▶️ Запускаем...");
                sessions.play().await?;
            }
        }
        None => {
            println!("❌ Нет активной медиа-сессии");
            println!("💡 Запустите Spotify, Яндекс.Музыку или другой плеер");
        }
    }
    
    Ok(())
}
```

## Требования

- **Rust:** 1.80+
- **Tokio:** 1.43+ (с features: `["full"]`)
- **Futures:** 0.3+

## Поддерживаемые платформы

| ОС | Мин. версия | Статус |
|----|-------------|--------|
| Windows 10/11 | 1803+ | ✅ |
| macOS | 12+ (Monterey) | ✅ |
| Linux | Любой с D-Bus | ✅ |

## Примеры

Больше примеров в репозитории:
- https://github.com/krosovok52/media-sessions/tree/main/examples

## Документация

Полная API документация:
- https://docs.rs/media-sessions

## Проблемы?

- **Нет активной сессии:** Убедитесь, что медиаплеер запущен
- **Windows:** Требуется версия 1803+
- **Linux:** Проверьте `dbus-send --session ...`

## Контакты

- **GitHub:** https://github.com/krosovok52/media-sessions
- **Telegram:** https://t.me/krosov_ok
- **Issues:** https://github.com/krosovok52/media-sessions/issues
