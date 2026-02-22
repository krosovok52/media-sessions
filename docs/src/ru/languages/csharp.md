# C# (.NET)

Использование Media Sessions из C# через P/Invoke.

## Установка

```bash
# 1. Собрать библиотеку
cargo build --release --features c-api

# 2. Скопировать DLL в проект
# Windows:
cp target/release/media_sessions_c.dll ./MyApp/

# Linux:
cp target/release/libmedia_sessions_c.so ./MyApp/

# macOS:
cp target/release/libmedia_sessions_c.dylib ./MyApp/
```

## Класс-обёртка

```csharp
using System;
using System.Runtime.InteropServices;

namespace MediaSessions
{
    /// <summary>
    /// Playback status enumeration.
    /// </summary>
    public enum PlaybackStatus
    {
        Playing = 0,
        Paused = 1,
        Stopped = 2,
    }

    /// <summary>
    /// Media result codes.
    /// </summary>
    public enum MediaResult
    {
        Ok = 0,
        Error = 1,
        NoSession = 2,
        NotSupported = 3,
        Timeout = 4,
        InvalidArg = 5,
    }

    /// <summary>
    /// C interop media info structure.
    /// </summary>
    [StructLayout(LayoutKind.Sequential)]
    public struct CMediaInfo
    {
        [MarshalAs(UnmanagedType.LPStr)]
        public string Title;

        [MarshalAs(UnmanagedType.LPStr)]
        public string Artist;

        [MarshalAs(UnmanagedType.LPStr)]
        public string Album;

        public ulong DurationSecs;

        public ulong PositionSecs;

        public int PlaybackStatus;

        [MarshalAs(UnmanagedType.I1)]
        public bool HasArtwork;

        public UIntPtr ArtworkLen;

        public IntPtr Artwork;

        public uint TrackNumber;

        public uint DiscNumber;

        [MarshalAs(UnmanagedType.LPStr)]
        public string Genre;

        public int Year;

        [MarshalAs(UnmanagedType.LPStr)]
        public string Url;

        [MarshalAs(UnmanagedType.LPStr)]
        public string ThumbnailUrl;
    }

    /// <summary>
    /// Media Sessions wrapper for .NET.
    /// </summary>
    public class MediaSessionsWrapper : IDisposable
    {
        private IntPtr _handle;
        private bool _disposed = false;

        // P/Invoke declarations
        [DllImport("media_sessions_c", CallingConvention = CallingConvention.StdCall)]
        private static extern IntPtr media_sessions_c_new();

        [DllImport("media_sessions_c", CallingConvention = CallingConvention.StdCall)]
        private static extern void media_sessions_c_free(IntPtr handle);

        [DllImport("media_sessions_c", CallingConvention = CallingConvention.StdCall)]
        private static extern IntPtr media_sessions_c_current(IntPtr handle);

        [DllImport("media_sessions_c", CallingConvention = CallingConvention.StdCall)]
        private static extern MediaResult media_sessions_c_play(IntPtr handle);

        [DllImport("media_sessions_c", CallingConvention = CallingConvention.StdCall)]
        private static extern MediaResult media_sessions_c_pause(IntPtr handle);

        [DllImport("media_sessions_c", CallingConvention = CallingConvention.StdCall)]
        private static extern MediaResult media_sessions_c_play_pause(IntPtr handle);

        [DllImport("media_sessions_c", CallingConvention = CallingConvention.StdCall)]
        private static extern MediaResult media_sessions_c_stop(IntPtr handle);

        [DllImport("media_sessions_c", CallingConvention = CallingConvention.StdCall)]
        private static extern MediaResult media_sessions_c_next(IntPtr handle);

        [DllImport("media_sessions_c", CallingConvention = CallingConvention.StdCall)]
        private static extern MediaResult media_sessions_c_previous(IntPtr handle);

        [DllImport("media_sessions_c", CallingConvention = CallingConvention.StdCall)]
        private static extern MediaResult media_sessions_c_seek(IntPtr handle, ulong secs);

        [DllImport("media_sessions_c", CallingConvention = CallingConvention.StdCall)]
        private static extern void media_sessions_c_free_info(IntPtr info);

        /// <summary>
        /// Create new Media Sessions instance.
        /// </summary>
        public MediaSessionsWrapper()
        {
            _handle = media_sessions_c_new();
            if (_handle == IntPtr.Zero)
                throw new InvalidOperationException("Failed to create Media Sessions instance");
        }

        /// <summary>
        /// Get current track info.
        /// </summary>
        public MediaInfo? Current()
        {
            if (_handle == IntPtr.Zero)
                return null;

            IntPtr infoPtr = media_sessions_c_current(_handle);
            if (infoPtr == IntPtr.Zero)
                return null;

            CMediaInfo cInfo = Marshal.PtrToStructure<CMediaInfo>(infoPtr);
            
            var info = new MediaInfo
            {
                Title = cInfo.Title,
                Artist = cInfo.Artist,
                Album = cInfo.Album,
                DurationSecs = cInfo.DurationSecs,
                PositionSecs = cInfo.PositionSecs,
                PlaybackStatus = (PlaybackStatus)cInfo.PlaybackStatus,
                Genre = cInfo.Genre,
                Year = cInfo.Year,
                TrackNumber = cInfo.TrackNumber,
                HasArtwork = cInfo.HasArtwork,
            };

            // Copy artwork if available
            if (cInfo.HasArtwork && cInfo.Artwork != IntPtr.Zero)
            {
                byte[] artwork = new byte[(int)cInfo.ArtworkLen];
                Marshal.Copy(cInfo.Artwork, artwork, 0, artwork.Length);
                info.Artwork = artwork;
            }

            media_sessions_c_free_info(infoPtr);
            return info;
        }

        /// <summary>
        /// Play.
        /// </summary>
        public bool Play() => media_sessions_c_play(_handle) == MediaResult.Ok;

        /// <summary>
        /// Pause.
        /// </summary>
        public bool Pause() => media_sessions_c_pause(_handle) == MediaResult.Ok;

        /// <summary>
        /// Toggle Play/Pause.
        /// </summary>
        public bool PlayPause() => media_sessions_c_play_pause(_handle) == MediaResult.Ok;

        /// <summary>
        /// Stop.
        /// </summary>
        public bool Stop() => media_sessions_c_stop(_handle) == MediaResult.Ok;

        /// <summary>
        /// Next track.
        /// </summary>
        public bool Next() => media_sessions_c_next(_handle) == MediaResult.Ok;

        /// <summary>
        /// Previous track.
        /// </summary>
        public bool Previous() => media_sessions_c_previous(_handle) == MediaResult.Ok;

        /// <summary>
        /// Seek to position.
        /// </summary>
        public bool Seek(ulong seconds) => media_sessions_c_seek(_handle, seconds) == MediaResult.Ok;

        /// <summary>
        /// Save artwork to file.
        /// </summary>
        public bool SaveArtwork(string filepath)
        {
            var info = Current();
            if (info == null || !info.HasArtwork || info.Artwork == null)
                return false;

            System.IO.File.WriteAllBytes(filepath, info.Artwork);
            return true;
        }

        /// <summary>
        /// Dispose.
        /// </summary>
        protected virtual void Dispose(bool disposing)
        {
            if (!_disposed)
            {
                if (_handle != IntPtr.Zero)
                {
                    media_sessions_c_free(_handle);
                    _handle = IntPtr.Zero;
                }
                _disposed = true;
            }
        }

        public void Dispose()
        {
            Dispose(true);
            GC.SuppressFinalize(this);
        }

        ~MediaSessionsWrapper()
        {
            Dispose(false);
        }
    }

    /// <summary>
    /// Media info class.
    /// </summary>
    public class MediaInfo
    {
        public string? Title { get; set; }
        public string? Artist { get; set; }
        public string? Album { get; set; }
        public ulong DurationSecs { get; set; }
        public ulong PositionSecs { get; set; }
        public PlaybackStatus PlaybackStatus { get; set; }
        public byte[]? Artwork { get; set; }
        public string? Genre { get; set; }
        public int? Year { get; set; }
        public uint TrackNumber { get; set; }
        public bool HasArtwork { get; set; }

        public string DisplayString => $"{Artist} - {Title}";

        public double ProgressPercent => DurationSecs > 0 
            ? (double)PositionSecs / DurationSecs * 100 
            : 0;
    }
}
```

## Примеры использования

### 1. Базовое использование

```csharp
using MediaSessions;

using var sessions = new MediaSessionsWrapper();

var info = sessions.Current();
if (info != null)
{
    Console.WriteLine($"🎵 {info.Artist} - {info.Title}");
    Console.WriteLine($"💿 {info.Album}");
    Console.WriteLine($"⏱ {info.PositionSecs}/{info.DurationSecs} seconds");
}

sessions.Play();
```

### 2. Простой плеер контроллер

```csharp
using MediaSessions;
using System.Threading;

using var sessions = new MediaSessionsWrapper();

// Play/Pause
sessions.Play();
Thread.Sleep(5000);
sessions.Pause();

// Next track
sessions.Next();

// Seek
sessions.Seek(30);
```

### 3. Console приложение

```csharp
using MediaSessions;

using var sessions = new MediaSessionsWrapper();

var info = sessions.Current();
if (info != null)
{
    var statusIcon = info.PlaybackStatus == PlaybackStatus.Playing ? "▶️" : "⏸️";
    
    Console.WriteLine("╔════════════════════════════════════════╗");
    Console.WriteLine("║         Now Playing                    ║");
    Console.WriteLine("╠════════════════════════════════════════╣");
    Console.WriteLine($"║  {statusIcon} {info.DisplayString,-20} ║");
    
    if (!string.IsNullOrEmpty(info.Album))
        Console.WriteLine($"║  💿 {info.Album,-28} ║");
    
    Console.WriteLine($"║  ⏱ {info.PositionSecs}/{info.DurationSecs}s ({info.ProgressPercent:F1}%){' ', -10} ║");
    Console.WriteLine("╚════════════════════════════════════════╝");
}
```

### 4. Сохранение обложки

```csharp
using MediaSessions;

using var sessions = new MediaSessionsWrapper();

if (sessions.SaveArtwork("cover.jpg"))
{
    Console.WriteLine("✅ Cover saved to cover.jpg");
}
else
{
    Console.WriteLine("ℹ️ No artwork available");
}
```

### 5. WPF приложение

```csharp
using System.Windows;
using System.Windows.Threading;
using MediaSessions;

public partial class MainWindow : Window
{
    private readonly MediaSessionsWrapper _sessions;
    private readonly DispatcherTimer _timer;

    public MainWindow()
    {
        InitializeComponent();

        _sessions = new MediaSessionsWrapper();
        _timer = new DispatcherTimer
        {
            Interval = TimeSpan.FromSeconds(1)
        };
        _timer.Tick += Timer_Tick;
        _timer.Start();
    }

    private void Timer_Tick(object? sender, EventArgs e)
    {
        var info = _sessions.Current();
        if (info != null)
        {
            TitleText.Text = info.Title;
            ArtistText.Text = info.Artist;
            AlbumText.Text = info.Album;
            StatusText.Text = info.PlaybackStatus.ToString();
            
            // Update progress
            Progress.Value = info.ProgressPercent;
        }
    }

    private void PlayButton_Click(object sender, RoutedEventArgs e)
    {
        _sessions.Play();
    }

    private void PauseButton_Click(object sender, RoutedEventArgs e)
    {
        _sessions.Pause();
    }

    private void NextButton_Click(object sender, RoutedEventArgs e)
    {
        _sessions.Next();
    }

    protected override void OnClosed(EventArgs e)
    {
        _sessions.Dispose();
        base.OnClosed(e);
    }
}
```

### 6. ASP.NET Core API

```csharp
using Microsoft.AspNetCore.Mvc;
using MediaSessions;

[ApiController]
[Route("api/[controller]")]
public class MediaController : ControllerBase
{
    private static readonly MediaSessionsWrapper _sessions = new();

    [HttpGet("status")]
    public ActionResult<MediaInfo> GetStatus()
    {
        var info = _sessions.Current();
        return info != null ? Ok(info) : NotFound("No active session");
    }

    [HttpPost("play")]
    public IActionResult Play()
    {
        return _sessions.Play() ? Ok() : StatusCode(500);
    }

    [HttpPost("pause")]
    public IActionResult Pause()
    {
        return _sessions.Pause() ? Ok() : StatusCode(500);
    }

    [HttpPost("next")]
    public IActionResult Next()
    {
        return _sessions.Next() ? Ok() : StatusCode(500);
    }

    protected override void Dispose(bool disposing)
    {
        _sessions.Dispose();
        base.Dispose(disposing);
    }
}
```

## Платформенные особенности

### Windows

```csharp
// DLL автоматически загружается из той же директории
[DllImport("media_sessions_c", ...)]
```

### Linux

```csharp
// Указать полное имя библиотеки
[DllImport("libmedia_sessions_c.so", ...)]
```

### macOS

```csharp
// Указать полное имя библиотеки
[DllImport("libmedia_sessions_c.dylib", ...)]
```

## Обработка ошибок

```csharp
try
{
    using var sessions = new MediaSessionsWrapper();
    
    var info = sessions.Current();
    if (info != null)
    {
        Console.WriteLine($"🎵 {info.DisplayString}");
    }
    else
    {
        Console.WriteLine("ℹ️ No active session");
    }
}
catch (InvalidOperationException ex)
{
    Console.WriteLine($"❌ Initialization error: {ex.Message}");
}
catch (Exception ex)
{
    Console.WriteLine($"❌ Error: {ex.Message}");
}
```

## См. также

- **[C API Reference](c-api.md)** — Полная документация C API
- **[Python](languages/python.md)** — Использование из Python
- **[C/C++](languages/c-cpp.md)** — Нативное использование
