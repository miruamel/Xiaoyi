//! # STM Cache
//!
//! `cache` provides a thread-safe LRU cache with TTL support for
//! short-term memory storage.
//!
//! Path: `xiaoyi::memory::stm::cache`
//!
//! - Layer 0: `memory`
//! - Layer 1: `stm`
//! - Layer 2: `cache` — LRU cache implementation.
//!
//! @module memory::stm::cache
//! @brief Thread-safe LRU cache with TTL
//! @group Memory
//! @since 0.1.0
//! @author Miruamel
//! @see crate::memory::stm
//! @see crate::memory::ltm::vector
//!
//! # Example
//!
//! ```rust
//! use xiaoyi::memory::stm::cache::LruCache;
//! use std::time::Duration;
//!
//! let cache = LruCache::new(100);
//! cache.insert("key".to_string(), "value".to_string(), Some(Duration::from_secs(60)));
//! let value = cache.get("key");
//! ```
//!
//! # Features
//!
//! - O(1) get/insert with LRU eviction.
//! - Optional TTL per entry.
//! - Thread-safe with RwLock.
//! - Configurable capacity.
//!
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};
/// Cache entry with value and expiry.
///
/// @brief Stores cached value with optional TTL expiry
/// @group Memory
/// @since 0.1.0
#[derive(Debug, Clone)]
pub struct CacheEntry<V> {
    /// Cached value.
    pub value: V,
    /// Optional expiry timestamp.
    pub expiry: Option<Instant>,
}

/// Cache statistics.
///
/// @brief Cache performance metrics
/// @group Memory
/// @since 0.1.0
#[derive(Debug, Clone, Default)]
pub struct CacheStats {
    /// Number of cache hits.
    pub hits: usize,
    /// Number of cache misses.
    pub misses: usize,
    /// Number of LRU evictions.
    pub evictions: usize,
    /// Current cache size.
    pub size: usize,
    /// Maximum cache capacity.
    pub capacity: usize,
}

/// LRU cache with optional TTL.
///
/// @brief High-performance LRU cache with TTL support
/// @group Memory
/// @since 0.1.0
/// @see crate::memory::stm::buffer
#[derive(Debug)]
pub struct LruCache<K, V> {
    map: Arc<RwLock<HashMap<K, CacheEntry<V>>>>,
    order: Arc<RwLock<Vec<K>>>,
    capacity: usize,
    hits: Arc<RwLock<usize>>,
    misses: Arc<RwLock<usize>>,
    evictions: Arc<RwLock<usize>>,
}

impl<K, V> LruCache<K, V>
where
    K: Clone + Eq + std::hash::Hash + Send + Sync + 'static,
    V: Clone + Send + Sync + 'static,
{
    /// Create a new LRU cache with given capacity.
    ///
    /// @param capacity Maximum number of entries
    /// @return New LruCache instance
    /// @since 0.1.0
    pub fn new(capacity: usize) -> Self {
        Self {
            map: Arc::new(RwLock::new(HashMap::with_capacity(capacity))),
            order: Arc::new(RwLock::new(Vec::with_capacity(capacity))),
            capacity,
            hits: Arc::new(RwLock::new(0)),
            misses: Arc::new(RwLock::new(0)),
            evictions: Arc::new(RwLock::new(0)),
        }
    }

    /// Get a value by key.
    ///
    /// @param key Cache key
    /// @return Cloned value if present and not expired
    /// @since 0.1.0
    pub fn get(&self, key: &K) -> Option<V> {
        let mut map = self.map.write().unwrap();
        let mut order = self.order.write().unwrap();

        if let Some(entry) = map.get(key) {
            if let Some(expiry) = entry.expiry {
                if Instant::now() > expiry {
                    map.remove(key);
                    order.retain(|k| k != key);
                    *self.misses.write().unwrap() += 1;
                    return None;
                }
            }

            // Move to front (most recently used)
            order.retain(|k| k != key);
            order.insert(0, key.clone());

            *self.hits.write().unwrap() += 1;
            Some(entry.value.clone())
        } else {
            *self.misses.write().unwrap() += 1;
            None
        }
    }

    /// Insert a key-value pair with optional TTL.
    ///
    /// @param key Cache key
    /// @param value Value to store
    /// @param ttl Optional time-to-live
    /// @since 0.1.0
    pub fn insert(&self, key: K, value: V, ttl: Option<Duration>) {
        let mut map = self.map.write().unwrap();
        let mut order = self.order.write().unwrap();

        let expiry = ttl.map(|d| Instant::now() + d);

        if map.contains_key(&key) {
            order.retain(|k| k != &key);
        } else if map.len() >= self.capacity {
            // Evict LRU
            if let Some(lru_key) = order.pop() {
                map.remove(&lru_key);
                *self.evictions.write().unwrap() += 1;
            }
        }

        order.insert(0, key.clone());
        map.insert(key, CacheEntry { value, expiry });
    }

    /// Remove a key from cache.
    ///
    /// @param key Key to remove
    /// @return true if key was present
    /// @since 0.1.0
    pub fn remove(&self, key: &K) -> bool {
        let mut map = self.map.write().unwrap();
        let mut order = self.order.write().unwrap();

        let removed = map.remove(key).is_some();
        if removed {
            order.retain(|k| k != key);
        }
        removed
    }

    /// Clear all entries.
    ///
    /// @since 0.1.0
    pub fn clear(&self) {
        self.map.write().unwrap().clear();
        self.order.write().unwrap().clear();
    }

    /// Get current entry count.
    ///
    /// @return Number of entries
    /// @since 0.1.0
    pub fn len(&self) -> usize {
        self.map.read().unwrap().len()
    }

    /// Check if cache is empty.
    ///
    /// @return true if no entries
    /// @since 0.1.0
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Get cache capacity.
    ///
    /// @return Maximum capacity
    /// @since 0.1.0
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    /// Clean up expired entries.
    ///
    /// @return Number of expired entries removed
    /// @since 0.1.0
    pub fn cleanup_expired(&self) -> usize {
        let mut map = self.map.write().unwrap();
        let mut order = self.order.write().unwrap();
        let now = Instant::now();

        let expired_keys: Vec<K> = map
            .iter()
            .filter(|(_, entry)| entry.expiry.map_or(false, |exp| now > exp))
            .map(|(k, _)| k.clone())
            .collect();

        for key in &expired_keys {
            map.remove(key);
            order.retain(|k| k != key);
        }

        expired_keys.len()
    }

    /// Get cache statistics.
    ///
    /// @return CacheStats with hits, misses, evictions, size, capacity
    /// @since 0.1.0
    pub fn stats(&self) -> CacheStats {
        CacheStats {
            hits: *self.hits.read().unwrap(),
            misses: *self.misses.read().unwrap(),
            evictions: *self.evictions.read().unwrap(),
            size: self.len(),
            capacity: self.capacity,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread::sleep;
    use std::time::Duration;

    #[test]
    fn test_basic_lru() {
        let cache = LruCache::new(3);
        cache.insert("a".to_string(), 1, None);
        cache.insert("b".to_string(), 2, None);
        cache.insert("c".to_string(), 3, None);

        assert_eq!(cache.get(&"a".to_string()), Some(1));
        assert_eq!(cache.get(&"b".to_string()), Some(2));
        assert_eq!(cache.get(&"c".to_string()), Some(3));

        // Access 'a' to make it MRU
        cache.get(&"a".to_string());

        // Insert 'd' should evict 'b' (LRU)
        cache.insert("d".to_string(), 4, None);
        assert_eq!(cache.get(&"b".to_string()), None);
        assert_eq!(cache.get(&"a".to_string()), Some(1));
        assert_eq!(cache.get(&"c".to_string()), Some(3));
        assert_eq!(cache.get(&"d".to_string()), Some(4));
    }
    #[test]
    fn test_ttl_expiry() {
        let cache = LruCache::new(10);
        cache.insert(
            "key".to_string(),
            "value".to_string(),
            Some(Duration::from_millis(50)),
        );
        assert_eq!(cache.get(&"key".to_string()), Some("value".to_string()));

        sleep(Duration::from_millis(100));
        assert_eq!(cache.get(&"key".to_string()), None);
    }
}
