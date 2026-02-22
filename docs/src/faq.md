# FAQ (Frequently Asked Questions)

## Common Questions

### Q: Why isn't my media player detected on Linux?

**A:** Make sure you have an MPRIS-compatible player running (Spotify, Firefox, mpv with `--input-mpremote-command`, VLC). Check D-Bus:

```bash
dbus-send --session --dest=org.freedesktop.DBus \
  --type=method_call --print-reply \
  /org/freedesktop/Bus org.freedesktop.DBus.ListNames | grep mpris
```

### Q: Why does macOS require Accessibility permissions?

**A:** MediaRemote is a private framework. Some features may be limited without Accessibility access. Grant access in System Preferences → Privacy & Security → Accessibility.

### Q: Can I control multiple players simultaneously?

**A:** Currently the library focuses on the active session. Multi-player support is planned for v0.3.

### Q: How do I get album artwork?

**A:** The `artwork` field in `MediaInfo` contains raw PNG/JPEG bytes when available. Check `artwork_format()` to determine the format.

```rust
if let Some(artwork) = &info.artwork {
    std::fs::write("cover.jpg", artwork)?;
}
```

### Q: Is the library compatible with wasm?

**A:** No, media session control requires native OS APIs. Consider using feature flags to disable on wasm targets.

### Q: Why is my volume control not working?

**A:** Volume control is only supported on Linux through MPRIS. Windows SMTC and macOS MediaRemote do not expose volume control APIs.

### Q: How do I handle errors?

**A:** Use pattern matching on `MediaError`:

```rust
use media_sessions::{MediaSessions, MediaError};

match sessions.current().await {
    Ok(Some(info)) => println!("Track: {}", info.display_string()),
    Ok(None) => println!("No active session"),
    Err(MediaError::NotSupported(platform)) => eprintln!("Not supported on {}", platform),
    Err(MediaError::NoSession) => eprintln!("No session"),
    Err(e) => eprintln!("Error: {}", e),
}
```

### Q: What is the minimum supported Rust version (MSRV)?

**A:** MSRV is 1.80+. The library uses features from Rust 1.80.

### Q: How do I enable logging?

**A:** Add the `tracing` feature and use `tracing-subscriber`:

```toml
[dependencies]
media-sessions = { version = "0.2", features = ["tracing"] }
tracing-subscriber = "0.3"
```

```rust
use tracing_subscriber;

tracing_subscriber::fmt()
    .with_max_level(tracing::Level::DEBUG)
    .init();
```

---

## Troubleshooting

For more detailed troubleshooting, see the [Troubleshooting Guide](guides/troubleshooting.md).

---

## Getting Help

- **GitHub Issues:** https://github.com/krosovok52/media-sessions/issues
- **Telegram:** [@krosov_ok](https://t.me/krosov_ok)
- **Documentation:** https://docs.rs/media-sessions
