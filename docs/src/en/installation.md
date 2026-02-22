# Installation

## Requirements

| Requirement | Version | Description |
|-------------|---------|-------------|
| **Rust** | 1.80+ | Minimum Supported Rust Version (MSRV) |
| **Tokio** | 1.0+ | Async runtime (required for API) |
| **CMake** | 3.1+ | Windows only (build dependency) |

## Installation from crates.io

Add to `Cargo.toml`:

```toml
[dependencies]
media-sessions = "0.2"
tokio = { version = "1", features = ["full"] }
futures = "0.3"
```

Or use cargo add:

```bash
cargo add media-sessions tokio futures
```

## Feature Flags

| Feature | Description | Dependencies |
|---------|-------------|--------------|
| `default` | All platforms | — |
| `windows` | Windows only | windows, windows-core |
| `macos` | macOS only | objc2, objc2-foundation |
| `linux` | Linux only | zbus |
| `tracing` | Tracing logs | tracing |
| `serde` | Serialization | serde |
| `c-api` | C FFI for other languages | — |

## Platform-specific Dependencies

### Windows

CMake required:

```bash
winget install Kitware.CMake
```

### Linux

D-Bus development files required:

```bash
# Debian/Ubuntu
sudo apt install libdbus-1-dev

# Fedora
sudo dnf install dbus-devel

# Arch
sudo pacman -S dbus
```

## Verify Installation

```bash
cargo new test_media
cd test_media
cargo add media-sessions tokio
cargo run
```

## Next Steps

- **[Quick Start](quickstart.md)** — First run in 5 minutes
- **[Introduction](introduction.md)** — What is Media Sessions
- **[Rust API](rust-api/README.md)** — API documentation
