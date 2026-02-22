#!/usr/bin/env bash
# setup-and-publish.sh - Полный скрипт для настройки и публикации документации
# Использование: ./setup-and-publish.sh

set -e

echo "╔═══════════════════════════════════════════════════════════╗"
echo "║   Публикация документации Media Sessions на GitHub Pages  ║"
echo "╚═══════════════════════════════════════════════════════════╝"
echo

# Цвета для вывода
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Функция для печати заголовков
print_header() {
    echo
    echo -e "${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo -e "${CYAN}$1${NC}"
    echo -e "${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo
}

# Функция для печати успеха
print_success() {
    echo -e "${GREEN}✅ $1${NC}"
}

# Функция для печати ошибки
print_error() {
    echo -e "${RED}❌ $1${NC}"
}

# Функция для печати предупреждения
print_warning() {
    echo -e "${YELLOW}⚠️  $1${NC}"
}

# Проверка наличия git
print_header "1. Проверка зависимостей"

if ! command -v git &> /dev/null; then
    print_error "git не найден. Установите git: https://git-scm.com/"
    exit 1
fi
print_success "git найден: $(git --version)"

# Проверка наличия Rust
if ! command -v cargo &> /dev/null; then
    print_error "cargo не найден. Установите Rust: https://rustup.rs/"
    exit 1
fi
print_success "cargo найден: $(cargo --version)"

# Проверка наличия mdbook или установка
if ! command -v mdbook &> /dev/null; then
    print_warning "mdbook не найден. Установка..."
    cargo install mdbook
    print_success "mdbook установлен"
else
    print_success "mdbook найден: $(mdbook --version)"
fi

# Проверка нахождения в репозитории
print_header "2. Проверка репозитория"

if [ ! -d ".git" ]; then
    print_error "Текущая директория не является git репозиторием"
    exit 1
fi
print_success "Git репозиторий найден"

# Проверка remote
if ! git remote get-url origin &> /dev/null; then
    print_error "origin remote не найден"
    echo "Добавьте remote:"
    echo "  git remote add origin https://github.com/YOUR_USERNAME/media-sessions"
    exit 1
fi

REMOTE_URL=$(git remote get-url origin)
print_success "Remote найден: $REMOTE_URL"

# Извлечение имени пользователя и репозитория
REPO_NAME=$(basename -s .git "$REMOTE_URL")
print_success "Репозиторий: $REPO_NAME"

# Сборка документации
print_header "3. Сборка документации"

cd docs

if [ ! -f "book.toml" ]; then
    print_error "book.toml не найден в docs/"
    exit 1
fi

print_success "Сборка документации..."
mdbook build

if [ ! -d "book" ]; then
    print_error "Директория book/ не создана"
    exit 1
fi

print_success "Документация собрана в docs/book/"

cd ..

# Проверка наличия ветки gh-pages
print_header "4. Настройка GitHub Pages"

if git show-ref --verify --quiet refs/heads/gh-pages; then
    print_success "Ветка gh-pages существует"
else
    print_warning "Ветка gh-pages не существует. Создание..."
    git checkout --orphan gh-pages
    git reset --hard
    git commit --allow-empty -m "Initial commit for GitHub Pages"
    git checkout -
    print_success "Ветка gh-pages создана"
fi

# Публикация
print_header "5. Публикация документации"

# Создание временной ветки
TEMP_BRANCH="gh-pages-temp-$$"

print_success "Создание временной ветки: $TEMP_BRANCH"
git worktree add -f "$TEMP_BRANCH" gh-pages 2>/dev/null || {
    git checkout --orphan "$TEMP_BRANCH"
    git reset --hard
}

# Копирование файлов
print_success "Копирование файлов документации..."
rm -rf "$TEMP_BRANCH"/*
cp -r docs/book/* "$TEMP_BRANCH"/

# Добавление .nojekyll
touch "$TEMP_BRANCH/.nojekyll"

# Коммит
cd "$TEMP_BRANCH"
git add .

if git diff --staged --quiet; then
    print_warning "Нет изменений для публикации"
    cd ..
    git worktree remove -f "$TEMP_BRANCH"
    print_success "Публикация не требуется"
    exit 0
fi

git commit -m "docs: обновить документацию ($(date '+%Y-%m-%d %H:%M'))"

# Публикация на GitHub
print_success "Публикация на GitHub..."
git push origin "$TEMP_BRANCH":gh-pages --force

cd ..
git worktree remove -f "$TEMP_BRANCH"

# Вывод результата
print_header "🎉 Публикация завершена!"

echo
echo -e "${GREEN}✅ Документация успешно опубликована!${NC}"
echo
echo -e "${CYAN}🌐 URL документации:${NC}"
echo "   https://krosovok52.github.io/media-sessions/"
echo
echo -e "${CYAN}📁 Ветка GitHub:${NC}"
echo "   gh-pages"
echo
echo -e "${CYAN}⏱️  Время публикации:${NC}"
echo "   Обычно занимает 1-2 минуты"
echo
echo -e "${YELLOW}📝 Примечание:${NC}"
echo "   Если документация не доступна, проверьте настройки:"
echo "   GitHub → Settings → Pages → Source: gh-pages branch"
echo

# Предложение открыть документацию
read -p "🌐 Открыть документацию в браузере? (y/n) " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
    if command -v xdg-open &> /dev/null; then
        xdg-open "https://krosovok52.github.io/media-sessions/"
    elif command -v open &> /dev/null; then
        open "https://krosovok52.github.io/media-sessions/"
    elif command -v start &> /dev/null; then
        start "https://krosovok52.github.io/media-sessions/"
    else
        print_warning "Не удалось открыть браузер автоматически"
    fi
fi

print_success "Готово!"
