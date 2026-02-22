# C/C++ Integration

Using Media Sessions from C/C++ via native API.

## Installation

### 1. Build the Library

```bash
cargo build --release --features c-api
```

### 2. Copy Files

```bash
# Copy header
cp c-api/media_sessions_c.h ./my_app/

# Copy library
# Windows:
cp target/release/media_sessions_c.dll ./my_app/
cp target/release/media_sessions_c.lib ./my_app/

# Linux:
cp target/release/libmedia_sessions_c.so ./my_app/

# macOS:
cp target/release/libmedia_sessions_c.dylib ./my_app/
```

## Usage

### C Example

```c
#include <stdio.h>
#include "media_sessions_c.h"

int main() {
    // Create session
    MediaSessionsHandle* sessions = media_sessions_c_new();
    if (!sessions) {
        fprintf(stderr, "Failed to create session\n");
        return 1;
    }

    printf("✅ Session created!\n");
    printf("📦 Version: %s\n", media_sessions_c_version());
    printf("🖥️  Platform: %s\n", media_sessions_c_platform());

    // Get current track
    CMediaInfo* info = media_sessions_c_current(sessions);
    if (info) {
        printf("\n╔════════════════════════════════════════╗\n");
        printf("║         Now Playing                    ║\n");
        printf("╠════════════════════════════════════════╣\n");
        printf("║ 🎵 %s - %s\n", info->artist, info->title);
        if (info->album) {
            printf("║ 💿 %s\n", info->album);
        }
        printf("║ ⏱ %lu/%lu seconds\n",
               (unsigned long)info->position_secs,
               (unsigned long)info->duration_secs);
        printf("╚════════════════════════════════════════╝\n");

        media_sessions_c_free_info(info);
    } else {
        printf("ℹ️  No active session\n");
    }

    // Play
    printf("\n▶️ Play...\n");
    MediaResult result = media_sessions_c_play(sessions);
    if (result == MEDIA_RESULT_OK) {
        printf("✅ Play successful\n");
    }

    // Seek to 30 seconds
    printf("⏩ Seek to 30s...\n");
    media_sessions_c_seek(sessions, 30);

    // Cleanup
    media_sessions_c_free(sessions);
    printf("✅ Done\n");

    return 0;
}
```

### C++ Example

```cpp
#include "media_sessions_c.h"
#include <iostream>
#include <string>

class MediaSessions {
private:
    MediaSessionsHandle* handle;

public:
    MediaSessions() {
        handle = media_sessions_c_new();
        if (!handle) {
            throw std::runtime_error("Failed to create session");
        }
    }

    ~MediaSessions() {
        if (handle) {
            media_sessions_c_free(handle);
        }
    }

    std::string getVersion() {
        return std::string(media_sessions_c_version());
    }

    std::string getPlatform() {
        return std::string(media_sessions_c_platform());
    }

    bool play() {
        return media_sessions_c_play(handle) == MEDIA_RESULT_OK;
    }

    bool pause() {
        return media_sessions_c_pause(handle) == MEDIA_RESULT_OK;
    }

    bool seek(uint64_t secs) {
        return media_sessions_c_seek(handle, secs) == MEDIA_RESULT_OK;
    }
};

int main() {
    try {
        MediaSessions sessions;

        std::cout << "✅ Session created!" << std::endl;
        std::cout << "📦 Version: " << sessions.getVersion() << std::endl;
        std::cout << "🖥️  Platform: " << sessions.getPlatform() << std::endl;

        sessions.play();
        std::cout << "▶️ Playing" << std::endl;

        sessions.seek(30);
        std::cout << "⏩ Seeked to 30s" << std::endl;

    } catch (const std::exception& e) {
        std::cerr << "❌ Error: " << e.what() << std::endl;
        return 1;
    }

    return 0;
}
```

## Compilation

### Windows (MSVC)

```bash
# Compile
cl main.c media_sessions_c.lib

# Or for C++
cl main.cpp media_sessions_c.lib
```

### Linux

```bash
# Compile C
gcc -o main main.c -L. -lmedia_sessions_c -Wl,-rpath,.

# Compile C++
g++ -o main main.cpp -L. -lmedia_sessions_c -Wl,-rpath,.

# Run
./main
```

### macOS

```bash
# Compile
clang -o main main.c -L. -lmedia_sessions_c

# Or C++
clang++ -o main main.cpp -L. -lmedia_sessions_c

# Run
./main
```

## Save Artwork Example

### C

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

### C++

```cpp
if (info && info->has_artwork) {
    std::ofstream file("cover.jpg", std::ios::binary);
    if (file) {
        file.write(reinterpret_cast<char*>(info->artwork), info->artwork_len);
        file.close();
        std::cout << "✅ Cover saved" << std::endl;
    }
}
```

## See Also

- **[Python](python.md)** — Using from Python
- **[C#](csharp.md)** — Using from C#
- **[Node.js](nodejs.md)** — Using from JavaScript
