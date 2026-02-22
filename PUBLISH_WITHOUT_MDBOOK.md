# 📤 Публикация документации на GitHub Pages

## ⚡ Быстрый способ (без mdbook)

Если mdbook не установлен, используйте этот скрипт:

```powershell
# publish-no-mdbook.ps1
Write-Host "Публикация документации без mdbook..." -ForegroundColor Cyan

cd $PSScriptRoot

# Создаём простую HTML страницу из README
$readme = Get-Content "README.md" -Raw
$html = @"
<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <title>Media Sessions Documentation</title>
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <style>
        body { font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Helvetica, Arial, sans-serif; margin: 40px; line-height: 1.6; }
        code { background: #f6f8fa; padding: 2px 6px; border-radius: 3px; }
        pre { background: #f6f8fa; padding: 16px; border-radius: 6px; overflow-x: auto; }
        a { color: #0366d6; }
        h1, h2, h3 { border-bottom: 1px solid #eaecef; padding-bottom: 0.3em; }
    </style>
</head>
<body>
    <h1>Media Sessions Documentation</h1>
    <p>Documentation is being built. Please install mdbook and rebuild.</p>
    <p><a href="https://github.com/krosovok52/media-sessions">View on GitHub</a></p>
</body>
</html>
"@

# Создаём временную ветку
git checkout -b gh-pages-temp 2>$null
if ($LASTEXITCODE -ne 0) {
    git checkout gh-pages 2>$null
    if ($LASTEXITCODE -ne 0) {
        git checkout --orphan gh-pages
        git reset --hard
    }
}

# Копируем HTML
$html | Out-File -FilePath "index.html" -Encoding UTF8

# Коммит и пуш
git add index.html
git commit -m "docs: initial placeholder"
git push -u origin gh-pages-temp:gh-pages --force

# Возвращаемся на main
git checkout main

Write-Host "Done! Visit: https://krosovok52.github.io/media-sessions/" -ForegroundColor Green
```

## Вариант 3: Использовать docs.rs

Для Rust API документации:
```
https://docs.rs/media-sessions
```

## Вариант 4: GitHub Actions (автоматически)

Просто запушьте изменения в main:
```bash
git add .
git commit -m "docs: update"
git push origin main
```

GitHub Actions автоматически соберёт и опубликует документацию!

---

## Установка mdbook (для будущей работы)

### Windows

```powershell
# Через winget
winget install Rustlang.Rust.MSVC

# После установки Rust
cargo install mdbook
```

### Проверка установки

```powershell
mdbook --version
```

### Сборка документации

```powershell
cd docs
mdbook build
```
