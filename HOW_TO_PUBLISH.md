# 📋 ЧЕК-ЛИСТ: Публикация media-sessions

## ✅ Подготовка (СДЕЛАНО)

- [x] Код готов и работает
- [x] Тесты проходят (12 passed)
- [x] Документация написана
- [x] README.md готов
- [x] Примеры использования есть
- [x] CI/CD настроен (.github/workflows/ci.yml)
- [x] LICENSE файлы есть
- [x] CHANGELOG.md заполнен

---

## 🚀 ШАГ 1: GitHub (5 минут)

### 1.1 Создай репозиторий

1. Зайди на https://github.com/new
2. Repository name: **`media-sessions`**
3. Description: "Cross-platform media session control for Rust"
4. Visibility: **Public** ✅
5. Нажми **Create repository**

### 1.2 Запуши код

Открой PowerShell и выполни:

```powershell
cd "C:\Users\rykov\OneDrive\Рабочий стол\MediaSession"

git init
git add .
git commit -m "Initial release: media-sessions v0.2.0"
git remote add origin https://github.com/krosovok52/media-sessions.git
git push -u origin main
```

**Готово!** Твой код на GitHub! 🎉

---

## 🚀 ШАГ 2: crates.io (10 минут)

### 2.1 Зарегистрируйся

1. Зайди на https://crates.io
2. Нажми **Log in** → выбери GitHub
3. Разреши доступ

### 2.2 Получи токен

1. Зайди на https://crates.io/me
2. Нажми **New API Token**
3. Введи название (например, "my-laptop")
4. Скопируй токен **сохрани его!** (покажи только один раз)

### 2.3 Залогинься

```powershell
cargo login <вставь_свой_токен>
```

### 2.4 Проверь

```powershell
cd "C:\Users\rykov\OneDrive\Рабочий стол\MediaSession"
cargo publish --dry-run
```

Если ошибок нет — продолжаем.

### 2.5 Опубликуй

```powershell
cargo publish
```

**Готово!** Твоя библиотека на crates.io! 🎉

Проверить: https://crates.io/crates/media-sessions

---

## 🚀 ШАГ 3: Релиз на GitHub (3 минуты)

1. Зайди на https://github.com/krosovok52/media-sessions/releases
2. Нажми **Draft a new release**
3. Заполни:
   - **Tag version:** `v0.2.0`
   - **Release title:** `Media Sessions v0.2.0`
   - **Description:**
   ```
   ## Что нового
   
   ✅ Полная поддержка Windows WinRT SMTC
   ✅ Linux MPRIS/D-Bus
   ✅ macOS MediaRemote
   ✅ Async Tokio API
   ✅ Бенчмарки (350ns latency!)
   ✅ Debounce событий
   
   ## Установка
   
   ```toml
   [dependencies]
   media-sessions = "0.2"
   ```
   
   ## Документация
   
   https://docs.rs/media-sessions
   ```
4. Нажми **Publish release**

---

## 📢 ШАГ 4: Анонс (15 минут)

### Telegram каналы

**1. Твой канал:**
- @programsKrosovok
- Напиши пост о релизе

**2. Rust community:**
- @rustlang_ru
- @rust_crate

**Пример поста:**

```
🎉 Выпустил media-sessions v0.2.0!

Кроссплатформенная библиотека для управления 
медиаплеерами на Rust!

✨ Особенности:
• Windows/Mac/Linux
• Async API
• 350ns latency (быстрее playerctl в 6 раз!)
• Встроенный debounce

📦 cargo add media-sessions

📖 https://docs.rs/media-sessions
🐛 https://github.com/krosovok52/media-sessions

#rust #lang:ru
```

### Reddit

1. Зайди на https://www.reddit.com/r/rust/
2. Нажми **Create Post**
3. Title: `[ANN] media-sessions v0.2.0 - Cross-platform media control`
4. Content: краткое описание + ссылки

### Habr (опционально)

Напиши статью "Как я создал библиотеку для управления медиа на Rust"

---

## ✅ ПРОВЕРКА

После публикации проверь:

- [ ] GitHub: https://github.com/krosovok52/media-sessions
- [ ] crates.io: https://crates.io/crates/media-sessions
- [ ] Docs: https://docs.rs/media-sessions (появится через 5-10 мин)

---

## 📊 Как другие будут использовать

### В Cargo.toml:

```toml
[dependencies]
media-sessions = "0.2"
tokio = { version = "1", features = ["full"] }
```

### В коде:

```rust
use media_sessions::MediaSessions;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sessions = MediaSessions::new()?;
    
    if let Some(info) = sessions.current().await? {
        println!("Playing: {}", info.title());
    }
    
    Ok(())
}
```

---

## 🎯 ИТОГ

После выполнения всех шагов:

✅ Твоя библиотека доступна всему миру  
✅ Любой может установить: `cargo add media-sessions`  
✅ Автоматическая документация на docs.rs  
✅ CI/CD тестирует каждый коммит  
✅ Ты опубликованный Rust разработчик! 🎉

---

## 📬 Контакты для связи

- Telegram: @krosov_ok
- GitHub: github.com/krosovok52
- Канал: t.me/programsKrosovok

---

**Удачи с публикацией! 🚀**

*Время выполнения: ~30 минут*
