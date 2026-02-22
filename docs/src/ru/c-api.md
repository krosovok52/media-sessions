# C API Reference

C API позволяет использовать Media Sessions из других языков: Python, C#, C++, Node.js и любых языков с FFI.

## Сборка

```bash
# Сборка библиотеки
cargo build --release --features c-api

# Выходные файлы:
# Windows: target/release/media_sessions_c.dll
# Linux: target/release/libmedia_sessions_c.so
# macOS: target/release/libmedia_sessions_c.dylib
```

## Заголовочный файл

```c
#ifndef MEDIA_SESSIONS_C_H
#define MEDIA_SESSIONS_C_H

#include <stdint.h>
#include <stdbool.h>
#include <stddef.h>

// Handle для сессии
typedef struct MediaSessionsHandle MediaSessionsHandle;

// Информация о треке
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

// Коды результатов
typedef enum {
    MEDIA_RESULT_OK = 0,
    MEDIA_RESULT_ERROR = 1,
    MEDIA_RESULT_NO_SESSION = 2,
    MEDIA_RESULT_NOT_SUPPORTED = 3,
    MEDIA_RESULT_TIMEOUT = 4,
    MEDIA_RESULT_INVALID_ARG = 5
} MediaResult;

// Создание/удаление
MediaSessionsHandle* media_sessions_c_new(void);
MediaSessionsHandle* media_sessions_c_new_with_debounce(uint32_t debounce_ms);
void media_sessions_c_free(MediaSessionsHandle* handle);

// Запрос информации
CMediaInfo* media_sessions_c_current(MediaSessionsHandle* handle);
char* media_sessions_c_active_app(MediaSessionsHandle* handle);

// Управление воспроизведением
MediaResult media_sessions_c_play(MediaSessionsHandle* handle);
MediaResult media_sessions_c_pause(MediaSessionsHandle* handle);
MediaResult media_sessions_c_play_pause(MediaSessionsHandle* handle);
MediaResult media_sessions_c_stop(MediaSessionsHandle* handle);
MediaResult media_sessions_c_next(MediaSessionsHandle* handle);
MediaResult media_sessions_c_previous(MediaSessionsHandle* handle);
MediaResult media_sessions_c_seek(MediaSessionsHandle* handle, uint64_t secs);

// Расширенное управление
MediaResult media_sessions_c_set_volume(MediaSessionsHandle* handle, double level);
MediaResult media_sessions_c_set_repeat_mode(MediaSessionsHandle* handle, int mode);
MediaResult media_sessions_c_set_shuffle(MediaSessionsHandle* handle, bool enabled);

// Утилиты
const char* media_sessions_c_version(void);
const char* media_sessions_c_platform(void);

// Освобождение памяти
void media_sessions_c_free_info(CMediaInfo* info);
void media_sessions_c_free_string(char* str);

#endif
```

## Функции

### Создание и удаление

#### `media_sessions_c_new()`

Создать новый экземпляр.

```c
MediaSessionsHandle* handle = media_sessions_c_new();
if (!handle) {
    fprintf(stderr, "Failed to create session\n");
    return 1;
}
```

#### `media_sessions_c_new_with_debounce(uint32_t debounce_ms)`

Создать с настройкой debounce.

```c
// 500ms debounce
MediaSessionsHandle* handle = media_sessions_c_new_with_debounce(500);
```

#### `media_sessions_c_free(handle)`

Освободить экземпляр.

```c
media_sessions_c_free(handle);
```

### Запрос информации

#### `media_sessions_c_current(handle)`

Получить текущий трек.

```c
CMediaInfo* info = media_sessions_c_current(handle);
if (info) {
    printf("Title: %s\n", info->title);
    printf("Artist: %s\n", info->artist);
    media_sessions_c_free_info(info);
}
```

#### `media_sessions_c_active_app(handle)`

Получить имя приложения.

```c
char* app = media_sessions_c_active_app(handle);
if (app) {
    printf("App: %s\n", app);
    media_sessions_c_free_string(app);
}
```

### Управление воспроизведением

| Функция | Описание | Возвращает |
|---------|----------|------------|
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

// Seek на 30 секунд
media_sessions_c_seek(handle, 30);
```

### Расширенное управление

#### `media_sessions_c_set_volume(handle, level)`

Установить громкость (0.0 - 1.0).

```c
// 50% громкость
media_sessions_c_set_volume(handle, 0.5);
```

#### `media_sessions_c_set_repeat_mode(handle, mode)`

Установить режим повтора.

```c
// 0=None, 1=One, 2=All
media_sessions_c_set_repeat_mode(handle, 1);  // Repeat one
```

#### `media_sessions_c_set_shuffle(handle, enabled)`

Включить shuffle.

```c
media_sessions_c_set_shuffle(handle, true);
```

### Утилиты

#### `media_sessions_c_version()`

Получить версию библиотеки.

```c
printf("Version: %s\n", media_sessions_c_version());
```

#### `media_sessions_c_platform()`

Получить платформу.

```c
printf("Platform: %s\n", media_sessions_c_platform());
```

## Типы данных

### CMediaInfo

```c
typedef struct {
    char* title;           // Название трека
    char* artist;          // Исполнитель
    char* album;           // Альбом
    uint64_t duration_secs; // Длительность (сек)
    uint64_t position_secs; // Позиция (сек)
    int playback_status;    // 0=Playing, 1=Paused, 2=Stopped
    bool has_artwork;       // Есть ли обложка
    size_t artwork_len;     // Размер обложки
    uint8_t* artwork;       // Байты обложки
    uint32_t track_number;  // Номер трека
    uint32_t disc_number;   // Номер диска
    char* genre;            // Жанр
    int32_t year;           // Год
    char* url;              // URL
    char* thumbnail_url;    // URL миниатюры
} CMediaInfo;
```

### MediaResult

```c
typedef enum {
    MEDIA_RESULT_OK = 0,           // Успех
    MEDIA_RESULT_ERROR = 1,        // Общая ошибка
    MEDIA_RESULT_NO_SESSION = 2,   // Нет сессии
    MEDIA_RESULT_NOT_SUPPORTED = 3,// Не поддерживается
    MEDIA_RESULT_TIMEOUT = 4,      // Таймаут
    MEDIA_RESULT_INVALID_ARG = 5   // Неверный аргумент
} MediaResult;
```

## Управление памятью

### Освобождение CMediaInfo

```c
CMediaInfo* info = media_sessions_c_current(handle);
if (info) {
    // Использование...
    media_sessions_c_free_info(info);  // Освободить всю структуру
}
```

### Освобождение строк

```c
char* app = media_sessions_c_active_app(handle);
if (app) {
    printf("App: %s\n", app);
    media_sessions_c_free_string(app);  // Освободить строку
}
```

## Примеры

### 1. Базовое использование (C)

```c
#include <stdio.h>
#include "media_sessions_c.h"

int main() {
    // Создание
    MediaSessionsHandle* sessions = media_sessions_c_new();
    if (!sessions) {
        printf("Failed to create sessions\n");
        return 1;
    }

    // Получить трек
    CMediaInfo* info = media_sessions_c_current(sessions);
    if (info) {
        printf("╔════════════════════════════════════════╗\n");
        printf!("║         Now Playing                    ║\n");
        printf!("╠════════════════════════════════════════╣\n");
        printf!("║ 🎵 %s - %s\n", info->artist, info->title);
        printf!("║ 💿 %s\n", info->album);
        printf!("║ ⏱ %lu/%lu seconds\n", 
                (unsigned long)info->position_secs,
                (unsigned long)info->duration_secs);
        printf!("╚════════════════════════════════════════╝\n");
        
        media_sessions_c_free_info(info);
    }

    // Управление
    media_sessions_c_play(sessions);

    // Cleanup
    media_sessions_c_free(sessions);
    return 0;
}
```

**Компиляция:**

```bash
# Windows (MSVC)
cl example.c media_sessions_c.lib

# Linux
gcc -o example example.c -L. -lmedia_sessions_c -Wl,-rpath,.

# macOS
clang -o example example.c -L. -lmedia_sessions_c
```

### 2. Сохранение обложки (C)

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

## См. также

- **[Python](languages/python.md)** — Использование из Python
- **[C#](languages/csharp.md)** — Использование из .NET
- **[C/C++](languages/c-cpp.md)** — Нативное использование
- **[Node.js](languages/nodejs.md)** — Использование из JavaScript
