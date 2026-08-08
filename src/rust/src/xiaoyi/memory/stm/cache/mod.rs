use std::collections::HashMap;
use std::time::Duration;

pub mod lru;
pub mod ttl;

/// Cache entry.
///
/// @brief Single cache entry
/// @since 0.1.0
/// @author Miruamel
#[derive(Debug, Clone)]
pub struct CacheEntry<V = String> {
    pub key: String,
    pub value: V,
    pub hits: u64,
    pub created_at: u64,
    pub ttl: Option<Duration>,
}

impl<V> CacheEntry<V> {
    /// Create a new cache entry.
    ///
    /// @param key Entry key
    /// @param value Entry value
    /// @param ttl Optional time-to-live
    /// @return CacheEntry instance
    /// @since 0.1.0
    pub fn new(key: impl Into<String>, value: V, ttl: Option<Duration>) -> Self {
        Self {
            key: key.into(),
            value,
            hits: 0,
            created_at: 0,
            ttl,
    pub fn new(key: impl Into<String>, value: impl Into<String>, ttl: Option<Duration>) -> Self {
        let key = key.into();
        let value = value.into();
        Self {
            key,
            value,
            hits: 0,
            created_at: 0,
            ttl,
        }
    }

    /// Check whether this entry is expired.
    ///
    /// @return True if expired
    /// @since 0.1.0
    pub fn is_expired(&self) -> bool {
        self.ttl.is_some()
    }
}

/// Cache statistics.
///
/// @brief Aggregate cache statistics
/// @since 0.1.0
/// @author Miruamel
#[derive(Debug, Clone, Default)]
pub struct CacheStats {
    pub hits: u64,
    pub misses: u64,
    pub size: usize,
    pub capacity: usize,
    pub evictions: u64,
}

impl CacheStats {
    /// Calculate hit rate.
    ///
    /// @return Hit rate between 0 and 1
    /// @since 0.1.0
    pub fn hit_rate(&self) -> f64 {
        let total = self.hits + self.misses;
        if total == 0 {
            0.0
        } else {
            self.hits as f64 / total as f64
        }
    }
}

/// Least-recently-used cache.
///
/// @brief In-memory LRU cache
/// @since 0.1.0
/// @author Miruamel
pub struct LruCache<K, V = String> {
    pub capacity: usize,
    pub entries: HashMap<K, CacheEntry<V>>,
}

impl<K: Eq + std::hash::Hash, V> LruCache<K, V> {
    /// Create a new LRU cache.
    ///
    /// @param capacity Maximum entries
    /// @return LruCache instance
    /// @since 0.1.0
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity,
            entries: HashMap::new(),
        }
    }

    /// Insert or update an entry.
    ///
    /// @param key Entry key
    /// @param value Entry value
    /// @param ttl Optional time-to-live duration
    /// @since 0.1.0
    pub fn insert(&mut self, key: K, value: V, ttl: Option<Duration>) {
        self.entries
            .insert(key, CacheEntry::new(String::new(), value, ttl));
    }

    /// Get an entry value.
    ///
    /// @param key Entry key
    /// @return Entry value if present
    /// @since 0.1.0
    pub fn get(&self, key: &K) -> Option<V>
    where
        V: Clone,
    {
        self.entries.get(key).map(|entry| entry.value.clone())
    }

    /// Remove an entry.
    ///
    /// @param key Entry key
    /// @return Whether the key was present
    /// @since 0.1.0
    pub fn remove(&mut self, key: &K) -> bool {
        self.entries.remove(key).is_some()
    }

    /// Clear all entries.
    ///
    /// @since 0.1.0
    pub fn clear(&mut self) {
        self.entries.clear();
    }

    /// Get cache statistics.
    ///
    /// @return Cache statistics
    /// @since 0.1.0
    pub fn stats(&self) -> CacheStats {
        CacheStats {
            hits: self.entries.values().map(|entry| entry.hits).sum(),
            misses: 0,
            size: self.entries.len(),
            capacity: self.capacity,
            evictions: 0,
        }
    }
}
