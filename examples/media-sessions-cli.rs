//! Command-line interface for media-sessions.
//!
//! # Usage
//!
//! ```bash
//! # Show current track
//! media-sessions-cli current
//!
//! # Control playback
//! media-sessions-cli play
//! media-sessions-cli pause
//! media-sessions-cli play-pause
//! media-sessions-cli stop
//! media-sessions-cli next
//! media-sessions-cli previous
//!
//! # Seek
//! media-sessions-cli seek 30  # Seek to 30 seconds
//!
//! # Volume (Linux only)
//! media-sessions-cli volume 0.5  # Set volume to 50%
//!
//! # Watch events
//! media-sessions-cli watch
//! ```

use std::time::Duration;

use media_sessions::{MediaSessionEvent, MediaSessions, PlaybackStatus};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        print_help();
        return Ok(());
    }

    let sessions = MediaSessions::new()?;
    let command = &args[1];

    match command.as_str() {
        "current" | "info" => {
            cmd_current(&sessions).await?;
        }
        "active" | "app" => {
            cmd_active(&sessions).await?;
        }
        "play" => {
            sessions.play().await?;
            println!("▶️ Playing");
        }
        "pause" => {
            sessions.pause().await?;
            println!("⏸ Paused");
        }
        "play-pause" | "toggle" => {
            sessions.play_pause().await?;
            println!("🔄 Toggled play/pause");
        }
        "stop" => {
            sessions.stop().await?;
            println!("⏹ Stopped");
        }
        "next" => {
            sessions.next().await?;
            println!("⏭ Next track");
        }
        "previous" | "prev" => {
            sessions.previous().await?;
            println!("⏮ Previous track");
        }
        "seek" => {
            if args.len() < 3 {
                eprintln!("Usage: media-sessions-cli seek <seconds>");
                std::process::exit(1);
            }
            let seconds: u64 = args[2].parse()?;
            sessions.seek(Duration::from_secs(seconds)).await?;
            println!("⏩ Seeked to {}s", seconds);
        }
        "volume" => {
            if args.len() < 3 {
                eprintln!("Usage: media-sessions-cli volume <0.0-1.0>");
                std::process::exit(1);
            }
            let volume: f64 = args[2].parse()?;
            if volume < 0.0 || volume > 1.0 {
                eprintln!("Volume must be between 0.0 and 1.0");
                std::process::exit(1);
            }
            sessions.set_volume(volume).await?;
            println!("🔊 Volume set to {:.0}%", volume * 100.0);
        }
        "watch" | "events" => {
            cmd_watch(&sessions)?;
        }
        "help" | "--help" | "-h" => {
            print_help();
        }
        "version" | "--version" | "-v" => {
            println!("media-sessions-cli v{}", env!("CARGO_PKG_VERSION"));
        }
        _ => {
            eprintln!("Unknown command: {}", command);
            print_help();
            std::process::exit(1);
        }
    }

    Ok(())
}

async fn cmd_current(sessions: &MediaSessions) -> Result<(), Box<dyn std::error::Error>> {
    match sessions.current().await? {
        Some(info) => {
            let status_icon = match info.playback_status {
                PlaybackStatus::Playing => "▶️",
                PlaybackStatus::Paused => "⏸",
                PlaybackStatus::Stopped => "⏹",
                PlaybackStatus::Transitioning => "⏳",
            };

            println!("╔═══════════════════════════════════════════════════════════╗");
            println!("║                    Now Playing                            ║");
            println!("╠═══════════════════════════════════════════════════════════╣");

            if info.title().is_empty() && info.artist().is_empty() {
                println!("║  No media information available                           ║");
            } else {
                let display = info.display_string();
                println!("║  {} {:<50} ║", truncate("🎵", 2), truncate(&display, 50));

                if let Some(album) = &info.album {
                    println!("║  💿 {:<51} ║", truncate(album, 51));
                }
            }

            println!("╠═══════════════════════════════════════════════════════════╣");
            println!("║  Status: {} {:<48} ║", status_icon, info.playback_status);

            if let (Some(duration), Some(position)) = (info.duration, info.position) {
                let progress = info.progress();
                let bar_width = 40;
                let filled = (progress * bar_width as f64) as usize;
                let bar: String = "█".repeat(filled) + &"░".repeat(bar_width - filled);

                println!(
                    "║  [{}] {}/{} ({:.1}%) {:>10} ║",
                    bar,
                    format_duration(position),
                    format_duration(duration),
                    info.progress_percent(),
                    ""
                );
            }

            println!("╚═══════════════════════════════════════════════════════════╝");
        }
        None => {
            println!("No active media session found.");
            println!("Please start a media player (Spotify, Firefox, etc.)");
        }
    }

    Ok(())
}

async fn cmd_active(sessions: &MediaSessions) -> Result<(), Box<dyn std::error::Error>> {
    match sessions.active_app().await? {
        Some(app) => {
            println!("📱 Active player: {}", app);
        }
        None => {
            println!("No active media session found.");
        }
    }

    Ok(())
}

fn cmd_watch(sessions: &MediaSessions) -> Result<(), Box<dyn std::error::Error>> {
    use futures::StreamExt;

    println!("📡 Watching for media events... (Press Ctrl+C to stop)");

    let rt = tokio::runtime::Runtime::new()?;
    rt.block_on(async {
        let mut stream = sessions.watch().await?;

        while let Some(event) = stream.next().await {
            match event? {
                MediaSessionEvent::MetadataChanged(info) => {
                    println!("\n🎵 Metadata changed:");
                    if info.title().is_empty() && info.artist().is_empty() {
                        println!("   <no metadata>");
                    } else {
                        println!("   {}", info.display_string());
                    }
                }
                MediaSessionEvent::PlaybackStatusChanged(status) => {
                    println!("\n▶️ Status: {}", status);
                }
                MediaSessionEvent::PositionChanged { position, .. } => {
                    println!("\n⏱ Position: {}", format_duration(position));
                }
                MediaSessionEvent::SessionOpened { app_name } => {
                    println!("\n📻 Session opened: {}", app_name);
                }
                MediaSessionEvent::SessionClosed => {
                    println!("\n📻 Session closed");
                }
                _ => {}
            }
        }

        Ok::<_, Box<dyn std::error::Error>>(())
    })?;

    Ok(())
}

fn print_help() {
    println!(
        r#"
🎵 media-sessions-cli v{}

Usage: media-sessions-cli <command> [arguments]

Commands:
  current, info          Show current track information
  active, app            Show active media player application
  play                   Start or resume playback
  pause                  Pause playback
  play-pause, toggle     Toggle play/pause
  stop                   Stop playback
  next                   Skip to next track
  previous, prev         Skip to previous track
  seek <seconds>         Seek to position in seconds
  volume <0.0-1.0>       Set volume level (Linux only)
  watch, events          Watch for media events
  help, --help, -h       Show this help message
  version, -v            Show version

Examples:
  media-sessions-cli current
  media-sessions-cli play
  media-sessions-cli seek 30
  media-sessions-cli volume 0.5
  media-sessions-cli watch

Platform: {}
"#,
        env!("CARGO_PKG_VERSION"),
        media_sessions::current_platform()
    );
}

fn truncate(s: &str, max_len: usize) -> &str {
    if s.len() <= max_len {
        s
    } else {
        &s[..max_len.saturating_sub(3).min(s.len())]
    }
}

fn format_duration(d: Duration) -> String {
    let secs = d.as_secs();
    let hours = secs / 3600;
    let mins = (secs % 3600) / 60;
    let secs = secs % 60;

    if hours > 0 {
        format!("{}:{:02}:{:02}", hours, mins, secs)
    } else {
        format!("{}:{:02}", mins, secs)
    }
}
