# 📚 Documentation Overview / Обзор документации

**Media Sessions** — Complete documentation package / Полный пакет документации

---

## 🗂️ File Structure / Структура файлов

```
MediaSession/
├── README.md                      # Main README (RU/EN)
├── QUICKSTART.md                  # Quick Start Guide (RU/EN)
├── DOCUMENTATION.md               # Full Documentation (Russian)
├── docs/
│   ├── README.md                  # mdBook documentation guide
│   ├── DEPLOY.md                  # Deployment instructions
│   ├── book.toml                  # mdBook configuration
│   └── src/
│       ├── SUMMARY.md             # Root table of contents
│       ├── ru/                    # Russian version
│       │   ├── SUMMARY.md         # RU table of contents
│       │   ├── README.md          # RU home page
│       │   └── ...                # RU documentation files
│       └── en/                    # English version
│           ├── SUMMARY.md         # EN table of contents
│           ├── README.md          # EN home page
│           └── ...                # EN documentation files
├── c-api/
│   ├── README.md                  # C API overview
│   ├── API_REFERENCE.md           # Complete C API reference
│   ├── python_example.py          # Python example
│   └── csharp_example.cs          # C# example
└── .github/
    └── workflows/
        └── deploy-docs.yml        # GitHub Actions for deployment
```

---

## 📖 Documentation Files / Файлы документации

### Main Documentation / Основная документация

| File / Файл | Language / Язык | Pages / Страниц | Description / Описание |
|-------------|-----------------|-----------------|------------------------|
| `README.md` | RU/EN | 1 | Project overview / Обзор проекта |
| `QUICKSTART.md` | RU/EN | 1 | 5-minute guide / 5-минутное руководство |
| `DOCUMENTATION.md` | RU | ~40 | Complete guide / Полное руководство |
| `docs/README.md` | RU/EN | 1 | mdBook guide / Руководство mdBook |

### C API Documentation / Документация C API

| File / Файл | Language / Язык | Pages / Страниц | Description / Описание |
|-------------|-----------------|-----------------|------------------------|
| `c-api/README.md` | RU/EN | 2 | C API overview / Обзор C API |
| `c-api/API_REFERENCE.md` | RU/EN | ~25 | Complete reference / Полный справочник |

### mdBook Documentation / Документация mdBook

| Directory / Директория | Language / Язык | Files / Файлов | Description / Описание |
|------------------------|-----------------|----------------|------------------------|
| `docs/src/ru/` | Russian | ~20 | Full RU documentation / Полная документация RU |
| `docs/src/en/` | English | ~20 | Full EN documentation / Полная документация EN |

---

## 🌐 Online Documentation / Онлайн документация

### GitHub Pages

After deployment, documentation will be available at:

После развёртывания документация будет доступна по адресу:

```
https://krosovok52.github.io/media-sessions/
```

### Features / Возможности

- ✅ **Multi-language** (Russian/English)
- ✅ **Full-text search**
- ✅ **Dark/Light themes**
- ✅ **Mobile responsive**
- ✅ **Syntax highlighting**
- ✅ **Navigation sidebar**

---

## 🚀 Quick Start / Быстрый старт

### View Documentation / Просмотр документации

```bash
# Install mdBook / Установить mdBook
cargo install mdbook

# Build / Собрать
cd docs
mdbook build

# Serve locally / Запустить локально
mdbook serve

# Open browser / Открыть в браузере
# http://localhost:3000
```

### Deploy to GitHub Pages / Опубликовать на GitHub Pages

```bash
# Automatic (GitHub Actions) / Автоматически (GitHub Actions)
# Just push to main branch / Просто push в ветку main

# Manual / Вручную
cd docs
mdbook build
git add docs/book
git commit -m "docs: deploy"
git subtree push --prefix docs/book origin gh-pages
```

---

## 📊 Documentation Statistics / Статистика документации

| Metric / Метрика | Count / Количество |
|------------------|-------------------|
| **Total files** | ~50 |
| **Total lines** | ~5000 |
| **Languages** | 2 (RU, EN) |
| **Code examples** | ~100 |
| **API functions documented** | ~30 |

---

## 🎯 Documentation Coverage / Покрытие документацией

### Components / Компоненты

| Component / Компонент | Documented? / Задокументировано? |
|----------------------|----------------------------------|
| Rust API | ✅ Yes / Да |
| C API | ✅ Yes / Да |
| Python binding | ✅ Yes / Да |
| C# binding | ✅ Yes / Да |
| C/C++ binding | ✅ Yes / Да |
| Node.js binding | ✅ Yes / Да |
| Windows backend | ✅ Yes / Да |
| Linux backend | ✅ Yes / Да |
| macOS backend | ✅ Yes / Да |
| Error handling | ✅ Yes / Да |
| Performance | ✅ Yes / Да |
| FAQ | ✅ Yes / Да |

---

## 📝 Maintenance / Поддержка

### Update Documentation / Обновление документации

1. Edit source files / Редактировать исходные файлы
2. Build and test / Собрать и протестировать
   ```bash
   cd docs
   mdbook build
   mdbook serve
   ```
3. Commit changes / Закоммитить изменения
   ```bash
   git add docs/
   git commit -m "docs: update"
   git push
   ```

### Add New Language / Добавить новый язык

1. Create directory / Создать директорию
   ```bash
   mkdir docs/src/<lang>
   ```
2. Copy structure / Скопировать структуру
   ```bash
   cp -r docs/src/en/* docs/src/<lang>/
   ```
3. Translate files / Перевести файлы
4. Update SUMMARY.md / Обновить SUMMARY.md

---

## ✅ Checklist / Контрольный список

Before publishing / Перед публикацией:

- [ ] All links work / Все ссылки работают
- [ ] UTF-8 encoding / Кодировка UTF-8
- [ ] No broken images / Нет битых изображений
- [ ] Table of contents updated / Оглавление обновлено
- [ ] Translations synchronized / Переводы синхронизированы
- [ ] Build passes without errors / Сборка без ошибок

---

## 📬 Support / Поддержка

- **Documentation issues:** Create GitHub issue / Создайте GitHub issue
- **Questions:** [@krosov_ok](https://t.me/krosov_ok)
- **Telegram channel:** [@programsKrosovok](https://t.me/programsKrosovok)

---

**Version / Версия:** 0.2.0  
**Last updated / Последнее обновление:** February 2026  
**License / Лицензия:** MIT OR Apache-2.0
