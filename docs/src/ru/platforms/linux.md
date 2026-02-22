# Linux

Управление медиа-сессиями на Linux через D-Bus и MPRIS 2.0.

## Обзор

На Linux библиотека использует D-Bus для взаимодействия с MPRIS (Media Player Remote Interfacing Specification) совместимыми плеерами.

```
┌─────────────────────────────────────────────────────────┐
│              Media Sessions (Rust)                      │
├─────────────────────────────────────────────────────────┤
│              D-Bus Session Bus                          │
├─────────────────────────────────────────────────────────┤
│         MPRIS 2.0 Interface (org.mpris.MediaPlayer2)    │
├───────────┬─────────────┬──────────┬─────────┬──────────┤
│  Spotify  │   Firefox   │   VLC    │   mpv   │ Rhythmbox│
└───────────┴─────────────┴──────────┴─────────┴──────────┘
```

## Требования

| Требование | Версия | Описание |
|------------|--------|----------|
| **D-Bus** | 1.10+ | Session bus |
| **Rust** | 1.80+ | Минимальная поддерживаемая версия |
| **GLib** | 2.40+ | Для D-Bus (обычно есть в системе) |

## Установка

### Системные зависимости

**Debian/Ubuntu:**

```bash
sudo apt install libdbus-1-dev pkg-config
```

**Fedora:**

```bash
sudo dnf install dbus-devel pkg-config
```

**Arch:**

```bash
sudo pacman -S dbus pkgconf
```

### Rust зависимости

```toml
[dependencies]
media-sessions = { version = "0.2", features = ["linux"] }
tokio = { version = "1", features = ["full"] }
```

Или только Linux:

```toml
[dependencies]
media-sessions = { version = "0.2", default-features = false, features = ["linux"] }
```

## Поддерживаемые плееры

### ✅ Полная поддержка

| Приложение | Версия | Примечания |
|------------|--------|------------|
| **Spotify** | Любая | Официальный клиент |
| **Firefox** | 50+ | С включенным MPRIS |
| **VLC** | 3.0+ | С MPRIS плагином |
| **mpv** | 0.30+ | С `--input-mpremote-command` |
| **Rhythmbox** | Любая | GNOME плеер |
| **Lollypop** | Любая | GNOME плеер |
| **Tauon** | Любая | Современный плеер |
| **Chromium** | 50+ | В браузере |

### ⚠️ Частичная поддержка

| Приложение | Версия | Примечания |
|------------|--------|------------|
| **Chrome** | Любая | Ограниченный MPRIS |
| **Audacious** | 4.0+ | Требуется плагин |

### ❌ Не поддерживаются

| Приложение | Причина |
|------------|---------|
| **XMMS2** | Не использует MPRIS 2.0 |
| **Old плееры** | До MPRIS 2.0 |

## Настройка плееров

### Firefox

1. Открыть `about:config`
2. Найти `media.hardwaremediakeys.enabled`
3. Установить в `true`

### mpv

Создать или отредактировать `~/.config/mpv/mpv.conf`:

```
input-mpremote-command=yes
```

### VLC

MPRIS должен быть включён по умолчанию. Проверить:

1. Инструменты → Настройки
2. Показать все → Интерфейсы
3. Включить "MPRIS"

## Проверка доступных плееров

### Через D-Bus

```bash
# Список всех MPRIS плееров
dbus-send --session --dest=org.freedesktop.DBus \
  --type=method_call --print-reply \
  /org/freedesktop/Bus org.freedesktop.DBus.ListNames | grep mpris
```

### Через playerctl (для сравнения)

```bash
# Установить playerctl
sudo apt install playerctl

# Проверить доступные плееры
playerctl -l

# Получить текущий трек
playerctl metadata
```

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
        println!("⏱ {}/{} seconds", info.position_secs(), info.duration_secs());
    }

    Ok(())
}
```

### 2. Проверка доступных плееров

```rust
use zbus::Connection;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let connection = Connection::session().await?;
    
    let proxy = zbus::fdo::DBusProxy::new(&connection).await?;
    let names = proxy.list_names().await?;
    
    println!("📻 Available MPRIS players:");
    for name in names.iter().filter(|n| n.starts_with("org.mpris.MediaPlayer2")) {
        println!("  - {}", name);
    }
    
    Ok(())
}
```

### 3. Управление громкостью (Linux only)

```rust
use media_sessions::MediaSessions;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sessions = MediaSessions::new()?;
    
    // Установить громкость 50%
    sessions.set_volume(0.5).await?;
    println!("🔊 Volume set to 50%");
    
    // Установить громкость 75%
    sessions.set_volume(0.75).await?;
    println!("🔊 Volume set to 75%");
    
    Ok(())
}
```

### 4. Режимы повтора (Linux only)

```rust
use media_sessions::{MediaSessions, RepeatMode};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sessions = MediaSessions::new()?;
    
    // Repeat one
    sessions.set_repeat_mode(RepeatMode::One).await?;
    println!("🔂 Repeat one enabled");
    
    // Repeat all
    sessions.set_repeat_mode(RepeatMode::All).await?;
    println!("🔁 Repeat all enabled");
    
    // Shuffle
    sessions.set_shuffle(true).await?;
    println!("🔀 Shuffle enabled");
    
    Ok(())
}
```

### 5. CLI утилита

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

        println!("╔════════════════════════════════════════╗");
        println!("║         Now Playing                    ║");
        println!("╠════════════════════════════════════════╣");
        println!("║ {} {}", icon, info.display_string());
        if let Some(album) = &info.album {
            println!("║ 💿 {}", album);
        }
        println!("║ 🎷 {:?}", info.genre());
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

## MPRIS спецификация

### Object Path

```
/org/mpris/MediaPlayer2
```

### Interfaces

- `org.mpris.MediaPlayer2` — основной интерфейс
- `org.mpris.MediaPlayer2.Player` — управление плеером
- `org.mpris.MediaPlayer2.TrackList` — список треков (опционально)
- `org.mpris.MediaPlayer2.Playlists` — плейлисты (опционально)

### Properties

```dbus
# Metadata
/org/mpris/MediaPlayer2 Player Metadata

# Playback status
/org/mpris/MediaPlayer2 Player PlaybackStatus

# Volume
/org/mpris/MediaPlayer2 Player Volume

# Position
/org/mpris/MediaPlayer2 Player Position
```

## Troubleshooting

### Плеер не обнаруживается

**Проверка D-Bus:**

```bash
dbus-send --session --dest=org.freedesktop.DBus \
  --type=method_call --print-reply \
  /org/freedesktop/Bus org.freedesktop.DBus.ListNames | grep mpris
```

**Решение:**

1. Убедитесь, что плеер запущен
2. Проверьте MPRIS поддержку плеера
3. Перезапустите D-Bus session

### Ошибка D-Bus

```
Error: Backend { platform: "linux", message: "Failed to connect to session bus" }
```

**Решение:**

```bash
# Проверка D-Bus
echo $DBUS_SESSION_BUS_ADDRESS

# Если пусто, запустить D-Bus
eval $(dbus-launch)

# Пересобрать с зависимостями
sudo apt install libdbus-1-dev
cargo build
```

### mpv не отвечает

**Решение:**

1. Добавить в `~/.config/mpv/mpv.conf`:
   ```
   input-mpremote-command=yes
   ```
2. Перезапустить mpv

### Firefox не показывает MPRIS

**Решение:**

1. Открыть `about:config`
2. Найти `media.hardwaremediakeys.enabled`
3. Установить в `true`
4. Перезапустить Firefox

## Производительность

| Операция | Время | Примечания |
|----------|-------|------------|
| `current()` | ~2.0 ms | D-Bus вызов |
| `watch()` first event | ~3.0 ms | Подписка на события |
| Event throughput | ~500/sec | Пропускная способность |

## Интеграция с systemd

### Создание сервиса

```ini
# ~/.config/systemd/user/media-monitor.service
[Unit]
Description=Media Sessions Monitor

[Service]
Type=simple
ExecStart=/home/user/.cargo/bin/media-monitor
Restart=on-failure

[Install]
WantedBy=default.target
```

**Использование:**

```bash
# Перезагрузить systemd
systemctl --user daemon-reload

# Включить автозапуск
systemctl --user enable media-monitor

# Запустить
systemctl --user start media-monitor

# Проверить статус
systemctl --user status media-monitor
```

## См. также

- **[Windows](platforms/windows.md)** — Windows реализация
- **[macOS](platforms/macos.md)** — macOS реализация
- **[Обработка ошибок](../guides/error-handling.md)** — Гайд по ошибкам
