# Contributing to media-sessions

Спасибо за интерес к проекту! Contributions приветствуются в любой форме:
- Bug reports
- Feature requests
- Pull requests
- Documentation improvements
- Benchmarks and performance improvements

## 📋 Guidelines

### Code Style

- Следуйте [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Используйте `cargo fmt` перед коммитом
- Избегайте `unwrap()` в публичном API, используйте proper error handling
- Добавляйте документацию для всех публичных items

### Testing

- Добавляйте тесты для нового функционала
- Все тесты должны проходить: `cargo test --all-features`
- Проверяйте clippy: `cargo clippy --all-targets -- -D warnings`

### Commit Messages

Используйте [Conventional Commits](https://www.conventionalcommits.org/):

```
feat: add macOS MediaRemote support
fix: handle D-Bus connection errors properly
docs: update README with usage examples
perf: improve current() latency by 40%
```

## 🚀 Development Setup

```bash
# Клонировать репозиторий
git clone https://github.com/krosovok52/media-sessions
cd media-sessions

# Установить Rust (если не установлен)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Запустить тесты
cargo test --all-features

# Запустить clippy
cargo clippy --all-targets -- -D warnings

# Отформатировать код
cargo fmt --all
```

## 📝 Pull Request Process

1. Создайте fork репозитория
2. Создайте feature branch: `git checkout -b feat/my-feature`
3. Внесите изменения и закоммитьте: `git commit -m 'feat: add my feature'`
4. Запушьте branch: `git push origin feat/my-feature`
5. Откройте Pull Request

### PR Checklist

- [ ] Код отформатирован (`cargo fmt`)
- [ ] Все тесты проходят (`cargo test`)
- [ ] Clippy warnings исправлены (`cargo clippy`)
- [ ] Документация обновлена
- [ ] Добавлены тесты для нового функционала

## 🐛 Reporting Bugs

Создайте issue с:
- Кратким описанием проблемы
- Шагами для воспроизведения
- Ожидаемым поведением
- Версией OS и Rust
- Примером кода (если применимо)

## 💡 Feature Requests

Feature requests приветствуются! Создайте issue с:
- Описанием функциональности
- Use case примером
- Возможными implementation деталями

## 🔒 Security

Если вы обнаружили security vulnerability, пожалуйста, напишите напрямую:
- Telegram: [@krosov_ok](https://t.me/krosov_ok)
- GitHub: [@krosovok52](https://github.com/krosovok52)

Не создавайте публичные issues для security проблем.

## 📄 License

Contributing, вы соглашаетесь что ваши contributions будут лицензированы под:
- MIT License
- Apache License 2.0

---

**Контакты:**
- Telegram: [@krosov_ok](https://t.me/krosov_ok)
- GitHub: [@krosovok52](https://github.com/krosovok52)
