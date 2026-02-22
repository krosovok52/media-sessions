using System;
using System.Runtime.InteropServices;
using System.Text;

namespace MediaSessions;

/// <summary>
/// Playback status enumeration.
/// </summary>
public enum PlaybackStatus
{
    Playing = 0,
    Paused = 1,
    Stopped = 2,
    Transitioning = 3
}

/// <summary>
/// Repeat mode enumeration.
/// </summary>
public enum RepeatMode
{
    None = 0,
    One = 1,
    All = 2
}

/// <summary>
/// Result codes for API functions.
/// </summary>
public enum MediaResult
{
    Ok = 0,
    Error = 1,
    NoSession = 2,
    NotSupported = 3,
    Timeout = 4,
    InvalidArg = 5
}

/// <summary>
/// Media information structure.
/// </summary>
[StructLayout(LayoutKind.Sequential)]
public struct CMediaInfo
{
    public IntPtr title;
    public IntPtr artist;
    public IntPtr album;
    public ulong duration_secs;
    public ulong position_secs;
    public int playback_status;
    [MarshalAs(UnmanagedType.I1)]
    public bool has_artwork;
    public IntPtr artwork_len;
    public IntPtr artwork;
    public uint track_number;
    public uint disc_number;
    public IntPtr genre;
    public int year;
    public IntPtr url;
    public IntPtr thumbnail_url;
}

/// <summary>
/// Managed wrapper for media information.
/// </summary>
public class MediaInfo
{
    public string Title { get; set; } = string.Empty;
    public string Artist { get; set; } = string.Empty;
    public string Album { get; set; } = string.Empty;
    public ulong DurationSecs { get; set; }
    public ulong PositionSecs { get; set; }
    public PlaybackStatus PlaybackStatus { get; set; }
    public bool HasArtwork { get; set; }
    public ulong ArtworkLen { get; set; }
    public byte[]? Artwork { get; set; }
    public uint TrackNumber { get; set; }
    public uint DiscNumber { get; set; }
    public string Genre { get; set; } = string.Empty;
    public int Year { get; set; }
    public string Url { get; set; } = string.Empty;
    public string ThumbnailUrl { get; set; } = string.Empty;

    public string DisplayString => string.IsNullOrEmpty(Artist) 
        ? Title 
        : string.IsNullOrEmpty(Title) 
            ? Artist 
            : $"{Artist} - {Title}";

    public double Progress => DurationSecs > 0 
        ? (double)PositionSecs / DurationSecs 
        : 0.0;

    public double ProgressPercent => Progress * 100.0;

    public static MediaInfo FromNative(CMediaInfo native)
    {
        var info = new MediaInfo
        {
            Title = Marshal.PtrToStringAnsi(native.title) ?? string.Empty,
            Artist = Marshal.PtrToStringAnsi(native.artist) ?? string.Empty,
            Album = Marshal.PtrToStringAnsi(native.album) ?? string.Empty,
            DurationSecs = native.duration_secs,
            PositionSecs = native.position_secs,
            PlaybackStatus = (PlaybackStatus)native.playback_status,
            HasArtwork = native.has_artwork,
            ArtworkLen = (ulong)native.artwork_len,
            TrackNumber = native.track_number,
            DiscNumber = native.disc_number,
            Genre = Marshal.PtrToStringAnsi(native.genre) ?? string.Empty,
            Year = native.year,
            Url = Marshal.PtrToStringAnsi(native.url) ?? string.Empty,
            ThumbnailUrl = Marshal.PtrToStringAnsi(native.thumbnail_url) ?? string.Empty,
        };

        if (native.has_artwork && native.artwork != IntPtr.Zero && native.artwork_len != IntPtr.Zero)
        {
            var artworkLen = (int)native.artwork_len;
            info.Artwork = new byte[artworkLen];
            Marshal.Copy(native.artwork, info.Artwork, 0, artworkLen);
        }

        return info;
    }

    public override string ToString()
    {
        var statusIcon = PlaybackStatus switch
        {
            PlaybackStatus.Playing => "▶️",
            PlaybackStatus.Paused => "⏸️",
            PlaybackStatus.Stopped => "⏹️",
            PlaybackStatus.Transitioning => "⏳",
            _ => ""
        };

        var sb = new StringBuilder();
        sb.AppendLine("╔═══════════════════════════════════════════════════════════╗");
        sb.AppendLine("║                    Now Playing                            ║");
        sb.AppendLine("╠═══════════════════════════════════════════════════════════╣");

        if (string.IsNullOrEmpty(Title) && string.IsNullOrEmpty(Artist))
        {
            sb.AppendLine("║  No media information available                           ║");
        }
        else
        {
            sb.AppendLine($"║  Title:  {Truncate(DisplayString, 52),-52} ║");
            if (!string.IsNullOrEmpty(Album))
            {
                sb.AppendLine($"║  Album:  {Truncate(Album, 52),-52} ║");
            }
        }

        sb.AppendLine("╠═══════════════════════════════════════════════════════════╣");
        sb.AppendLine($"║  Status: {statusIcon} {PlaybackStatus,-45} ║");

        if (DurationSecs > 0)
        {
            var progress = Progress;
            var barWidth = 40;
            var filled = (int)(progress * barWidth);
            var bar = new string('█', filled) + new string('░', barWidth - filled);

            sb.AppendLine($"║  [{bar}] {FormatTime(PositionSecs)}/{FormatTime(DurationSecs)} ({ProgressPercent:F1}%) {'',14} ║");
        }

        sb.AppendLine("╚═══════════════════════════════════════════════════════════╝");
        return sb.ToString();
    }

    private static string Truncate(string value, int maxLength)
    {
        if (string.IsNullOrEmpty(value)) return value;
        return value.Length <= maxLength ? value : value.Substring(0, maxLength - 3) + "...";
    }

    private static string FormatTime(ulong secs)
    {
        var hours = secs / 3600;
        var mins = (secs % 3600) / 60;
        var seconds = secs % 60;
        return hours > 0 
            ? $"{hours}:{mins:D2}:{seconds:D2}" 
            : $"{mins}:{seconds:D2}";
    }
}

/// <summary>
/// Main class for interacting with media sessions.
/// </summary>
public class MediaSessionsWrapper : IDisposable
{
    private IntPtr _handle = IntPtr.Zero;
    private bool _disposed = false;

    public MediaSessionsWrapper(uint debounceMs = 0)
    {
        _handle = debounceMs > 0 
            ? NativeMethods.media_sessions_c_new_with_debounce(debounceMs) 
            : NativeMethods.media_sessions_c_new();

        if (_handle == IntPtr.Zero)
        {
            throw new InvalidOperationException("Failed to create MediaSessions instance");
        }
    }

    public MediaInfo? Current()
    {
        ThrowIfDisposed();

        var infoPtr = NativeMethods.media_sessions_c_current(_handle);
        if (infoPtr == IntPtr.Zero)
        {
            return null;
        }

        var nativeInfo = Marshal.PtrToStructure<CMediaInfo>(infoPtr);
        var managedInfo = MediaInfo.FromNative(nativeInfo);

        NativeMethods.media_sessions_c_free_info(infoPtr);
        return managedInfo;
    }

    public string? ActiveApp()
    {
        ThrowIfDisposed();

        var appPtr = NativeMethods.media_sessions_c_active_app(_handle);
        if (appPtr == IntPtr.Zero)
        {
            return null;
        }

        var app = Marshal.PtrToStringAnsi(appPtr);
        NativeMethods.media_sessions_c_free_string(appPtr);
        return app;
    }

    public bool Play() => CallNativeMethod(NativeMethods.media_sessions_c_play, _handle);
    public bool Pause() => CallNativeMethod(NativeMethods.media_sessions_c_pause, _handle);
    public bool PlayPause() => CallNativeMethod(NativeMethods.media_sessions_c_play_pause, _handle);
    public bool Stop() => CallNativeMethod(NativeMethods.media_sessions_c_stop, _handle);
    public bool Next() => CallNativeMethod(NativeMethods.media_sessions_c_next, _handle);
    public bool Previous() => CallNativeMethod(NativeMethods.media_sessions_c_previous, _handle);

    public bool Seek(ulong positionSecs)
    {
        ThrowIfDisposed();
        return NativeMethods.media_sessions_c_seek(_handle, positionSecs) == (int)MediaResult.Ok;
    }

    public bool SetVolume(double volume)
    {
        ThrowIfDisposed();
        if (volume < 0.0 || volume > 1.0)
        {
            throw new ArgumentOutOfRangeException(nameof(volume), "Volume must be between 0.0 and 1.0");
        }
        return NativeMethods.media_sessions_c_set_volume(_handle, volume) == (int)MediaResult.Ok;
    }

    public bool SetRepeatMode(RepeatMode mode)
    {
        ThrowIfDisposed();
        return NativeMethods.media_sessions_c_set_repeat_mode(_handle, (int)mode) == (int)MediaResult.Ok;
    }

    public bool SetShuffle(bool enabled)
    {
        ThrowIfDisposed();
        return NativeMethods.media_sessions_c_set_shuffle(_handle, enabled) == (int)MediaResult.Ok;
    }

    public static string Version()
    {
        var ptr = NativeMethods.media_sessions_c_version();
        return Marshal.PtrToStringAnsi(ptr) ?? "unknown";
    }

    public static string Platform()
    {
        var ptr = NativeMethods.media_sessions_c_platform();
        return Marshal.PtrToStringAnsi(ptr) ?? "unknown";
    }

    private static bool CallNativeMethod(Func<IntPtr, int> method, IntPtr handle)
    {
        try
        {
            return method(handle) == (int)MediaResult.Ok;
        }
        catch (Exception)
        {
            return false;
        }
    }

    private void ThrowIfDisposed()
    {
        if (_disposed)
        {
            throw new ObjectDisposedException(nameof(MediaSessionsWrapper));
        }
    }

    protected virtual void Dispose(bool disposing)
    {
        if (!_disposed)
        {
            if (_handle != IntPtr.Zero)
            {
                NativeMethods.media_sessions_c_free(_handle);
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

    private static class NativeMethods
    {
        private const string LibraryName = "media_sessions_c";

        [DllImport(LibraryName, CallingConvention = CallingConvention.StdCall)]
        public static extern IntPtr media_sessions_c_new();

        [DllImport(LibraryName, CallingConvention = CallingConvention.StdCall)]
        public static extern IntPtr media_sessions_c_new_with_debounce(uint debounce_ms);

        [DllImport(LibraryName, CallingConvention = CallingConvention.StdCall)]
        public static extern void media_sessions_c_free(IntPtr handle);

        [DllImport(LibraryName, CallingConvention = CallingConvention.StdCall)]
        public static extern IntPtr media_sessions_c_current(IntPtr handle);

        [DllImport(LibraryName, CallingConvention = CallingConvention.StdCall)]
        public static extern void media_sessions_c_free_info(IntPtr info);

        [DllImport(LibraryName, CallingConvention = CallingConvention.StdCall)]
        public static extern IntPtr media_sessions_c_active_app(IntPtr handle);

        [DllImport(LibraryName, CallingConvention = CallingConvention.StdCall)]
        public static extern void media_sessions_c_free_string(IntPtr str);

        [DllImport(LibraryName, CallingConvention = CallingConvention.StdCall)]
        public static extern int media_sessions_c_play(IntPtr handle);

        [DllImport(LibraryName, CallingConvention = CallingConvention.StdCall)]
        public static extern int media_sessions_c_pause(IntPtr handle);

        [DllImport(LibraryName, CallingConvention = CallingConvention.StdCall)]
        public static extern int media_sessions_c_play_pause(IntPtr handle);

        [DllImport(LibraryName, CallingConvention = CallingConvention.StdCall)]
        public static extern int media_sessions_c_stop(IntPtr handle);

        [DllImport(LibraryName, CallingConvention = CallingConvention.StdCall)]
        public static extern int media_sessions_c_next(IntPtr handle);

        [DllImport(LibraryName, CallingConvention = CallingConvention.StdCall)]
        public static extern int media_sessions_c_previous(IntPtr handle);

        [DllImport(LibraryName, CallingConvention = CallingConvention.StdCall)]
        public static extern int media_sessions_c_seek(IntPtr handle, ulong position_secs);

        [DllImport(LibraryName, CallingConvention = CallingConvention.StdCall)]
        public static extern int media_sessions_c_set_volume(IntPtr handle, double volume);

        [DllImport(LibraryName, CallingConvention = CallingConvention.StdCall)]
        public static extern int media_sessions_c_set_repeat_mode(IntPtr handle, int mode);

        [DllImport(LibraryName, CallingConvention = CallingConvention.StdCall)]
        public static extern int media_sessions_c_set_shuffle(IntPtr handle, [MarshalAs(UnmanagedType.I1)] bool enabled);

        [DllImport(LibraryName, CallingConvention = CallingConvention.StdCall)]
        public static extern IntPtr media_sessions_c_version();

        [DllImport(LibraryName, CallingConvention = CallingConvention.StdCall)]
        public static extern IntPtr media_sessions_c_platform();
    }
}

/// <summary>
/// Example program demonstrating media-sessions usage from C#.
/// </summary>
public class Program
{
    public static void Main(string[] args)
    {
        Console.WriteLine("🎵 media-sessions C# Example");
        Console.WriteLine($"   Version: {MediaSessionsWrapper.Version()}");
        Console.WriteLine($"   Platform: {MediaSessionsWrapper.Platform()}");
        Console.WriteLine();

        try
        {
            using var sessions = new MediaSessionsWrapper(500);
            Console.WriteLine("✅ Media sessions initialized");
            Console.WriteLine();

            Console.WriteLine("📻 Querying current media session...");
            var info = sessions.Current();

            if (info != null)
            {
                Console.WriteLine(info);

                Console.WriteLine("\n🎛️ Demo: Toggling play/pause...");
                sessions.PlayPause();
                System.Threading.Thread.Sleep(500);
                sessions.PlayPause();
                Console.WriteLine("✅ Demo complete!");
            }
            else
            {
                Console.WriteLine("   No active media session found.");
                Console.WriteLine("   Please start a media player (Spotify, Firefox, etc.)");
            }

            var app = sessions.ActiveApp();
            if (!string.IsNullOrEmpty(app))
            {
                Console.WriteLine($"\n📱 Active player: {app}");
            }

            Console.WriteLine("\n📋 What would you like to do?");
            Console.WriteLine("   1. Watch events (polling)");
            Console.WriteLine("   2. Exit");
            Console.WriteLine();

            Console.Write("> ");
            var choice = Console.ReadLine();

            if (choice == "1")
            {
                Console.WriteLine("\n📡 Watching for media events... (Press Ctrl+C to stop)");
                string? lastTitle = null;

                try
                {
                    while (true)
                    {
                        var currentInfo = sessions.Current();
                        if (currentInfo != null)
                        {
                            if (currentInfo.Title != lastTitle)
                            {
                                lastTitle = currentInfo.Title;
                                Console.WriteLine($"\n🎵 Now playing: {currentInfo}");
                            }
                        }
                        System.Threading.Thread.Sleep(1000);
                    }
                }
                catch (OperationCanceledException)
                {
                    Console.WriteLine("\n\nGoodbye! 👋");
                }
            }
            else
            {
                Console.WriteLine("Goodbye! 👋");
            }
        }
        catch (InvalidOperationException ex)
        {
            Console.WriteLine($"❌ Error: {ex.Message}");
            Console.WriteLine("   Please ensure the media_sessions_c library is built and available.");
            Environment.Exit(1);
        }
    }
}
