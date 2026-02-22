# Troubleshooting

Решение распространённых проблем с Media Sessions.

## Ошибки инициализации

### "Platform not supported"

**Ошибка:**

```
Error: NotSupported("unknown")
```

**Причина:** Библиотека собрана без платформенных feature flags.

**Решение:**

```toml
# Cargo.toml
[dependencies]
media-sessions = { version = "0.2", features = ["windows"] }  # Или linux/macos
```

### "No active media session"

**Ошибка:**

```
Error: NoSession
```

**Причина:** Нет запущенных медиа-плееров.

**Решение:**

1. Запустите медиа-плеер (Spotify, VLC, браузер с YouTube)
2. Начните воспроизведение
3. Проверьте снова

**Проверка:**

```rust
match sessions.current().await {
    Ok(Some(info)) => println!("Found: {}", info.display_string()),
    Ok(None) => println!("No active session - start a player"),
    Err(e) => eprintln!("Error: {}", e),
}
```

## Windows проблемы

### CMake ошибки сборки

**Ошибка:**

```
error: failed to run custom build command for `windows-rs`
CMake Error: Could not find CMake
```

**Решение:**

```bash
# Установить CMake
winget install Kitware.CMake

# Или скачать с https://cmake.org/download/

# Очистить и пересобрать
cargo clean
cargo build
```

### Плеер не обнаруживается

**Проблема:** Spotify/VLC не определяется

**Решение:**

**Для Spotify Desktop:**

1. Открыть настройки Spotify
2. Включить "Show media information when playing"

**Для VLC:**

1. Инструменты → Настройки
2. Включить "Show media info on taskbar"

**Для Firefox:**

1. Убедитесь, что видео играет
2. Попробуйте другой браузер (Edge/Chrome)

### SMTC не работает

**Проблема:** Windows 10 1803 или старше

**Решение:**

1. Проверьте версию Windows:
   ```
   winver
   ```
2. Обновитесь до Windows 10 1903+

## macOS проблемы

### Accessibility permissions

**Ошибка:**

```
Error: Backend { platform: "macos", message: "Access denied" }
```

**Решение:**

1. **System Preferences** → **Privacy & Security** → **Accessibility**
2. Нажмите **🔓** для разблокировки
3. Нажмите **+** и добавьте ваш терминал/IDE
4. Перезапустите приложение

```
┌────────────────────────────────────────────────────────┐
│  Privacy & Security > Accessibility                    │
│  ┌─────────────────────────────────────────────────┐  │
│  │ ☑ Terminal.app                                   │  │
│  │ ☑ Visual Studio Code.app                         │  │
│  │ ☐ Your App                                       │  │
│  └─────────────────────────────────────────────────┘  │
│                                                        │
│  [+]  [-]                                             │
└────────────────────────────────────────────────────────┘
```

### MediaRemote не работает

**Проблема:** macOS старше 12.0

**Решение:**

1. Проверьте версию macOS:
   ```
   sw_vers
   ```
2. Требуется macOS 12.0+ (Monterey)

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

## Linux проблемы

### D-Bus ошибки

**Ошибка:**

```
Error: Backend { platform: "linux", message: "Failed to connect to session bus" }
```

**Решение:**

```bash
# Проверка D-Bus
echo $DBUS_SESSION_BUS_ADDRESS

# Если пусто, запустить D-Bus
eval $(dbus-launch)

# Установить зависимости
sudo apt install libdbus-1-dev pkg-config

# Пересобрать
cargo clean
cargo build
```

### MPRIS плеер не обнаруживается

**Проверка:**

```bash
dbus-send --session --dest=org.freedesktop.DBus \
  --type=method_call --print-reply \
  /org/freedesktop/Bus org.freedesktop.DBus.ListNames | grep mpris
```

**Решение:**

**Для Firefox:**

1. Открыть `about:config`
2. Включить `media.hardwaremediakeys.enabled`

**Для mpv:**

Добавить в `~/.config/mpv/mpv.conf`:

```
input-mpremote-command=yes
```

**Для VLC:**

1. Инструменты → Настройки
2. Показать все → Интерфейсы
3. Включить "MPRIS"

### Volume control не работает

**Проблема:** `set_volume()` не имеет эффекта

**Причина:** Не все плееры поддерживают volume через MPRIS.

**Решение:**

Используйте системный volume control:

```rust
// Используйте platform-specific API для volume
// Например, libpulsebinding для PulseAudio
```

## Общие проблемы

### Таймауты операций

**Ошибка:**

```
Error: Timeout(5s)
```

**Решение:**

```rust
// Увеличить timeout
let sessions = MediaSessions::builder()
    .operation_timeout(Duration::from_secs(10))
    .build()?;
```

### Частые события (spam)

**Проблема:** Слишком много событий в потоке

**Решение:**

```rust
// Увеличить debounce
let sessions = MediaSessions::builder()
    .debounce_duration(Duration::from_millis(1000))
    .build()?;
```

### Обложки не загружаются

**Проблема:** `artwork` всегда `None`

**Причина:** Не все платформы поддерживают обложки.

**Поддержка:**

| Платформа | Artwork |
|-----------|---------|
| Windows | ❌ Не поддерживается SMTC |
| macOS | 🟡 Частично |
| Linux | ✅ Поддерживается |

**Решение:**

Используйте alternative API для обложек:

```rust
// Для Spotify можно использовать Spotify Web API
// Для YouTube - scraping (не рекомендуется)
```

### Высокая задержка

**Проблема:** `current()` занимает > 100ms

**Решение:**

1. **Кэширование:**

```rust
use std::time::{Duration, Instant};

struct CachedInfo {
    info: Option<MediaInfo>,
    timestamp: Instant,
}

impl CachedInfo {
    fn is_fresh(&self) -> bool {
        self.timestamp.elapsed() < Duration::from_millis(500)
    }
}
```

2. **Увеличить debounce:**

```rust
let sessions = MediaSessions::builder()
    .debounce_duration(Duration::from_millis(500))
    .build()?;
```

## Отладка

### Включение логов

**С tracing:**

```toml
[dependencies]
tracing = "0.1"
tracing-subscriber = "0.3"
```

```rust
use tracing_subscriber;

tracing_subscriber::fmt()
    .with_max_level(tracing::Level::DEBUG)
    .init();
```

**С log:**

```toml
[dependencies]
env_logger = "0.10"
log = "0.4"
```

```rust
env_logger::Builder::from_env(
    env_logger::Env::default().default_filter_or("debug")
).init();
```

### Проверка версии

```rust
println!("Version: {}", media_sessions::VERSION);
println!("Platform: {}", std::env::consts::OS);
```

### Diagnostic утилита

```rust
use media_sessions::MediaSessions;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 Media Sessions Diagnostic");
    println!("=".repeat(40));
    
    println!("Platform: {}", std::env::consts::OS);
    println!("Arch: {}", std::env::consts::ARCH);
    
    match MediaSessions::new() {
        Ok(sessions) => {
            println!("✅ Initialization: OK");
            
            match sessions.current().await {
                Ok(Some(info)) => {
                    println!("✅ Current track: {}", info.display_string());
                }
                Ok(None) => {
                    println!("ℹ️ No active session");
                }
                Err(e) => {
                    println!("❌ Query error: {}", e);
                }
            }
        }
        Err(e) => {
            println!("❌ Initialization: {}", e);
        }
    }
    
    Ok(())
}
```

## Получение помощи

### GitHub Issues

1. Проверьте существующие issues: https://github.com/krosovok52/media-sessions/issues
2. Создайте новый issue с:
   - Версией библиотеки
   - Платформой (OS, версия)
   - Шагами для воспроизведения
   - Логами ошибки

### Информация для отладки

```
Версия: media-sessions 0.2.0
Платформа: Windows 11 22H2
Rust: 1.80.0

Шаги:
1. cargo run
2. Ошибка: ...

Логи:
[DEBUG] ...
[ERROR] ...
```

## См. также

- **[Обработка ошибок](error-handling.md)** — Error handling
- **[Платформы](platforms/windows.md)** — Платформенные особенности
- **[GitHub Issues](https://github.com/krosovok52/media-sessions/issues)** — Сообщить о проблеме
