# Error Handling

Complete guide to error handling in Media Sessions.

## Error Types

### MediaError

The main error type:

```rust
pub enum MediaError {
    /// Platform not supported
    NotSupported(String),

    /// No active media session
    NoSession,

    /// Backend error
    Backend {
        platform: String,
        message: String,
    },

    /// Operation timeout
    Timeout(Duration),

    /// Invalid argument
    InvalidArgument(String),
}
```

## Pattern Matching

### Basic Handling

```rust
use media_sessions::{MediaSessions, MediaError};

match MediaSessions::new() {
    Ok(sessions) => { /* OK */ }
    Err(MediaError::NotSupported(platform)) => {
        eprintln!("Platform {} not supported", platform);
    }
    Err(MediaError::NoSession) => {
        eprintln!("No active media session");
    }
    Err(MediaError::Backend { platform, message }) => {
        eprintln!("Backend error on {}: {}", platform, message);
    }
    Err(MediaError::Timeout(duration)) => {
        eprintln!("Timeout after {:?}", duration);
    }
    Err(e) => {
        eprintln!("Error: {}", e);
    }
}
```

## ? Operator

### Simple Propagation

```rust
fn get_current_track() -> Result<Option<MediaInfo>, MediaError> {
    let sessions = MediaSessions::new()?;  // ? propagates error
    let info = sessions.current().await?;  // ? propagates error
    Ok(info)
}
```

### Error Conversion

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Media error: {0}")]
    Media(#[from] media_sessions::MediaError),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

async fn run() -> Result<(), AppError> {
    let sessions = MediaSessions::new()?;  // Auto-converts
    Ok(())
}
```

## Logging Errors

### With tracing

```rust
use media_sessions::MediaSessions;
use tracing::{error, warn, info};

async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let sessions = match MediaSessions::new() {
        Ok(s) => s,
        Err(e) => {
            error!(error = %e, "Failed to initialize");
            return Err(e.into());
        }
    };

    match sessions.current().await {
        Ok(Some(info)) => {
            info!(title = info.title(), "Track updated");
        }
        Ok(None) => {
            warn!("No active session");
        }
        Err(e) => {
            error!(error = %e, "Failed to get track");
        }
    }

    Ok(())
}
```

## Best Practices

### 1. Always Handle NoSession

```rust
match sessions.current().await {
    Ok(Some(info)) => { /* Has track */ }
    Ok(None) => { /* No session - normal */ }
    Err(e) => { /* Error */ }
}
```

### 2. Log at Appropriate Level

```rust
// Info for normal events
info!("Track changed: {}", info.display_string());

// Warn for expected issues
warn!("No active session");

// Error for unexpected errors
error!("Backend error: {}", e);
```

### 3. Use Context

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

### 4. Avoid unwrap in Production

```rust
// ❌ Bad
let info = sessions.current().await.unwrap();

// ✅ Good
let info = match sessions.current().await {
    Ok(Some(info)) => info,
    Ok(None) => return,  // Or default
    Err(e) => {
        error!("Error: {}", e);
        return;
    }
};
```

## See Also

- **[MediaSessions](rust-api/media-sessions.md)** — Main class
- **[tracing](https://docs.rs/tracing)** — Logging
- **[anyhow](https://docs.rs/anyhow)** — Error handling
