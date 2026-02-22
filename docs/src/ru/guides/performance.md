# Производительность

Руководство по оптимизации производительности Media Sessions.

## Бенчмарки

### Результаты (Windows 11, Ryzen 9 7950X)

| Операция | media-sessions | playerctl | mediaremote-rs |
|----------|---------------|-----------|----------------|
| `current()` latency | **~350 ns** | ~2.3 ms | ~1.8 ms |
| `watch()` first event | **~600 ns** | N/A | N/A |
| Event throughput | **~850/sec** | ~100/sec | N/A |

### Запуск бенчмарков

```bash
# Все бенчмарки
cargo bench --bench media_sessions

# Конкретный бенчмарк
cargo bench --bench media_sessions -- current_latency

# HTML отчёт
cargo bench --bench media_sessions -- --report
```

## Оптимизация запросов

### Кэширование

```rust
use media_sessions::MediaSessions;
use std::time::{Duration, Instant};

struct CachedMediaInfo {
    info: Option<MediaInfo>,
    timestamp: Instant,
    ttl: Duration,
}

impl CachedMediaInfo {
    fn new(ttl: Duration) -> Self {
        Self {
            info: None,
            timestamp: Instant::now(),
            ttl,
        }
    }
    
    async fn get_or_refresh(
        &mut self,
        sessions: &MediaSessions,
    ) -> Result<Option<&MediaInfo>, MediaError> {
        if self.info.is_none() || self.timestamp.elapsed() > self.ttl {
            self.info = sessions.current().await?;
            self.timestamp = Instant::now();
        }
        Ok(self.info.as_ref())
    }
}

// Использование
let mut cache = CachedMediaInfo::new(Duration::from_secs(1));
let info = cache.get_or_refresh(&sessions).await?;
```

### Debounce событий

```rust
use media_sessions::MediaSessions;
use std::time::Duration;

// 500ms debounce для фильтрации быстрых изменений
let sessions = MediaSessions::builder()
    .debounce_duration(Duration::from_millis(500))
    .build()?;
```

**Рекомендации по debounce:**

| Сценарий | Рекомендуемый debounce |
|----------|------------------------|
| UI обновление | 300-500ms |
| Логирование | 500-800ms |
| Highload мониторинг | 100-200ms |
| Real-time синхронизация | 0-50ms |

## Оптимизация async кода

### Избегайте лишних await

```rust
// ❌ Плохо: последовательные запросы
let info = sessions.current().await?;
let app = sessions.active_app().await?;

// ✅ Хорошо: параллельные запросы
let (info, app) = tokio::try_join!(
    sessions.current(),
    sessions.active_app()
)?;
```

### Буферизация событий

```rust
use futures::{StreamExt, stream};

let mut stream = sessions
    .watch()
    .await?
    .buffered(10);  // Буферизация 10 событий

while let Some(event) = stream.next().await {
    // Обработка
}
```

### Rate limiting

```rust
use tokio::time::{interval, Duration};

let mut interval = interval(Duration::from_millis(100));

loop {
    interval.tick().await;  // Ограничение 10 запросов/сек
    
    if let Some(info) = sessions.current().await? {
        // Обработка
    }
}
```

## Память

### Избегайте клонирования

```rust
// ❌ Плохо: лишние клоны
let title = info.title().to_string();
let artist = info.artist().to_string();

// ✅ Хорошо: заимствование
let title = info.title();
let artist = info.artist();
```

### Оптимизация обложек

```rust
// ❌ Плохо: загрузка больших обложек
if let Some(artwork) = &info.artwork {
    let large_image = image::load_from_memory(artwork)?;
}

// ✅ Хорошо: проверка размера перед загрузкой
if let Some(artwork) = &info.artwork {
    if artwork.len() < 1024 * 1024 {  // < 1MB
        // Обработка
    }
}
```

## Платформенная оптимизация

### Windows

```rust
// Windows SMTC очень быстрый, можно polling
#[cfg(target_os = "windows")]
let debounce = Duration::from_millis(100);

// Linux D-Bus медленнее, нужен больший debounce
#[cfg(target_os = "linux")]
let debounce = Duration::from_millis(500);

// macOS MediaRemote средний
#[cfg(target_os = "macos")]
let debounce = Duration::from_millis(300);
```

### Linux D-Bus оптимизация

```rust
// Подписка на события вместо polling
let mut stream = sessions.watch().await?;

while let Some(event) = stream.next().await {
    // События приходят автоматически
}
```

## Профилирование

### Cargo flamegraph

```bash
# Установить flamegraph
cargo install flamegraph

# Запустить профилирование
cargo flamegraph --example basic_usage

# Открыть результат
# target/flamegraph/svg/basic_usage.svg
```

### tokio-console

```bash
# Установить tokio-console
cargo install tokio-console

# Запустить с console-subscriber
RUSTFLAGS="--cfg tokio_unstable" cargo run --example basic_usage

# В другом терминале
tokio-console
```

## Best Practices

### 1. Используйте builder для настройки

```rust
let sessions = MediaSessions::builder()
    .debounce_duration(Duration::from_millis(300))
    .operation_timeout(Duration::from_secs(3))
    .enable_artwork(false)  // Отключить если не нужно
    .build()?;
```

### 2. Избегайте polling когда возможно

```rust
// ❌ Плохо: постоянный polling
loop {
    let info = sessions.current().await?;
    tokio::time::sleep(Duration::from_millis(100)).await;
}

// ✅ Хорошо: событийная модель
let mut stream = sessions.watch().await?;
while let Some(event) = stream.next().await {
    // Обработка событий
}
```

### 3. Batch операции

```rust
// ❌ Плохо: много отдельных запросов
for _ in 0..10 {
    sessions.next().await?;
}

// ✅ Хорошо: одна операция (если поддерживается)
// (зависит от плеера)
```

### 4. Lazy инициализация

```rust
struct MediaController {
    sessions: OnceCell<MediaSessions>,
}

impl MediaController {
    async fn get_sessions(&self) -> Result<&MediaSessions, MediaError> {
        self.sessions.get_or_try_init(|| async {
            MediaSessions::new()
        }).await
    }
}
```

## Troubleshooting

### Высокая задержка

**Проблема:** `current()` занимает > 10ms

**Решение:**

1. Проверьте платформу (Linux D-Bus медленнее)
2. Увеличьте timeout
3. Используйте кэширование

### Частые события

**Проблема:** Слишком много событий

**Решение:**

```rust
let sessions = MediaSessions::builder()
    .debounce_duration(Duration::from_millis(1000))  // Увеличить
    .build()?;
```

### Утечки памяти

**Проблема:** Рост потребления памяти

**Решение:**

1. Проверьте обработку обложек
2. Используйте `drop()` для очистки
3. Избегайте циклических ссылок

## См. также

- **[События](rust-api/events.md)** — Оптимизация событий
- **[Обработка ошибок](error-handling.md)** — Error handling
- **[tokio-console](https://github.com/tokio-rs/console)** — Профилирование
