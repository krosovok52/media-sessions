# macOS

Управление медиа-сессиями на macOS через MediaRemote.framework.

## Обзор

На macOS библиотека использует приватный `MediaRemote.framework` для взаимодействия с системными медиа-сессиями.

```
┌─────────────────────────────────────────────────────────┐
│              Media Sessions (Rust)                      │
├─────────────────────────────────────────────────────────┤
│          MediaRemote.framework (Private)                │
├─────────────────────────────────────────────────────────┤
│  Spotify  │  Apple Music  │  VLC  │  Safari  │  Chrome  │
└───────────┴───────────────┴───────┴─────────┴───────────┘
```

## Требования

| Требование | Версия | Описание |
|------------|--------|----------|
| **macOS** | 12.0+ (Monterey) | Минимальная версия |
| **Rust** | 1.80+ | Минимальная поддерживаемая версия |
| **Xcode** | 13.0+ | Для сборки |

## Установка

```toml
[dependencies]
media-sessions = { version = "0.2", features = ["macos"] }
tokio = { version = "1", features = ["full"] }
```

Или только macOS:

```toml
[dependencies]
media-sessions = { version = "0.2", default-features = false, features = ["macos"] }
```

## Permissions (Разрешения)

### Accessibility Permissions

Для некоторых функций может потребоваться доступ Accessibility.

**Настройка:**

1. **System Preferences** → **Privacy & Security** → **Accessibility**
2. Нажмите **+** и добавьте ваш терминал или приложение
3. Перезапустите приложение

```
┌────────────────────────────────────────────────────────┐
│  Privacy & Security > Accessibility                    │
│  ┌─────────────────────────────────────────────────┐  │
│  │ ☑ Terminal                                       │  │
│  │ ☑ Visual Studio Code                             │  │
│  │ ☐ Your App                                       │  │
│  └─────────────────────────────────────────────────┘  │
│                                                        │
│  [+]  [-]                                             │
└────────────────────────────────────────────────────────┘
```

### Проверка доступа

```rust
// Проверка доступности
fn check_accessibility() -> bool {
    #[cfg(target_os = "macos")]
    {
        use objc2_foundation::NSBundle;
        // Проверка через AXIsProcessTrusted
        unsafe { macos_accessibility::AXIsProcessTrusted() }
    }
    #[cfg(not(target_os = "macos"))]
    {
        true
    }
}
```

## Поддерживаемые плееры

### ✅ Полная поддержка

| Приложение | Версия | Примечания |
|------------|--------|------------|
| **Spotify** | Любая | Desktop приложение |
| **Apple Music** | Любая | Системный плеер |
| **YouTube** | Safari/Chrome | В браузере |
| **SoundCloud** | Safari/Chrome | В браузере |
| **VLC** | 3.0+ | С включенными media keys |
| **IINA** | Любая | Современный плеер |

### ⚠️ Частичная поддержка

| Приложение | Версия | Примечания |
|------------|--------|------------|
| **Firefox** | Любая | Ограниченный доступ |
| **QuickTime** | Любая | Только базовый контроль |

### ❌ Не поддерживаются

| Приложение | Причина |
|------------|---------|
| **Old плееры** | До macOS 12 |
| **Не-MPRIS** | Без MediaRemote |

## Настройка плееров

### VLC

1. VLC → Preferences
2. Show All → Advanced
3. Enable "Control Center" integration

### Firefox

1. Открыть `about:config`
2. Включить `media.hardwaremediakeys.enabled`

## Примеры использования

### 1. Базовое использование

```rust
use media_sessions::MediaSessions;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sessions = MediaSessions::new()?;

    if let Some(info) = sessions.current().await? {
        println!("🎵 {} - {}", info.artist(), info.title());
        println!("💿 {}", info.album());
    }

    Ok(())
}
```

### 2. Проверка permissions

```rust
use media_sessions::MediaSessions;

fn check_permissions() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(target_os = "macos")]
    {
        println!("🔒 Checking Accessibility permissions...");
        
        // Попытка создания сессии
        match MediaSessions::new() {
            Ok(_) => println!("✅ Permissions OK"),
            Err(e) => {
                eprintln!("❌ Error: {}", e);
                eprintln!("💡 Please grant Accessibility permissions:");
                eprintln!("   System Preferences → Privacy & Security → Accessibility");
                return Err(e.into());
            }
        }
    }
    
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    check_permissions()?;
    
    let sessions = MediaSessions::new()?;
    
    if let Some(info) = sessions.current().await? {
        println!("🎵 {} - {}", info.display_string());
    }
    
    Ok(())
}
```

### 3. Мониторинг событий

```rust
use media_sessions::{MediaSessions, MediaSessionEvent};
use futures::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sessions = MediaSessions::builder()
        .debounce_duration(std::time::Duration::from_millis(500))
        .build()?;
    
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
    
    Ok(())
}
```

## Ограничения MediaRemote

### Не поддерживается

| Функция | Причина |
|---------|---------|
| **Volume control** | Ограниченный доступ |
| **Repeat mode** | Частично поддерживается |
| **Shuffle** | Частично поддерживается |
| **Artwork** | Ограниченный доступ |

### Примечания

- MediaRemote — **приватный фреймворк** Apple
- API может измениться в будущих версиях macOS
- Некоторые функции требуют Accessibility permissions

## Troubleshooting

### Плеер не обнаруживается

**Проверка:**

```bash
# Проверка активных сессий
log show --predicate 'eventMessage contains "NowPlaying"' --last 1m
```

**Решение:**

1. Убедитесь, что плеер запущен
2. Проверьте Accessibility permissions
3. Перезапустите плеер

### Ошибка доступа

```
Error: Backend { platform: "macos", message: "Access denied" }
```

**Решение:**

1. System Preferences → Privacy & Security → Accessibility
2. Добавьте ваш терминал/IDE
3. Перезапустите приложение

### Ошибка сборки

```
error: failed to run custom build command for `objc2`
```

**Решение:**

```bash
# Установить Xcode Command Line Tools
xcode-select --install

# Очистить и пересобрать
cargo clean
cargo build
```

## Производительность

| Операция | Время | Примечания |
|----------|-------|------------|
| `current()` | ~1.8 ms | Асинхронный вызов |
| `watch()` first event | ~2.5 ms | Первое событие |
| Event throughput | ~400/sec | Пропускная способность |

## См. также

- **[Windows](platforms/windows.md)** — Windows реализация
- **[Linux](platforms/linux.md)** — Linux реализация
- **[Обработка ошибок](../guides/error-handling.md)** — Гайд по ошибкам
