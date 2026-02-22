# C API Reference

C API allows using Media Sessions from other languages: Python, C#, C++, Node.js, and any language with FFI.

## Building

```bash
# Build the library
cargo build --release --features c-api

# Output files:
# Windows: target/release/media_sessions_c.dll
# Linux: target/release/libmedia_sessions_c.so
# macOS: target/release/libmedia_sessions_c.dylib
```

## Header File

```c
#ifndef MEDIA_SESSIONS_C_H
#define MEDIA_SESSIONS_C_H

#include <stdint.h>
#include <stdbool.h>
#include <stddef.h>

// Session handle
typedef struct MediaSessionsHandle MediaSessionsHandle;

// Track information
typedef struct {
    char* title;
    char* artist;
    char* album;
    uint64_t duration_secs;
    uint64_t position_secs;
    int playback_status;  // 0=Playing, 1=Paused, 2=Stopped
    bool has_artwork;
    size_t artwork_len;
    uint8_t* artwork;
    uint32_t track_number;
    uint32_t disc_number;
    char* genre;
    int32_t year;
    char* url;
    char* thumbnail_url;
} CMediaInfo;

// Result codes
typedef enum {
    MEDIA_RESULT_OK = 0,
    MEDIA_RESULT_ERROR = 1,
    MEDIA_RESULT_NO_SESSION = 2,
    MEDIA_RESULT_NOT_SUPPORTED = 3,
    MEDIA_RESULT_TIMEOUT = 4,
    MEDIA_RESULT_INVALID_ARG = 5
} MediaResult;

// Create/Delete
MediaSessionsHandle* media_sessions_c_new(void);
MediaSessionsHandle* media_sessions_c_new_with_debounce(uint32_t debounce_ms);
void media_sessions_c_free(MediaSessionsHandle* handle);

// Query information
CMediaInfo* media_sessions_c_current(MediaSessionsHandle* handle);
char* media_sessions_c_active_app(MediaSessionsHandle* handle);

// Playback control
MediaResult media_sessions_c_play(MediaSessionsHandle* handle);
MediaResult media_sessions_c_pause(MediaSessionsHandle* handle);
MediaResult media_sessions_c_play_pause(MediaSessionsHandle* handle);
MediaResult media_sessions_c_stop(MediaSessionsHandle* handle);
MediaResult media_sessions_c_next(MediaSessionsHandle* handle);
MediaResult media_sessions_c_previous(MediaSessionsHandle* handle);
MediaResult media_sessions_c_seek(MediaSessionsHandle* handle, uint64_t secs);

// Extended control
MediaResult media_sessions_c_set_volume(MediaSessionsHandle* handle, double level);
MediaResult media_sessions_c_set_repeat_mode(MediaSessionsHandle* handle, int mode);
MediaResult media_sessions_c_set_shuffle(MediaSessionsHandle* handle, bool enabled);

// Utilities
const char* media_sessions_c_version(void);
const char* media_sessions_c_platform(void);

// Free memory
void media_sessions_c_free_info(CMediaInfo* info);
void media_sessions_c_free_string(char* str);

#endif
```

## Functions

### Create and Delete

#### `media_sessions_c_new()`

Create a new instance.

```c
MediaSessionsHandle* handle = media_sessions_c_new();
if (!handle) {
    fprintf(stderr, "Failed to create session\n");
    return 1;
}
```

#### `media_sessions_c_new_with_debounce(uint32_t debounce_ms)`

Create with debounce setting.

```c
// 500ms debounce
MediaSessionsHandle* handle = media_sessions_c_new_with_debounce(500);
```

#### `media_sessions_c_free(handle)`

Free the instance.

```c
media_sessions_c_free(handle);
```

### Query Information

#### `media_sessions_c_current(handle)`

Get current track.

```c
CMediaInfo* info = media_sessions_c_current(handle);
if (info) {
    printf("Title: %s\n", info->title);
    printf("Artist: %s\n", info->artist);
    media_sessions_c_free_info(info);
}
```

#### `media_sessions_c_active_app(handle)`

Get application name.

```c
char* app = media_sessions_c_active_app(handle);
if (app) {
    printf("App: %s\n", app);
    media_sessions_c_free_string(app);
}
```

### Playback Control

| Function | Description | Returns |
|----------|-------------|---------|
| `media_sessions_c_play(handle)` | Play | MediaResult |
| `media_sessions_c_pause(handle)` | Pause | MediaResult |
| `media_sessions_c_play_pause(handle)` | Toggle | MediaResult |
| `media_sessions_c_stop(handle)` | Stop | MediaResult |
| `media_sessions_c_next(handle)` | Next track | MediaResult |
| `media_sessions_c_previous(handle)` | Previous track | MediaResult |
| `media_sessions_c_seek(handle, secs)` | Seek | MediaResult |

```c
// Play
if (media_sessions_c_play(handle) == MEDIA_RESULT_OK) {
    printf("Play successful\n");
}

// Seek to 30 seconds
media_sessions_c_seek(handle, 30);
```

### Extended Control

#### `media_sessions_c_set_volume(handle, level)`

Set volume (0.0 - 1.0).

```c
// 50% volume
media_sessions_c_set_volume(handle, 0.5);
```

#### `media_sessions_c_set_repeat_mode(handle, mode)`

Set repeat mode.

```c
// 0=None, 1=One, 2=All
media_sessions_c_set_repeat_mode(handle, 1);  // Repeat one
```

#### `media_sessions_c_set_shuffle(handle, enabled)`

Enable shuffle.

```c
media_sessions_c_set_shuffle(handle, true);
```

### Utilities

#### `media_sessions_c_version()`

Get library version.

```c
printf("Version: %s\n", media_sessions_c_version());
```

#### `media_sessions_c_platform()`

Get platform name.

```c
printf("Platform: %s\n", media_sessions_c_platform());
```

## Data Types

### CMediaInfo

```c
typedef struct {
    char* title;           // Track title
    char* artist;          // Artist
    char* album;           // Album
    uint64_t duration_secs; // Duration (seconds)
    uint64_t position_secs; // Position (seconds)
    int playback_status;    // 0=Playing, 1=Paused, 2=Stopped
    bool has_artwork;       // Has artwork
    size_t artwork_len;     // Artwork size
    uint8_t* artwork;       // Artwork bytes
    uint32_t track_number;  // Track number
    uint32_t disc_number;   // Disc number
    char* genre;            // Genre
    int32_t year;           // Year
    char* url;              // URL
    char* thumbnail_url;    // Thumbnail URL
} CMediaInfo;
```

### MediaResult

```c
typedef enum {
    MEDIA_RESULT_OK = 0,           // Success
    MEDIA_RESULT_ERROR = 1,        // General error
    MEDIA_RESULT_NO_SESSION = 2,   // No session
    MEDIA_RESULT_NOT_SUPPORTED = 3,// Not supported
    MEDIA_RESULT_TIMEOUT = 4,      // Timeout
    MEDIA_RESULT_INVALID_ARG = 5   // Invalid argument
} MediaResult;
```

## Memory Management

### Free CMediaInfo

```c
CMediaInfo* info = media_sessions_c_current(handle);
if (info) {
    // Use...
    media_sessions_c_free_info(info);  // Free entire structure
}
```

### Free Strings

```c
char* app = media_sessions_c_active_app(handle);
if (app) {
    printf("App: %s\n", app);
    media_sessions_c_free_string(app);  // Free string
}
```

## Examples

### 1. Basic Usage (C)

```c
#include <stdio.h>
#include "media_sessions_c.h"

int main() {
    // Create
    MediaSessionsHandle* sessions = media_sessions_c_new();
    if (!sessions) {
        printf("Failed to create sessions\n");
        return 1;
    }

    // Get track
    CMediaInfo* info = media_sessions_c_current(sessions);
    if (info) {
        printf("╔════════════════════════════════════════╗\n");
        printf("║         Now Playing                    ║\n");
        printf("╠════════════════════════════════════════╣\n");
        printf("║ 🎵 %s - %s\n", info->artist, info->title);
        printf("║ 💿 %s\n", info->album);
        printf("╚════════════════════════════════════════╝\n");

        media_sessions_c_free_info(info);
    }

    // Control
    media_sessions_c_play(sessions);

    // Cleanup
    media_sessions_c_free(sessions);
    return 0;
}
```

**Compile:**

```bash
# Windows (MSVC)
cl example.c media_sessions_c.lib

# Linux
gcc -o example example.c -L. -lmedia_sessions_c -Wl,-rpath,.

# macOS
clang -o example example.c -L. -lmedia_sessions_c
```

### 2. Save Artwork (C)

```c
if (info && info->has_artwork) {
    FILE* f = fopen("cover.jpg", "wb");
    if (f) {
        fwrite(info->artwork, 1, info->artwork_len, f);
        fclose(f);
        printf("✅ Cover saved to cover.jpg\n");
    }
}
```

## See Also

- **[Python](languages/python.md)** — Using from Python
- **[C#](languages/csharp.md)** — Using from .NET
- **[C/C++](languages/c-cpp.md)** — Native usage
- **[Node.js](languages/nodejs.md)** — Using from JavaScript
