//! Basic usage example for media-sessions crate.
//!
//! This example demonstrates:
//! 1. Creating a MediaSessions instance
//! 2. Querying current media information
//! 3. Controlling playback (play/pause/next/previous)
//! 4. Subscribing to media events
//!
//! # Running the Example
//!
//! ```bash
//! cargo run --example basic_usage
//! ```

use std::time::Duration;

use futures::StreamExt;
use media_sessions::{MediaInfo, MediaSessions, MediaSessionEvent, PlaybackStatus};

/// Prints media info in a formatted way.
fn print_media_info(info: &MediaInfo) {
    println!("╔═══════════════════════════════════════════════════════════╗");
    println!("║                    Now Playing                            ║");
    println!("╠═══════════════════════════════════════════════════════════╣");

    if info.title().is_empty() && info.artist().is_empty() {
        println!("║  No media information available                           ║");
    } else {
        println!("║  Title:  {:<52} ║", truncate(&info.display_string(), 52));

        if let Some(album) = &info.album {
            println!("║  Album:  {:<52} ║", truncate(album, 52));
        }
    }

    println!("╠═══════════════════════════════════════════════════════════╣");

    let status_icon = match info.playback_status {
        PlaybackStatus::Playing => "▶️",
        PlaybackStatus::Paused => "⏸️",
        PlaybackStatus::Stopped => "⏹️",
        PlaybackStatus::Transitioning => "⏳",
    };
    println!("║  Status: {} {:<48} ║", status_icon, info.playback_status);

    if let (Some(duration), Some(position)) = (info.duration, info.position) {
        let progress = info.progress();
        let bar_width = 40;
        let filled = (progress * bar_width as f64) as usize;
        let bar: String = std::iter::repeat('█')
            .take(filled)
            .chain(std::iter::repeat('░').take(bar_width - filled))
            .collect();

        println!(
            "║  [{}] {}/{} ({:.1}%) {:>14} ║",
            bar,
            format_duration(position),
            format_duration(duration),
            info.progress_percent(),
            ""
        );
    }

    println!("╚═══════════════════════════════════════════════════════════╝");
}

/// Truncates a string to fit within the specified width.
fn truncate(s: &str, width: usize) -> String {
    if s.len() <= width {
        s.to_string()
    } else {
        format!("{}...", &s[..width.saturating_sub(3)])
    }
}

/// Formats a duration as MM:SS or HH:MM:SS.
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

/// Event watcher that prints events as they arrive.
async fn watch_events(sessions: &MediaSessions) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n📡 Watching for media events... (Press Ctrl+C to stop)");

    let mut stream = sessions.watch().await?;

    while let Some(event_result) = stream.next().await {
        match event_result {
            Ok(event) => match event {
                MediaSessionEvent::MetadataChanged(info) => {
                    println!("\n🎵 Metadata changed:");
                    print_media_info(&info);
                }
                MediaSessionEvent::PlaybackStatusChanged(status) => {
                    println!("\n▶️ Playback status: {}", status);
                }
                MediaSessionEvent::PositionChanged { position, .. } => {
                    println!("\n⏱️ Position: {}", format_duration(position));
                }
                MediaSessionEvent::SessionOpened { app_name } => {
                    println!("\n📻 New session opened: {}", app_name);
                }
                MediaSessionEvent::SessionClosed => {
                    println!("\n📻 Session closed");
                }
                _ => {}
            },
            Err(e) => {
                eprintln!("❌ Event error: {}", e);
            }
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🎵 media-sessions v0.2.0");
    println!("   Cross-platform media control for Rust\n");

    // Create media sessions with custom configuration
    let sessions = MediaSessions::builder()
        .debounce_duration(Duration::from_millis(500))
        .operation_timeout(Duration::from_secs(3))
        .enable_artwork(true)
        .build()?;

    println!("✅ Media sessions initialized");
    println!("   Platform: {}", std::env::consts::OS);

    // Show current media info
    println!("\n📻 Querying current media session...");

    match sessions.current().await? {
        Some(info) => {
            print_media_info(&info);

            // Demo controls
            println!("\n🎛️ Demo: Toggling play/pause...");
            sessions.play_pause().await?;
            tokio::time::sleep(Duration::from_millis(500)).await;
            sessions.play_pause().await?;

            println!("\n✅ Demo complete!");
        }
        None => {
            println!("   No active media session found.");
            println!("   Please start a media player (Spotify, Firefox, etc.)");
        }
    }

    // Ask user what to do
    println!("\n📋 What would you like to do?");
    println!("   1. Watch events");
    println!("   2. Exit");
    println!();

    print!("> ");
    use std::io::Write;
    std::io::stdout().flush()?;

    let mut choice = String::new();
    std::io::stdin().read_line(&mut choice)?;

    match choice.trim() {
        "1" => watch_events(&sessions).await?,
        _ => {
            println!("Goodbye! 👋");
        }
    }

    Ok(())
}
