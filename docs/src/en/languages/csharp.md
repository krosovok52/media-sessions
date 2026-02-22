# C# (.NET) Integration

Using Media Sessions from C# via P/Invoke.

## Installation

### 1. Build the Library

```bash
cargo build --release --features c-api
```

### 2. Copy DLL

```bash
# Windows:
cp target/release/media_sessions_c.dll ./MyApp/

# Linux:
cp target/release/libmedia_sessions_c.so ./MyApp/
```

## Usage

### Basic Example

```csharp
using System;
using System.Runtime.InteropServices;

class Program
{
    [DllImport("media_sessions_c")]
    private static extern IntPtr media_sessions_c_new();

    [DllImport("media_sessions_c")]
    private static extern void media_sessions_c_free(IntPtr handle);

    [DllImport("media_sessions_c")]
    private static extern int media_sessions_c_play(IntPtr handle);

    static void Main()
    {
        var handle = media_sessions_c_new();
        media_sessions_c_play(handle);
        media_sessions_c_free(handle);
    }
}
```

### Full Wrapper Class

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

    public class MediaSessionsWrapper : IDisposable
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
        private static extern int media_sessions_c_pause(IntPtr handle);

        [DllImport("media_sessions_c", CallingConvention = CallingConvention.StdCall)]
        private static extern IntPtr media_sessions_c_current(IntPtr handle);

        public MediaSessionsWrapper()
        {
            _handle = media_sessions_c_new();
            if (_handle == IntPtr.Zero)
                throw new InvalidOperationException("Failed to create session");
        }

        public bool Play()
        {
            return media_sessions_c_play(_handle) == 0;
        }

        public bool Pause()
        {
            return media_sessions_c_pause(_handle) == 0;
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
            using var sessions = new MediaSessionsWrapper();

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

### WPF Example

```csharp
using System;
using System.Windows;
using MediaSessions;

public partial class MainWindow : Window
{
    private MediaSessionsWrapper _sessions;

    public MainWindow()
    {
        InitializeComponent();
        _sessions = new MediaSessionsWrapper();
    }

    private void PlayButton_Click(object sender, RoutedEventArgs e)
    {
        _sessions.Play();
    }

    private void PauseButton_Click(object sender, RoutedEventArgs e)
    {
        _sessions.Pause();
    }

    protected override void OnClosed(EventArgs e)
    {
        _sessions.Dispose();
        base.OnClosed(e);
    }
}
```

## API Reference

### MediaSessionsWrapper

| Method | Description | Returns |
|--------|-------------|---------|
| `Play()` | Play | `bool` |
| `Pause()` | Pause | `bool` |
| `GetCurrent()` | Get current track | `CMediaInfo?` |
| `Dispose()` | Free resources | `void` |

### CMediaInfo

| Field | Type | Description |
|-------|------|-------------|
| `title` | `string` | Track title |
| `artist` | `string` | Artist |
| `album` | `string` | Album |
| `duration_secs` | `ulong` | Duration |
| `position_secs` | `ulong` | Position |
| `playback_status` | `int` | Status |

## See Also

- **[Python](python.md)** — Using from Python
- **[C/C++](c-cpp.md)** — Using from C/C++
