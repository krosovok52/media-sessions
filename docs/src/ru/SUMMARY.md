# Media Sessions Documentation (Русская версия)

# Введение

- [Главная](README.md)
- [Введение](introduction.md)
- [Установка](installation.md)
- [Quick Start (5 мин)](quickstart.md)

# Rust API

- [Обзор](rust-api/README.md)
- [MediaSessions](rust-api/media-sessions.md)
  - [Создание](rust-api/media-sessions.md#создание-экземпляра)
  - [Методы управления](rust-api/media-sessions.md#методы-управления)
  - [Builder pattern](rust-api/media-sessions.md#builder-паттерн)
  - [Примеры](rust-api/media-sessions.md#примеры-использования)
- [MediaInfo](rust-api/media-info.md)
  - [Поля](rust-api/media-info.md#поля-структуры)
  - [Методы](rust-api/media-info.md#методы)
  - [Примеры](rust-api/media-info.md#примеры-использования)
- [PlaybackStatus](rust-api/playback-status.md)
- [RepeatMode](rust-api/repeat-mode.md)
- [События](rust-api/events.md)
  - [MediaSessionEvent](rust-api/events.md#mediasessionevent)
  - [Поток watch()](rust-api/events.md#получение-потока-событий)
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
  - [Класс-обёртка](languages/python.md#класс-обёртка)
  - [Примеры](languages/python.md#примеры-использования)
- [C# (.NET)](languages/csharp.md)
  - [P/Invoke](languages/csharp.md#класс-обёртка)
  - [Примеры](languages/csharp.md#примеры-использования)
- [C/C++](languages/c-cpp.md)
  - [Пример на C](languages/c-cpp.md#пример-на-c)
  - [Пример на C++](languages/c-cpp.md#пример-на-c)
  - [CMake](languages/c-cpp.md#cmake-проект)
- [Node.js](languages/nodejs.md)
  - [ffi-napi](languages/nodejs.md#класс-обёртка)
  - [Примеры](languages/nodejs.md#примеры-использования)

# Платформы

- [Windows](platforms/windows.md)
  - [SMTC API](platforms/windows.md#обзор)
  - [Поддерживаемые плееры](platforms/windows.md#поддерживаемые-плееры)
  - [Настройка](platforms/windows.md#настройка-плееров)
  - [Ограничения](platforms/windows.md#ограничения-smtc-api)
- [macOS](platforms/macos.md)
  - [MediaRemote](platforms/macos.md#обзор)
  - [Permissions](platforms/macos.md#permissions-разрешения)
  - [Поддерживаемые плееры](platforms/macos.md#поддерживаемые-плееры)
- [Linux](platforms/linux.md)
  - [MPRIS/D-Bus](platforms/linux.md#обзор)
  - [Проверка плееров](platforms/linux.md#проверка-доступных-плееров)
  - [Настройка](platforms/linux.md#настройка-плееров)

# Гайды

- [Обработка ошибок](guides/error-handling.md)
  - [Типы ошибок](guides/error-handling.md#типы-ошибок)
  - [Pattern matching](guides/error-handling.md#pattern-matching)
  - [Логирование](guides/error-handling.md#логирование-ошибок)
- [Производительность](guides/performance.md)
  - [Бенчмарки](guides/performance.md#бенчмарки)
  - [Оптимизация](guides/performance.md#оптимизация-запросов)
  - [Профилирование](guides/performance.md#профилирование)
- [Интеграция](guides/integration.md)
  - [Rust проект](guides/integration.md#rust-проект)
  - [Web-сервер](guides/integration.md#web-сервер)
  - [Desktop](guides/integration.md#desktop-приложение)
- [Troubleshooting](guides/troubleshooting.md)
  - [Ошибки инициализации](guides/troubleshooting.md#ошибки-инициализации)
  - [Windows](guides/troubleshooting.md#windows-проблемы)
  - [macOS](guides/troubleshooting.md#macos-проблемы)
  - [Linux](guides/troubleshooting.md#linux-проблемы)

# Справка

- [FAQ](../faq.md)
- [Changelog](../CHANGELOG.md)
- [Contributing](../CONTRIBUTING.md)
