//! # Property-Based Cache Tests
//!
//! Proptest property tests for `xiaoyi::LruCache`.
//!
//! @module tests::property_cache
//! @brief Property-based tests for LRU cache
//! @group Memory
//! @since 0.1.0
//! @author Miruamel
//! @see crate::memory::stm::cache::LruCache

use proptest::prelude::*;
use xiaoyi::{LruCache, CacheStats};

proptest! {
    #[test]
    fn test_insert_get_consistency(
        keys in prop::collection::vec("[a-z]{1,10}", 1..50),
        values in prop::collection::vec("[a-z]{1,20}", 1..50)
    ) {
        // Ensure unique keys by deduplicating
        let mut seen = std::collections::HashSet::new();
        let unique_pairs: Vec<_> = keys.iter().zip(values.iter())
            .filter(|(k, _)| seen.insert(k.clone()))
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();
        
        let cache = LruCache::new(1000);
        for (k, v) in &unique_pairs {
            cache.insert(k.clone(), v.clone(), None);
        }
        for (k, v) in &unique_pairs {
            let got: Option<String> = cache.get(k);
            prop_assert_eq!(got, Some(v.clone()));
        }
    }

    #[test]
    fn test_lru_eviction_order(
        keys in prop::collection::vec("[a-z]{1,5}", 10..100),
        values in prop::collection::vec("[a-z]{1,5}", 10..100),
        capacity in 5usize..20
    ) {
        // Ensure unique keys by deduplicating
        let mut seen = std::collections::HashSet::new();
        let unique_pairs: Vec<_> = keys.iter().zip(values.iter())
            .filter(|(k, _)| seen.insert(k.clone()))
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();
        
        let cache = LruCache::new(capacity);
        for (k, v) in &unique_pairs {
            cache.insert(k.clone(), v.clone(), None);
        }

        // Keys beyond capacity should be evicted (LRU)
        let len = unique_pairs.len();
        let evicted_count = len.saturating_sub(capacity);
        let mut found = 0;
        for k in &unique_pairs.iter().map(|(k, _)| k).cloned().collect::<Vec<_>>() {
            if cache.get(k).is_some() {
                found += 1;
            }
        }
        prop_assert!(found <= capacity);
    }

    #[test]
    fn test_capacity_bound(
        keys in prop::collection::vec("[a-z]{1,10}", 1..200),
        values in prop::collection::vec("[a-z]{1,10}", 1..200),
        capacity in 1usize..50
    ) {
        // Ensure unique keys
        let mut seen = std::collections::HashSet::new();
        let unique_pairs: Vec<_> = keys.iter().zip(values.iter())
            .filter(|(k, _)| seen.insert(k.clone()))
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();

        let cache = LruCache::new(capacity);
        for (k, v) in &unique_pairs {
            cache.insert(k.clone(), v.clone(), None);
        }
        let stats: CacheStats = cache.stats();
        prop_assert!(stats.size <= capacity as usize);
    }

    #[test]
    fn test_ttl_expiration(
        keys in prop::collection::vec("[a-z]{1,10}", 1..20),
        values in prop::collection::vec("[a-z]{1,10}", 1..20)
    ) {
        // Ensure unique keys
        let mut seen = std::collections::HashSet::new();
        let unique_pairs: Vec<_> = keys.iter().zip(values.iter())
            .filter(|(k, _)| seen.insert(k.clone()))
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();

        let cache = LruCache::new(100);
        for (k, v) in &unique_pairs {
            // Very short TTL - should expire almost immediately
            cache.insert(k.clone(), v.clone(), Some(std::time::Duration::from_millis(1)));
        }
        // Small delay
        std::thread::sleep(std::time::Duration::from_millis(10));
        for k in unique_pairs.iter().map(|(k, _)| k) {
            let got: Option<String> = cache.get(k);
            prop_assert_eq!(got, None);
        }
    }

    #[test]
    fn test_update_existing_key(
        key in "[a-z]{1,10}",
        values in prop::collection::vec("[a-z]{1,10}", 2..10)
    ) {
        let cache = LruCache::new(10);
        for v in &values {
            cache.insert(key.clone(), v.clone(), None);
        }
        let got: Option<String> = cache.get(&key);
        prop_assert_eq!(got, Some(values.last().unwrap().clone()));
    }

    #[test]
    fn test_remove_then_get(
        keys in prop::collection::vec("[a-z]{1,10}", 1..20),
        values in prop::collection::vec("[a-z]{1,10}", 1..20)
    ) {
        // Ensure unique keys
        let mut seen = std::collections::HashSet::new();
        let unique_pairs: Vec<_> = keys.iter().zip(values.iter())
            .filter(|(k, _)| seen.insert(k.clone()))
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();

        let cache = LruCache::new(100);
        for (k, v) in &unique_pairs {
            cache.insert(k.clone(), v.clone(), None);
        }
        if let Some((k, _)) = unique_pairs.first() {
            cache.remove(k);
            let got: Option<String> = cache.get(k);
            prop_assert_eq!(got, None);
        }
    }

    #[test]
    fn test_clear_removes_all(
        keys in prop::collection::vec("[a-z]{1,10}", 1..50),
        values in prop::collection::vec("[a-z]{1,10}", 1..50)
    ) {
        // Ensure unique keys
        let mut seen = std::collections::HashSet::new();
        let unique_pairs: Vec<_> = keys.iter().zip(values.iter())
            .filter(|(k, _)| seen.insert(k.clone()))
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();

        let cache = LruCache::new(100);
        for (k, v) in &unique_pairs {
            cache.insert(k.clone(), v.clone(), None);
        }
        cache.clear();
        let stats: CacheStats = cache.stats();
        prop_assert_eq!(stats.size, 0);
    }

    #[test]
    fn test_stats_hit_miss_counts(
        keys in prop::collection::vec("[a-z]{1,10}", 5..20),
        values in prop::collection::vec("[a-z]{1,10}", 5..20),
        extra_keys in prop::collection::vec("[a-z]{1,10}", 1..10)
    ) {
        // Ensure unique keys for inserted entries
        let mut seen = std::collections::HashSet::new();
        let unique_pairs: Vec<_> = keys.iter().zip(values.iter())
            .filter(|(k, _)| seen.insert(k.clone()))
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();

        // Ensure extra_keys don't overlap with keys
        let key_set: std::collections::HashSet<_> = unique_pairs.iter().map(|(k, _)| k).collect();
        let miss_keys: Vec<_> = extra_keys.iter()
            .filter(|k| !key_set.contains(*k))
            .cloned()
            .collect();

        let cache = LruCache::new(100);
        for (k, v) in &unique_pairs {
            cache.insert(k.clone(), v.clone(), None);
        }
        // Hits
        for k in unique_pairs.iter().map(|(k, _)| k) {
            let _: Option<String> = cache.get(k);
        }
        // Misses (keys that definitely don't exist)
        for k in &miss_keys {
            let _: Option<String> = cache.get(k);
        }
        let stats: CacheStats = cache.stats();
        prop_assert_eq!(stats.hits, unique_pairs.len());
        prop_assert_eq!(stats.misses, miss_keys.len());
    }
}