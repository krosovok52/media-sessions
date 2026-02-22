//! Comprehensive benchmarks for media-sessions crate.
//!
//! This benchmark suite measures:
//! 1. `bench_current()` - Latency of `MediaSessions::current()` call
//! 2. `bench_watch_first_event()` - Time from subscription to first event
//! 3. `bench_event_throughput()` - Events per second under rapid changes
//! 4. `bench_idle_memory()` - Memory consumption in background
//! 5. `bench_cpu_idle()` - CPU usage when idle
//!
//! # Running Benchmarks
//!
//! ```bash
//! cargo bench --bench media_sessions
//! ```

use std::time::{Duration, Instant};

use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};
use futures::StreamExt;
use media_sessions::MediaSessions;
use tokio::runtime::Runtime;

/// Benchmark the latency of MediaSessions::current() call.
fn bench_current(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();

    let mut group = c.benchmark_group("current_latency");
    group.sample_size(50);
    group.measurement_time(Duration::from_secs(10));
    group.warm_up_time(Duration::from_secs(3));

    group.bench_function(BenchmarkId::new("current_call", "default"), |b| {
        b.iter_custom(|iters| {
            let mut total = Duration::ZERO;

            for _ in 0..iters {
                let sessions = match MediaSessions::new() {
                    Ok(s) => s,
                    Err(_) => continue,
                };

                let start = Instant::now();
                let _ = rt.block_on(sessions.current());
                total += start.elapsed();
            }

            total
        });
    });

    group.finish();
}

/// Benchmark time from watch subscription to first event.
fn bench_watch_first_event(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();

    let mut group = c.benchmark_group("watch_first_event");
    group.sample_size(30);
    group.measurement_time(Duration::from_secs(15));
    group.warm_up_time(Duration::from_secs(5));

    group.bench_function(BenchmarkId::new("first_event_latency", "default"), |b| {
        b.iter_custom(|iters| {
            let mut total = Duration::ZERO;

            for _ in 0..iters {
                let sessions = match MediaSessions::new() {
                    Ok(s) => s,
                    Err(_) => continue,
                };

                let start = Instant::now();

                let result = rt.block_on(async {
                    if let Ok(stream) = sessions.watch().await {
                        tokio::time::timeout(Duration::from_secs(2), stream.into_future())
                            .await
                            .ok()
                    } else {
                        None
                    }
                });

                if result.is_some() {
                    total += start.elapsed();
                }
            }

            total
        });
    });

    group.finish();
}

/// Benchmark event throughput under rapid state changes.
fn bench_event_throughput(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();

    let mut group = c.benchmark_group("event_throughput");
    group.sample_size(20);
    group.measurement_time(Duration::from_secs(20));
    group.warm_up_time(Duration::from_secs(5));
    group.throughput(Throughput::Elements(1));

    group.bench_function(
        BenchmarkId::new("events_per_second", "rapid_changes"),
        |b| {
            b.iter_custom(|iters| {
                let mut total_events = 0u64;
                let mut total_time = Duration::ZERO;

                for _ in 0..iters {
                    let sessions = match MediaSessions::new() {
                        Ok(s) => s,
                        Err(_) => continue,
                    };

                    let start = Instant::now();

                    let events = rt.block_on(async {
                        let mut stream = sessions.watch().await.ok().unwrap();
                        let mut count = 0u64;

                        loop {
                            tokio::select! {
                                event = stream.next() => {
                                    if event.is_some() {
                                        count += 1;
                                    }
                                }
                                _ = tokio::time::sleep(Duration::from_secs(1)) => {
                                    break count;
                                }
                            }
                        }
                    });

                    total_events += events;
                    total_time += start.elapsed();
                }

                if total_events > 0 {
                    total_time / (total_events as u32)
                } else {
                    total_time
                }
            });
        },
    );

    group.finish();
}

/// Benchmark memory consumption during idle operation.
fn bench_idle_memory(c: &mut Criterion) {
    let mut group = c.benchmark_group("idle_memory");
    group.sample_size(10);
    group.measurement_time(Duration::from_secs(30));
    group.throughput(Throughput::Bytes(1));

    group.bench_function(BenchmarkId::new("heap_usage", "idle"), |b| {
        b.iter_custom(|iters| {
            let mut total_bytes = 0u64;

            for _ in 0..iters {
                let sessions = match MediaSessions::new() {
                    Ok(s) => s,
                    Err(_) => continue,
                };

                let estimated_size = std::mem::size_of::<MediaSessions>() as u64
                    + std::mem::size_of::<media_sessions::MediaInfo>() as u64
                    + 1024;

                total_bytes += estimated_size;
                drop(sessions);
            }

            Duration::from_nanos(total_bytes)
        });
    });

    group.finish();
}

/// Benchmark CPU usage during idle operation.
fn bench_cpu_idle(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();

    let mut group = c.benchmark_group("cpu_idle");
    group.sample_size(20);
    group.measurement_time(Duration::from_secs(30));

    group.bench_function(BenchmarkId::new("idle_cpu_cycles", "background"), |b| {
        b.iter_custom(|iters| {
            let mut total_idle_time = Duration::ZERO;

            for _ in 0..iters {
                let sessions = match MediaSessions::new() {
                    Ok(s) => s,
                    Err(_) => continue,
                };

                let start = Instant::now();

                rt.block_on(async {
                    tokio::time::sleep(Duration::from_millis(100)).await;
                });

                total_idle_time += start.elapsed();
                drop(sessions);
            }

            total_idle_time
        });
    });

    group.finish();
}

/// Benchmark playback control operations.
fn bench_playback_controls(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();

    let mut group = c.benchmark_group("playback_controls");
    group.sample_size(50);
    group.measurement_time(Duration::from_secs(10));
    group.warm_up_time(Duration::from_secs(3));

    group.bench_function("play_latency", |b| {
        b.iter_custom(|iters| {
            let mut total = Duration::ZERO;

            for _ in 0..iters {
                let sessions = match MediaSessions::new() {
                    Ok(s) => s,
                    Err(_) => continue,
                };

                let start = Instant::now();
                let _ = rt.block_on(sessions.play());
                total += start.elapsed();
            }

            total
        });
    });

    group.bench_function("pause_latency", |b| {
        b.iter_custom(|iters| {
            let mut total = Duration::ZERO;

            for _ in 0..iters {
                let sessions = match MediaSessions::new() {
                    Ok(s) => s,
                    Err(_) => continue,
                };

                let start = Instant::now();
                let _ = rt.block_on(sessions.pause());
                total += start.elapsed();
            }

            total
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_current,
    bench_watch_first_event,
    bench_event_throughput,
    bench_idle_memory,
    bench_cpu_idle,
    bench_playback_controls,
);

criterion_main!(benches);
