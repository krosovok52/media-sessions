# Обработка ошибок

Полное руководство по обработке ошибок в Media Sessions.

## Типы ошибок

### MediaError

Основной тип ошибок библиотеки:

```rust
pub enum MediaError {
    /// Платформа не поддерживается
    NotSupported(String),
    
    /// Нет активной медиа-сессии
    NoSession,
    
    /// Ошибка бэкенда
    Backend {
        platform: String,
        message: String,
    },
    
    /// Таймаут операции
    Timeout(Duration),
    
    /// Неверный аргумент
    InvalidArgument(String),
    
    /// D-Bus ошибка (Linux)
    #[cfg(target_os = "linux")]
    DBus(#[from] zbus::Error),
    
    /// WinRT ошибка (Windows)
    #[cfg(target_os = "windows")]
    Winrt(#[from] windows::core::Error),
}
```

## Pattern matching

### Базовая обработка

```rust
use media_sessions::{MediaSessions, MediaError};

match MediaSessions::new() {
    Ok(sessions) => {
        println!("✅ Initialized");
    }
    Err(MediaError::NotSupported(platform)) => {
        eprintln!("❌ Platform {} not supported", platform);
    }
    Err(MediaError::NoSession) => {
        eprintln!("ℹ️ No active media session");
    }
    Err(MediaError::Backend { platform, message }) => {
        eprintln!("❌ Backend error on {}: {}", platform, message);
    }
    Err(MediaError::Timeout(duration)) => {
        eprintln!("⏱ Operation timed out after {:?}", duration);
    }
    Err(e) => {
        eprintln!("❌ Error: {}", e);
    }
}
```

### С использованием if let

```rust
let sessions = match MediaSessions::new() {
    Ok(s) => s,
    Err(MediaError::NotSupported(_)) => {
        eprintln!("Platform not supported");
        return;
    }
    Err(e) => {
        eprintln!("Error: {}", e);
        return;
    }
};
```

## ? оператор

### Простая передача ошибок

```rust
fn get_current_track() -> Result<Option<MediaInfo>, MediaError> {
    let sessions = MediaSessions::new()?;  // ? передаёт ошибку
    let info = sessions.current().await?;  // ? передаёт ошибку
    Ok(info)
}
```

### Преобразование ошибок

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Media error: {0}")]
    Media(#[from] media_sessions::MediaError),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Other error: {0}")]
    Other(String),
}

async fn run() -> Result<(), AppError> {
    let sessions = MediaSessions::new()?;  // Автоматически конвертируется
    let info = sessions.current().await?;
    Ok(())
}
```

## Обработка в async коде

### Tokio spawn

```rust
use media_sessions::{MediaSessions, MediaError};

tokio::spawn(async {
    let sessions = match MediaSessions::new() {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Spawn error: {}", e);
            return;
        }
    };
    
    match sessions.current().await {
        Ok(Some(info)) => println!("Track: {}", info.display_string()),
        Ok(None) => println!("No active session"),
        Err(e) => eprintln!("Query error: {}", e),
    }
});
```

### Tokio select

```rust
use media_sessions::{MediaSessions, MediaSessionEvent};
use futures::StreamExt;

loop {
    tokio::select! {
        event = stream.next() => {
            match event {
                Some(Ok(event)) => {
                    // Обработка события
                }
                Some(Err(e)) => {
                    eprintln!("Stream error: {}", e);
                    break;
                }
                None => {
                    eprintln!("Stream ended");
                    break;
                }
            }
        }
        _ = shutdown_rx.recv() => {
            break;
        }
    }
}
```

## Логирование ошибок

### С tracing

```rust
use media_sessions::MediaSessions;
use tracing::{error, warn, info};

async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let sessions = match MediaSessions::new() {
        Ok(s) => s,
        Err(e) => {
            error!(error = %e, "Failed to initialize Media Sessions");
            return Err(e.into());
        }
    };
    
    match sessions.current().await {
        Ok(Some(info)) => {
            info!(
                title = info.title(),
                artist = info.artist(),
                "Track updated"
            );
        }
        Ok(None) => {
            warn!("No active media session");
        }
        Err(e) => {
            error!(error = %e, "Failed to get current track");
        }
    }
    
    Ok(())
}
```

### С log

```rust
use media_sessions::MediaSessions;
use log::{error, warn, info};

match sessions.current().await {
    Ok(Some(info)) => info!("Playing: {}", info.display_string()),
    Ok(None) => warn!("No session"),
    Err(e) => error!("Error: {}", e),
}
```

## Retry логика

### С экспоненциальной задержкой

```rust
use media_sessions::{MediaSessions, MediaError};
use std::time::Duration;

async fn with_retry<F, T>(mut f: F, max_retries: u32) -> Result<T, MediaError>
where
    F: FnMut() -> futures::future::BoxFuture<'static, Result<T, MediaError>>,
{
    let mut delay = Duration::from_millis(100);
    
    for attempt in 0..max_retries {
        match f().await {
            Ok(result) => return Ok(result),
            Err(MediaError::Timeout(_)) if attempt < max_retries - 1 => {
                tokio::time::sleep(delay).await;
                delay *= 2;  // Экспоненциальная задержка
            }
            Err(e) => return Err(e),
        }
    }
    
    Err(MediaError::Timeout(delay))
}

// Использование
let info = with_retry(
    || Box::pin(sessions.current()),
    3
).await?;
```

### С tokio::retry

```rust
use tokio_retry::{Retry, strategy::ExponentialBackoff};

let strategy = ExponentialBackoff::from_millis(100).take(3);

let info = Retry::spawn(strategy, || {
    sessions.current()
}).await?;
```

## Graceful degradation

### Fallback значения

```rust
let sessions = MediaSessions::new().unwrap_or_else(|_| {
    eprintln!("⚠️ Running without media sessions");
    // Вернуть mock или отключить функциональность
});

let info = sessions.current().await.unwrap_or_else(|_| {
    eprintln!("⚠️ Using cached data");
    None  // Или кэшированные данные
});
```

### Feature detection

```rust
async fn try_set_volume(
    sessions: &MediaSessions,
    level: f64,
) -> Result<(), MediaError> {
    match sessions.set_volume(level).await {
        Ok(()) => Ok(()),
        Err(MediaError::NotSupported(_)) => {
            warn!("Volume control not supported");
            Ok(())  // Игнорировать
        }
        Err(e) => Err(e),
    }
}
```

## Пользовательские сообщения

### Для CLI

```rust
fn format_error(e: &MediaError) -> String {
    match e {
        MediaError::NotSupported(platform) => {
            format!("Media sessions not supported on {}", platform)
        }
        MediaError::NoSession => {
            "No media player is currently active".to_string()
        }
        MediaError::Backend { platform, message } => {
            format!("Backend error on {}: {}", platform, message)
        }
        MediaError::Timeout(d) => {
            format!("Operation timed out after {}ms", d.as_millis())
        }
        _ => format!("Error: {}", e),
    }
}

// Использование
match sessions.current().await {
    Ok(info) => { /* ... */ }
    Err(e) => {
        eprintln!("❌ {}", format_error(&e));
    }
}
```

### Для GUI

```rust
enum UserMessage {
    Info(String),
    Warning(String),
    Error(String),
}

fn to_user_message(e: &MediaError) -> UserMessage {
    match e {
        MediaError::NoSession => {
            UserMessage::Info("No media player is active".to_string())
        }
        MediaError::NotSupported(_) => {
            UserMessage::Warning("Media control not supported on this platform".to_string())
        }
        _ => UserMessage::Error(format!("Media error: {}", e)),
    }
}
```

## Тестирование ошибок

### Unit тесты

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_no_session_error() {
        let sessions = MediaSessions::new().unwrap();
        
        // Имитация отсутствия сессии
        // (зависит от реализации)
    }
    
    #[tokio::test]
    async fn test_timeout_error() {
        // Тест таймаута
    }
}
```

### Mocking

```rust
#[cfg(test)]
use mockall::{automock, predicate::*};

#[automock]
trait MediaSessionsTrait {
    async fn current(&self) -> Result<Option<MediaInfo>, MediaError>;
}

#[tokio::test]
async fn test_error_handling() {
    let mut mock = MockMediaSessionsTrait::new();
    mock.expect_current()
        .times(1)
        .returning(|| Err(MediaError::NoSession));
    
    assert!(matches!(
        mock.current().await,
        Err(MediaError::NoSession)
    ));
}
```

## Best Practices

### 1. Всегда обрабатывайте NoSession

```rust
match sessions.current().await {
    Ok(Some(info)) => { /* Есть трек */ }
    Ok(None) => { /* Нет сессии - это нормально */ }
    Err(e) => { /* Ошибка */ }
}
```

### 2. Логируйте на соответствующем уровне

```rust
// Info для нормальных событий
info!("Track changed: {}", info.display_string());

// Warn для ожидаемых проблем
warn!("No active session");

// Error для неожиданных ошибок
error!("Backend error: {}", e);
```

### 3. Используйте контекст

```rust
use anyhow::{Context, Result};

async fn run() -> Result<()> {
    let sessions = MediaSessions::new()
        .context("Failed to initialize media sessions")?;
    
    let info = sessions.current().await
        .context("Failed to get current track")?;
    
    Ok(())
}
```

### 4. Избегайте unwrap в production

```rust
// ❌ Плохо
let info = sessions.current().await.unwrap();

// ✅ Хорошо
let info = match sessions.current().await {
    Ok(Some(info)) => info,
    Ok(None) => return,  // Или default значение
    Err(e) => {
        error!("Error: {}", e);
        return;
    }
};
```

## См. также

- **[MediaSessions](rust-api/media-sessions.md)** — Главный класс
- **[tracing](https://docs.rs/tracing)** — Логирование
- **[anyhow](https://docs.rs/anyhow)** — Обработка ошибок
- **[thiserror](https://docs.rs/thiserror)** — Пользовательские ошибки
