# 📚 Публикация документации Media Sessions

Это руководство по сборке и публикации документации на GitHub Pages.

---

## 🚀 Быстрый старт

### 1. Установить mdBook

```bash
# Используя cargo (рекомендуется)
cargo install mdbook

# Или winget (Windows)
winget install Rust.mdbook

# Или homebrew (macOS)
brew install mdbook

# Или snap (Linux)
snap install mdbook
```

### 2. Собрать документацию

```bash
# Bash (Linux/macOS)
./publish-docs.sh build

# PowerShell (Windows)
.\publish-docs.ps1 build

# Или вручную
cd docs
mdbook build
```

### 3. Опубликовать на GitHub Pages

```bash
# Способ 1: Автоматически через GitHub Actions (рекомендуется)
git add docs/
git commit -m "docs: update documentation"
git push

# Способ 2: Вручную через git subtree
./publish-docs.sh deploy
# или
.\publish-docs.ps1 deploy
```

---

## 📖 Команды скрипта

### Bash (Linux/macOS)

| Команда | Описание |
|---------|----------|
| `./publish-docs.sh build` | Собрать документацию |
| `./publish-docs.sh serve` | Запустить локальный сервер |
| `./publish-docs.sh deploy` | Опубликовать на GitHub Pages |
| `./publish-docs.sh push` | Push в main (auto-deploy) |
| `./publish-docs.sh clean` | Очистить сборку |
| `./publish-docs.sh check` | Проверить mdBook |
| `./publish-docs.sh help` | Показать справку |

### PowerShell (Windows)

| Команда | Описание |
|---------|----------|
| `.\publish-docs.ps1 build` | Собрать документацию |
| `.\publish-docs.ps1 serve` | Запустить локальный сервер |
| `.\publish-docs.ps1 deploy` | Опубликовать на GitHub Pages |
| `.\publish-docs.ps1 push` | Push в main (auto-deploy) |
| `.\publish-docs.ps1 clean` | Очистить сборку |
| `.\publish-docs.ps1 check` | Проверить mdBook |
| `.\publish-docs.ps1 help` | Показать справку |

---

## 🔧 Ручная сборка

### 1. Собрать документацию

```bash
cd docs
mdbook build
```

### 2. Проверить локально

```bash
cd docs
mdbook serve --open
```

Документация откроется в браузере по адресу `http://localhost:3000`.

### 3. Опубликовать вручную

```bash
# Создать .nojekyll файл
touch docs/book/.nojekyll

# Опубликовать
git subtree push --prefix docs/book origin gh-pages
```

---

## 🌐 GitHub Pages настройка

### 1. Включить GitHub Pages

1. Перейти в **Settings** → **Pages**
2. Выбрать **Source**: Deploy from a branch
3. Выбрать ветку: `gh-pages`
4. Выбрать папку: `/ (root)`
5. Нажать **Save**

### 2. Проверить URL

Документация будет доступна по адресу:

```
https://krosovok52.github.io/media-sessions/
```

---

## 🔄 Автоматический деплой

GitHub Actions workflow автоматически публикует документацию при:

- Push в ветку `main`
- Изменениях в `docs/` директории
- Ручном запуске из вкладки Actions

### Файл workflow

`.github/workflows/deploy-docs.yml`:

```yaml
name: Deploy Documentation

on:
  push:
    branches: [main]
    paths: ['docs/**', 'book.toml']
  workflow_dispatch:

jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Install mdBook
        run: |
          curl -L https://github.com/rust-lang/mdBook/releases/download/v0.4.36/mdbook-v0.4.36-x86_64-unknown-linux-gnu.tar.gz | tar xz
          chmod +x mdbook
          sudo mv mdbook /usr/local/bin/
      - name: Build with mdBook
        run: cd docs && mdbook build
      - name: Deploy to GitHub Pages
        uses: peaceiris/actions-gh-pages@v4
        with:
          github_token: ${{ secrets.GITHUB_TOKEN }}
          publish_dir: ./docs/book
```

---

## 📝 Структура документации

```
docs/
├── book.toml              # Конфигурация mdBook
├── src/
│   ├── SUMMARY.md         # Главное оглавление
│   ├── index.md           # Главная страница
│   ├── quickstart.md      # Быстрый старт
│   ├── ru/                # Русская версия
│   │   ├── SUMMARY.md     # Оглавление RU
│   │   ├── README.md      # Главная RU
│   │   ├── introduction.md
│   │   ├── installation.md
│   │   ├── rust-api/      # Rust API RU
│   │   ├── languages/     # Языки RU
│   │   ├── platforms/     # Платформы RU
│   │   └── guides/        # Гайды RU
│   └── en/                # English version
│       ├── SUMMARY.md     # Table of contents EN
│       └── README.md      # Home page EN
└── book/                  # Сгенерированная документация
    ├── index.html
    ├── ru/                # Russian HTML
    └── en/                # English HTML
```

---

## 🎨 Темы и стили

### Изменить тему

В `docs/book.toml`:

```toml
[output.html]
default-theme = "navy"  # light, navy, ayu, coal
```

### Добавить кастомные стили

Создать `docs/src/theme/custom.css`:

```css
.sidebar {
    background-color: #f5f5f5;
}
```

В `book.toml`:

```toml
[output.html]
additional-css = ["theme/custom.css"]
```

---

## 🐛 Troubleshooting

### mdbook не найден

```bash
# Проверить установку
which mdbook

# Переустановить
cargo install mdbook --force
```

### Ошибки сборки

```bash
# Очистить кэш
rm -rf docs/book

# Собрать заново
cd docs
mdbook clean
mdbook build
```

### Проблемы с кодировкой

Убедитесь, что все файлы в UTF-8:

```bash
# Проверить (Linux/macOS)
file docs/src/ru/*.md

# Конвертировать
iconv -f WINDOWS-1251 -t UTF-8 input.md > output.md
```

### Git subtree ошибка

```bash
# Проверить git статус
git status

# Убедиться, что все закоммичено
git add .
git commit -m "Save changes"

# Попробовать снова
git subtree push --prefix docs/book origin gh-pages
```

---

## ✅ Чеклист перед публикацией

- [ ] Все ссылки работают
- [ ] Кодировка UTF-8
- [ ] Нет битых изображений
- [ ] Оглавление обновлено
- [ ] Переводы синхронизированы
- [ ] Сборка проходит без ошибок
- [ ] `.nojekyll` файл существует

---

## 📚 Ресурсы

- **mdBook документация:** https://rust-lang.github.io/mdBook/
- **GitHub Pages:** https://pages.github.com/
- **Примеры тем:** https://github.com/rust-lang/mdBook/tree/master/examples

---

**Версия:** 0.2.0 | **Последнее обновление:** Февраль 2026
