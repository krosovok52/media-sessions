//! Media controls example: demonstrates all playback control functions.
//!
//! # Running
//!
//! ```bash
//! cargo run --example media_controls
//! ```

use std::time::Duration;

use media_sessions::{MediaSessions, RepeatMode};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🎛️ Media Controls Demo\n");

    let sessions = MediaSessions::new()?;

    // Check if there's an active session
    match sessions.current().await? {
        Some(info) => {
            println!("📻 Active session: {} - {}", info.artist(), info.title());
        }
        None => {
            println!("⚠️ No active media session found.");
            println!("Some controls may not work without an active player.");
        }
    }

    println!("\n🎮 Running control demo...\n");

    // Play/Pause toggle
    println!("⏯️ Toggling play/pause...");
    sessions.play_pause().await?;
    tokio::time::sleep(Duration::from_millis(500)).await;

    // Play
    println!("▶️ Playing...");
    sessions.play().await?;
    tokio::time::sleep(Duration::from_millis(500)).await;

    // Pause
    println!("⏸️ Pausing...");
    sessions.pause().await?;
    tokio::time::sleep(Duration::from_millis(500)).await;

    // Next track
    println!("⏭️ Skipping to next track...");
    let _ = sessions.next().await;
    tokio::time::sleep(Duration::from_millis(500)).await;

    // Previous track
    println!("⏮️ Skipping to previous track...");
    let _ = sessions.previous().await;
    tokio::time::sleep(Duration::from_millis(500)).await;

    // Seek forward
    println!("⏩ Seeking forward 10 seconds...");
    if let Ok(Some(info)) = sessions.current().await {
        if let Some(pos) = info.position {
            let new_pos = pos + Duration::from_secs(10);
            let _ = sessions.seek(new_pos).await;
        }
    }
    tokio::time::sleep(Duration::from_millis(500)).await;

    // Seek backward
    println!("⏪ Seeking backward 10 seconds...");
    if let Ok(Some(info)) = sessions.current().await {
        if let Some(pos) = info.position {
            let new_pos = pos.saturating_sub(Duration::from_secs(10));
            let _ = sessions.seek(new_pos).await;
        }
    }
    tokio::time::sleep(Duration::from_millis(500)).await;

    // Volume control (Linux only)
    #[cfg(target_os = "linux")]
    {
        println!("🔊 Setting volume to 50%...");
        let _ = sessions.set_volume(0.5).await;
        tokio::time::sleep(Duration::from_millis(500)).await;
    }

    // Repeat mode (Linux only)
    #[cfg(target_os = "linux")]
    {
        println!("🔁 Setting repeat mode to All...");
        let _ = sessions.set_repeat_mode(RepeatMode::All).await;
        tokio::time::sleep(Duration::from_millis(500)).await;
    }

    // Shuffle (Linux only)
    #[cfg(target_os = "linux")]
    {
        println!("🔀 Enabling shuffle...");
        let _ = sessions.set_shuffle(true).await;
        tokio::time::sleep(Duration::from_millis(500)).await;
    }

    println!("\n✅ Demo complete!");

    Ok(())
}
