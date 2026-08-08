//! # Memory STM Tests
//!
//! Tests for `xiaoyi::LruCache` and related types.
//!
//! @module tests::memory_stm
//! @brief Unit tests for LRU cache
//! @group Memory
//! @since 0.1.0
//! @author Miruamel
//! @see crate::memory::stm

use pretty_assertions::assert_eq;
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};
use xiaoyi::{CacheEntry, CacheStats, LruCache};

#[test]
fn test_lru_cache_new() {
    let cache: LruCache<String, String> = LruCache::new(10);
    assert_eq!(cache.capacity(), 10);
    assert_eq!(cache.len(), 0);
    assert!(cache.is_empty());
}

#[test]
fn test_lru_cache_capacity_zero() {
    let cache: LruCache<String, String> = LruCache::new(0);
    assert_eq!(cache.capacity(), 0);
    cache.insert("key".to_string(), "value".to_string(), None);
    // With capacity 0, insertion still works (no eviction possible)
    assert_eq!(cache.len(), 1);
    assert_eq!(cache.get(&"key".to_string()), Some("value".to_string()));
}

#[test]
fn test_lru_cache_insert_and_get() {
    let cache = LruCache::new(10);
    cache.insert("a".to_string(), "1".to_string(), None);
    cache.insert("b".to_string(), "2".to_string(), None);

    assert_eq!(cache.get(&"a".to_string()), Some("1".to_string()));
    assert_eq!(cache.get(&"b".to_string()), Some("2".to_string()));
    assert_eq!(cache.get(&"c".to_string()), None);
    assert_eq!(cache.len(), 2);
}

#[test]
fn test_lru_cache_eviction() {
    let cache = LruCache::new(2);
    cache.insert("a".to_string(), "1".to_string(), None);
    cache.insert("b".to_string(), "2".to_string(), None);
    cache.insert("c".to_string(), "3".to_string(), None); // Should evict 'a'

    assert_eq!(cache.len(), 2);
    assert_eq!(cache.get(&"a".to_string()), None); // Evicted
    assert_eq!(cache.get(&"b".to_string()), Some("2".to_string()));
    assert_eq!(cache.get(&"c".to_string()), Some("3".to_string()));
}

#[test]
fn test_lru_cache_lru_order() {
    let cache = LruCache::new(3);
    cache.insert("a".to_string(), "1".to_string(), None);
    cache.insert("b".to_string(), "2".to_string(), None);
    cache.insert("c".to_string(), "3".to_string(), None);

    // Access 'a' to make it recently used
    assert_eq!(cache.get(&"a".to_string()), Some("1".to_string()));

    // Insert 'd' - should evict 'b' (least recently used)
    cache.insert("d".to_string(), "4".to_string(), None);

    assert_eq!(cache.get(&"a".to_string()), Some("1".to_string()));
    assert_eq!(cache.get(&"b".to_string()), None); // Evicted
    assert_eq!(cache.get(&"c".to_string()), Some("3".to_string()));
    assert_eq!(cache.get(&"d".to_string()), Some("4".to_string()));
}

#[test]
fn test_lru_cache_update_existing() {
    let cache = LruCache::new(10);
    cache.insert("key".to_string(), "old".to_string(), None);
    assert_eq!(cache.get(&"key".to_string()), Some("old".to_string()));

    cache.insert("key".to_string(), "new".to_string(), None);
    assert_eq!(cache.get(&"key".to_string()), Some("new".to_string()));
    assert_eq!(cache.len(), 1);
}

#[test]
fn test_lru_cache_ttl_expiry() {
    let cache = LruCache::new(10);
    cache.insert(
        "key".to_string(),
        "value".to_string(),
        Some(Duration::from_millis(50)),
    );

    assert_eq!(cache.get(&"key".to_string()), Some("value".to_string()));

    std::thread::sleep(Duration::from_millis(100));
    assert_eq!(cache.get(&"key".to_string()), None); // Expired
}

#[test]
fn test_lru_cache_ttl_no_expiry() {
    let cache = LruCache::new(10);
    cache.insert("key".to_string(), "value".to_string(), None); // No TTL

    std::thread::sleep(Duration::from_millis(100));
    assert_eq!(cache.get(&"key".to_string()), Some("value".to_string())); // Still there
}

#[test]
fn test_lru_cache_remove() {
    let cache = LruCache::new(10);
    cache.insert("a".to_string(), "1".to_string(), None);
    cache.insert("b".to_string(), "2".to_string(), None);

    assert_eq!(cache.remove(&"a".to_string()), true);
    assert_eq!(cache.len(), 1);
    assert_eq!(cache.get(&"a".to_string()), None);
    assert_eq!(cache.get(&"b".to_string()), Some("2".to_string()));
    assert_eq!(cache.remove(&"missing".to_string()), false);
}

#[test]
fn test_lru_cache_clear() {
    let cache = LruCache::new(10);
    cache.insert("a".to_string(), "1".to_string(), None);
    cache.insert("b".to_string(), "2".to_string(), None);

    cache.clear();
    assert_eq!(cache.len(), 0);
    assert!(cache.is_empty());
    assert_eq!(cache.get(&"a".to_string()), None);
}

#[test]
fn test_lru_cache_stats() {
    let cache = LruCache::new(10);
    cache.insert("a".to_string(), "1".to_string(), None);
    cache.get(&"a".to_string()); // hit
    cache.get(&"missing".to_string()); // miss
    cache.get(&"missing2".to_string()); // miss

    let stats = cache.stats();
    assert_eq!(stats.hits, 1);
    assert_eq!(stats.misses, 2);
    assert_eq!(stats.size, 1); // entry_count -> size
    assert_eq!(stats.evictions, 0);
}

#[test]
fn test_lru_cache_thread_safety() {
    let cache = Arc::new(LruCache::new(1000));
    let mut handles = vec![];

    // Spawn multiple writer threads
    for i in 0..10 {
        let cache = Arc::clone(&cache);
        handles.push(thread::spawn(move || {
            for j in 0..100 {
                let key = format!("{}-{}", i, j);
                cache.insert(key.clone(), j.to_string(), None);
            }
        }));
    }

    for h in handles {
        h.join().unwrap();
    }

    // Verify some values
    assert!(cache.len() > 0);
    assert!(cache.len() <= 1000);
}

#[test]
fn test_lru_cache_concurrent_read_write() {
    let cache = Arc::new(LruCache::new(1000));
    let cache_read = Arc::clone(&cache);
    let cache_write = Arc::clone(&cache);

    let read_handle = thread::spawn(move || {
        for _ in 0..1000 {
            let _ = cache_read.get(&"any".to_string());
        }
    });

    let write_handle = thread::spawn(move || {
        for i in 0..1000 {
            cache_write.insert(format!("key{}", i), i.to_string(), None);
        }
    });

    read_handle.join().unwrap();
    write_handle.join().unwrap();
}

#[test]
fn test_cache_entry() {
    let entry = CacheEntry {
        value: "value".to_string(),
        expiry: Some(Instant::now() + Duration::from_secs(60)),
    };
    assert_eq!(entry.value, "value");
    assert!(entry.expiry.is_some());

    let expired = CacheEntry {
        value: "value".to_string(),
        expiry: Some(Instant::now() - Duration::from_millis(10)),
    };
    assert!(expired.expiry.map_or(false, |e| e < Instant::now()));

    let no_ttl = CacheEntry {
        value: "value".to_string(),
        expiry: None,
    };
    assert!(no_ttl.expiry.is_none());
}

#[test]
fn test_cache_entry_debug() {
    let entry = CacheEntry {
        value: "test".to_string(),
        expiry: None,
    };
    let debug = format!("{:?}", entry);
    assert!(debug.contains("test"));
}

#[test]
fn test_cache_stats_default() {
    let stats = CacheStats::default();
    assert_eq!(stats.hits, 0);
    assert_eq!(stats.misses, 0);
    assert_eq!(stats.size, 0); // entry_count -> size
    assert_eq!(stats.evictions, 0);
    assert_eq!(stats.capacity, 0);
}

#[test]
fn test_lru_cache_large_capacity() {
    let cache = LruCache::new(10000);
    for i in 0..5000 {
        cache.insert(format!("key{}", i), i.to_string(), None);
    }
    assert_eq!(cache.len(), 5000);

    // Add more to test eviction
    for i in 5000..15000 {
        cache.insert(format!("key{}", i), i.to_string(), None);
    }
    assert_eq!(cache.len(), 10000);
}
