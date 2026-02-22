# 📚 Media Sessions Documentation Index

Добро пожаловать в документацию **Media Sessions** — cross-platform библиотеки для управления медиа-сессиями.

---

## 🗂️ Навигация по документации

### Для начинающих

| Документ | Описание | Время чтения |
|----------|----------|--------------|
| [QUICKSTART.md](QUICKSTART.md) | 5-минутное руководство для начала работы | 5 мин |
| [README.md](README.md) | Обзор проекта, возможности, установка | 10 мин |
| [DOCUMENTATION.md](DOCUMENTATION.md) | Полная документация по всем API | 30 мин |

### Для разработчиков

| Документ | Описание | Языки |
|----------|----------|-------|
| [Rust API](DOCUMENTATION.md#rust-api) | Rust API reference и примеры | Rust |
| [C API](c-api/API_REFERENCE.md) | C FFI для других языков | C |
| [Python](c-api/API_REFERENCE.md#python) | Python ctypes binding | Python |
| [C#](c-api/API_REFERENCE.md#c-net) | .NET P/Invoke binding | C# |
| [C/C++](c-api/API_REFERENCE.md#cc) | Нативный C API | C/C++ |
| [Node.js](DOCUMENTATION.md#nodejs) | Node.js ffi-napi binding | JavaScript |

### Примеры кода

| Путь | Описание |
|------|----------|
| [`examples/`](examples/) | Rust примеры использования |
| [`c-api/python_example.py`](c-api/python_example.py) | Python пример |
| [`c-api/csharp_example.cs`](c-api/csharp_example.cs) | C# пример |
| [`examples/media-sessions-cli.rs`](examples/media-sessions-cli.rs) | CLI утилита на Rust |

### Технические документы

| Документ | Описание |
|----------|----------|
| [CONTRIBUTING.md](CONTRIBUTING.md) | Руководство для контрибьюторов |
| [CHANGELOG.md](CHANGELOG.md) | История изменений |
| [SECURITY.md](SECURITY.md) | Политика безопасности |
| [PUBLISHING_GUIDE.md](PUBLISHING_GUIDE.md) | Руководство по публикации |

---

## 🎯 Быстрый выбор

### Я хочу...

| Цель | Документ |
|------|----------|
| **Начать использовать** | [QUICKSTART.md](QUICKSTART.md) |
| **Узнать все возможности** | [DOCUMENTATION.md](DOCUMENTATION.md) |
| **Интегрировать с Python** | [c-api/API_REFERENCE.md#python](c-api/API_REFERENCE.md#python) |
| **Интегрировать с C#** | [c-api/API_REFERENCE.md#c-net](c-api/API_REFERENCE.md#c-net) |
| **Понять архитектуру** | [DOCUMENTATION.md#архитектура](DOCUMENTATION.md#архитектура) |
| **Обработать ошибки** | [DOCUMENTATION.md#обработка-ошибок](DOCUMENTATION.md#обработка-ошибок) |
| **Оптимизировать производительность** | [DOCUMENTATION.md#производительность](DOCUMENTATION.md#производительность) |
| **Решить проблему** | [DOCUMENTATION.md#faq](DOCUMENTATION.md#faq) |

---

## 📋 Содержание основной документации

### 1. [Быстрый старт](DOCUMENTATION.md#быстрый-старт)
- Rust (3 минуты)
- Python (2 минуты)
- C# (3 минуты)
- C++ (5 минут)
- Node.js (5 минут)

### 2. [Установка](DOCUMENTATION.md#установка)
- Из crates.io
- Из Git
- Feature flags

### 3. [Архитектура](DOCUMENTATION.md#архитектура)
- Компоненты библиотеки
- Схема взаимодействия
- Платформенные бэкенды

### 4. [Rust API](DOCUMENTATION.md#rust-api)
- `MediaSessions` — главный класс
- `MediaInfo` — метаданные
- `PlaybackStatus` — статусы
- `MediaSessionEvent` — события
- Примеры использования

### 5. [C API](DOCUMENTATION.md#c-api)
- Функции
- Типы данных
- Управление памятью

### 6. [Языковые binding](DOCUMENTATION.md#python)
- Python
- C#
- C/C++
- Node.js

### 7. [Платформенные особенности](DOCUMENTATION.md#платформенные-особенности)
- Windows (SMTC)
- macOS (MediaRemote)
- Linux (MPRIS)

### 8. [Обработка ошибок](DOCUMENTATION.md#обработка-ошибок)
- Типы ошибок
- Примеры обработки

### 9. [Производительность](DOCUMENTATION.md#производительность)
- Бенчмарки
- Сравнение с аналогами
- Оптимизация

### 10. [FAQ](DOCUMENTATION.md#faq)
- Частые вопросы
- Решение проблем

---

## 🔗 Внешние ресурсы

- **Crates.io:** https://crates.io/crates/media-sessions
- **Docs.rs:** https://docs.rs/media-sessions
- **GitHub:** https://github.com/krosovok52/media-sessions
- **Telegram:** https://t.me/programsKrosovok

---

## 📬 Контакты

- **Автор:** krosov_ok
- **Email:** через GitHub
- **Telegram:** [@krosov_ok](https://t.me/krosov_ok)
- **Канал:** [@programsKrosovok](https://t.me/programsKrosovok)

---

## 📄 Лицензия

Dual-licensed под **MIT** и **Apache 2.0**.

См. [LICENSE-MIT](LICENSE-MIT) и [LICENSE-APACHE](LICENSE-APACHE).

---

*Последнее обновление: Февраль 2026*

*Версия библиотеки: 0.2.0*
