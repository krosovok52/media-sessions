# Node.js

Использование Media Sessions из Node.js через ffi-napi.

## Установка

```bash
# 1. Собрать библиотеку
cargo build --release --features c-api

# 2. Скопировать DLL
# Windows:
cp target/release/media_sessions_c.dll ./

# Linux:
cp target/release/libmedia_sessions_c.so ./

# macOS:
cp target/release/libmedia_sessions_c.dylib ./

# 3. Установить зависимости
npm install ffi-napi ref-napi
```

## Класс-обёртка

```javascript
// media-sessions.js
const ffi = require('ffi-napi');
const ref = require('ref-napi');
const fs = require('fs');

// Типы данных
const CMediaInfo = ref.types.struct({
    title: ref.types.CString,
    artist: ref.types.CString,
    album: ref.types.CString,
    duration_secs: ref.types.uint64,
    position_secs: ref.types.uint64,
    playback_status: ref.types.int,
    has_artwork: ref.types.bool,
    artwork_len: ref.types.size_t,
    artwork: ref.types.pointer,
    track_number: ref.types.uint32,
    disc_number: ref.types.uint32,
    genre: ref.types.CString,
    year: ref.types.int32,
    url: ref.types.CString,
    thumbnail_url: ref.types.CString,
});

// Загрузка библиотеки
const LIB_PATH = process.platform === 'win32' 
    ? './media_sessions_c.dll'
    : process.platform === 'darwin'
    ? './libmedia_sessions_c.dylib'
    : './libmedia_sessions_c.so';

const lib = ffi.Library(LIB_PATH, {
    'media_sessions_c_new': ['pointer', []],
    'media_sessions_c_free': ['void', ['pointer']],
    'media_sessions_c_current': ['pointer', ['pointer']],
    'media_sessions_c_active_app': ['pointer', ['pointer']],
    'media_sessions_c_play': ['int', ['pointer']],
    'media_sessions_c_pause': ['int', ['pointer']],
    'media_sessions_c_play_pause': ['int', ['pointer']],
    'media_sessions_c_stop': ['int', ['pointer']],
    'media_sessions_c_next': ['int', ['pointer']],
    'media_sessions_c_previous': ['int', ['pointer']],
    'media_sessions_c_seek': ['int', ['pointer', ['uint64']],
    'media_sessions_c_free_info': ['void', ['pointer']],
    'media_sessions_c_free_string': ['void', ['pointer']],
    'media_sessions_c_version': ['string', []],
    'media_sessions_c_platform': ['string', []],
});

class MediaSessions {
    constructor() {
        this.handle = lib.media_sessions_c_new();
        if (!this.handle) {
            throw new Error('Failed to create Media Sessions instance');
        }
    }

    current() {
        const infoPtr = lib.media_sessions_c_current(this.handle);
        if (infoPtr.isNull()) {
            return null;
        }

        const info = infoPtr.readStruct(CMediaInfo);
        
        const result = {
            title: info.title || null,
            artist: info.artist || null,
            album: info.album || null,
            duration_secs: Number(info.duration_secs),
            position_secs: Number(info.position_secs),
            playback_status: ['Playing', 'Paused', 'Stopped'][info.playback_status] || 'Unknown',
            genre: info.genre || null,
            year: info.year || null,
            track_number: info.track_number,
            has_artwork: info.has_artwork,
        };

        // Копирование обложки
        if (info.has_artwork && Number(info.artwork_len) > 0) {
            const artworkBuffer = Buffer.from(
                info.artwork.readUInt8Array(Number(info.artwork_len))
            );
            result.artwork = artworkBuffer;
        }

        lib.media_sessions_c_free_info(infoPtr);
        return result;
    }

    activeApp() {
        const appPtr = lib.media_sessions_c_active_app(this.handle);
        if (appPtr.isNull()) {
            return null;
        }
        const app = appPtr.readCString();
        lib.media_sessions_c_free_string(appPtr);
        return app;
    }

    play() {
        return lib.media_sessions_c_play(this.handle) === 0;
    }

    pause() {
        return lib.media_sessions_c_pause(this.handle) === 0;
    }

    playPause() {
        return lib.media_sessions_c_play_pause(this.handle) === 0;
    }

    stop() {
        return lib.media_sessions_c_stop(this.handle) === 0;
    }

    next() {
        return lib.media_sessions_c_next(this.handle) === 0;
    }

    previous() {
        return lib.media_sessions_c_previous(this.handle) === 0;
    }

    seek(seconds) {
        return lib.media_sessions_c_seek(this.handle, seconds) === 0;
    }

    saveArtwork(filepath) {
        const info = this.current();
        if (!info || !info.has_artwork || !info.artwork) {
            return false;
        }
        fs.writeFileSync(filepath, info.artwork);
        return true;
    }

    static version() {
        return lib.media_sessions_c_version();
    }

    static platform() {
        return lib.media_sessions_c_platform();
    }

    dispose() {
        if (this.handle) {
            lib.media_sessions_c_free(this.handle);
            this.handle = null;
        }
    }
}

module.exports = { MediaSessions };
```

## Примеры использования

### 1. Базовое использование

```javascript
const { MediaSessions } = require('./media-sessions');

console.log(`Media Sessions v${MediaSessions.version()}`);
console.log(`Platform: ${MediaSessions.platform()}`);

const sessions = new MediaSessions();

const info = sessions.current();
if (info) {
    console.log(`🎵 ${info.artist} - ${info.title}`);
    console.log(`💿 ${info.album}`);
    console.log(`⏱ ${info.position_secs}/${info.duration_secs} seconds`);
    console.log(`▶️ ${info.playback_status}`);
}

sessions.play();
```

### 2. Простой плеер контроллер

```javascript
const { MediaSessions } = require('./media-sessions');

const sessions = new MediaSessions();

// Play/Pause
sessions.play();
setTimeout(() => {
    sessions.pause();
}, 5000);

// Next track
setTimeout(() => {
    sessions.next();
}, 10000);

// Seek
setTimeout(() => {
    sessions.seek(30);
}, 15000);
```

### 3. Монитор текущего трека

```javascript
const { MediaSessions } = require('./media-sessions');

const sessions = new MediaSessions();

console.log('🎵 Media Sessions Monitor');
console.log('='.repeat(40));

setInterval(() => {
    const info = sessions.current();
    if (info) {
        const icon = info.playback_status === 'Playing' ? '▶️' : '⏸️';
        process.stdout.write(`\r${icon} ${info.artist} - ${info.title}          `);
    }
}, 1000);
```

### 4. CLI утилита

```javascript
#!/usr/bin/env node
const { MediaSessions } = require('./media-sessions');

const command = process.argv[2];

const sessions = new MediaSessions();

switch (command) {
    case 'current':
        const info = sessions.current();
        if (info) {
            console.log(`${info.artist} - ${info.title}`);
            console.log(info.album);
            console.log(`${info.position_secs}/${info.duration_secs}s`);
        } else {
            console.log('No active session');
        }
        break;

    case 'play':
        sessions.play();
        console.log('▶️ Playing');
        break;

    case 'pause':
        sessions.pause();
        console.log('⏸️ Paused');
        break;

    case 'next':
        sessions.next();
        console.log('⏭️ Next track');
        break;

    case 'prev':
        sessions.previous();
        console.log('⏮️ Previous track');
        break;

    case 'seek':
        const secs = parseInt(process.argv[3], 10);
        sessions.seek(secs);
        console.log(`⏱ Seeked to ${secs}s`);
        break;

    default:
        console.log('Usage: media-sessions <command>');
        console.log('Commands: current, play, pause, next, prev, seek <s>');
        process.exit(1);
}
```

### 5. Express API

```javascript
const express = require('express');
const { MediaSessions } = require('./media-sessions');

const app = express();
const sessions = new MediaSessions();

app.get('/status', (req, res) => {
    const info = sessions.current();
    if (info) {
        res.json(info);
    } else {
        res.status(404).json({ error: 'No active session' });
    }
});

app.post('/play', (req, res) => {
    res.json({ success: sessions.play() });
});

app.post('/pause', (req, res) => {
    res.json({ success: sessions.pause() });
});

app.post('/next', (req, res) => {
    res.json({ success: sessions.next() });
});

app.post('/seek/:seconds', (req, res) => {
    const secs = parseInt(req.params.seconds, 10);
    res.json({ success: sessions.seek(secs) });
});

app.get('/cover', (req, res) => {
    const info = sessions.current();
    if (info && info.artwork) {
        res.set('Content-Type', 'image/jpeg');
        res.send(info.artwork);
    } else {
        res.status(404).json({ error: 'No artwork' });
    }
});

app.listen(3000, () => {
    console.log('🎵 Media Sessions API on port 3000');
});
```

### 6. Electron приложение

```javascript
// main.js
const { app, BrowserWindow, ipcMain } = require('electron');
const { MediaSessions } = require('./media-sessions');

let mainWindow;
const sessions = new MediaSessions();

function createWindow() {
    mainWindow = new BrowserWindow({
        width: 400,
        height: 600,
        webPreferences: {
            preload: __dirname + '/preload.js',
        },
    });

    mainWindow.loadFile('index.html');
}

app.whenReady().then(createWindow);

// IPC handlers
ipcMain.handle('get-current', () => {
    return sessions.current();
});

ipcMain.handle('play', () => {
    return sessions.play();
});

ipcMain.handle('pause', () => {
    return sessions.pause();
});

ipcMain.handle('next', () => {
    return sessions.next();
});

// Polling для обновлений
setInterval(() => {
    const info = sessions.current();
    if (mainWindow && info) {
        mainWindow.webContents.send('media-update', info);
    }
}, 1000);
```

```javascript
// preload.js
const { contextBridge, ipcRenderer } = require('electron');

contextBridge.exposeInMainWorld('media', {
    getCurrent: () => ipcRenderer.invoke('get-current'),
    play: () => ipcRenderer.invoke('play'),
    pause: () => ipcRenderer.invoke('pause'),
    next: () => ipcRenderer.invoke('next'),
    onUpdate: (callback) => ipcRenderer.on('media-update', (_, info) => callback(info)),
});
```

```html
<!-- index.html -->
<!DOCTYPE html>
<html>
<head>
    <style>
        body { font-family: system-ui; padding: 20px; }
        .now-playing { text-align: center; }
        .controls { margin-top: 20px; }
        button { padding: 10px 20px; margin: 5px; }
    </style>
</head>
<body>
    <div class="now-playing">
        <h2 id="title">No track</h2>
        <p id="artist"></p>
        <p id="status"></p>
    </div>
    <div class="controls">
        <button onclick="window.media.play()">▶️</button>
        <button onclick="window.media.pause()">⏸️</button>
        <button onclick="window.media.next()">⏭️</button>
    </div>
    <script>
        window.media.onUpdate((info) => {
            document.getElementById('title').textContent = info.title;
            document.getElementById('artist').textContent = info.artist;
            document.getElementById('status').textContent = info.playback_status;
        });
    </script>
</body>
</html>
```

## Обработка ошибок

```javascript
const { MediaSessions } = require('./media-sessions');

try {
    const sessions = new MediaSessions();
    
    const info = sessions.current();
    if (info) {
        console.log(`🎵 ${info.artist} - ${info.title}`);
    } else {
        console.log('ℹ️ No active session');
    }
} catch (error) {
    console.error('❌ Error:', error.message);
}
```

## Платформенные особенности

### Windows

```javascript
const lib = ffi.Library('./media_sessions_c.dll', {
    // ...
});
```

### Linux

```javascript
const lib = ffi.Library('./libmedia_sessions_c.so', {
    // ...
});
```

### macOS

```javascript
const lib = ffi.Library('./libmedia_sessions_c.dylib', {
    // ...
});
```

## Производительность

| Операция | Время |
|----------|-------|
| `current()` | ~2-5 ms |
| `play()` | ~1-3 ms |
| `seek()` | ~1-3 ms |

## См. также

- **[C API Reference](c-api.md)** — Полная документация C API
- **[Python](languages/python.md)** — Использование из Python
- **[C#](languages/csharp.md)** — Использование из .NET
