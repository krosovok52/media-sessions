//! Event watcher example: subscribe to media session events.
//!
//! # Running
//!
//! ```bash
//! cargo run --example event_watcher
//! ```

use std::time::Duration;

use futures::StreamExt;
use media_sessions::{MediaSessionEvent, MediaSessions};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("📡 Media Session Event Watcher\n");

    let sessions = MediaSessions::builder()
        .debounce_duration(Duration::from_millis(300))
        .build()?;

    println!("✅ Listening for events... (Ctrl+C to stop)\n");

    let mut stream = sessions.watch().await?;

    while let Some(event_result) = stream.next().await {
        match event_result {
            Ok(event) => match event {
                MediaSessionEvent::MetadataChanged(info) => {
                    println!("🎵 Metadata: {} - {}", info.artist(), info.title());
                }
                MediaSessionEvent::PlaybackStatusChanged(status) => {
                    println!("▶️ Status: {}", status);
                }
                MediaSessionEvent::PositionChanged { position, .. } => {
                    println!("⏱️ Position: {}s", position.as_secs());
                }
                MediaSessionEvent::SessionOpened { app_name } => {
                    println!("📻 Session opened: {}", app_name);
                }
                MediaSessionEvent::SessionClosed => {
                    println!("📻 Session closed");
                }
                MediaSessionEvent::VolumeChanged { volume } => {
                    println!("🔊 Volume: {:.0}%", volume * 100.0);
                }
                _ => {}
            },
            Err(e) => {
                eprintln!("❌ Error: {}", e);
            }
        }
    }

    Ok(())
}
