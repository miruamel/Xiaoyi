//! Memory STM (Short-Term Memory) cache.
//!
//! Path: `xiaoyi::memory::stm::cache`
//!
//! Layer hierarchy:
//! - 0: memory
//! - 1: stm
//! - 2: cache/buffer/sliding/recent
//! - 3: entry/eviction/policy
//!
//! Fast in-memory cache with TTL and LRU eviction for recent context.

use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;

#[derive(Debug, Clone)]
pub struct CacheEntry<V> {
    pub value: V,
    pub created_at: Instant,
    pub expires_at: Option<Instant>,
    pub access_count: u64,
}

#[derive(Debug, Clone)]
pub struct StmCache<K, V> {
    inner: Arc<RwLock<HashMap<K, CacheEntry<V>>>>,
    max_size: usize,
    default_ttl: Option<Duration>,
}

impl<K, V> StmCache<K, V>
where
    K: Eq + std::hash::Hash + Clone,
    V: Clone,
{
    pub fn new(max_size: usize) -> Self {
        Self {
            inner: Arc::new(RwLock::new(HashMap::new())),
            max_size,
            default_ttl: None,
        }
    }

    pub fn with_ttl(mut self, ttl: Duration) -> Self {
        self.default_ttl = Some(ttl);
        self
    }

    pub async fn get(&self, key: &K) -> Option<V> {
        let mut map = self.inner.write().await;
        if let Some(entry) = map.get_mut(key) {
            if entry.is_expired() {
                map.remove(key);
                return None;
            }
            entry.access_count += 1;
            Some(entry.value.clone())
        } else {
            None
        }
    }

    pub async fn insert(&self, key: K, value: V) {
        let mut map = self.inner.write().await;
        if map.len() >= self.max_size {
            self.evict_lru(&mut map);
        }
        let now = Instant::now();
        map.insert(key, CacheEntry {
            value,
            created_at: now,
            expires_at: self.default_ttl.map(|ttl| now + ttl),
            access_count: 0,
        });
    }

    fn evict_lru(&self, map: &mut HashMap<K, CacheEntry<V>>) {
        if let Some((k, _)) = map.iter().min_by_key(|(_, e)| e.access_count) {
            let k = k.clone();
            map.remove(&k);
        }
    }
}

impl<V> CacheEntry<V> {
    fn is_expired(&self) -> bool {
        self.expires_at.map(|e| Instant::now() > e).unwrap_or(false)
    }
}