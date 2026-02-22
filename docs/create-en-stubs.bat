@echo off
REM Create EN documentation stubs that redirect to RU version

cd /d "%~dp0docs\src\en"

REM Rust API stubs
echo # MediaSessions^
^
For full documentation, see [Russian version](../../ru/rust-api/media-sessions.md). > rust-api/media-sessions.md

echo # MediaInfo^
^
For full documentation, see [Russian version](../../ru/rust-api/media-info.md). > rust-api/media-info.md

echo # PlaybackStatus^
^
For full documentation, see [Russian version](../../ru/rust-api/playback-status.md). > rust-api/playback-status.md

echo # RepeatMode^
^
For full documentation, see [Russian version](../../ru/rust-api/repeat-mode.md). > rust-api/repeat-mode.md

echo # Events^
^
For full documentation, see [Russian version](../../ru/rust-api/events.md). > rust-api/events.md

REM Languages stubs
echo # C API^
^
For full documentation, see [Russian version](../../ru/c-api.md). > c-api.md

echo # Python^
^
For full documentation, see [Russian version](../../ru/languages/python.md). > languages/python.md

echo # C#^
^
For full documentation, see [Russian version](../../ru/languages/csharp.md). > languages/csharp.md

echo # C/C++^
^
For full documentation, see [Russian version](../../ru/languages/c-cpp.md). > languages/c-cpp.md

echo # Node.js^
^
For full documentation, see [Russian version](../../ru/languages/nodejs.md). > languages/nodejs.md

REM Platforms stubs
echo # Windows^
^
For full documentation, see [Russian version](../../ru/platforms/windows.md). > platforms/windows.md

echo # macOS^
^
For full documentation, see [Russian version](../../ru/platforms/macos.md). > platforms/macos.md

echo # Linux^
^
For full documentation, see [Russian version](../../ru/platforms/linux.md). > platforms/linux.md

REM Guides stubs
echo # Error Handling^
^
For full documentation, see [Russian version](../../ru/guides/error-handling.md). > guides/error-handling.md

echo # Performance^
^
For full documentation, see [Russian version](../../ru/guides/performance.md). > guides/performance.md

echo # Integration^
^
For full documentation, see [Russian version](../../ru/guides/integration.md). > guides/integration.md

echo # Troubleshooting^
^
For full documentation, see [Russian version](../../ru/guides/troubleshooting.md). > guides/troubleshooting.md

echo # FAQ^
^
For full documentation, see [Russian version](../faq.md). > faq.md

echo Done! Created EN stub files.
