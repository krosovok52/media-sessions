//! Media information types and playback status enumeration.

use std::time::Duration;

/// Playback status of a media session.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PlaybackStatus {
    /// Media is currently playing.
    #[default]
    Playing,
    /// Media is currently paused.
    Paused,
    /// Media is stopped.
    Stopped,
    /// Media is in a transitional state.
    Transitioning,
}

impl PlaybackStatus {
    /// Returns `true` if the status is [`PlaybackStatus::Playing`].
    #[must_use]
    pub const fn is_playing(self) -> bool {
        matches!(self, Self::Playing)
    }

    /// Returns `true` if the status is [`PlaybackStatus::Paused`].
    #[must_use]
    pub const fn is_paused(self) -> bool {
        matches!(self, Self::Paused)
    }

    /// Returns `true` if the status is [`PlaybackStatus::Stopped`].
    #[must_use]
    pub const fn is_stopped(self) -> bool {
        matches!(self, Self::Stopped)
    }

    /// Returns a human-readable string representation.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Playing => "playing",
            Self::Paused => "paused",
            Self::Stopped => "stopped",
            Self::Transitioning => "transitioning",
        }
    }
}

impl std::fmt::Display for PlaybackStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Complete metadata for a media track.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MediaInfo {
    /// Track title.
    pub title: Option<String>,
    /// Track artist or performer.
    pub artist: Option<String>,
    /// Album name.
    pub album: Option<String>,
    /// Total duration of the track.
    pub duration: Option<Duration>,
    /// Current playback position.
    pub position: Option<Duration>,
    /// Current playback status.
    pub playback_status: PlaybackStatus,
    /// Raw artwork image bytes.
    pub artwork: Option<Vec<u8>>,
    /// Track number within the album.
    pub track_number: Option<u32>,
    /// Disc number for multi-disc albums.
    pub disc_number: Option<u32>,
    /// Genre classification.
    pub genre: Option<String>,
    /// Release year.
    pub year: Option<i32>,
    /// Source URL or identifier.
    pub url: Option<String>,
    /// Auto-generated thumbnail URL.
    pub thumbnail_url: Option<String>,
    /// Media type hint.
    pub media_type: Option<MediaType>,
}

/// Type of media content.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum MediaType {
    /// Music track.
    Music,
    /// Video content.
    Video,
    /// Podcast episode.
    Podcast,
    /// Audiobook chapter.
    Audiobook,
    /// Radio stream.
    Radio,
    /// Movie or TV show.
    Movie,
    /// Unknown or unspecified type.
    Unknown,
}

impl MediaInfo {
    /// Returns the track title or an empty string if unavailable.
    #[must_use]
    pub fn title(&self) -> &str {
        self.title.as_deref().unwrap_or("")
    }

    /// Returns the artist name or an empty string if unavailable.
    #[must_use]
    pub fn artist(&self) -> &str {
        self.artist.as_deref().unwrap_or("")
    }

    /// Returns the album name or an empty string if unavailable.
    #[must_use]
    pub fn album(&self) -> &str {
        self.album.as_deref().unwrap_or("")
    }

    /// Returns a formatted display string "Artist - Title".
    #[must_use]
    pub fn display_string(&self) -> String {
        if self.artist().is_empty() {
            self.title().to_string()
        } else if self.title().is_empty() {
            self.artist().to_string()
        } else {
            format!("{} - {}", self.artist(), self.title())
        }
    }

    /// Returns the duration in seconds, or 0 if unavailable.
    #[must_use]
    pub fn duration_secs(&self) -> u64 {
        self.duration.map_or(0, |d| d.as_secs())
    }

    /// Returns the position in seconds, or 0 if unavailable.
    #[must_use]
    pub fn position_secs(&self) -> u64 {
        self.position.map_or(0, |p| p.as_secs())
    }

    /// Returns the progress as a percentage (0.0 to 1.0).
    #[must_use]
    pub fn progress(&self) -> f64 {
        match (self.duration, self.position) {
            (Some(dur), Some(pos)) => {
                let dur_secs = dur.as_secs_f64();
                if dur_secs > 0.0 {
                    pos.as_secs_f64().min(dur_secs) / dur_secs
                } else {
                    0.0
                }
            }
            _ => 0.0,
        }
    }

    /// Returns the progress as a percentage (0 to 100).
    #[must_use]
    pub fn progress_percent(&self) -> f64 {
        self.progress() * 100.0
    }

    /// Returns true if the track is currently playing.
    #[must_use]
    pub const fn is_playing(&self) -> bool {
        self.playback_status.is_playing()
    }

    /// Returns true if the track is currently paused.
    #[must_use]
    pub const fn is_paused(&self) -> bool {
        self.playback_status.is_paused()
    }

    /// Returns the artwork format hint if available.
    #[must_use]
    pub fn artwork_format(&self) -> Option<&'static str> {
        self.artwork
            .as_ref()
            .and_then(|data| match data.as_slice() {
                [0x89, 0x50, 0x4E, 0x47, ..] => Some("PNG"),
                [0xFF, 0xD8, 0xFF, ..] => Some("JPEG"),
                [0x47, 0x49, 0x46, 0x38, ..] => Some("GIF"),
                _ => None,
            })
    }
}

impl std::fmt::Display for MediaInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display_string())?;
        if let Some(album) = &self.album {
            write!(f, " ({album})")?;
        }
        if let Some(year) = self.year {
            write!(f, " [{year}]")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_playback_status_display() {
        assert_eq!(PlaybackStatus::Playing.to_string(), "playing");
        assert_eq!(PlaybackStatus::Paused.to_string(), "paused");
        assert_eq!(PlaybackStatus::Stopped.to_string(), "stopped");
    }

    #[test]
    fn test_media_info_display() {
        let info = MediaInfo {
            title: Some("Title".to_string()),
            artist: Some("Artist".to_string()),
            album: Some("Album".to_string()),
            year: Some(2024),
            ..Default::default()
        };
        assert_eq!(info.to_string(), "Artist - Title (Album) [2024]");
    }

    #[test]
    fn test_progress_calculation() {
        let info = MediaInfo {
            duration: Some(Duration::from_secs(200)),
            position: Some(Duration::from_secs(50)),
            ..Default::default()
        };
        assert!((info.progress() - 0.25).abs() < f64::EPSILON);
        assert!((info.progress_percent() - 25.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_artwork_format_detection() {
        let png_info = MediaInfo {
            artwork: Some(vec![0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A]),
            ..Default::default()
        };
        assert_eq!(png_info.artwork_format(), Some("PNG"));

        let jpeg_info = MediaInfo {
            artwork: Some(vec![0xFF, 0xD8, 0xFF, 0xE0]),
            ..Default::default()
        };
        assert_eq!(jpeg_info.artwork_format(), Some("JPEG"));
    }
}
