//! # Short-Term Memory
//!
//! `stm` provides in-memory caching and buffering for hot agent state.
//!
//! Path: `xiaoyi::memory::stm`
//!
//! - Layer 0: `memory`
//! - Layer 1: `stm` — short-term memory.
//! - Layer 2: `cache` — LRU/TLFU cache.
//! - Layer 3: `buffer` — ring buffer for streams.
//!
//! @module memory::stm
//! @brief In-memory cache and buffer for hot state
//! @group Memory
//! @since 0.1.0
//! @author Miruamel
//! @see crate::memory
//! @see crate::memory::stm::cache
pub mod cache;

// Re-exports from cache
pub use cache::{LruCache as StmCache, CacheEntry, CacheStats, LruCache};