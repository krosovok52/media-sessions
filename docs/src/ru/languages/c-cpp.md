# C/C++

Нативное использование Media Sessions из C и C++.

## Установка

```bash
# 1. Собрать библиотеку
cargo build --release --features c-api

# 2. Выходные файлы:
# Windows: target/release/media_sessions_c.dll
# Linux: target/release/libmedia_sessions_c.so
# macOS: target/release/libmedia_sessions_c.dylib

# 3. Заголовочный файл
# Скопировать media_sessions_c.h из c-api/ в проект
```

## Пример на C

### Базовое использование

```c
#include <stdio.h>
#include <stdlib.h>
#include "media_sessions_c.h"

int main() {
    // Создание экземпляра
    MediaSessionsHandle* sessions = media_sessions_c_new();
    if (!sessions) {
        fprintf(stderr, "Failed to create sessions\n");
        return 1;
    }

    printf("✅ Media Sessions initialized\n");
    printf("Version: %s\n", media_sessions_c_version());
    printf("Platform: %s\n", media_sessions_c_platform());

    // Получить текущий трек
    CMediaInfo* info = media_sessions_c_current(sessions);
    if (info) {
        printf("\n╔════════════════════════════════════════╗\n");
        printf("║         Now Playing                    ║\n");
        printf("╠════════════════════════════════════════╣\n");
        printf("║ 🎵 %s - %s\n", info->artist, info->title);
        
        if (info->album) {
            printf("║ 💿 %s\n", info->album);
        }
        
        if (info->genre) {
            printf("║ 🎷 %s\n", info->genre);
        }
        
        printf("║ ⏱ %lu/%lu seconds\n", 
               (unsigned long)info->position_secs,
               (unsigned long)info->duration_secs);
        
        const char* status_icons[] = {"▶️", "⏸️", "⏹️"};
        printf("║ %s Status\n", status_icons[info->playback_status]);
        
        if (info->has_artwork) {
            printf("║ 🖼️ Artwork available (%zu bytes)\n", info->artwork_len);
        }
        
        printf("╚════════════════════════════════════════╝\n");

        // Освобождение памяти
        media_sessions_c_free_info(info);
    } else {
        printf("ℹ️ No active media session\n");
    }

    // Управление воспроизведением
    printf("\n🎮 Controls:\n");
    
    if (media_sessions_c_play(sessions) == MEDIA_RESULT_OK) {
        printf("✅ Play\n");
    }
    
    // Seek на 30 секунд
    if (media_sessions_c_seek(sessions, 30) == MEDIA_RESULT_OK) {
        printf("✅ Seek to 30s\n");
    }

    // Следующий трек
    if (media_sessions_c_next(sessions) == MEDIA_RESULT_OK) {
        printf("✅ Next track\n");
    }

    // Cleanup
    media_sessions_c_free(sessions);
    printf("\n👋 Cleanup complete\n");

    return 0;
}
```

### Компиляция

**Windows (MSVC):**

```batch
cl /I. example.c media_sessions_c.lib /Fe:example.exe
```

**Linux:**

```bash
gcc -o example example.c -L. -lmedia_sessions_c -Wl,-rpath,.
```

**macOS:**

```bash
clang -o example example.c -L. -lmedia_sessions_c
```

## Пример на C++

### C++ обёртка

```cpp
// media_sessions.hpp
#pragma once

#include <string>
#include <optional>
#include <vector>
#include <memory>
#include "media_sessions_c.h"

namespace media_sessions {

enum class PlaybackStatus {
    Playing = 0,
    Paused = 1,
    Stopped = 2,
};

struct MediaInfo {
    std::string title;
    std::string artist;
    std::string album;
    uint64_t duration_secs;
    uint64_t position_secs;
    PlaybackStatus playback_status;
    std::vector<uint8_t> artwork;
    std::optional<std::string> genre;
    std::optional<int> year;
    std::optional<uint32_t> track_number;
    bool has_artwork;

    std::string display_string() const {
        return artist + " - " + title;
    }

    double progress_percent() const {
        return duration_secs > 0 
            ? static_cast<double>(position_secs) / duration_secs * 100.0 
            : 0.0;
    }
};

class MediaSessions {
public:
    MediaSessions() : handle_(media_sessions_c_new()) {
        if (!handle_) {
            throw std::runtime_error("Failed to create Media Sessions instance");
        }
    }

    explicit MediaSessions(uint32_t debounce_ms) 
        : handle_(media_sessions_c_new_with_debounce(debounce_ms)) {
        if (!handle_) {
            throw std::runtime_error("Failed to create Media Sessions instance");
        }
    }

    ~MediaSessions() {
        if (handle_) {
            media_sessions_c_free(handle_);
        }
    }

    // Non-copyable
    MediaSessions(const MediaSessions&) = delete;
    MediaSessions& operator=(const MediaSessions&) = delete;

    // Movable
    MediaSessions(MediaSessions&& other) noexcept 
        : handle_(other.handle_) {
        other.handle_ = nullptr;
    }

    MediaSessions& operator=(MediaSessions&& other) noexcept {
        if (this != &other) {
            if (handle_) {
                media_sessions_c_free(handle_);
            }
            handle_ = other.handle_;
            other.handle_ = nullptr;
        }
        return *this;
    }

    std::optional<MediaInfo> current() const {
        CMediaInfo* info = media_sessions_c_current(handle_);
        if (!info) {
            return std::nullopt;
        }

        MediaInfo result;
        result.title = info->title ? info->title : "";
        result.artist = info->artist ? info->artist : "";
        result.album = info->album ? info->album : "";
        result.duration_secs = info->duration_secs;
        result.position_secs = info->position_secs;
        result.playback_status = static_cast<PlaybackStatus>(info->playback_status);
        result.has_artwork = info->has_artwork;

        if (info->genre) {
            result.genre = info->genre;
        }

        if (info->year) {
            result.year = info->year;
        }

        result.track_number = info->track_number;

        if (info->has_artwork && info->artwork_len > 0) {
            result.artwork.assign(
                info->artwork, 
                info->artwork + info->artwork_len
            );
        }

        media_sessions_c_free_info(info);
        return result;
    }

    bool play() const {
        return media_sessions_c_play(handle_) == MEDIA_RESULT_OK;
    }

    bool pause() const {
        return media_sessions_c_pause(handle_) == MEDIA_RESULT_OK;
    }

    bool play_pause() const {
        return media_sessions_c_play_pause(handle_) == MEDIA_RESULT_OK;
    }

    bool stop() const {
        return media_sessions_c_stop(handle_) == MEDIA_RESULT_OK;
    }

    bool next() const {
        return media_sessions_c_next(handle_) == MEDIA_RESULT_OK;
    }

    bool previous() const {
        return media_sessions_c_previous(handle_) == MEDIA_RESULT_OK;
    }

    bool seek(uint64_t seconds) const {
        return media_sessions_c_seek(handle_, seconds) == MEDIA_RESULT_OK;
    }

    bool save_artwork(const std::string& filepath) const {
        auto info = current();
        if (!info || !info->has_artwork || info->artwork.empty()) {
            return false;
        }

        FILE* f = fopen(filepath.c_str(), "wb");
        if (!f) {
            return false;
        }

        fwrite(info->artwork.data(), 1, info->artwork.size(), f);
        fclose(f);
        return true;
    }

    static std::string version() {
        return media_sessions_c_version();
    }

    static std::string platform() {
        return media_sessions_c_platform();
    }

private:
    MediaSessionsHandle* handle_;
};

} // namespace media_sessions
```

### Использование C++

```cpp
// main.cpp
#include <iostream>
#include <thread>
#include <chrono>
#include "media_sessions.hpp"

int main() {
    try {
        media_sessions::MediaSessions sessions;

        std::cout << "✅ Media Sessions v" 
                  << media_sessions::MediaSessions::version() << std::endl;
        std::cout << "Platform: " 
                  << media_sessions::MediaSessions::platform() << std::endl;

        // Получить текущий трек
        auto info = sessions.current();
        if (info) {
            std::cout << "\n╔════════════════════════════════════════╗" << std::endl;
            std::cout << "║         Now Playing                    ║" << std::endl;
            std::cout << "╠════════════════════════════════════════╣" << std::endl;
            std::cout << "║ 🎵 " << info->display_string() << std::endl;
            
            if (!info->album.empty()) {
                std::cout << "║ 💿 " << info->album << std::endl;
            }
            
            if (info->genre) {
                std::cout << "║ 🎷 " << *info->genre << std::endl;
            }
            
            std::cout << "║ ⏱ " << info->position_secs << "/" 
                      << info->duration_secs << " seconds" << std::endl;
            std::cout << "║ 📊 " << info->progress_percent() << "%" << std::endl;
            
            const char* icons[] = {"▶️", "⏸️", "⏹️"};
            std::cout << "║ " << icons[static_cast<int>(info->playback_status)] 
                      << " Status" << std::endl;
            
            std::cout << "╚════════════════════════════════════════╝" << std::endl;
        }

        // Управление
        sessions.play();
        std::this_thread::sleep_for(std::chrono::seconds(5));
        sessions.pause();

        // Seek
        sessions.seek(30);

        // Next track
        sessions.next();

        // Сохранение обложки
        if (sessions.save_artwork("cover.jpg")) {
            std::cout << "✅ Cover saved to cover.jpg" << std::endl;
        }

    } catch (const std::exception& e) {
        std::cerr << "❌ Error: " << e.what() << std::endl;
        return 1;
    }

    return 0;
}
```

### Компиляция C++

**Windows (MSVC):**

```batch
cl /EHsc /I. main.cpp media_sessions_c.lib /Fe:media_player.exe
```

**Linux:**

```bash
g++ -std=c++17 -o media_player main.cpp -L. -lmedia_sessions_c -Wl,-rpath,.
```

**macOS:**

```bash
clang++ -std=c++17 -o media_player main.cpp -L. -lmedia_sessions_c
```

### CMake проект

```cmake
# CMakeLists.txt
cmake_minimum_required(VERSION 3.10)
project(MediaSessionsExample)

set(CMAKE_CXX_STANDARD 17)

# Найти библиотеку
find_library(MEDIA_SESSIONS_LIB 
    NAMES media_sessions_c libmedia_sessions_c
    PATHS ${CMAKE_SOURCE_DIR}/lib
)

find_path(MEDIA_SESSIONS_INCLUDE 
    NAMES media_sessions_c.h
    PATHS ${CMAKE_SOURCE_DIR}/include
)

# Создать исполняемый файл
add_executable(media_player main.cpp)

target_include_directories(media_player PRIVATE ${MEDIA_SESSIONS_INCLUDE})
target_link_libraries(media_player ${MEDIA_SESSIONS_LIB})

# Копировать DLL на Windows
if(WIN32)
    add_custom_command(TARGET media_player POST_BUILD
        COMMAND ${CMAKE_COMMAND} -E copy_if_different
        ${CMAKE_SOURCE_DIR}/lib/media_sessions_c.dll
        $<TARGET_FILE_DIR:media_player>
    )
endif()
```

## Примеры

### 1. Монитор текущего трека (C)

```c
#include <stdio.h>
#include <unistd.h>
#include "media_sessions_c.h"

int main() {
    MediaSessionsHandle* sessions = media_sessions_c_new();
    if (!sessions) return 1;

    printf("🎵 Media Sessions Monitor\n");
    printf("=" 40 "\n");

    while (1) {
        CMediaInfo* info = media_sessions_c_current(sessions);
        if (info) {
            const char* icons[] = {"▶️", "⏸️", "⏹️"};
            printf("\r%s %s - %s", 
                   icons[info->playback_status],
                   info->artist, 
                   info->title);
            fflush(stdout);
            media_sessions_c_free_info(info);
        }
        sleep(1);
    }

    media_sessions_c_free(sessions);
    return 0;
}
```

### 2. CLI утилита (C++)

```cpp
#include <iostream>
#include <string>
#include "media_sessions.hpp"

int main(int argc, char* argv[]) {
    try {
        media_sessions::MediaSessions sessions;

        if (argc < 2) {
            std::cout << "Usage: " << argv[0] << " <command>\n"
                      << "Commands: current, play, pause, next, prev, seek <s>\n";
            return 1;
        }

        std::string cmd = argv[1];

        if (cmd == "current") {
            auto info = sessions.current();
            if (info) {
                std::cout << info->display_string() << "\n";
                std::cout << info->album << "\n";
            } else {
                std::cout << "No active session\n";
            }
        } else if (cmd == "play") {
            sessions.play();
            std::cout << "▶️ Playing\n";
        } else if (cmd == "pause") {
            sessions.pause();
            std::cout << "⏸️ Paused\n";
        } else if (cmd == "next") {
            sessions.next();
            std::cout << "⏭️ Next track\n";
        } else if (cmd == "prev") {
            sessions.previous();
            std::cout << "⏮️ Previous track\n";
        } else if (cmd == "seek" && argc > 2) {
            int secs = std::stoi(argv[2]);
            sessions.seek(secs);
            std::cout << "⏱ Seeked to " << secs << "s\n";
        } else {
            std::cerr << "Unknown command: " << cmd << "\n";
            return 1;
        }

    } catch (const std::exception& e) {
        std::cerr << "Error: " << e.what() << "\n";
        return 1;
    }

    return 0;
}
```

## См. также

- **[C API Reference](c-api.md)** — Полная документация C API
- **[Python](languages/python.md)** — Использование из Python
- **[C#](languages/csharp.md)** — Использование из .NET
