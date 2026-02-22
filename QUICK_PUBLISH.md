# 🚀 Шпаргалка по публикации документации

## Быстрая команда (один скрипт)

### Windows
```powershell
.\setup-and-publish.ps1
```

### Linux/macOS
```bash
./setup-and-publish.sh
```

---

## Ручная публикация (по шагам)

```bash
# 1. Установить mdbook
cargo install mdbook

# 2. Собрать документацию
cd docs
mdbook build

# 3. Опубликовать
git checkout --orphan gh-pages
git reset --hard
cp -r book/* .
touch .nojekyll
git add .
git commit -m "docs: publish"
git push origin gh-pages --force
git checkout main
```

---

## Автоматическая публикация

Просто запушьте изменения в `main`:

```bash
git add .
git commit -m "docs: обновить документацию"
git push origin main
```

GitHub Actions автоматически опубликует документацию!

---

## Проверка локально

```bash
cd docs
mdbook serve
# Открыть http://localhost:3000
```

---

## URL документации

```
https://krosovok52.github.io/media-sessions/
```

---

## Настройка GitHub Pages (если нужно вручную)

1. GitHub → Settings → Pages
2. Source: Deploy from a branch
3. Branch: gh-pages
4. Save

---

## Troubleshooting

| Проблема | Решение |
|----------|---------|
| mdbook не найден | `cargo install mdbook` |
| 404 страница | Подождать 1-2 минуты |
| Ошибка push | `git push origin gh-pages --force` |
| Конфликт worktree | `git worktree prune` |

---

## Скрипты

| Скрипт | Описание |
|--------|----------|
| `publish-docs.sh` / `.ps1` | Быстрая публикация |
| `setup-and-publish.sh` / `.ps1` | Полная настройка + публикация |

---

**Всё! 🎉**
