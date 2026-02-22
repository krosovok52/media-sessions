//! Simple example: get current track and print it.
//!
//! # Running
//!
//! ```bash
//! cargo run --example simple_player
//! ```

use media_sessions::MediaSessions;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sessions = MediaSessions::new()?;

    match sessions.current().await? {
        Some(info) => {
            println!("🎵 Now playing: {}", info.display_string());
            if let Some(album) = &info.album {
                println!("💿 Album: {}", album);
            }
            println!("▶️ Status: {}", info.playback_status);
        }
        None => {
            println!("No active media session found.");
            println!("Start a media player (Spotify, Firefox, etc.)");
        }
    }

    Ok(())
}
