# Python Integration

Using Media Sessions from Python via ctypes.

## Installation

### 1. Build the Library

```bash
# Build release version
cargo build --release --features c-api

# Copy DLL
# Windows:
cp target/release/media_sessions_c.dll ./

# Linux:
cp target/release/libmedia_sessions_c.so ./

# macOS:
cp target/release/libmedia_sessions_c.dylib ./
```

### 2. Install Dependencies

```bash
pip install ctypes  # Built into Python
```

## Usage

### Basic Example

```python
import ctypes
from ctypes import c_void_p, c_int32, c_uint64, c_char_p

# Load library
lib = ctypes.CDLL('./media_sessions_c.dll')  # or .so / .dylib

# Setup prototypes
lib.media_sessions_c_new.argtypes = []
lib.media_sessions_c_new.restype = c_void_p

lib.media_sessions_c_current.argtypes = [c_void_p]
lib.media_sessions_c_current.restype = c_void_p

lib.media_sessions_c_play.argtypes = [c_void_p]
lib.media_sessions_c_play.restype = c_int32

lib.media_sessions_c_free.argtypes = [c_void_p]
lib.media_sessions_c_free.restype = None

# Create session
handle = lib.media_sessions_c_new()
if not handle:
    raise RuntimeError("Failed to create session")

# Get current track
info_ptr = lib.media_sessions_c_current(handle)

# Cleanup
lib.media_sessions_c_free(handle)
```

### Full Wrapper Class

```python
import ctypes
from ctypes import (
    c_void_p, c_char_p, c_uint64, c_int32,
    c_double, c_bool, c_size_t, Structure, POINTER
)

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
    ]

class MediaSessions:
    def __init__(self, dll_path='./media_sessions_c.dll'):
        self.lib = ctypes.CDLL(dll_path)
        
        # Setup prototypes
        self.lib.media_sessions_c_new.argtypes = []
        self.lib.media_sessions_c_new.restype = c_void_p
        
        self.lib.media_sessions_c_current.argtypes = [c_void_p]
        self.lib.media_sessions_c_current.restype = c_void_p
        
        self.lib.media_sessions_c_play.argtypes = [c_void_p]
        self.lib.media_sessions_c_play.restype = c_int32
        
        self.lib.media_sessions_c_pause.argtypes = [c_void_p]
        self.lib.media_sessions_c_pause.restype = c_int32
        
        self.lib.media_sessions_c_free.argtypes = [c_void_p]
        self.lib.media_sessions_c_free.restype = None
        
        # Create session
        self.handle = self.lib.media_sessions_c_new()
        if not self.handle:
            raise RuntimeError("Failed to create session")
    
    def current(self):
        info_ptr = self.lib.media_sessions_c_current(self.handle)
        if info_ptr:
            return ctypes.cast(info_ptr, ctypes.POINTER(CMediaInfo)).contents
        return None
    
    def play(self):
        return self.lib.media_sessions_c_play(self.handle) == 0
    
    def pause(self):
        return self.lib.media_sessions_c_pause(self.handle) == 0
    
    def __del__(self):
        if hasattr(self, 'handle') and self.handle:
            self.lib.media_sessions_c_free(self.handle)

# Usage
if __name__ == '__main__':
    sessions = MediaSessions()
    info = sessions.current()
    if info:
        print(f"🎵 {info.artist.decode()} - {info.title.decode()}")
        print(f"💿 {info.album.decode()}")
        print(f"⏱ {info.position_secs}/{info.duration_secs} seconds")
```

### Discord Rich Presence Example

```python
from pypresence import Presence
import time
from media_sessions import MediaSessions

# Initialize Discord RPC
rpc = Presence("YOUR_CLIENT_ID")
rpc.connect()

# Initialize Media Sessions
sessions = MediaSessions()

print("Discord Rich Presence for Media Sessions")
print("Press Ctrl+C to stop")

try:
    while True:
        info = sessions.current()
        if info:
            rpc.update(
                state=info.title.decode(),
                details=info.artist.decode(),
                large_image="album_art",
                large_text=info.album.decode() if info.album else None,
                start=time.time() - info.position_secs,
                end=time.time() + (info.duration_secs - info.position_secs)
            )
        else:
            rpc.clear()
        time.sleep(1)
except KeyboardInterrupt:
    rpc.close()
```

## API Reference

### MediaSessions Class

| Method | Description | Returns |
|--------|-------------|---------|
| `__init__(dll_path)` | Create instance | `MediaSessions` |
| `current()` | Get current track | `CMediaInfo` or `None` |
| `play()` | Play | `bool` |
| `pause()` | Pause | `bool` |
| `play_pause()` | Toggle | `bool` |
| `next()` | Next track | `bool` |
| `previous()` | Previous track | `bool` |
| `seek(secs)` | Seek | `bool` |

### CMediaInfo Structure

| Field | Type | Description |
|-------|------|-------------|
| `title` | `str` | Track title |
| `artist` | `str` | Artist name |
| `album` | `str` | Album name |
| `duration_secs` | `int` | Duration in seconds |
| `position_secs` | `int` | Position in seconds |
| `playback_status` | `int` | 0=Playing, 1=Paused, 2=Stopped |
| `has_artwork` | `bool` | Has artwork |
| `artwork` | `bytes` | Artwork bytes |

## Troubleshooting

### DLL Not Found

```python
# Use full path
sessions = MediaSessions('/full/path/to/media_sessions_c.dll')
```

### Python Crash on Exit (Windows)

```python
import os
os._exit(0)  # Instead of sys.exit()
```

### Unicode Errors

```python
# Decode bytes to string
title = info.title.decode('utf-8', errors='ignore')
```

## See Also

- **[C API Reference](../c-api.md)** — Full C API documentation
- **[C#](csharp.md)** — Using from C#
- **[Node.js](nodejs.md)** — Using from JavaScript
