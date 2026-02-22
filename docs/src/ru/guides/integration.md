# Интеграция в проект

Руководство по интеграции Media Sessions в различные проекты.

## Rust проект

### Добавление зависимости

```toml
# Cargo.toml
[dependencies]
media-sessions = "0.2"
tokio = { version = "1", features = ["full"] }
futures = "0.3"
```

### Базовая структура

```rust
// src/media.rs
use media_sessions::{MediaSessions, MediaInfo, MediaSessionEvent};
use futures::StreamExt;

pub struct MediaController {
    sessions: MediaSessions,
}

impl MediaController {
    pub fn new() -> Result<Self, media_sessions::MediaError> {
        let sessions = MediaSessions::new()?;
        Ok(Self { sessions })
    }
    
    pub async fn get_current(&self) -> Result<Option<MediaInfo>, media_sessions::MediaError> {
        self.sessions.current().await
    }
    
    pub async fn play(&self) -> Result<(), media_sessions::MediaError> {
        self.sessions.play().await
    }
    
    pub async fn watch(&self) -> Result<impl futures::Stream<Item = MediaSessionEvent>, media_sessions::MediaError> {
        self.sessions.watch().await
    }
}
```

### Интеграция с существующим кодом

```rust
// src/main.rs
mod media;

use media::MediaController;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let controller = MediaController::new()?;
    
    if let Some(info) = controller.get_current().await? {
        println!("🎵 {}", info.display_string());
    }
    
    Ok(())
}
```

## Web-сервер

### Axum

```rust
use axum::{
    Router,
    routing::get,
    extract::State,
    Json,
};
use media_sessions::{MediaSessions, MediaInfo};
use std::sync::Arc;
use tokio::sync::RwLock;

struct AppState {
    media: MediaSessions,
    current_track: RwLock<Option<MediaInfo>>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let media = MediaSessions::new()?;
    
    let state = Arc::new(AppState {
        media,
        current_track: RwLock::new(None),
    });
    
    // Фоновая задача для обновления
    let state_clone = state.clone();
    tokio::spawn(async move {
        let mut stream = state_clone.media.watch().await.unwrap();
        while let Some(event) = stream.next().await {
            if let Ok(MediaSessionEvent::MetadataChanged(info)) = event {
                *state_clone.current_track.write().await = Some(info);
            }
        }
    });
    
    let app = Router::new()
        .route("/media/status", get(get_status))
        .route("/media/play", get(play))
        .route("/media/pause", get(pause))
        .route("/media/next", get(next))
        .with_state(state);
    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;
    
    Ok(())
}

async fn get_status(
    State(state): State<Arc<AppState>>,
) -> Json<Option<MediaInfo>> {
    Json(state.current_track.read().await.clone())
}

async fn play(
    State(state): State<Arc<AppState>>,
) -> Result<(), (http::StatusCode, String)> {
    state.media.play().await
        .map_err(|e| (http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}

async fn pause(
    State(state): State<Arc<AppState>>,
) -> Result<(), (http::StatusCode, String)> {
    state.media.pause().await
        .map_err(|e| (http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}

async fn next(
    State(state): State<Arc<AppState>>,
) -> Result<(), (http::StatusCode, String)> {
    state.media.next().await
        .map_err(|e| (http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}
```

### Actix-web

```rust
use actix_web::{web, App, HttpServer, get, HttpResponse};
use media_sessions::MediaSessions;
use std::sync::Arc;

struct AppState {
    media: MediaSessions,
}

#[get("/media/status")]
async fn get_status(data: web::Data<Arc<AppState>>) -> HttpResponse {
    match data.media.current().await {
        Ok(Some(info)) => HttpResponse::Ok().json(info),
        Ok(None) => HttpResponse::NotFound().finish(),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[get("/media/play")]
async fn play(data: web::Data<Arc<AppState>>) -> HttpResponse {
    match data.media.play().await {
        Ok(()) => HttpResponse::Ok().finish(),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let media = MediaSessions::new().unwrap();
    let state = Arc::new(AppState { media });
    
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            .route("/media/status", web::get().to(get_status))
            .route("/media/play", web::get().to(play))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
```

## Desktop приложение

### Tauri

```rust
// src-tauri/src/main.rs
use media_sessions::MediaSessions;

#[tauri::command]
fn get_current_track(sessions: tauri::State<MediaSessions>) -> Option<MediaInfo> {
    futures::executor::block_on(sessions.current()).ok().flatten()
}

#[tauri::command]
fn play(sessions: tauri::State<MediaSessions>) -> bool {
    futures::executor::block_on(sessions.play()).is_ok()
}

fn main() {
    let sessions = MediaSessions::new().unwrap();
    
    tauri::Builder::default()
        .manage(sessions)
        .invoke_handler(tauri::generate_handler![
            get_current_track,
            play,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

```javascript
// frontend/src/App.jsx
import { invoke } from '@tauri-apps/api/tauri'

function App() {
  const [track, setTrack] = useState(null)
  
  useEffect(() => {
    const fetchTrack = async () => {
      const info = await invoke('get_current_track')
      setTrack(info)
    }
    
    fetchTrack()
    const interval = setInterval(fetchTrack, 1000)
    return () => clearInterval(interval)
  }, [])
  
  const handlePlay = async () => {
    await invoke('play')
  }
  
  return (
    <div>
      {track && (
        <div>
          <h2>{track.title}</h2>
          <p>{track.artist}</p>
          <button onClick={handlePlay}>▶️ Play</button>
        </div>
      )}
    </div>
  )
}
```

### Iced

```rust
use iced::{button, Button, Column, Command, Element, Text, Update};
use media_sessions::{MediaSessions, MediaInfo};

enum Message {
    TrackUpdated(Option<MediaInfo>),
    PlayPressed,
    PausePressed,
    NextPressed,
}

struct App {
    sessions: MediaSessions,
    current_track: Option<MediaInfo>,
    play_button: button::State,
    pause_button: button::State,
    next_button: button::State,
}

impl Update for App {
    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::TrackUpdated(info) => {
                self.current_track = info;
                Command::none()
            }
            Message::PlayPressed => {
                let _ = self.sessions.play();
                Command::none()
            }
            Message::PausePressed => {
                let _ = self.sessions.pause();
                Command::none()
            }
            Message::NextPressed => {
                let _ = self.sessions.next();
                Command::none()
            }
        }
    }
}

impl Application for App {
    fn view(&mut self) -> Element<Message> {
        let mut column = Column::new();
        
        if let Some(info) = &self.current_track {
            column = column
                .push(Text::new(info.title()))
                .push(Text::new(info.artist()));
        }
        
        column = column
            .push(Button::new(&mut self.play_button, Text::new("▶️"))
                .on_press(Message::PlayPressed))
            .push(Button::new(&mut self.pause_button, Text::new("⏸️"))
                .on_press(Message::PausePressed))
            .push(Button::new(&mut self.next_button, Text::new("⏭️"))
                .on_press(Message::NextPressed));
        
        column.into()
    }
}
```

## Discord Rich Presence

```rust
use discord_rich_presence::{DiscordIpc, DiscordIpcClient};
use media_sessions::{MediaSessions, MediaSessionEvent};
use futures::StreamExt;

struct DiscordPresence {
    client: DiscordIpcClient,
}

impl DiscordPresence {
    fn new(client_id: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let mut client = DiscordIpcClient::new(client_id)?;
        client.connect()?;
        Ok(Self { client })
    }
    
    fn update(&mut self, info: &MediaInfo) -> Result<(), Box<dyn std::error::Error>> {
        use discord_rich_presence::activity::Activity;
        
        let activity = Activity::new()
            .state(info.title())
            .details(info.artist())
            .large_image("cover")
            .large_text(info.album());
        
        self.client.set_activity(activity)?;
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sessions = MediaSessions::new()?;
    let mut presence = DiscordPresence::new("YOUR_CLIENT_ID")?;
    
    let mut stream = sessions.watch().await?;
    
    while let Some(event) = stream.next().await {
        if let MediaSessionEvent::MetadataChanged(info) = event? {
            let _ = presence.update(&info);
        }
    }
    
    Ok(())
}
```

## Telegram бот

```rust
use teloxide::{prelude::*, Bot};
use media_sessions::MediaSessions;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting media bot...");
    
    let bot = Bot::from_env();
    let sessions = MediaSessions::new().unwrap();
    
    let handler = Update::filter_message()
        .branch(dptree::case["/current"].endpoint(current_handler))
        .branch(dptree::case["/play"].endpoint(play_handler))
        .branch(dptree::case["/pause"].endpoint(pause_handler));
    
    Dispatcher::builder(bot, handler)
        .dependencies(dptree::deps![sessions])
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}

async fn current_handler(
    bot: Bot,
    msg: Message,
    sessions: MediaSessions,
) -> Result<(), teloxide::RequestError> {
    match sessions.current().await {
        Ok(Some(info)) => {
            bot.send_message(
                msg.chat.id,
                format!("🎵 {} - {}", info.artist(), info.title()),
            ).await?;
        }
        Ok(None) => {
            bot.send_message(msg.chat.id, "ℹ️ No active session").await?;
        }
        Err(e) => {
            bot.send_message(msg.chat.id, format!("❌ Error: {}", e)).await?;
        }
    }
    Ok(())
}

async fn play_handler(
    bot: Bot,
    msg: Message,
    sessions: MediaSessions,
) -> Result<(), teloxide::RequestError> {
    let _ = sessions.play().await;
    bot.send_message(msg.chat.id, "▶️ Playing").await?;
    Ok(())
}

async fn pause_handler(
    bot: Bot,
    msg: Message,
    sessions: MediaSessions,
) -> Result<(), teloxide::RequestError> {
    let _ = sessions.pause().await;
    bot.send_message(msg.chat.id, "⏸️ Paused").await?;
    Ok(())
}
```

## CLI утилита

```rust
use clap::{Parser, Subcommand};
use media_sessions::{MediaSessions, PlaybackStatus};

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Show current track
    Current,
    /// Play
    Play,
    /// Pause
    Pause,
    /// Toggle play/pause
    Toggle,
    /// Next track
    Next,
    /// Previous track
    Prev,
    /// Seek to position
    Seek {
        #[arg(value_name = "SECONDS")]
        seconds: u64,
    },
    /// Set volume
    Volume {
        #[arg(value_name = "LEVEL")]
        level: f64,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let sessions = MediaSessions::new()?;
    
    match cli.command {
        Commands::Current => {
            if let Some(info) = sessions.current().await? {
                let icon = match info.playback_status {
                    PlaybackStatus::Playing => "▶️",
                    PlaybackStatus::Paused => "⏸️",
                    PlaybackStatus::Stopped => "⏹️",
                    _ => "⏳",
                };
                
                println!("{} {}", icon, info.display_string());
                
                if let Some(album) = &info.album {
                    println!("💿 {}", album);
                }
                
                println!("⏱ {}/{} ({:.1}%)", 
                    info.position_secs(),
                    info.duration_secs(),
                    info.progress_percent()
                );
            } else {
                println!("ℹ️ No active session");
            }
        }
        Commands::Play => {
            sessions.play().await?;
            println!("▶️ Playing");
        }
        Commands::Pause => {
            sessions.pause().await?;
            println!("⏸️ Paused");
        }
        Commands::Toggle => {
            sessions.play_pause().await?;
        }
        Commands::Next => {
            sessions.next().await?;
            println!("⏭️ Next track");
        }
        Commands::Prev => {
            sessions.previous().await?;
            println!("⏮️ Previous track");
        }
        Commands::Seek { seconds } => {
            sessions.seek(std::time::Duration::from_secs(seconds)).await?;
            println!("⏱ Seeked to {}s", seconds);
        }
        Commands::Volume { level } => {
            sessions.set_volume(level).await?;
            println!("🔊 Volume set to {:.0}%", level * 100.0);
        }
    }
    
    Ok(())
}
```

## См. также

- **[Примеры на GitHub](https://github.com/krosovok52/media-sessions/tree/main/examples)** — Больше примеров
- **[C API](c-api.md)** — Интеграция с другими языками
- **[Обработка ошибок](error-handling.md)** — Error handling
