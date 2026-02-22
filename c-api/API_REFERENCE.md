# C API Documentation

Полная документация по использованию C API для интеграции с другими языками.

---

## 📦 Сборка

### Windows

```bash
cargo build --release --features c-api
# Результат: target/release/media_sessions.dll
```

### Linux

```bash
cargo build --release --features c-api
# Результат: target/release/libmedia_sessions_c.so
```

### macOS

```bash
cargo build --release --features c-api
# Результат: target/release/libmedia_sessions_c.dylib
```

---

## 📋 API Reference

### Управление сессиями

| Функция | Описание |
|---------|----------|
| `media_sessions_c_new()` | Создать новую сессию |
| `media_sessions_c_new_with_debounce(ms)` | Создать с debounce (мс) |
| `media_sessions_c_free(handle)` | Освободить сессию |

### Получение информации

| Функция | Возвращает |
|---------|-----------|
| `media_sessions_c_current(handle)` | `CMediaInfo*` — текущий трек |
| `media_sessions_c_active_app(handle)` | `char*` — имя приложения |

### Управление воспроизведением

| Функция | Коды возврата |
|---------|--------------|
| `media_sessions_c_play(handle)` | `MediaResult` |
| `media_sessions_c_pause(handle)` | `MediaResult` |
| `media_sessions_c_play_pause(handle)` | `MediaResult` |
| `media_sessions_c_stop(handle)` | `MediaResult` |
| `media_sessions_c_next(handle)` | `MediaResult` |
| `media_sessions_c_previous(handle)` | `MediaResult` |
| `media_sessions_c_seek(handle, secs)` | `MediaResult` |

### Расширенные настройки

| Функция | Параметры |
|---------|----------|
| `media_sessions_c_set_volume(handle, volume)` | `volume`: 0.0–1.0 |
| `media_sessions_c_set_repeat_mode(handle, mode)` | `mode`: 0=None, 1=One, 2=All |
| `media_sessions_c_set_shuffle(handle, enabled)` | `enabled`: true/false |

### Утилиты

| Функция | Описание |
|---------|----------|
| `media_sessions_c_version()` | Версия библиотеки |
| `media_sessions_c_platform()` | Платформа (windows/linux/macos) |
| `media_sessions_c_free_string(str)` | Освободить строку |
| `media_sessions_c_free_info(info)` | Освободить MediaInfo |
| `media_sessions_c_free_artwork(data, len)` | Освободить обложку |

---

## 📊 Типы данных

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

### PlaybackStatus

```c
typedef enum {
    MEDIA_STATUS_PLAYING = 0,    // ▶️
    MEDIA_STATUS_PAUSED = 1,     // ⏸️
    MEDIA_STATUS_STOPPED = 2,    // ⏹️
    MEDIA_STATUS_TRANSITIONING = 3 // ⏳
} MediaPlaybackStatus;
```

### RepeatMode

```c
typedef enum {
    MEDIA_REPEAT_NONE = 0,  // Выключен
    MEDIA_REPEAT_ONE = 1,   // Один трек
    MEDIA_REPEAT_ALL = 2    // Все треки
} MediaRepeatMode;
```

### CMediaInfo

```c
typedef struct {
    char* title;              // Название трека
    char* artist;             // Исполнитель
    char* album;              // Альбом
    uint64_t duration_secs;   // Длительность (сек)
    uint64_t position_secs;   // Позиция (сек)
    MediaPlaybackStatus playback_status;
    bool has_artwork;         // Есть ли обложка
    size_t artwork_len;       // Размер обложки (байты)
    uint8_t* artwork;         // Данные обложки
    uint32_t track_number;    // Номер трека
    uint32_t disc_number;     // Номер диска
    char* genre;              // Жанр
    int32_t year;             // Год
    char* url;                // URL источника
    char* thumbnail_url;      // URL миниатюры
} CMediaInfo;
```

---

## 💻 Примеры использования

### C

```c
#include <stdio.h>
#include "media_sessions_c.h"

int main() {
    // Создание сессии
    MediaSessionsHandle* sessions = media_sessions_c_new();
    if (!sessions) {
        printf("❌ Не удалось создать сессию\n");
        return 1;
    }
    
    printf("✅ Сессия создана\n");
    printf("📦 Версия: %s\n", media_sessions_c_version());
    printf("🖥️ Платформа: %s\n", media_sessions_c_platform());
    
    // Получение текущего трека
    CMediaInfo* info = media_sessions_c_current(sessions);
    if (info) {
        printf("\n🎵 Сейчас играет:\n");
        printf("   Title:  %s\n", info->title);
        printf("   Artist: %s\n", info->artist);
        printf("   Album:  %s\n", info->album);
        printf("   Status: %d\n", info->playback_status);
        printf("   Time:   %lu/%lu секунд\n", 
               (unsigned long)info->position_secs,
               (unsigned long)info->duration_secs);
        
        // Освобождение памяти
        media_sessions_c_free_info(info);
    } else {
        printf("ℹ️ Нет активной сессии\n");
    }
    
    // Управление
    printf("\n▶️ Play...\n");
    media_sessions_c_play(sessions);
    
    // Перемотка на 30 секунд
    printf("⏩ Seek to 30s...\n");
    media_sessions_c_seek(sessions, 30);
    
    // Освобождение
    media_sessions_c_free(sessions);
    printf("✅ Готово\n");
    
    return 0;
}
```

**Компиляция:**

```bash
# Windows (MSVC)
cl main.c media_sessions_c.lib

# Linux
gcc -o main main.c -L. -lmedia_sessions_c -Wl,-rpath,.

# macOS
clang -o main main.c -L. -lmedia_sessions_c
```

---

### Python (ctypes)

```python
import ctypes
from ctypes import (
    c_void_p, c_char_p, c_uint64, c_uint32, c_int32,
    c_double, c_bool, c_size_t, Structure, POINTER
)

# Загрузка библиотеки
lib = ctypes.CDLL('./media_sessions_c.dll')

# Настройка прототипов
lib.media_sessions_c_new.argtypes = []
lib.media_sessions_c_new.restype = c_void_p

lib.media_sessions_c_current.argtypes = [c_void_p]
lib.media_sessions_c_current.restype = c_void_p

lib.media_sessions_c_free.argtypes = [c_void_p]
lib.media_sessions_c_free.restype = None

lib.media_sessions_c_play.argtypes = [c_void_p]
lib.media_sessions_c_play.restype = c_int32

lib.media_sessions_c_seek.argtypes = [c_void_p, c_uint64]
lib.media_sessions_c_seek.restype = c_int32

# Создание сессии
handle = lib.media_sessions_c_new()
if not handle:
    raise RuntimeError("Failed to create session")

print(f"✅ Session created: {handle}")

# Получение информации
info_ptr = lib.media_sessions_c_current(handle)
if info_ptr:
    # Доступ к полям структуры
    class CMediaInfo(Structure):
        _fields_ = [
            ("title", c_char_p),
            ("artist", c_char_p),
            ("album", c_char_p),
            ("duration_secs", c_uint64),
            ("position_secs", c_uint64),
            ("playback_status", c_int32),
        ]
    
    info = ctypes.cast(info_ptr, POINTER(CMediaInfo)).contents
    
    print(f"🎵 Title:  {info.title.decode()}")
    print(f"   Artist: {info.artist.decode()}")
    print(f"   Status: {info.playback_status}")
else:
    print("ℹ️ No active session")

# Управление
lib.media_sessions_c_play(handle)
lib.media_sessions_c_seek(handle, 30)

# Cleanup
lib.media_sessions_c_free(handle)
```

---

### C# (P/Invoke)

```csharp
using System;
using System.Runtime.InteropServices;

namespace MediaSessions
{
    [StructLayout(LayoutKind.Sequential)]
    public struct CMediaInfo
    {
        [MarshalAs(UnmanagedType.LPStr)]
        public string title;
        
        [MarshalAs(UnmanagedType.LPStr)]
        public string artist;
        
        [MarshalAs(UnmanagedType.LPStr)]
        public string album;
        
        public ulong duration_secs;
        public ulong position_secs;
        public int playback_status;
    }
    
    public class MediaSessions : IDisposable
    {
        private IntPtr _handle;
        private bool _disposed = false;
        
        [DllImport("media_sessions_c", CallingConvention = CallingConvention.StdCall)]
        private static extern IntPtr media_sessions_c_new();
        
        [DllImport("media_sessions_c", CallingConvention = CallingConvention.StdCall)]
        private static extern void media_sessions_c_free(IntPtr handle);
        
        [DllImport("media_sessions_c", CallingConvention = CallingConvention.StdCall)]
        private static extern int media_sessions_c_play(IntPtr handle);
        
        [DllImport("media_sessions_c", CallingConvention = CallingConvention.StdCall)]
        private static extern IntPtr media_sessions_c_current(IntPtr handle);
        
        public MediaSessions()
        {
            _handle = media_sessions_c_new();
            if (_handle == IntPtr.Zero)
                throw new InvalidOperationException("Failed to create session");
        }
        
        public bool Play()
        {
            return media_sessions_c_play(_handle) == 0;
        }
        
        public CMediaInfo? GetCurrent()
        {
            var ptr = media_sessions_c_current(_handle);
            if (ptr == IntPtr.Zero)
                return null;
            
            return Marshal.PtrToStructure<CMediaInfo>(ptr);
        }
        
        protected virtual void Dispose(bool disposing)
        {
            if (!_disposed)
            {
                if (_handle != IntPtr.Zero)
                    media_sessions_c_free(_handle);
                _disposed = true;
            }
        }
        
        public void Dispose()
        {
            Dispose(true);
            GC.SuppressFinalize(this);
        }
    }
    
    class Program
    {
        static void Main()
        {
            using var sessions = new MediaSessions();
            
            var info = sessions.GetCurrent();
            if (info.HasValue)
            {
                Console.WriteLine($"🎵 {info.Value.artist} - {info.Value.title}");
            }
            
            sessions.Play();
        }
    }
}
```

---

### Node.js (ffi-napi)

```javascript
const ffi = require('ffi-napi');
const ref = require('ref-napi');

// Загрузка библиотеки
const lib = ffi.Library('./media_sessions_c', {
    'media_sessions_c_new': ['pointer', []],
    'media_sessions_c_free': ['void', ['pointer']],
    'media_sessions_c_current': ['pointer', ['pointer']],
    'media_sessions_c_play': ['int', ['pointer']],
    'media_sessions_c_pause': ['int', ['pointer']],
    'media_sessions_c_seek': ['int', ['pointer', 'uint64']],
});

// Создание сессии
const handle = lib.media_sessions_c_new();
console.log(`✅ Session created: ${handle}`);

// Получение информации
const infoPtr = lib.media_sessions_c_current(handle);
if (infoPtr !== null) {
    // Обработка структуры (требуется определить layout)
    console.log('🎵 Track info available');
}

// Управление
lib.media_sessions_c_play(handle);
lib.media_sessions_c_seek(handle, 30);

// Cleanup
lib.media_sessions_c_free(handle);
```

---

## 🔧 Управление памятью

### Важно!

Все строки и структуры, возвращаемые библиотекой, должны быть освобождены:

```c
// Получение строки
char* app = media_sessions_c_active_app(sessions);
if (app) {
    printf("App: %s\n", app);
    media_sessions_c_free_string(app);  // ✅ Освободить
}

// Получение структуры
CMediaInfo* info = media_sessions_c_current(sessions);
if (info) {
    printf("Title: %s\n", info->title);
    media_sessions_c_free_info(info);  // ✅ Освободить всё
}

// Обложка
if (info->has_artwork) {
    fwrite(info->artwork, 1, info->artwork_len, fp);
    media_sessions_c_free_artwork(info->artwork, info->artwork_len);  // ✅
}
```

### Python

В Python ctypes автоматически управляет памятью для простых типов, но для структур нужно освобождать вручную:

```python
info_ptr = lib.media_sessions_c_current(handle)
if info_ptr:
    info = info_ptr.contents
    # ... использование
    lib.media_sessions_c_free_info(info_ptr)  # ✅
```

---

## 📝 Обработка ошибок

### C

```c
MediaResult result = media_sessions_c_play(sessions);

switch (result) {
    case MEDIA_RESULT_OK:
        printf("✅ Success\n");
        break;
    case MEDIA_RESULT_NO_SESSION:
        printf("⚠️ No active session\n");
        break;
    case MEDIA_RESULT_NOT_SUPPORTED:
        printf("⚠️ Not supported on this platform\n");
        break;
    default:
        printf("❌ Error: %d\n", result);
}
```

### Python

```python
result = lib.media_sessions_c_play(handle)
if result != 0:
    error_messages = {
        1: "General error",
        2: "No session",
        3: "Not supported",
        4: "Timeout",
        5: "Invalid argument"
    }
    raise RuntimeError(f"Play failed: {error_messages.get(result, 'Unknown')}")
```

---

## 🖥️ Платформенные ограничения

### Windows

- ❌ `set_volume()` — не поддерживается SMTC
- ❌ `set_repeat_mode()` — не поддерживается
- ❌ `set_shuffle()` — не поддерживается
- ❌ `get_artwork()` — не доступно через SMTC

### macOS

- ⚠️ Требуется Accessibility permissions
- ⚠️ Некоторые функции ограничены

### Linux

- ✅ Полная поддержка через MPRIS
- ⚠️ Требуется запущенный MPRIS-плеер

---

## 📚 Дополнительные ресурсы

- [Полная документация](../DOCUMENTATION.md)
- [Python пример](python_example.py)
- [C# пример](csharp_example.cs)
- [Заголовочный файл](media_sessions_c.h)

---

*Для вопросов: [@krosov_ok](https://t.me/krosov_ok)*
