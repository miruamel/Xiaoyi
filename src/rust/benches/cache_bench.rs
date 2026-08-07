//! # LRU Cache Benchmarks
//!
//! Criterion benchmarks for `xiaoyi::memory::stm::cache::LruCache`.
//!
//! @module benches::cache_bench
//! @brief Criterion benchmarks for LRU cache
//! @group Memory
//! @since 0.1.0
//! @author Miruamel
//! @see crate::memory::stm::cache::LruCache

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId, Throughput};
use xiaoyi::memory::stm::cache::LruCache;

fn bench_insert(c: &mut Criterion) {
    let mut group = c.benchmark_group("insert");
    for capacity in [100, 1000, 10000].iter() {
        group.throughput(Throughput::Elements(*capacity as u64));
        group.bench_with_input(BenchmarkId::from_parameter(capacity), capacity, |b, &cap| {
            b.iter(|| {
                let cache = LruCache::new(cap);
                for i in 0..cap {
                    cache.insert(
                        black_box(format!("key{}", i)),
                        black_box(format!("value{}", i)),
                        None,
                    );
                }
            });
        });
    }
    group.finish();
}

fn bench_get(c: &mut Criterion) {
    let mut group = c.benchmark_group("get");
    for capacity in [100, 1000, 10000].iter() {
        group.throughput(Throughput::Elements(*capacity as u64));
        group.bench_with_input(BenchmarkId::from_parameter(capacity), capacity, |b, &cap| {
            let cache = LruCache::new(cap);
            for i in 0..cap {
                cache.insert(format!("key{}", i), format!("value{}", i), None);
            }
            b.iter(|| {
                for i in 0..cap {
                    let _: Option<String> = cache.get(black_box(&format!("key{}", i)));
                }
            });
        });
    }
    group.finish();
}

fn bench_get_miss(c: &mut Criterion) {
    let mut group = c.benchmark_group("get_miss");
    for capacity in [100, 1000, 10000].iter() {
        group.throughput(Throughput::Elements(*capacity as u64));
        group.bench_with_input(BenchmarkId::from_parameter(capacity), capacity, |b, &cap| {
            let cache = LruCache::new(cap);
            for i in 0..cap {
                cache.insert(format!("key{}", i), format!("value{}", i), None);
            }
            b.iter(|| {
                for i in 0..cap {
                    let _: Option<String> = cache.get(black_box(&format!("missing{}", i)));
                }
            });
        });
    }
    group.finish();
}

fn bench_mixed_workload(c: &mut Criterion) {
    let mut group = c.benchmark_group("mixed");
    for capacity in [1000, 10000].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(capacity), capacity, |b, &cap| {
            let cache = LruCache::new(cap);
            // Pre-populate
            for i in 0..cap {
                cache.insert(format!("key{}", i), format!("value{}", i), None);
            }
            b.iter(|| {
                // 70% reads, 30% writes
                for i in 0..cap {
                    if i % 10 < 7 {
                        let _: Option<String> = cache.get(black_box(&format!("key{}", i)));
                    } else {
                        cache.insert(
                            black_box(format!("newkey{}", i)),
                            black_box(format!("newval{}", i)),
                            None,
                        );
                    }
                }
            });
        });
    }
    group.finish();
}

fn bench_lru_eviction(c: &mut Criterion) {
    let mut group = c.benchmark_group("lru_eviction");
    for capacity in [100, 1000].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(capacity), capacity, |b, &cap| {
            b.iter(|| {
                let cache = LruCache::new(cap);
                // Insert more than capacity to trigger eviction
                for i in 0..cap * 2 {
                    cache.insert(
                        black_box(format!("key{}", i)),
                        black_box(format!("value{}", i)),
                        None,
                    );
                }
            });
        });
    }
    group.finish();
}

fn bench_concurrent_access(c: &mut Criterion) {
    use std::sync::Arc;
    use std::thread;

    let mut group = c.benchmark_group("concurrent");
    for threads in [2, 4, 8].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(threads), threads, |b, &n_threads| {
            b.iter(|| {
                let cache = Arc::new(LruCache::new(10000));
                let mut handles = vec![];
                let ops_per_thread = 10000 / n_threads;

                for t in 0..n_threads {
                    let cache = cache.clone();
                    handles.push(thread::spawn(move || {
                        for i in 0..ops_per_thread {
                            let idx = t * ops_per_thread + i;
                            if idx % 2 == 0 {
                                cache.insert(
                                    format!("key{}", idx),
                                    format!("value{}", idx),
                                    None,
                                );
                            } else {
                                let _: Option<String> = cache.get(&format!("key{}", idx));
                            }
                        }
                    }));
                }
                for h in handles {
                    h.join().unwrap();
                }
            });
        });
    }
    group.finish();
}

fn bench_ttl(c: &mut Criterion) {
    let mut group = c.benchmark_group("ttl");
    for capacity in [1000, 10000].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(capacity), capacity, |b, &cap| {
            b.iter(|| {
                let cache = LruCache::new(cap);
                for i in 0..cap {
                    cache.insert(
                        black_box(format!("key{}", i)),
                        black_box(format!("value{}", i)),
                        black_box(Some(60)), // 60 second TTL
                    );
                }
            });
        });
    }
    group.finish();
}

criterion_group!(
    benches,
    bench_insert,
    bench_get,
    bench_get_miss,
    bench_mixed_workload,
    bench_lru_eviction,
    bench_concurrent_access,
    bench_ttl
);
criterion_main!(benches);