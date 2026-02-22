#!/usr/bin/env python3
"""
Python example for media-sessions C API.

This example demonstrates how to use the media-sessions library
from Python using ctypes FFI bindings.

Requirements:
    - Build the C API: cargo build --release --features c-api
    - Copy the library to this directory or system path

Usage:
    python python_example.py
"""

import ctypes
import sys
import os

# Set UTF-8 encoding for Windows console
if sys.platform == 'win32':
    try:
        sys.stdout.reconfigure(encoding='utf-8')
    except AttributeError:
        # Python < 3.7
        import codecs
        sys.stdout = codecs.getwriter('utf-8')(sys.stdout.buffer)

from ctypes import (
    c_void_p, c_char_p, c_uint8, c_uint64, c_uint32,
    c_int32, c_double, c_bool, c_size_t, Structure, POINTER
)
from enum import IntEnum


# ============================================================================
# Enumerations
# ============================================================================

class PlaybackStatus(IntEnum):
    PLAYING = 0
    PAUSED = 1
    STOPPED = 2
    TRANSITIONING = 3


class RepeatMode(IntEnum):
    NONE = 0
    ONE = 1
    ALL = 2


class MediaResult(IntEnum):
    OK = 0
    ERROR = 1
    NO_SESSION = 2
    NOT_SUPPORTED = 3
    TIMEOUT = 4
    INVALID_ARG = 5


# ============================================================================
# Structures
# ============================================================================

class CMediaInfo(Structure):
    """Media information structure."""
    _fields_ = [
        ("title", c_char_p),
        ("artist", c_char_p),
        ("album", c_char_p),
        ("duration_secs", c_uint64),
        ("position_secs", c_uint64),
        ("playback_status", c_int32),
        ("has_artwork", c_bool),
        ("artwork_len", c_size_t),
        ("artwork", POINTER(c_uint8)),
        ("track_number", c_uint32),
        ("disc_number", c_uint32),
        ("genre", c_char_p),
        ("year", c_int32),
        ("url", c_char_p),
        ("thumbnail_url", c_char_p),
    ]


# ============================================================================
# Library loading
# ============================================================================

def truncate(s, max_len):
    """Truncate string to max length."""
    if len(s) <= max_len:
        return s
    return s[:max_len - 3] + "..."


def load_library():
    """Load the media_sessions C library."""
    # Try different library names based on platform
    import platform
    import os
    
    system = platform.system()
    
    # Get absolute path to library in same directory as this script
    script_dir = os.path.dirname(os.path.abspath(__file__))
    
    if system == "Windows":
        lib_names = [
            os.path.join(script_dir, "media_sessions_c.dll"),
            os.path.join(script_dir, "..", "target", "release", "media_sessions_c.dll"),
            "media_sessions_c.dll"
        ]
    elif system == "Darwin":
        lib_names = [
            os.path.join(script_dir, "libmedia_sessions_c.dylib"),
            "libmedia_sessions_c.dylib"
        ]
    else:  # Linux
        lib_names = [
            os.path.join(script_dir, "libmedia_sessions_c.so"),
            "libmedia_sessions_c.so"
        ]
    
    # Try to load from paths
    for lib_path in lib_names:
        try:
            if os.path.exists(lib_path):
                return ctypes.CDLL(lib_path)
        except OSError as e:
            print(f"Trying {lib_path}... failed: {e}")
            continue
    
    # Try system library path
    if system == "Windows":
        lib_name = "media_sessions_c.dll"
    elif system == "Darwin":
        lib_name = "libmedia_sessions_c.dylib"
    else:
        lib_name = "libmedia_sessions_c.so"
    
    try:
        return ctypes.CDLL(lib_name)
    except OSError as e:
        print(f"❌ Failed to load library: {e}")
        print(f"   Please build with: cargo build --release --features c-api")
        print(f"   And copy the library to this directory or system path.")
        sys.exit(1)


# ============================================================================
# API wrapper class
# ============================================================================

class MediaSessions:
    """Python wrapper for media-sessions C API."""

    def __init__(self, debounce_ms=None):
        """Initialize media sessions."""
        self.lib = load_library()
        self._setup_prototypes()

        if debounce_ms is not None:
            self.handle = self.lib.media_sessions_c_new_with_debounce(debounce_ms)
        else:
            self.handle = self.lib.media_sessions_c_new()

        if not self.handle:
            # Try to get more info about the error
            platform_name = self.lib.media_sessions_c_platform()
            version = self.lib.media_sessions_c_version()
            raise RuntimeError(
                f"Failed to create MediaSessions instance. "
                f"Platform: {platform_name}, Version: {version}"
            )
    
    def _setup_prototypes(self):
        """Setup function prototypes."""
        lib = self.lib

        # media_sessions_c_new
        lib.media_sessions_c_new.argtypes = []
        lib.media_sessions_c_new.restype = c_void_p

        # media_sessions_c_new_with_debounce
        lib.media_sessions_c_new_with_debounce.argtypes = [c_uint64]
        lib.media_sessions_c_new_with_debounce.restype = c_void_p

        # media_sessions_c_free
        lib.media_sessions_c_free.argtypes = [c_void_p]
        lib.media_sessions_c_free.restype = None

        # media_sessions_c_current
        lib.media_sessions_c_current.argtypes = [c_void_p]
        lib.media_sessions_c_current.restype = POINTER(CMediaInfo)

        # media_sessions_c_free_info
        lib.media_sessions_c_free_info.argtypes = [POINTER(CMediaInfo)]
        lib.media_sessions_c_free_info.restype = None

        # media_sessions_c_active_app
        lib.media_sessions_c_active_app.argtypes = [c_void_p]
        lib.media_sessions_c_active_app.restype = c_char_p

        # media_sessions_c_free_string
        lib.media_sessions_c_free_string.argtypes = [c_char_p]
        lib.media_sessions_c_free_string.restype = None

        # Playback controls
        for func_name in [
            "media_sessions_c_play", "media_sessions_c_pause",
            "media_sessions_c_play_pause", "media_sessions_c_stop",
            "media_sessions_c_next", "media_sessions_c_previous"
        ]:
            getattr(lib, func_name).argtypes = [c_void_p]
            getattr(lib, func_name).restype = c_int32

        # media_sessions_c_seek
        lib.media_sessions_c_seek.argtypes = [c_void_p, c_uint64]
        lib.media_sessions_c_seek.restype = c_int32

        # Extended controls
        lib.media_sessions_c_set_volume.argtypes = [c_void_p, c_double]
        lib.media_sessions_c_set_volume.restype = c_int32

        lib.media_sessions_c_set_repeat_mode.argtypes = [c_void_p, c_int32]
        lib.media_sessions_c_set_repeat_mode.restype = c_int32

        lib.media_sessions_c_set_shuffle.argtypes = [c_void_p, c_bool]
        lib.media_sessions_c_set_shuffle.restype = c_int32

        # Utility functions
        lib.media_sessions_c_version.argtypes = []
        lib.media_sessions_c_version.restype = c_char_p

        lib.media_sessions_c_platform.argtypes = []
        lib.media_sessions_c_platform.restype = c_char_p

    def current(self):
        """Get current media information."""
        info_ptr = self.lib.media_sessions_c_current(self.handle)
        if not info_ptr:
            return None
        
        info = info_ptr.contents
        
        # Helper to decode UTF-8 strings
        def decode_str(val):
            if val is None:
                return ""
            try:
                return val.decode('utf-8')
            except (UnicodeDecodeError, AttributeError):
                return str(val) if val else ""
        
        result = {
            'title': decode_str(info.title),
            'artist': decode_str(info.artist),
            'album': decode_str(info.album),
            'genre': decode_str(info.genre),
            'url': decode_str(info.url),
            'duration_secs': info.duration_secs,
            'position_secs': info.position_secs,
            'playback_status': info.playback_status,
            'has_artwork': info.has_artwork,
            'artwork_len': info.artwork_len,
            'track_number': info.track_number,
            'disc_number': info.disc_number,
            'year': info.year,
        }
        
        # Don't free - Rust already freed when returning from function
        # self.lib.media_sessions_c_free_info(info_ptr)
        
        return result
    
    def active_app(self):
        """Get active application name."""
        app = self.lib.media_sessions_c_active_app(self.handle)
        if app:
            result = app.decode()
            self.lib.media_sessions_c_free_string(app)
            return result
        return None
    
    def play(self):
        """Start/resume playback."""
        return self.lib.media_sessions_c_play(self.handle) == MediaResult.OK.value
    
    def pause(self):
        """Pause playback."""
        return self.lib.media_sessions_c_pause(self.handle) == MediaResult.OK.value
    
    def play_pause(self):
        """Toggle play/pause."""
        return self.lib.media_sessions_c_play_pause(self.handle) == MediaResult.OK.value
    
    def stop(self):
        """Stop playback."""
        return self.lib.media_sessions_c_stop(self.handle) == MediaResult.OK.value
    
    def next(self):
        """Skip to next track."""
        return self.lib.media_sessions_c_next(self.handle) == MediaResult.OK.value
    
    def previous(self):
        """Skip to previous track."""
        return self.lib.media_sessions_c_previous(self.handle) == MediaResult.OK.value
    
    def seek(self, position_secs):
        """Seek to position."""
        return self.lib.media_sessions_c_seek(self.handle, position_secs) == MediaResult.OK.value
    
    def set_volume(self, volume):
        """Set volume (0.0 to 1.0)."""
        return self.lib.media_sessions_c_set_volume(self.handle, volume) == MediaResult.OK.value
    
    def set_repeat_mode(self, mode):
        """Set repeat mode."""
        return self.lib.media_sessions_c_set_repeat_mode(self.handle, mode.value) == MediaResult.OK.value
    
    def set_shuffle(self, enabled):
        """Set shuffle mode."""
        return self.lib.media_sessions_c_set_shuffle(self.handle, enabled) == MediaResult.OK.value
    
    @staticmethod
    def version():
        """Get library version."""
        try:
            lib = load_library()
            lib.media_sessions_c_version.argtypes = []
            lib.media_sessions_c_version.restype = c_char_p
            version = lib.media_sessions_c_version()
            return version.decode() if version else "unknown"
        except Exception as e:
            return f"error: {e}"
    
    @staticmethod
    def platform():
        """Get current platform."""
        try:
            lib = load_library()
            lib.media_sessions_c_platform.argtypes = []
            lib.media_sessions_c_platform.restype = c_char_p
            platform = lib.media_sessions_c_platform()
            return platform.decode() if platform else "unknown"
        except Exception as e:
            return f"error: {e}"


# ============================================================================
# Main example
# ============================================================================

def main():
    """Main example function."""
    import sys
    print("Starting...", flush=True)
    print(f"Python: {sys.version}", flush=True)
    
    try:
        print("Getting version...", flush=True)
        version = MediaSessions.version()
        print(f"Version: {version}", flush=True)
        
        print("Getting platform...", flush=True)
        platform = MediaSessions.platform()
        print(f"Platform: {platform}", flush=True)
        
        print("Creating sessions...", flush=True)
        sessions = MediaSessions(debounce_ms=500)
        print("OK: Media sessions initialized\n", flush=True)
    except Exception as e:
        print(f"Error: {e}", flush=True)
        import traceback
        traceback.print_exc()
        return
    
    # Get current media info
    print("Querying current media session...", flush=True)
    info = sessions.current()

    if info:
        # Debug: print raw values
        print("\n🔍 [DEBUG] Raw metadata:", flush=True)
        print(f"   title: '{info.get('title', '')}'", flush=True)
        print(f"   artist: '{info.get('artist', '')}'", flush=True)
        print(f"   album: '{info.get('album', '')}'", flush=True)
        print(f"   year: {info.get('year', 0)}, track: {info.get('track_number', 0)}", flush=True)
        print(f"   duration: {info.get('duration_secs', 0)}s, position: {info.get('position_secs', 0)}s", flush=True)
        print(flush=True)

        # Print formatted
        title = info.get('title', '')
        artist = info.get('artist', '')
        album = info.get('album', '')
        status = info.get('playback_status', 0)
        duration = info.get('duration_secs', 0)
        position = info.get('position_secs', 0)

        status_names = {0: "▶ Playing", 1: "⏸ Paused", 2: "⏹ Stopped", 3: "⏳ Transitioning"}
        status_str = status_names.get(status, "Unknown")

        print("\n╔═══════════════════════════════════════════════════════════╗", flush=True)
        print("║                    Now Playing                            ║", flush=True)
        print("╠═══════════════════════════════════════════════════════════╣", flush=True)

        if title or artist:
            display = f"{artist} - {title}" if artist and title else (title or artist)
            print(f"║  {truncate(display, 55):<55} ║", flush=True)
            if album:
                print(f"║  Album: {truncate(album, 47):<47} ║", flush=True)
        else:
            print("║  No media information available                           ║", flush=True)

        print("╠═══════════════════════════════════════════════════════════╣", flush=True)
        print(f"║  Status: {status_str:<46} ║", flush=True)

        if duration > 0:
            progress = (position / duration) * 100
            bar = "█" * int(progress/100 * 40) + "░" * (40 - int(progress/100 * 40))
            print(f"║  [{bar}] {position}s/{duration}s ({progress:.1f}%) ║", flush=True)

        print("╚═══════════════════════════════════════════════════════════╝", flush=True)

        # Demo controls
        print("\n🎛️ Demo: Toggling play/pause...", flush=True)
        sessions.play_pause()
        import time
        time.sleep(0.5)
        sessions.play_pause()
        print("✅ Demo complete!", flush=True)
    else:
        print("   No active media session found.", flush=True)
        print("   Please start a media player (Spotify, Firefox, etc.)", flush=True)

    # Show active app
    app = sessions.active_app()
    if app:
        print(f"\n📱 Active player: {app}")
    else:
        print(f"\n📱 Active player: <none>")

    print("\n✅ Python example completed successfully!")

    # Use _exit to avoid cleanup that causes crashes
    import os
    os._exit(0)


if __name__ == "__main__":
    main()
