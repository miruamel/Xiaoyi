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
  private readonly _maxSize: number;
  public readonly maxSize: number;
  private accessOrder: string[] = []; // most recent first

  /**
   * Create LRU cache.
   *
   * @param options - Cache options
   * @since 0.1.0
   */
  constructor(options: { maxSize?: number } = {}) {
    this._maxSize = options.maxSize ?? 1000;
    this.maxSize = this._maxSize;
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

    // Move to front (most recent)
    this.moveToFront(key);
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
    if (this._maxSize === 0) return; // No storage allowed

    const exists = this.cache.has(key);

    if (!exists && this.cache.size >= this._maxSize) {
      this.evict();
    }

    this.cache.set(key, {
      value,
      lastAccess: Date.now(),
      accessCount: exists ? this.cache.get(key)!.accessCount + 1 : 1,
    });

    // Move to front (most recent)
    this.moveToFront(key);
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
    const deleted = this.cache.delete(key);
    if (deleted) {
      this.removeFromOrder(key);
    }
    return deleted;
  }

  /**
   * Clear all entries.
   *
   * @since 0.1.0
   */
  clear(): void {
    this.cache.clear();
    this.accessOrder = [];
  }

  /**
   * Get cache size.
   *
   * @returns Number of entries
   * @since 0.1.0
   */
  get size(): number {
    return this.cache.size;
  }

  /**
   * Get all keys in LRU order (most recent first).
   *
   * @returns Array of keys
   * @since 0.1.0
   */
  keys(): string[] {
    return [...this.accessOrder];
  }

  /**
   * Get all entries in LRU order (most recent first).
   *
   * @returns Array of [key, value] pairs
   * @since 0.1.0
   */
  entries(): [string, T][] {
    return this.accessOrder.map((k) => [k, this.cache.get(k)!.value]);
  }

  /**
   * Get all values in LRU order (most recent first).
   *
   * @returns Array of values
   * @since 0.1.0
   */
  values(): T[] {
    return this.accessOrder.map((k) => this.cache.get(k)!.value);
  }

  /**
   * Move key to front (most recent).
   */
  private moveToFront(key: string): void {
    this.removeFromOrder(key);
    this.accessOrder.unshift(key);
  }

  /**
   * Remove key from access order.
   */
  private removeFromOrder(key: string): void {
    const idx = this.accessOrder.indexOf(key);
    if (idx >= 0) {
      this.accessOrder.splice(idx, 1);
    }
  }

  /**
   * Evict least recently used entry.
   *
   * @since 0.1.0
   */
  private evict(): void {
    if (this.accessOrder.length > 0) {
      const lruKey = this.accessOrder.pop()!;
      this.cache.delete(lruKey);
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
      maxSize: this._maxSize,
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