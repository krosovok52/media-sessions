# Установка

## Требования

| Требование | Версия | Описание |
|------------|--------|----------|
| **Rust** | 1.80+ | Минимальная поддерживаемая версия (MSRV) |
| **Tokio** | 1.0+ | Async runtime (требуется для API) |
| **CMake** | 3.1+ | Только для Windows (build dependency) |

## Установка из crates.io

Добавьте в `Cargo.toml`:

```toml
[dependencies]
media-sessions = "0.2"
tokio = { version = "1", features = ["full"] }
futures = "0.3"
```

Или используйте cargo add:

```bash
cargo add media-sessions tokio futures
```

## Установка из Git (development версия)

```toml
[dependencies]
media-sessions = { git = "https://github.com/krosovok52/media-sessions" }
```

## Feature Flags

Библиотека поддерживает селективную сборку для уменьшения размера бинарника:

| Фича | Описание | Зависимости |
|------|----------|-------------|
| `default` | Все платформы | — |
| `windows` | Только Windows | windows, windows-core |
| `macos` | Только macOS | objc2, objc2-foundation, core-foundation |
| `linux` | Только Linux | zbus |
| `tracing` | Tracing логи | tracing |
| `serde` | Сериализация | serde |
| `c-api` | C FFI для других языков | — |

### Примеры конфигурации

**Только Windows:**

```toml
[dependencies]
media-sessions = { version = "0.2", default-features = false, features = ["windows"] }
```

**Linux + tracing:**

```toml
[dependencies]
media-sessions = { version = "0.2", default-features = false, features = ["linux", "tracing"] }
```

**Все платформы + serde:**

```toml
[dependencies]
media-sessions = { version = "0.2", features = ["serde"] }
```

## Платформенные зависимости

### Windows

Требуется CMake для сборки:

```bash
# Установить через winget
winget install Kitware.CMake

# Или скачать с https://cmake.org/download/
```

### macOS

Никаких дополнительных зависимостей не требуется.

### Linux

Требуется D-Bus development файлы:

```bash
# Debian/Ubuntu
sudo apt install libdbus-1-dev

# Fedora
sudo dnf install dbus-devel

# Arch
sudo pacman -S dbus
```

## Проверка установки

Создайте тестовый проект:

```bash
cargo new test_media
cd test_media
cargo add media-sessions tokio futures
```

В `src/main.rs`:

```rust
use media_sessions::MediaSessions;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sessions = MediaSessions::new()?;
    println!("✅ Media Sessions initialized!");
    println!("Platform: {}", std::env::consts::OS);
    
    if let Some(info) = sessions.current().await? {
        println!("🎵 {} - {}", info.artist(), info.title());
    } else {
        println!("ℹ️ Нет активной медиа-сессии");
    }
    
    Ok(())
}
```

Запустите:

```bash
cargo run
```

## Сборка C API

Для использования из других языков:

```bash
# Сборка библиотеки
cargo build --release --features c-api

# Выходные файлы:
# Windows: target/release/media_sessions_c.dll
# Linux: target/release/libmedia_sessions_c.so
# macOS: target/release/libmedia_sessions_c.dylib
```

## Troubleshooting

### Ошибка: `package 'media-sessions v0.2.0' cannot be built`

**Решение:** Обновите Rust:

```bash
rustup update stable
```

### Ошибка: `cannot find function 'CoInitializeEx'`

**Решение:** Убедитесь, что feature `windows` включён:

```toml
[dependencies]
media-sessions = { version = "0.2", features = ["windows"] }
```

### Ошибка на Linux: `failed to run custom build command for zbus`

**Решение:** Установите D-Bus development файлы:

```bash
sudo apt install libdbus-1-dev
```

## Следующие шаги

- **[Quick Start](quickstart.md)** — Первый запуск за 5 минут
- **[Введение](introduction.md)** — Что такое Media Sessions
- **[Rust API](rust-api/README.md)** — Описание API
