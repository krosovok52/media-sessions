/**
 * @file media_sessions_c.h
 * @brief C API for Media Sessions library
 * 
 * Cross-platform media session control for Rust with C FFI bindings.
 * Supports Windows, macOS, and Linux.
 * 
 * @version 0.2.0
 * @author krosov_ok
 * @license MIT OR Apache-2.0
 */

#ifndef MEDIA_SESSIONS_C_H
#define MEDIA_SESSIONS_C_H

#ifdef __cplusplus
extern "C" {
#endif

#include <stdint.h>
#include <stdbool.h>
#include <stddef.h>

/* ============================================================================
 * Platform-specific types
 * ============================================================================ */

#if defined(_WIN32) || defined(_WIN64)
    #define MEDIA_SESSIONS_API __declspec(dllexport)
    #define MEDIA_SESSIONS_CALL __stdcall
#elif defined(__linux__) || defined(__APPLE__)
    #define MEDIA_SESSIONS_API __attribute__((visibility("default")))
    #define MEDIA_SESSIONS_CALL
#else
    #define MEDIA_SESSIONS_API
    #define MEDIA_SESSIONS_CALL
#endif

/* ============================================================================
 * Enumerations
 * ============================================================================ */

/**
 * @brief Playback status codes
 */
typedef enum {
    MEDIA_STATUS_PLAYING = 0,
    MEDIA_STATUS_PAUSED = 1,
    MEDIA_STATUS_STOPPED = 2,
    MEDIA_STATUS_TRANSITIONING = 3
} MediaPlaybackStatus;

/**
 * @brief Repeat mode codes
 */
typedef enum {
    MEDIA_REPEAT_NONE = 0,
    MEDIA_REPEAT_ONE = 1,
    MEDIA_REPEAT_ALL = 2
} MediaRepeatMode;

/**
 * @brief Result codes for API functions
 */
typedef enum {
    MEDIA_RESULT_OK = 0,
    MEDIA_RESULT_ERROR = 1,
    MEDIA_RESULT_NO_SESSION = 2,
    MEDIA_RESULT_NOT_SUPPORTED = 3,
    MEDIA_RESULT_TIMEOUT = 4,
    MEDIA_RESULT_INVALID_ARG = 5
} MediaResult;

/**
 * @brief Event types for callbacks
 */
typedef enum {
    MEDIA_EVENT_METADATA_CHANGED = 0,
    MEDIA_EVENT_PLAYBACK_STATUS_CHANGED = 1,
    MEDIA_EVENT_POSITION_CHANGED = 2,
    MEDIA_EVENT_SESSION_OPENED = 3,
    MEDIA_EVENT_SESSION_CLOSED = 4,
    MEDIA_EVENT_ARTWORK_CHANGED = 5,
    MEDIA_EVENT_VOLUME_CHANGED = 6,
    MEDIA_EVENT_REPEAT_MODE_CHANGED = 7
} MediaEventType;

/* ============================================================================
 * Opaque handles
 * ============================================================================ */

/**
 * @brief Handle to MediaSessions instance
 */
typedef struct MediaSessionsHandle MediaSessionsHandle;

/**
 * @brief Handle to event callback
 */
typedef struct EventCallbackHandle EventCallbackHandle;

/* ============================================================================
 * Data structures
 * ============================================================================ */

/**
 * @brief Media information structure
 */
typedef struct {
    char* title;              /**< Track title */
    char* artist;             /**< Track artist */
    char* album;              /**< Album name */
    uint64_t duration_secs;   /**< Duration in seconds */
    uint64_t position_secs;   /**< Current position in seconds */
    MediaPlaybackStatus playback_status; /**< Current playback status */
    bool has_artwork;         /**< Whether artwork is available */
    size_t artwork_len;       /**< Artwork size in bytes */
    uint8_t* artwork;         /**< Artwork data (PNG/JPEG) */
    uint32_t track_number;    /**< Track number in album */
    uint32_t disc_number;     /**< Disc number for multi-disc albums */
    char* genre;              /**< Genre */
    int32_t year;             /**< Release year */
    char* url;                /**< Source URL */
    char* thumbnail_url;      /**< Thumbnail URL */
} CMediaInfo;

/**
 * @brief Event callback function type
 * @param event_type Type of event (MediaEventType)
 * @param data Event-specific data (may be NULL)
 * @param user_data User-provided context pointer
 */
typedef void (MEDIA_SESSIONS_CALL *MediaEventCallback)(
    int32_t event_type,
    const void* data,
    void* user_data
);

/* ============================================================================
 * Core functions
 * ============================================================================ */

/**
 * @brief Create a new MediaSessions instance with default settings
 * @return Handle to MediaSessions, or NULL on error
 */
MEDIA_SESSIONS_API MediaSessionsHandle* MEDIA_SESSIONS_CALL 
media_sessions_c_new(void);

/**
 * @brief Create a new MediaSessions instance with custom debounce duration
 * @param debounce_ms Debounce duration in milliseconds
 * @return Handle to MediaSessions, or NULL on error
 */
MEDIA_SESSIONS_API MediaSessionsHandle* MEDIA_SESSIONS_CALL 
media_sessions_c_new_with_debounce(uint64_t debounce_ms);

/**
 * @brief Free a MediaSessions handle
 * @param handle Handle to free
 */
MEDIA_SESSIONS_API void MEDIA_SESSIONS_CALL 
media_sessions_c_free(MediaSessionsHandle* handle);

/**
 * @brief Get current media information
 * @param handle MediaSessions handle
 * @return Pointer to CMediaInfo (must be freed), or NULL if no session
 */
MEDIA_SESSIONS_API CMediaInfo* MEDIA_SESSIONS_CALL 
media_sessions_c_current(MediaSessionsHandle* handle);

/**
 * @brief Free a CMediaInfo structure and its fields
 * @param info Pointer to CMediaInfo to free
 */
MEDIA_SESSIONS_API void MEDIA_SESSIONS_CALL 
media_sessions_c_free_info(CMediaInfo* info);

/**
 * @brief Free a C string allocated by the library
 * @param s String to free
 */
MEDIA_SESSIONS_API void MEDIA_SESSIONS_CALL 
media_sessions_c_free_string(char* s);

/**
 * @brief Free artwork data allocated by the library
 * @param data Artwork data pointer
 * @param len Artwork data length
 */
MEDIA_SESSIONS_API void MEDIA_SESSIONS_CALL 
media_sessions_c_free_artwork(uint8_t* data, size_t len);

/**
 * @brief Get the active application name
 * @param handle MediaSessions handle
 * @return Application name (must be freed), or NULL on error
 */
MEDIA_SESSIONS_API char* MEDIA_SESSIONS_CALL 
media_sessions_c_active_app(MediaSessionsHandle* handle);

/* ============================================================================
 * Playback control functions
 * ============================================================================ */

/**
 * @brief Start or resume playback
 * @param handle MediaSessions handle
 * @return MediaResult code
 */
MEDIA_SESSIONS_API MediaResult MEDIA_SESSIONS_CALL 
media_sessions_c_play(MediaSessionsHandle* handle);

/**
 * @brief Pause playback
 * @param handle MediaSessions handle
 * @return MediaResult code
 */
MEDIA_SESSIONS_API MediaResult MEDIA_SESSIONS_CALL 
media_sessions_c_pause(MediaSessionsHandle* handle);

/**
 * @brief Toggle play/pause state
 * @param handle MediaSessions handle
 * @return MediaResult code
 */
MEDIA_SESSIONS_API MediaResult MEDIA_SESSIONS_CALL 
media_sessions_c_play_pause(MediaSessionsHandle* handle);

/**
 * @brief Stop playback completely
 * @param handle MediaSessions handle
 * @return MediaResult code
 */
MEDIA_SESSIONS_API MediaResult MEDIA_SESSIONS_CALL 
media_sessions_c_stop(MediaSessionsHandle* handle);

/**
 * @brief Skip to next track
 * @param handle MediaSessions handle
 * @return MediaResult code
 */
MEDIA_SESSIONS_API MediaResult MEDIA_SESSIONS_CALL 
media_sessions_c_next(MediaSessionsHandle* handle);

/**
 * @brief Skip to previous track
 * @param handle MediaSessions handle
 * @return MediaResult code
 */
MEDIA_SESSIONS_API MediaResult MEDIA_SESSIONS_CALL 
media_sessions_c_previous(MediaSessionsHandle* handle);

/**
 * @brief Seek to specified position
 * @param handle MediaSessions handle
 * @param position_secs Position in seconds
 * @return MediaResult code
 */
MEDIA_SESSIONS_API MediaResult MEDIA_SESSIONS_CALL 
media_sessions_c_seek(MediaSessionsHandle* handle, uint64_t position_secs);

/* ============================================================================
 * Extended control functions
 * ============================================================================ */

/**
 * @brief Set volume level
 * @param handle MediaSessions handle
 * @param volume Volume level (0.0 to 1.0)
 * @return MediaResult code
 */
MEDIA_SESSIONS_API MediaResult MEDIA_SESSIONS_CALL 
media_sessions_c_set_volume(MediaSessionsHandle* handle, double volume);

/**
 * @brief Set repeat mode
 * @param handle MediaSessions handle
 * @param mode Repeat mode (None/One/All)
 * @return MediaResult code
 */
MEDIA_SESSIONS_API MediaResult MEDIA_SESSIONS_CALL 
media_sessions_c_set_repeat_mode(MediaSessionsHandle* handle, MediaRepeatMode mode);

/**
 * @brief Set shuffle mode
 * @param handle MediaSessions handle
 * @param enabled True to enable shuffle, false to disable
 * @return MediaResult code
 */
MEDIA_SESSIONS_API MediaResult MEDIA_SESSIONS_CALL 
media_sessions_c_set_shuffle(MediaSessionsHandle* handle, bool enabled);

/* ============================================================================
 * Utility functions
 * ============================================================================ */

/**
 * @brief Get library version string
 * @return Version string (static, do not free)
 */
MEDIA_SESSIONS_API const char* MEDIA_SESSIONS_CALL 
media_sessions_c_version(void);

/**
 * @brief Get current platform name
 * @return Platform name: "windows", "linux", "macos", or "unknown" (static, do not free)
 */
MEDIA_SESSIONS_API const char* MEDIA_SESSIONS_CALL 
media_sessions_c_platform(void);

/* ============================================================================
 * Event callback functions (future implementation)
 * ============================================================================ */

/**
 * @brief Register event callback (placeholder)
 * @param handle MediaSessions handle
 * @param callback Callback function
 * @param user_data User context pointer
 * @return Event callback handle, or NULL on error
 */
MEDIA_SESSIONS_API EventCallbackHandle* MEDIA_SESSIONS_CALL 
media_sessions_c_register_callback(
    MediaSessionsHandle* handle,
    MediaEventCallback callback,
    void* user_data
);

/**
 * @brief Free an event callback handle
 * @param handle Event callback handle to free
 */
MEDIA_SESSIONS_API void MEDIA_SESSIONS_CALL 
media_sessions_c_free_callback(EventCallbackHandle* handle);

#ifdef __cplusplus
}
#endif

#endif /* MEDIA_SESSIONS_C_H */
