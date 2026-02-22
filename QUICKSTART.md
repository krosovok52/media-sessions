# 🚀 Quick Start Guide — Media Sessions

5-минутное руководство для начала работы.

---

## Rust (3 минуты)

### 1. Создание проекта

```bash
cargo new my_media_app
cd my_media_app
cargo add media-sessions tokio futures
```

### 2. Базовый пример

```rust
// src/main.rs
use media_sessions::MediaSessions;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sessions = MediaSessions::new()?;
    
    if let Some(info) = sessions.current().await? {
        println!("🎵 {} - {}", info.artist(), info.title());
    }
    
    Ok(())
}
```

### 3. Запуск

```bash
cargo run
```

---

## Python (2 минуты)

### 1. Сборка библиотеки

```bash
cd MediaSession
cargo build --release --features c-api
cp target/release/media_sessions.dll ./my_project/
```

### 2. Простой скрипт

```python
# main.py
import ctypes

lib = ctypes.CDLL('./media_sessions.dll')

# Настройка
lib.media_sessions_c_new.argtypes = []
lib.media_sessions_c_new.restype = c_void_p

# Создание
handle = lib.media_sessions_c_new()
print("Session created:", handle)

# Cleanup
lib.media_sessions_c_free(handle)
```

### 3. Запуск

```bash
python main.py
```

---

## C# (3 минуты)

### 1. Подготовка

```bash
cd MediaSession
cargo build --release --features c-api
cp target/release/media_sessions.dll ./MyApp/
```

### 2. Проект

```bash
dotnet new console -n MyApp
cd MyApp
```

### 3. Код

```csharp
// Program.cs
using System.Runtime.InteropServices;

class Program
{
    [DllImport("media_sessions_c")]
    private static extern IntPtr media_sessions_c_new();
    
    static void Main()
    {
        var handle = media_sessions_c_new();
        Console.WriteLine($"Session: {handle}");
    }
}
```

### 4. Запуск

```bash
dotnet run
```

---

## C++ (5 минут)

### 1. Подготовка

```bash
cd MediaSession
cargo build --release --features c-api
cp target/release/media_sessions.dll ./my_app/
cp c-api/media_sessions_c.h ./my_app/
```

### 2. Код

```cpp
// main.cpp
#include "media_sessions_c.h"
#include <iostream>

int main() {
    auto sessions = media_sessions_c_new();
    std::cout << "Session: " << sessions << std::endl;
    media_sessions_c_free(sessions);
    return 0;
}
```

### 3. Компиляция (Windows MSVC)

```bash
cl main.cpp media_sessions_c.lib
./main.exe
```

---

## Node.js (5 минут)

### 1. Установка

```bash
cd MediaSession
cargo build --release --features c-api
cp target/release/media_sessions.dll ./my_app/

cd ../my_app
npm init -y
npm install ffi-napi ref-napi
```

### 2. Код

```javascript
// index.js
const ffi = require('ffi-napi');

const lib = ffi.Library('./media_sessions_c', {
    'media_sessions_c_new': ['pointer', []]
});

const handle = lib.media_sessions_c_new();
console.log('Session:', handle);
```

### 3. Запуск

```bash
node index.js
```

---

## Что дальше?

1. **Читать полную документацию:** [`DOCUMENTATION.md`](DOCUMENTATION.md)
2. **Смотреть примеры:** [`examples/`](examples/)
3. **C API примеры:** [`c-api/`](c-api/)

---

## Проблемы?

### DLL не найден

**Windows:**
```powershell
# Проверить путь
dir .\media_sessions.dll

# Полный путь
$env:PATH += ";.\target\release"
```

**Linux:**
```bash
export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:$(pwd)
```

### Нет активной сессии

1. Запустите медиаплеер (Spotify, Firefox, etc.)
2. Начните воспроизведение
3. Проверьте снова

### Python crash на Windows

Используйте `os._exit(0)` вместо `sys.exit()`:

```python
import os
os._exit(0)
```

---

## Полезные ссылки

- 📚 [Полная документация](DOCUMENTATION.md)
- 🦀 [Rust примеры](examples/)
- 🐍 [Python примеры](c-api/python_example.py)
- 🔷 [C# примеры](c-api/csharp_example.cs)
- 📖 [API Docs](https://docs.rs/media-sessions)

---

*Happy coding! 🎉*
