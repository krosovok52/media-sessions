# Publishing Guide

## Подготовка к публикации

### 1. Проверка перед публикацией

```bash
# Убедитесь что все тесты проходят
cargo test --all-features

# Проверьте clippy
cargo clippy --all-targets -- -D warnings

# Проверьте форматирование
cargo fmt --all -- --check

# Проверьте документацию
cargo doc --all-features --no-deps

# Проверьте что крейт может быть опубликован
cargo publish --dry-run
```

### 2. Обновление версии

Отредактируйте `Cargo.toml`:

```toml
[package]
version = "0.2.0"  # Обновите версию
```

Обновите `CHANGELOG.md` с новыми изменениями.

### 3. Публикация на crates.io

```bash
# Логин (один раз)
cargo login <your_api_token>

# Получить токен: https://crates.io/me

# Публикация
cargo publish
```

### 4. Создание Git тега

```bash
git add .
git commit -m "release: v0.2.0"
git tag -a v0.2.0 -m "Version 0.2.0"
git push origin main
git push origin v0.2.0
```

### 5. GitHub Release

1. Перейдите на https://github.com/krosovok52/media-sessions/releases
2. Нажмите "Draft a new release"
3. Выберите тег v0.2.0
4. Добавьте changelog из CHANGELOG.md
5. Нажмите "Publish release"

## Проверка после публикации

```bash
# Проверьте на crates.io
cargo search media-sessions

# Установите и протестируйте
cargo install media-sessions
```

## Автоматическая публикация (CI/CD)

Для автоматической публикации при создании тега, добавьте в `.github/workflows/release.yml`:

```yaml
name: Release

on:
  push:
    tags:
      - 'v*'

jobs:
  publish:
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v4
    
    - name: Install Rust
      uses: dtolnay/rust-action@stable
    
    - name: Publish to crates.io
      env:
        CARGO_REGISTRY_TOKEN: ${{ secrets.CARGO_REGISTRY_TOKEN }}
      run: cargo publish
```

## Секреты для GitHub Actions

Добавьте токен crates.io в GitHub Secrets:

1. Settings → Secrets and variables → Actions
2. New repository secret
3. Name: `CARGO_REGISTRY_TOKEN`
4. Value: ваш токен из https://crates.io/me

---

**Author:** krosov_ok  
**Telegram:** [@krosov_ok](https://t.me/krosov_ok)  
**GitHub:** [@krosovok52](https://github.com/krosovok52)
