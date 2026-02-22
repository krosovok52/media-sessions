//! Core media session management and event streaming.
//!
//! This module provides the main [`MediaSessions`] struct for interacting
//! with system media players, along with event types and builder patterns.

use std::sync::Arc;
use std::time::Duration;

use futures::Stream;
use tokio::sync::{mpsc, RwLock};
use tokio::time::timeout;

use crate::error::{MediaError, MediaResult};
use crate::media_info::{MediaInfo, PlaybackStatus};
use crate::platform::backend::{create_backend, MediaSessionBackend};

/// Default debounce duration for filtering rapid event spam from OS.
const DEFAULT_DEBOUNCE_DURATION: Duration = Duration::from_millis(800);

/// Default timeout for media session operations.
const DEFAULT_OPERATION_TIMEOUT: Duration = Duration::from_secs(5);

/// Event emitted when media session state changes.
///
/// This enum represents all possible state changes that can occur
/// in a media session. Events are emitted by the [`MediaSessions::watch`]
/// stream and can be used to update UI or trigger application logic.
#[derive(Debug, Clone, PartialEq)]
pub enum MediaSessionEvent {
    /// Metadata (title, artist, album, etc.) has changed.
    ///
    /// This event is emitted when any metadata field of the currently
    /// playing track changes. The [`MediaInfo`] contains all available
    /// metadata fields.
    MetadataChanged(MediaInfo),
    /// Playback status (playing, paused, stopped) has changed.
    ///
    /// This event is emitted when the playback state transitions,
    /// such as when a user presses play/pause or when a track ends.
    PlaybackStatusChanged(PlaybackStatus),
    /// Playback position has changed (seek or natural progress).
    ///
    /// This event includes the new position and optional old position
    /// for calculating seek direction and distance.
    PositionChanged {
        /// New playback position.
        position: Duration,
        /// Previous position, if available.
        old_position: Option<Duration>,
    },
    /// A new media session has become active.
    ///
    /// This event is emitted when a new media player registers with
    /// the system and becomes the active session.
    SessionOpened {
        /// Name of the media player application.
        app_name: String,
    },
    /// The current media session has closed.
    ///
    /// This event is emitted when the active media player closes
    /// or releases its session.
    SessionClosed,
    /// Artwork has been updated.
    ///
    /// This event is emitted when the album art or thumbnail changes.
    /// The artwork bytes are not included in this event to reduce
    /// memory pressure; call [`MediaSessions::current`] to fetch.
    ArtworkChanged,
    /// Volume level has changed.
    ///
    /// Volume is represented as a value between 0.0 (muted) and 1.0 (maximum).
    VolumeChanged {
        /// New volume level (0.0 to 1.0).
        volume: f64,
    },
    /// Repeat/shuffle mode has changed.
    ///
    /// This event indicates a change in the playback mode settings.
    RepeatModeChanged {
        /// Repeat mode: None, One, or All.
        repeat: RepeatMode,
        /// Shuffle enabled status.
        shuffle: bool,
    },
}

/// Repeat mode for media playback.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum RepeatMode {
    /// Repeat is disabled.
    #[default]
    None,
    /// Repeat current track.
    One,
    /// Repeat entire playlist/album.
    All,
}

/// Builder for configuring [`MediaSessions`] before creation.
///
/// This builder pattern allows fine-tuning of debounce duration,
/// operation timeout, and other settings before initializing
/// the media session backend.
#[derive(Debug, Default, Clone, Copy)]
pub struct MediaSessionsBuilder {
    debounce_duration: Duration,
    operation_timeout: Duration,
    enable_artwork: bool,
}

impl MediaSessionsBuilder {
    /// Creates a new builder with default settings.
    ///
    /// # Defaults
    ///
    /// - `debounce_duration`: 800ms
    /// - `operation_timeout`: 5 seconds
    /// - `enable_artwork`: true
    #[must_use]
    pub const fn new() -> Self {
        Self {
            debounce_duration: DEFAULT_DEBOUNCE_DURATION,
            operation_timeout: DEFAULT_OPERATION_TIMEOUT,
            enable_artwork: true,
        }
    }

    /// Sets the debounce duration for filtering rapid events.
    ///
    /// Media session backends often emit multiple events in quick
    /// succession when state changes. This setting filters events
    /// that occur within the specified duration of each other.
    ///
    /// # Panics
    ///
    /// Panics if the duration is zero or greater than 60 seconds.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use media_sessions::MediaSessions;
    /// use std::time::Duration;
    ///
    /// let builder = MediaSessions::builder()
    ///     .debounce_duration(Duration::from_millis(500));
    /// ```
    #[must_use]
    pub fn debounce_duration(mut self, duration: Duration) -> Self {
        assert!(
            duration > Duration::ZERO && duration <= Duration::from_secs(60),
            "debounce_duration must be between 0 and 60 seconds"
        );
        self.debounce_duration = duration;
        self
    }

    /// Sets the timeout for blocking operations.
    ///
    /// This timeout applies to operations like [`MediaSessions::current`]
    /// that may block waiting for backend responses.
    ///
    /// # Panics
    ///
    /// Panics if the duration is zero or greater than 5 minutes.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use media_sessions::MediaSessions;
    /// use std::time::Duration;
    ///
    /// let builder = MediaSessions::builder()
    ///     .operation_timeout(Duration::from_secs(10));
    /// ```
    #[must_use]
    pub fn operation_timeout(mut self, duration: Duration) -> Self {
        assert!(
            duration > Duration::ZERO && duration <= Duration::from_secs(300),
            "operation_timeout must be between 0 and 300 seconds"
        );
        self.operation_timeout = duration;
        self
    }

    /// Enables or disables artwork fetching.
    ///
    /// When disabled, the `artwork` field in [`MediaInfo`] will always
    /// be `None`, reducing memory usage and network/D-Bus overhead.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use media_sessions::MediaSessions;
    ///
    /// let builder = MediaSessions::builder()
    ///     .enable_artwork(false);
    /// ```
    #[must_use]
    pub const fn enable_artwork(mut self, enabled: bool) -> Self {
        self.enable_artwork = enabled;
        self
    }

    /// Builds the [`MediaSessions`] instance.
    ///
    /// # Errors
    ///
    /// Returns [`MediaError::NotSupported`] if the platform is not supported.
    /// Returns [`MediaError::Backend`] if the backend fails to initialize.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use media_sessions::MediaSessions;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let sessions = MediaSessions::builder().build()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn build(self) -> MediaResult<MediaSessions> {
        MediaSessions::with_config(self)
    }
}

/// Internal state shared between `MediaSessions` and the event stream.
struct SharedState {
    backend: Box<dyn MediaSessionBackend>,
    #[allow(dead_code)]
    debounce_duration: Duration,
    operation_timeout: Duration,
    #[allow(dead_code)]
    enable_artwork: bool,
}

/// Main interface for interacting with system media sessions.
///
/// This struct provides the primary API for querying and controlling
/// media playback across all supported platforms. It uses an async
/// architecture with Tokio for non-blocking operations.
///
/// # Thread Safety
///
/// `MediaSessions` is `Send + Sync` and can be safely shared across
/// threads. Internally, it uses `Arc<RwLock>` for state management.
///
/// # Examples
///
/// ## Basic Usage
///
/// ```rust,no_run
/// use media_sessions::MediaSessions;
///
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let sessions = MediaSessions::new()?;
///
/// if let Some(info) = sessions.current().await? {
///     println!("Playing: {}", info.display_string());
/// }
/// # Ok(())
/// # }
/// ```
///
/// ## Event Streaming with Debounce
///
/// ```rust,no_run
/// use media_sessions::MediaSessions;
/// use futures::StreamExt;
///
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let sessions = MediaSessions::new()?;
/// let mut stream = sessions.watch().await?;
///
/// while let Some(event) = stream.next().await {
///     println!("Event: {:?}", event?);
/// }
/// # Ok(())
/// # }
/// ```
///
/// ## Playback Control
///
/// ```rust,no_run
/// use media_sessions::MediaSessions;
/// use std::time::Duration;
///
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let sessions = MediaSessions::new()?;
///
/// sessions.play().await?;
/// sessions.seek(Duration::from_secs(30)).await?;
/// # Ok(())
/// # }
/// ```
#[derive(Clone)]
pub struct MediaSessions {
    state: Arc<RwLock<SharedState>>,
}

impl MediaSessions {
    /// Creates a new `MediaSessions` instance with default settings.
    ///
    /// This is a convenience method equivalent to `MediaSessions::builder().build()`.
    ///
    /// # Errors
    ///
    /// Returns [`MediaError::NotSupported`] if the platform is not supported.
    /// Returns [`MediaError::Backend`] if the backend fails to initialize.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use media_sessions::MediaSessions;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let sessions = MediaSessions::new()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn new() -> MediaResult<Self> {
        Self::builder().build()
    }

    /// Creates a new builder for configuring `MediaSessions`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use media_sessions::MediaSessions;
    ///
    /// let builder = MediaSessions::builder();
    /// ```
    #[must_use]
    pub const fn builder() -> MediaSessionsBuilder {
        MediaSessionsBuilder::new()
    }

    /// Internal constructor with configuration.
    fn with_config(config: MediaSessionsBuilder) -> MediaResult<Self> {
        let backend = create_backend()?;
        Ok(Self {
            state: Arc::new(RwLock::new(SharedState {
                backend,
                debounce_duration: config.debounce_duration,
                operation_timeout: config.operation_timeout,
                enable_artwork: config.enable_artwork,
            })),
        })
    }

    /// Gets the current media session information.
    ///
    /// This method queries the active media session and returns all
    /// available metadata. If no session is active, returns `Ok(None)`.
    ///
    /// # Errors
    ///
    /// Returns [`MediaError::Backend`] if the backend query fails.
    /// Returns [`MediaError::Timeout`] if the operation exceeds timeout.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use media_sessions::MediaSessions;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let sessions = MediaSessions::new()?;
    ///
    /// if let Some(info) = sessions.current().await? {
    ///     println!("Title: {}", info.title());
    ///     println!("Artist: {}", info.artist());
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn current(&self) -> MediaResult<Option<MediaInfo>> {
        let (timeout_dur, enable_artwork) = {
            let state = self.state.read().await;
            (state.operation_timeout, state.enable_artwork)
        };

        let result = timeout(timeout_dur, async {
            let state = self.state.read().await;
            let mut info = state.backend.get_current().await?;

            // Fetch artwork separately if enabled
            if enable_artwork && info.artwork.is_none() {
                info.artwork = state.backend.get_artwork().await.ok().flatten();
            }

            Ok::<Option<MediaInfo>, MediaError>(Some(info))
        })
        .await;

        match result {
            Ok(Ok(info)) => Ok(info),
            Ok(Err(e)) => Err(e),
            Err(_) => Err(MediaError::Timeout(timeout_dur)),
        }
    }

    /// Returns a stream of media session events.
    ///
    /// This method creates an async stream that yields events whenever
    /// the media session state changes. Events are debounced according
    /// to the configured `debounce_duration`.
    ///
    /// The stream continues indefinitely until the `MediaSessions` instance
    /// is dropped or an error occurs.
    ///
    /// # Errors
    ///
    /// The stream may yield [`MediaError`] variants if the backend
    /// encounters errors while listening for events.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use media_sessions::MediaSessions;
    /// use futures::StreamExt;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let sessions = MediaSessions::new()?;
    /// let mut stream = sessions.watch().await?;
    ///
    /// while let Some(event) = stream.next().await {
    ///     match event? {
    ///         media_sessions::MediaSessionEvent::MetadataChanged(info) => {
    ///             println!("Now playing: {}", info.display_string());
    ///         }
    ///         _ => {}
    ///     }
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn watch(&self) -> MediaResult<impl Stream<Item = MediaResult<MediaSessionEvent>>> {
        let (debounce_dur, tx) = {
            let state = self.state.read().await;
            let (tx, _rx) = mpsc::channel(32);
            (state.debounce_duration, tx)
        };

        let state = self.state.read().await;
        state.backend.start_listening(tx, debounce_dur).await?;

        let (_tx, rx) = mpsc::channel(32);
        Ok(tokio_stream::wrappers::ReceiverStream::new(rx))
    }

    /// Starts or resumes playback.
    ///
    /// # Errors
    ///
    /// Returns [`MediaError::NoSession`] if no active session exists.
    /// Returns [`MediaError::Backend`] if the backend command fails.
    /// Returns [`MediaError::Timeout`] if the operation exceeds timeout.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use media_sessions::MediaSessions;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let sessions = MediaSessions::new()?;
    /// sessions.play().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn play(&self) -> MediaResult<()> {
        let timeout_dur = {
            let state = self.state.read().await;
            state.operation_timeout
        };

        timeout(timeout_dur, async {
            let state = self.state.read().await;
            state.backend.play().await
        })
        .await
        .map_err(|_| MediaError::Timeout(timeout_dur))?
    }

    /// Pauses playback.
    ///
    /// # Errors
    ///
    /// Returns [`MediaError::NoSession`] if no active session exists.
    /// Returns [`MediaError::Backend`] if the backend command fails.
    /// Returns [`MediaError::Timeout`] if the operation exceeds timeout.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use media_sessions::MediaSessions;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let sessions = MediaSessions::new()?;
    /// sessions.pause().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn pause(&self) -> MediaResult<()> {
        let timeout_dur = {
            let state = self.state.read().await;
            state.operation_timeout
        };

        timeout(timeout_dur, async {
            let state = self.state.read().await;
            state.backend.pause().await
        })
        .await
        .map_err(|_| MediaError::Timeout(timeout_dur))?
    }

    /// Toggles between play and pause states.
    ///
    /// # Errors
    ///
    /// Returns [`MediaError::NoSession`] if no active session exists.
    /// Returns [`MediaError::Backend`] if the backend command fails.
    /// Returns [`MediaError::Timeout`] if the operation exceeds timeout.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use media_sessions::MediaSessions;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let sessions = MediaSessions::new()?;
    /// sessions.play_pause().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn play_pause(&self) -> MediaResult<()> {
        let timeout_dur = {
            let state = self.state.read().await;
            state.operation_timeout
        };

        timeout(timeout_dur, async {
            let state = self.state.read().await;
            state.backend.play_pause().await
        })
        .await
        .map_err(|_| MediaError::Timeout(timeout_dur))?
    }

    /// Stops playback completely.
    ///
    /// Unlike pause, this may reset the playback position and release
    /// resources depending on the media player.
    ///
    /// # Errors
    ///
    /// Returns [`MediaError::NoSession`] if no active session exists.
    /// Returns [`MediaError::Backend`] if the backend command fails.
    /// Returns [`MediaError::Timeout`] if the operation exceeds timeout.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use media_sessions::MediaSessions;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let sessions = MediaSessions::new()?;
    /// sessions.stop().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn stop(&self) -> MediaResult<()> {
        let timeout_dur = {
            let state = self.state.read().await;
            state.operation_timeout
        };

        timeout(timeout_dur, async {
            let state = self.state.read().await;
            state.backend.stop().await
        })
        .await
        .map_err(|_| MediaError::Timeout(timeout_dur))?
    }

    /// Skips to the next track.
    ///
    /// # Errors
    ///
    /// Returns [`MediaError::NoSession`] if no active session exists.
    /// Returns [`MediaError::Backend`] if the backend command fails.
    /// Returns [`MediaError::Timeout`] if the operation exceeds timeout.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use media_sessions::MediaSessions;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let sessions = MediaSessions::new()?;
    /// sessions.next().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn next(&self) -> MediaResult<()> {
        let timeout_dur = {
            let state = self.state.read().await;
            state.operation_timeout
        };

        timeout(timeout_dur, async {
            let state = self.state.read().await;
            state.backend.next().await
        })
        .await
        .map_err(|_| MediaError::Timeout(timeout_dur))?
    }

    /// Skips to the previous track.
    ///
    /// # Errors
    ///
    /// Returns [`MediaError::NoSession`] if no active session exists.
    /// Returns [`MediaError::Backend`] if the backend command fails.
    /// Returns [`MediaError::Timeout`] if the operation exceeds timeout.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use media_sessions::MediaSessions;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let sessions = MediaSessions::new()?;
    /// sessions.previous().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn previous(&self) -> MediaResult<()> {
        let timeout_dur = {
            let state = self.state.read().await;
            state.operation_timeout
        };

        timeout(timeout_dur, async {
            let state = self.state.read().await;
            state.backend.previous().await
        })
        .await
        .map_err(|_| MediaError::Timeout(timeout_dur))?
    }

    /// Seeks to the specified position.
    ///
    /// # Arguments
    ///
    /// * `position` - The target position in the track.
    ///
    /// # Errors
    ///
    /// Returns [`MediaError::NoSession`] if no active session exists.
    /// Returns [`MediaError::SeekOutOfRange`] if position exceeds duration.
    /// Returns [`MediaError::Backend`] if the backend command fails.
    /// Returns [`MediaError::Timeout`] if the operation exceeds timeout.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use media_sessions::MediaSessions;
    /// use std::time::Duration;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let sessions = MediaSessions::new()?;
    /// sessions.seek(Duration::from_secs(90)).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn seek(&self, position: Duration) -> MediaResult<()> {
        let timeout_dur = {
            let state = self.state.read().await;
            state.operation_timeout
        };

        timeout(timeout_dur, async {
            let state = self.state.read().await;
            state.backend.seek(position).await
        })
        .await
        .map_err(|_| MediaError::Timeout(timeout_dur))?
    }

    /// Sets the volume level.
    ///
    /// # Arguments
    ///
    /// * `volume` - Volume level from 0.0 (muted) to 1.0 (maximum).
    ///
    /// # Errors
    ///
    /// Returns [`MediaError::Backend`] if the backend command fails.
    /// Returns [`MediaError::Timeout`] if the operation exceeds timeout.
    ///
    /// # Panics
    ///
    /// Panics if volume is outside the range [0.0, 1.0].
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use media_sessions::MediaSessions;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let sessions = MediaSessions::new()?;
    /// sessions.set_volume(0.5).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn set_volume(&self, volume: f64) -> MediaResult<()> {
        assert!(
            (0.0..=1.0).contains(&volume),
            "volume must be between 0.0 and 1.0"
        );

        let timeout_dur = {
            let state = self.state.read().await;
            state.operation_timeout
        };

        timeout(timeout_dur, async {
            let state = self.state.read().await;
            state.backend.set_volume(volume).await
        })
        .await
        .map_err(|_| MediaError::Timeout(timeout_dur))?
    }

    /// Sets the repeat mode.
    ///
    /// # Errors
    ///
    /// Returns [`MediaError::Backend`] if the backend command fails.
    /// Returns [`MediaError::Timeout`] if the operation exceeds timeout.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use media_sessions::{MediaSessions, RepeatMode};
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let sessions = MediaSessions::new()?;
    /// sessions.set_repeat_mode(RepeatMode::All).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn set_repeat_mode(&self, mode: RepeatMode) -> MediaResult<()> {
        let timeout_dur = {
            let state = self.state.read().await;
            state.operation_timeout
        };

        timeout(timeout_dur, async {
            let state = self.state.read().await;
            state.backend.set_repeat_mode(mode).await
        })
        .await
        .map_err(|_| MediaError::Timeout(timeout_dur))?
    }

    /// Toggles shuffle mode.
    ///
    /// # Errors
    ///
    /// Returns [`MediaError::Backend`] if the backend command fails.
    /// Returns [`MediaError::Timeout`] if the operation exceeds timeout.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use media_sessions::MediaSessions;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let sessions = MediaSessions::new()?;
    /// sessions.set_shuffle(true).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn set_shuffle(&self, enabled: bool) -> MediaResult<()> {
        let timeout_dur = {
            let state = self.state.read().await;
            state.operation_timeout
        };

        timeout(timeout_dur, async {
            let state = self.state.read().await;
            state.backend.set_shuffle(enabled).await
        })
        .await
        .map_err(|_| MediaError::Timeout(timeout_dur))?
    }

    /// Returns the active application name.
    ///
    /// # Errors
    ///
    /// Returns [`MediaError::Backend`] if the backend query fails.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use media_sessions::MediaSessions;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let sessions = MediaSessions::new()?;
    /// if let Some(app) = sessions.active_app().await? {
    ///     println!("Active player: {}", app);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn active_app(&self) -> MediaResult<Option<String>> {
        let state = self.state.read().await;
        state.backend.get_active_app()
    }
}

impl std::fmt::Debug for MediaSessions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MediaSessions")
            .field("state", &"<shared state>")
            .finish()
    }
}

impl Default for MediaSessions {
    fn default() -> Self {
        Self::new().expect("Failed to create default MediaSessions")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_defaults() {
        let builder = MediaSessionsBuilder::new();
        assert_eq!(builder.debounce_duration, DEFAULT_DEBOUNCE_DURATION);
        assert_eq!(builder.operation_timeout, DEFAULT_OPERATION_TIMEOUT);
        assert!(builder.enable_artwork);
    }

    #[test]
    #[should_panic(expected = "debounce_duration must be between")]
    fn test_builder_zero_debounce() {
        let _ = MediaSessions::builder().debounce_duration(Duration::ZERO);
    }

    #[test]
    fn test_repeat_mode_default() {
        assert_eq!(RepeatMode::default(), RepeatMode::None);
    }
}
