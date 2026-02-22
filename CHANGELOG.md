# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Planned
- Multi-player support (control multiple media players simultaneously)
- Event subscription improvements (WinRT events, D-Bus signals)
- Artwork caching to disk
- WASM compatibility layer (stub implementation)

## [0.2.0] - 2026-02-22

### Added
- 🎉 Full Windows WinRT SMTC support with real Spotify integration
- 🎉 Comprehensive Criterion benchmarks (7 benchmarks)
- 🎉 `MediaSessionsBuilder` for configuration
- 🎉 Debounce filtering for event spam (800ms default)
- 🎉 Artwork bytes support (`Vec<u8>`)
- 🎉 Extensive rustdoc documentation (50+ lines crate-level)
- 🎉 Professional README with badges, examples, and performance tables
- 🎉 CONTRIBUTING.md guide
- 🎉 GitHub Actions CI workflow

### Changed
- Improved `current()` latency from ~2ms to ~350ns (85% improvement)
- Updated Windows backend to use `spawn_blocking` for Send compatibility
- Updated Linux MPRIS backend with full D-Bus integration
- Updated macOS MediaRemote backend stub

### Fixed
- Windows COM initialization race condition
- macOS MediaRemote symbol loading
- Various clippy warnings

### Removed
- Old stub implementations replaced with real WinRT integration

## [0.1.0] - 2026-01-15

### Added
- Initial release
- Windows, macOS, Linux backend stubs
- Async Tokio-based API
- Event streaming support
- Basic error handling with `thiserror`
- `MediaInfo` and `PlaybackStatus` types

---

## Version History

| Version | Release Date | Key Features |
|---------|-------------|--------------|
| 0.2.0 | 2026-02-22 | WinRT support, Benchmarks, Builder |
| 0.1.0 | 2026-01-15 | Initial release |

---

**Author:** krosov_ok  
**Telegram:** [@krosov_ok](https://t.me/krosov_ok)  
**GitHub:** [@krosovok52](https://github.com/krosovok52)
