# Media Sessions Documentation (Русская версия)

# Введение

- [Главная](README.md)
- [Quick Start (5 мин)](quickstart.md)
- [Что такое Media Sessions?](introduction.md)
- [Установка](installation.md)

# Rust API

- [Обзор](rust-api/README.md)
- [MediaSessions](rust-api/media-sessions.md)
  - [Создание](rust-api/media-sessions.md#создание)
  - [Методы управления](rust-api/media-sessions.md#методы-управления)
  - [Builder pattern](rust-api/media-sessions.md#builder-pattern)
- [MediaInfo](rust-api/media-info.md)
  - [Поля структуры](rust-api/media-info.md#поля)
  - [Методы](rust-api/media-info.md#методы)
  - [Примеры использования](rust-api/media-info.md#примеры)
- [PlaybackStatus](rust-api/playback-status.md)
- [RepeatMode](rust-api/repeat-mode.md)
- [События](rust-api/events.md)
  - [MediaSessionEvent](rust-api/events.md#mediasessionevent)
  - [Поток watch()](rust-api/events.md#поток-watch)
  - [Debounce](rust-api/events.md#debounce)

# C API

- [C API Reference](c-api.md)
  - [Сборка](c-api.md#сборка)
  - [Функции](c-api.md#функции)
  - [Типы данных](c-api.md#типы-данных)
  - [Управление памятью](c-api.md#управление-памятью)

# Языки

- [Python](languages/python.md)
  - [Установка](languages/python.md#установка)
  - [Пример использования](languages/python.md#пример)
  - [Обработка ошибок](languages/python.md#ошибки)
- [C# (.NET)](languages/csharp.md)
  - [P/Invoke](languages/csharp.md#p-invoke)
  - [Класс-обёртка](languages/csharp.md#обёртка)
- [C/C++](languages/c-cpp.md)
  - [Компиляция](languages/c-cpp.md#компиляция)
  - [Примеры](languages/c-cpp.md#примеры)
- [Node.js](languages/nodejs.md)
  - [ffi-napi](languages/nodejs.md#ffi-napi)

# Платформы

- [Windows](platforms/windows.md)
  - [SMTC API](platforms/windows.md#smtc)
  - [Поддерживаемые плееры](platforms/windows.md#плееры)
  - [Ограничения](platforms/windows.md#ограничения)
- [macOS](platforms/macos.md)
  - [MediaRemote](platforms/macos.md#mediaremote)
  - [Permissions](platforms/macos.md#permissions)
- [Linux](platforms/linux.md)
  - [MPRIS](platforms/linux.md#mpris)
  - [D-Bus](platforms/linux.md#d-bus)

# Гайды

- [Обработка ошибок](guides/error-handling.md)
- [Производительность](guides/performance.md)
- [Интеграция в проект](guides/integration.md)
- [Тестирование](guides/testing.md)
- [Отладка](guides/debugging.md)

# Справка

- [FAQ](faq.md)
- [Troubleshooting](troubleshooting.md)
- [Changelog](../CHANGELOG.md)
