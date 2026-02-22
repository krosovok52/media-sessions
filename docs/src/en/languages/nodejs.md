# Node.js Integration

Using Media Sessions from Node.js via ffi-napi.

## Installation

### 1. Build the Library

```bash
cargo build --release --features c-api
```

### 2. Copy DLL

```bash
# Windows:
cp target/release/media_sessions_c.dll ./my_app/

# Linux:
cp target/release/libmedia_sessions_c.so ./my_app/

# macOS:
cp target/release/libmedia_sessions_c.dylib ./my_app/
```

### 3. Install Node.js Dependencies

```bash
npm init -y
npm install ffi-napi ref-napi
```

## Usage

### Basic Example

```javascript
const ffi = require('ffi-napi');
const ref = require('ref-napi');

// Load library
const lib = ffi.Library('./media_sessions_c', {
    'media_sessions_c_new': ['pointer', []],
    'media_sessions_c_free': ['void', ['pointer']],
    'media_sessions_c_current': ['pointer', ['pointer']],
    'media_sessions_c_play': ['int', ['pointer']],
    'media_sessions_c_pause': ['int', ['pointer']],
    'media_sessions_c_seek': ['int', ['pointer', 'uint64']],
    'media_sessions_c_version': ['string', []],
    'media_sessions_c_platform': ['string', []],
});

// Create session
const handle = lib.media_sessions_c_new();
if (!handle) {
    console.error('❌ Failed to create session');
    process.exit(1);
}

console.log('✅ Session created:', handle);
console.log('📦 Version:', lib.media_sessions_c_version());
console.log('🖥️  Platform:', lib.media_sessions_c_platform());

// Play
const result = lib.media_sessions_c_play(handle);
if (result === 0) {
    console.log('▶️ Play successful');
}

// Seek to 30 seconds
lib.media_sessions_c_seek(handle, 30);
console.log('⏩ Seeked to 30s');

// Cleanup
lib.media_sessions_c_free(handle);
console.log('✅ Done');
```

### Full Wrapper Class

```javascript
const ffi = require('ffi-napi');
const ref = require('ref-napi');

// Define CMediaInfo structure
const CMediaInfo = ref.types.struct({
    title: ref.types.CString,
    artist: ref.types.CString,
    album: ref.types.CString,
    duration_secs: ref.types.uint64,
    position_secs: ref.types.uint64,
    playback_status: ref.types.int,
    has_artwork: ref.types.bool,
    artwork_len: ref.types.size_t,
    artwork: ref.refType(ref.types.uint8),
});

class MediaSessions {
    constructor(dllPath = './media_sessions_c') {
        this.lib = ffi.Library(dllPath, {
            'media_sessions_c_new': ['pointer', []],
            'media_sessions_c_free': ['void', ['pointer']],
            'media_sessions_c_current': ['pointer', ['pointer']],
            'media_sessions_c_play': ['int', ['pointer']],
            'media_sessions_c_pause': ['int', ['pointer']],
            'media_sessions_c_next': ['int', ['pointer']],
            'media_sessions_c_previous': ['int', ['pointer']],
            'media_sessions_c_seek': ['int', ['pointer', 'uint64']],
        });

        this.handle = this.lib.media_sessions_c_new();
        if (!this.handle) {
            throw new Error('Failed to create session');
        }
    }

    current() {
        const infoPtr = this.lib.media_sessions_c_current(this.handle);
        if (infoPtr.isNull()) {
            return null;
        }
        return infoPtr.deref();
    }

    play() {
        return this.lib.media_sessions_c_play(this.handle) === 0;
    }

    pause() {
        return this.lib.media_sessions_c_pause(this.handle) === 0;
    }

    next() {
        return this.lib.media_sessions_c_next(this.handle) === 0;
    }

    previous() {
        return this.lib.media_sessions_c_previous(this.handle) === 0;
    }

    seek(secs) {
        return this.lib.media_sessions_c_seek(this.handle, secs) === 0;
    }

    dispose() {
        if (this.handle) {
            this.lib.media_sessions_c_free(this.handle);
            this.handle = null;
        }
    }
}

// Usage
const sessions = new MediaSessions();

const info = sessions.current();
if (info) {
    console.log(`🎵 ${info.artist} - ${info.title}`);
    console.log(`💿 ${info.album}`);
    console.log(`⏱ ${info.position_secs}/${info.duration_secs} seconds`);
}

sessions.play();
sessions.dispose();
```

### Electron Example

```javascript
// main.js (Electron main process)
const { app, BrowserWindow, ipcMain } = require('electron');
const MediaSessions = require('./media-sessions');

let sessions;

function createWindow() {
    const win = new BrowserWindow({
        width: 800,
        height: 600,
        webPreferences: {
            preload: path.join(__dirname, 'preload.js'),
        },
    });

    sessions = new MediaSessions();

    win.loadFile('index.html');
}

app.whenReady().then(createWindow);

ipcMain.handle('get-current-track', () => {
    const info = sessions.current();
    if (info) {
        return {
            title: info.title,
            artist: info.artist,
            album: info.album,
            duration: info.duration_secs,
            position: info.position_secs,
        };
    }
    return null;
});

ipcMain.handle('play', () => sessions.play());
ipcMain.handle('pause', () => sessions.pause());

app.on('will-quit', () => {
    sessions.dispose();
});
```

```javascript
// preload.js
const { contextBridge, ipcRenderer } = require('electron');

contextBridge.exposeInMainWorld('media', {
    getCurrentTrack: () => ipcRenderer.invoke('get-current-track'),
    play: () => ipcRenderer.invoke('play'),
    pause: () => ipcRenderer.invoke('pause'),
});
```

```html
<!-- index.html -->
<!DOCTYPE html>
<html>
<body>
    <h1>Media Controller</h1>
    <div id="track">Loading...</div>
    <button onclick="play()">▶️ Play</button>
    <button onclick="pause()">⏸️ Pause</button>

    <script>
        async function loadTrack() {
            const track = await window.media.getCurrentTrack();
            if (track) {
                document.getElementById('track').textContent =
                    `${track.artist} - ${track.title}`;
            }
        }

        function play() {
            window.media.play();
        }

        function pause() {
            window.media.pause();
        }

        loadTrack();
        setInterval(loadTrack, 1000);
    </script>
</body>
</html>
```

## Troubleshooting

### Module Not Found

```bash
# Install dependencies
npm install ffi-napi ref-napi

# For Electron, use electron-rebuild
npm install --save-dev electron-rebuild
npx electron-rebuild
```

### DLL Not Found

```javascript
// Use full path
const sessions = new MediaSessions('/full/path/to/media_sessions_c');
```

## See Also

- **[Python](python.md)** — Using from Python
- **[C#](csharp.md)** — Using from C#
- **[C/C++](c-cpp.md)** — Using from C/C++
