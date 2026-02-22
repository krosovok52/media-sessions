# C API for Media Sessions

This directory contains the C FFI API for using `media-sessions` from other programming languages.

## Building

### Build the C API Library

```bash
# Build release version
cargo build --release --features c-api

# Build debug version
cargo build --features c-api
```

### Output Files

| Platform | Library File |
|----------|-------------|
| Windows | `target/release/media_sessions_c.dll` |
| Linux | `target/release/libmedia_sessions_c.so` |
| macOS | `target/release/libmedia_sessions_c.dylib` |

## Installation

### Windows

Copy `target/release/media_sessions_c.dll` to:
- Your application directory, or
- System directory (`C:\Windows\System32`)

### Linux

```bash
# Copy to system library path
sudo cp target/release/libmedia_sessions_c.so /usr/lib/
sudo ldconfig

# Or add to LD_LIBRARY_PATH
export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:$(pwd)/target/release
```

### macOS

```bash
# Copy to system library path
sudo cp target/release/libmedia_sessions_c.dylib /usr/local/lib/

# Or add to DYLD_LIBRARY_PATH
export DYLD_LIBRARY_PATH=$DYLD_LIBRARY_PATH:$(pwd)/target/release
```

## Usage Examples

### Python

```python
from media_sessions import MediaSessions

# Create instance
sessions = MediaSessions()

# Get current track
info = sessions.current()
if info:
    print(f"Now playing: {info.artist.decode()} - {info.title.decode()}")

# Control playback
sessions.play()
sessions.pause()
sessions.next()
sessions.seek(30)  # Seek to 30 seconds

# Set volume (0.0 to 1.0)
sessions.set_volume(0.5)
```

Run the example:
```bash
python c-api/python_example.py
```

### C#

```csharp
using MediaSessions;

using var sessions = new MediaSessionsWrapper();

// Get current track
var info = sessions.Current();
if (info != null)
{
    Console.WriteLine($"Now playing: {info.Artist} - {info.Title}");
}

// Control playback
sessions.Play();
sessions.Pause();
sessions.Next();
sessions.Seek(30);

// Set volume
sessions.SetVolume(0.5);
```

Build and run:
```bash
# Copy library to output directory
cp target/release/media_sessions_c.dll .  # Windows
# or
cp target/release/libmedia_sessions_c.so .  # Linux

# Compile
csc /platform:x64 c-api/csharp_example.cs

# Run
csharp_example.exe
```

### C/C++

```c
#include "media_sessions_c.h"
#include <stdio.h>

int main() {
    // Create instance
    MediaSessionsHandle* sessions = media_sessions_c_new();
    if (!sessions) {
        printf("Failed to create sessions\n");
        return 1;
    }

    // Get current track
    CMediaInfo* info = media_sessions_c_current(sessions);
    if (info) {
        printf("Now playing: %s - %s\n", info->artist, info->title);
        media_sessions_c_free_info(info);
    }

    // Control playback
    media_sessions_c_play(sessions);
    media_sessions_c_pause(sessions);
    media_sessions_c_next(sessions);
    media_sessions_c_seek(sessions, 30);

    // Cleanup
    media_sessions_c_free(sessions);
    return 0;
}
```

Compile:
```bash
gcc -o example example.c -I./c-api -L./target/release -lmedia_sessions_c
./example
```

### Node.js (via node-ffi-napi)

```javascript
const ffi = require('ffi-napi');
const ref = require('ref-napi');

const lib = ffi.Library('media_sessions_c', {
    'media_sessions_c_new': ['pointer', []],
    'media_sessions_c_free': ['void', ['pointer']],
    'media_sessions_c_current': ['pointer', ['pointer']],
    'media_sessions_c_play': ['int', ['pointer']],
    'media_sessions_c_pause': ['int', ['pointer']],
});

const sessions = lib.media_sessions_c_new();
const info = lib.media_sessions_c_current(sessions);
// ... process info
lib.media_sessions_c_free(sessions);
```

## API Reference

### Core Functions

| Function | Description |
|----------|-------------|
| `media_sessions_c_new()` | Create new instance |
| `media_sessions_c_new_with_debounce(ms)` | Create with custom debounce |
| `media_sessions_c_free(handle)` | Free instance |
| `media_sessions_c_current(handle)` | Get current media info |
| `media_sessions_c_active_app(handle)` | Get active app name |

### Playback Control

| Function | Description |
|----------|-------------|
| `media_sessions_c_play(handle)` | Play/resume |
| `media_sessions_c_pause(handle)` | Pause |
| `media_sessions_c_play_pause(handle)` | Toggle play/pause |
| `media_sessions_c_stop(handle)` | Stop |
| `media_sessions_c_next(handle)` | Next track |
| `media_sessions_c_previous(handle)` | Previous track |
| `media_sessions_c_seek(handle, secs)` | Seek to position |

### Extended Control

| Function | Description |
|----------|-------------|
| `media_sessions_c_set_volume(handle, volume)` | Set volume (0.0-1.0) |
| `media_sessions_c_set_repeat_mode(handle, mode)` | Set repeat mode |
| `media_sessions_c_set_shuffle(handle, enabled)` | Toggle shuffle |

### Utility

| Function | Description |
|----------|-------------|
| `media_sessions_c_version()` | Get library version |
| `media_sessions_c_platform()` | Get platform name |
| `media_sessions_c_free_string(str)` | Free string |
| `media_sessions_c_free_info(info)` | Free media info |
| `media_sessions_c_free_artwork(data, len)` | Free artwork |

## Data Structures

### CMediaInfo

```c
typedef struct {
    char* title;              // Track title
    char* artist;             // Track artist
    char* album;              // Album name
    uint64_t duration_secs;   // Duration in seconds
    uint64_t position_secs;   // Current position
    MediaPlaybackStatus playback_status;
    bool has_artwork;
    size_t artwork_len;
    uint8_t* artwork;         // Raw image bytes
    uint32_t track_number;
    uint32_t disc_number;
    char* genre;
    int32_t year;
    char* url;
    char* thumbnail_url;
} CMediaInfo;
```

### Enums

```c
typedef enum {
    MEDIA_STATUS_PLAYING = 0,
    MEDIA_STATUS_PAUSED = 1,
    MEDIA_STATUS_STOPPED = 2,
    MEDIA_STATUS_TRANSITIONING = 3
} MediaPlaybackStatus;

typedef enum {
    MEDIA_REPEAT_NONE = 0,
    MEDIA_REPEAT_ONE = 1,
    MEDIA_REPEAT_ALL = 2
} MediaRepeatMode;

typedef enum {
    MEDIA_RESULT_OK = 0,
    MEDIA_RESULT_ERROR = 1,
    MEDIA_RESULT_NO_SESSION = 2,
    MEDIA_RESULT_NOT_SUPPORTED = 3,
    MEDIA_RESULT_TIMEOUT = 4,
    MEDIA_RESULT_INVALID_ARG = 5
} MediaResult;
```

## Memory Management

**Important:** All memory allocated by the library must be freed:

```c
// Free strings
media_sessions_c_free_string(info->title);

// Free artwork
media_sessions_c_free_artwork(info->artwork, info->artwork_len);

// Free entire CMediaInfo struct
media_sessions_c_free_info(info);

// Free MediaSessions handle
media_sessions_c_free(sessions);
```

## Error Handling

Functions return `MediaResult` codes:

- `MEDIA_RESULT_OK (0)` - Success
- `MEDIA_RESULT_ERROR (1)` - General error
- `MEDIA_RESULT_NO_SESSION (2)` - No active session
- `MEDIA_RESULT_NOT_SUPPORTED (3)` - Not supported on platform
- `MEDIA_RESULT_TIMEOUT (4)` - Operation timed out
- `MEDIA_RESULT_INVALID_ARG (5)` - Invalid argument

## Platform Support

| Platform | Status | Notes |
|----------|--------|-------|
| Windows 10/11 | ✅ Stable | SMTC API |
| Linux | ✅ Stable | MPRIS via D-Bus |
| macOS | ⚠️ Limited | MediaRemote requires permissions |

## Troubleshooting

### Library not found

**Python:**
```python
# Specify full path
sessions = MediaSessions()
sessions.lib = ctypes.CDLL("/full/path/to/libmedia_sessions_c.so")
```

**C#:**
```csharp
// Use DllImport with full path
[DllImport("/full/path/to/libmedia_sessions_c.so")]
```

### Permission denied (Linux)

```bash
sudo chmod +x /usr/lib/libmedia_sessions_c.so
sudo ldconfig
```

### Access denied (macOS)

macOS requires Accessibility permissions for MediaRemote:
1. System Preferences → Privacy & Security → Accessibility
2. Add your terminal/IDE

## License

Dual-licensed under MIT OR Apache-2.0.
