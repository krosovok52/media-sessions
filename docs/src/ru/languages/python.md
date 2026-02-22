# Python

Использование Media Sessions из Python через ctypes.

## Установка

```bash
# 1. Собрать библиотеку
cargo build --release --features c-api

# 2. Скопировать DLL в проект
# Windows:
cp target/release/media_sessions_c.dll ./

# Linux:
cp target/release/libmedia_sessions_c.so ./

# macOS:
cp target/release/libmedia_sessions_c.dylib ./
```

## Класс-обёртка

```python
import ctypes
from ctypes import (
    c_void_p, c_char_p, c_uint8, c_uint64, c_uint32,
    c_int32, c_double, c_bool, c_size_t, Structure, POINTER
)
from typing import Optional

class CMediaInfo(Structure):
    _fields_ = [
        ("title", c_char_p),
        ("artist", c_char_p),
        ("album", c_char_p),
        ("duration_secs", c_uint64),
        ("position_secs", c_uint64),
        ("playback_status", c_int32),
        ("has_artwork", c_bool),
        ("artwork_len", c_size_t),
        ("artwork", POINTER(c_uint8)),
        ("track_number", c_uint32),
        ("disc_number", c_uint32),
        ("genre", c_char_p),
        ("year", c_int32),
        ("url", c_char_p),
        ("thumbnail_url", c_char_p),
    ]

class MediaSessions:
    """Python wrapper for Media Sessions C API."""
    
    PLAYBACK_STATUS = {
        0: "Playing",
        1: "Paused",
        2: "Stopped",
    }
    
    def __init__(self, library_path: str = "./media_sessions_c.dll"):
        self._lib = ctypes.CDLL(library_path)
        self._setup_prototypes()
        self._handle = self._lib.media_sessions_c_new()
        
        if not self._handle:
            raise RuntimeError("Failed to create Media Sessions instance")
    
    def _setup_prototypes(self):
        """Setup C function prototypes."""
        # Creation/Deletion
        self._lib.media_sessions_c_new.argtypes = []
        self._lib.media_sessions_c_new.restype = c_void_p
        
        self._lib.media_sessions_c_free.argtypes = [c_void_p]
        self._lib.media_sessions_c_free.restype = None
        
        # Current track
        self._lib.media_sessions_c_current.argtypes = [c_void_p]
        self._lib.media_sessions_c_current.restype = POINTER(CMediaInfo)
        
        # Playback control
        self._lib.media_sessions_c_play.argtypes = [c_void_p]
        self._lib.media_sessions_c_play.restype = c_int32
        
        self._lib.media_sessions_c_pause.argtypes = [c_void_p]
        self._lib.media_sessions_c_pause.restype = c_int32
        
        self._lib.media_sessions_c_play_pause.argtypes = [c_void_p]
        self._lib.media_sessions_c_play_pause.restype = c_int32
        
        self._lib.media_sessions_c_stop.argtypes = [c_void_p]
        self._lib.media_sessions_c_stop.restype = c_int32
        
        self._lib.media_sessions_c_next.argtypes = [c_void_p]
        self._lib.media_sessions_c_next.restype = c_int32
        
        self._lib.media_sessions_c_previous.argtypes = [c_void_p]
        self._lib.media_sessions_c_previous.restype = c_int32
        
        self._lib.media_sessions_c_seek.argtypes = [c_void_p, c_uint64]
        self._lib.media_sessions_c_seek.restype = c_int32
        
        # Free info
        self._lib.media_sessions_c_free_info.argtypes = [POINTER(CMediaInfo)]
        self._lib.media_sessions_c_free_info.restype = None
    
    def current(self) -> Optional[dict]:
        """Get current track info."""
        info_ptr = self._lib.media_sessions_c_current(self._handle)
        
        if not info_ptr:
            return None
        
        info = info_ptr.contents
        
        result = {
            "title": info.title.decode('utf-8') if info.title else None,
            "artist": info.artist.decode('utf-8') if info.artist else None,
            "album": info.album.decode('utf-8') if info.album else None,
            "duration_secs": info.duration_secs,
            "position_secs": info.position_secs,
            "playback_status": self.PLAYBACK_STATUS.get(info.playback_status, "Unknown"),
            "genre": info.genre.decode('utf-8') if info.genre else None,
            "year": info.year if info.year else None,
            "track_number": info.track_number,
            "has_artwork": info.has_artwork,
        }
        
        # Save artwork if available
        if info.has_artwork and info.artwork_len > 0:
            artwork_bytes = bytes(info.artwork[:info.artwork_len])
            result["artwork"] = artwork_bytes
        
        self._lib.media_sessions_c_free_info(info_ptr)
        return result
    
    def play(self) -> bool:
        """Play."""
        return self._lib.media_sessions_c_play(self._handle) == 0
    
    def pause(self) -> bool:
        """Pause."""
        return self._lib.media_sessions_c_pause(self._handle) == 0
    
    def play_pause(self) -> bool:
        """Toggle Play/Pause."""
        return self._lib.media_sessions_c_play_pause(self._handle) == 0
    
    def stop(self) -> bool:
        """Stop."""
        return self._lib.media_sessions_c_stop(self._handle) == 0
    
    def next(self) -> bool:
        """Next track."""
        return self._lib.media_sessions_c_next(self._handle) == 0
    
    def previous(self) -> bool:
        """Previous track."""
        return self._lib.media_sessions_c_previous(self._handle) == 0
    
    def seek(self, seconds: int) -> bool:
        """Seek to position."""
        return self._lib.media_sessions_c_seek(self._handle, seconds) == 0
    
    def save_artwork(self, filepath: str) -> bool:
        """Save artwork to file."""
        info = self.current()
        if info and info.get("artwork"):
            with open(filepath, 'wb') as f:
                f.write(info["artwork"])
            return True
        return False
    
    def __del__(self):
        """Cleanup."""
        if hasattr(self, '_handle') and self._handle:
            self._lib.media_sessions_c_free(self._handle)
```

## Примеры использования

### 1. Базовое использование

```python
from media_sessions import MediaSessions

# Создание экземпляра
sessions = MediaSessions()

# Получить текущий трек
info = sessions.current()
if info:
    print(f"🎵 {info['artist']} - {info['title']}")
    print(f"💿 {info['album']}")
    print(f"⏱ {info['position_secs']}/{info['duration_secs']} seconds")
    print(f"▶️ {info['playback_status']}")

# Управление
sessions.play()
sessions.seek(30)  # Перемотать на 30 секунд
```

### 2. Простой плеер контроллер

```python
import time
from media_sessions import MediaSessions

sessions = MediaSessions()

# Play/Pause цикл
sessions.play()
time.sleep(5)
sessions.pause()
time.sleep(2)
sessions.play_pause()  # Toggle

# Следующий трек
sessions.next()
```

### 3. Монитор текущего трека

```python
import time
from media_sessions import MediaSessions

sessions = MediaSessions()

print("🎵 Media Sessions Monitor")
print("=" * 40)

try:
    while True:
        info = sessions.current()
        if info:
            status_icon = "▶️" if info['playback_status'] == "Playing" else "⏸️"
            print(f"\r{status_icon} {info['artist']} - {info['title']}", end='')
        time.sleep(1)
except KeyboardInterrupt:
    print("\n👋 Stopped")
```

### 4. Сохранение обложки

```python
from media_sessions import MediaSessions

sessions = MediaSessions()

info = sessions.current()
if info and info.get('has_artwork'):
    sessions.save_artwork("cover.jpg")
    print("✅ Обложка сохранена в cover.jpg")
else:
    print("ℹ️ Обложка не доступна")
```

### 5. CLI утилита

```python
#!/usr/bin/env python3
import sys
from media_sessions import MediaSessions

def main():
    try:
        sessions = MediaSessions()
    except RuntimeError as e:
        print(f"❌ Error: {e}", file=sys.stderr)
        return 1
    
    info = sessions.current()
    if not info:
        print("ℹ️ Нет активной медиа-сессии")
        return 0
    
    # Форматированный вывод
    status_icon = "▶️" if info['playback_status'] == "Playing" else "⏸️"
    
    print("╔════════════════════════════════════════╗")
    print("║         Now Playing                    ║")
    print("╠════════════════════════════════════════╣")
    print(f"║  {status_icon} {info['artist']} - {info['title']:<20} ║")
    
    if info.get('album'):
        print(f"║  💿 {info['album']:<28} ║")
    
    if info.get('genre'):
        print(f"║  🎷 {info['genre']:<28} ║")
    
    progress = (info['position_secs'] / info['duration_secs'] * 100) if info['duration_secs'] > 0 else 0
    print(f"║  ⏱ {info['position_secs']}/{info['duration_secs']}s ({progress:.1f}%){' ' * 10} ║")
    print("╚════════════════════════════════════════╝")
    
    return 0

if __name__ == "__main__":
    sys.exit(main())
```

### 6. Интеграция с Discord Rich Presence

```python
from pypresence import Presence
import time
from media_sessions import MediaSessions

# Discord RPC
RPC = Presence("YOUR_CLIENT_ID")
RPC.connect()

sessions = MediaSessions()

last_track = None

while True:
    info = sessions.current()
    
    if info and info.get('title'):
        track_key = f"{info['artist']}-{info['title']}"
        
        if track_key != last_track:
            RPC.update(
                state=info['title'],
                details=info['artist'],
                large_image="cover",
                large_text=info.get('album', ''),
                buttons=[{"label": "Listen", "url": "https://spotify.com"}]
            )
            last_track = track_key
    
    time.sleep(15)
```

## Обработка ошибок

```python
from media_sessions import MediaSessions

try:
    sessions = MediaSessions()
except RuntimeError as e:
    print(f"❌ Failed to initialize: {e}")
    exit(1)

try:
    info = sessions.current()
    if info:
        print(f"🎵 {info['title']}")
    else:
        print("ℹ️ No active session")
except Exception as e:
    print(f"❌ Error: {e}")
```

## Платформенные особенности

### Windows

```python
# DLL путь
sessions = MediaSessions("./media_sessions_c.dll")
```

### Linux

```python
# SO путь
sessions = MediaSessions("./libmedia_sessions_c.so")
```

### macOS

```python
# DYLIB путь
sessions = MediaSessions("./libmedia_sessions_c.dylib")
```

## Производительность

| Операция | Время |
|----------|-------|
| `current()` | ~1-2 ms |
| `play()` | ~1-2 ms |
| `seek()` | ~1-2 ms |

## См. также

- **[C API Reference](c-api.md)** — Полная документация C API
- **[C#](languages/csharp.md)** — Использование из .NET
- **[Node.js](languages/nodejs.md)** — Использование из JavaScript
