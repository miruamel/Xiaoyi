/**
 * # STM Cache
 *
 * `cache` provides caching layer for STM with LRU eviction.
 *
 * Path: `xiaoyi.memory.stm.cache`
 *
 * @module memory.stm.cache
 * @brief LRU cache for STM entries
 * @group Memory
 * @since 0.1.0
 * @author Miruamel
 * @see memory.stm
 * @see memory.stm.context
 * @security Cache may contain sensitive conversation data. Clear on logout.
 */
import { StmEntry } from "..";

/**
 * Cache entry with access tracking.
 *
 * @brief Cached STM entry with LRU metadata
 * @group Memory
 * @since 0.1.0
 */
export interface CacheEntry<T = StmEntry> {
  /** Cached value. */
  value: T;
  /** Last access timestamp. */
  lastAccess: number;
  /** Access count. */
  accessCount: number;
}

/**
 * LRU cache implementation.
 *
 * @brief Least-recently-used cache for STM
 * @group Memory
 * @since 0.1.0
 * @threadsafe
 * @example
 * ```typescript
 * const cache = new LruCache<StmEntry>({ maxSize: 1000 });
 * cache.set("key", entry);
 * const entry = cache.get("key");
 * ```
 */
export class LruCache<T = StmEntry> {
  private cache = new Map<string, CacheEntry<T>>();
  private readonly maxSize: number;

  /**
   * Create LRU cache.
   *
   * @param options - Cache options
   * @since 0.1.0
   */
  constructor(options: { maxSize: number }) {
    this.maxSize = options.maxSize;
  }

  /**
   * Get value by key.
   *
   * @param key - Cache key
   * @returns Value or undefined
   * @since 0.1.0
   */
  get(key: string): T | undefined {
    const entry = this.cache.get(key);
    if (!entry) return undefined;

    entry.lastAccess = Date.now();
    entry.accessCount++;
    return entry.value;
  }

  /**
   * Set value.
   *
   * @param key - Cache key
   * @param value - Value to cache
   * @since 0.1.0
   */
  set(key: string, value: T): void {
    if (this.cache.size >= this.maxSize && !this.cache.has(key)) {
      this.evict();
    }

    this.cache.set(key, {
      value,
      lastAccess: Date.now(),
      accessCount: 1,
    });
  }

  /**
   * Check if key exists.
   *
   * @param key - Cache key
   * @returns true if exists
   * @since 0.1.0
   */
  has(key: string): boolean {
    return this.cache.has(key);
  }

  /**
   * Delete key.
   *
   * @param key - Cache key
   * @returns true if deleted
   * @since 0.1.0
   */
  delete(key: string): boolean {
    return this.cache.delete(key);
  }

  /**
   * Clear all entries.
   *
   * @since 0.1.0
   */
  clear(): void {
    this.cache.clear();
  }

  /**
   * Get cache size.
   *
   * @returns Number of entries
   * @since 0.1.0
   */
  size(): number {
    return this.cache.size;
  }

  /**
   * Get all keys.
   *
   * @returns Array of keys
   * @since 0.1.0
   */
  keys(): string[] {
    return Array.from(this.cache.keys());
  }

  /**
   * Evict least recently used entry.
   *
   * @since 0.1.0
   */
  private evict(): void {
    let oldestKey: string | null = null;
    let oldestTime = Infinity;

    for (const [key, entry] of this.cache) {
      if (entry.lastAccess < oldestTime) {
        oldestTime = entry.lastAccess;
        oldestKey = key;
      }
    }

    if (oldestKey) {
      this.cache.delete(oldestKey);
    }
  }

  /**
   * Get cache statistics.
   *
   * @returns Cache stats
   * @since 0.1.0
   */
  stats(): { size: number; maxSize: number; hitRate: number } {
    let totalAccess = 0;
    for (const entry of this.cache.values()) {
      totalAccess += entry.accessCount;
    }

    return {
      size: this.cache.size,
      maxSize: this.maxSize,
      hitRate: totalAccess > 0 ? totalAccess / this.cache.size : 0,
    };
  }
}

/**
 * Create default STM cache.
 *
 * @returns Default LRU cache
 * @since 0.1.0
 * @group Memory
 */
export function createStmCache(): LruCache<StmEntry> {
  return new LruCache({ maxSize: 1000 });
}