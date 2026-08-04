/**
 * Memory STM (Short-Term Memory) cache.
 *
 * Path: xiaoyi.memory.stm.cache
 *
 * Layer hierarchy:
 * - 0: memory
 * - 1: stm
 * - 2: cache/buffer/sliding/recent
 * - 3: entry/eviction/policy
 *
 * Fast in-memory cache with TTL and LRU eviction for recent context.
 */

export interface CacheEntry<V> {
  value: V;
  createdAt: number;
  expiresAt: number | null;
  accessCount: number;
}

export class StmCache<K, V> {
  private cache = new Map<K, CacheEntry<V>>();
  private order: K[] = [];
  private readonly maxSize: number;
  private readonly defaultTtl: number | null;

  constructor(maxSize: number, defaultTtl: number | null = null) {
    this.maxSize = maxSize;
    this.defaultTtl = defaultTtl;
  }

  get(key: K): V | undefined {
    const entry = this.cache.get(key);
    if (!entry) return undefined;

    if (entry.expiresAt !== null && Date.now() > entry.expiresAt) {
      this.cache.delete(key);
      this.order = this.order.filter((k) => k !== key);
      return undefined;
    }

    entry.accessCount++;
    // Move to end (MRU)
    this.order = this.order.filter((k) => k !== key);
    this.order.push(key);
    return entry.value;
  }

  insert(key: K, value: V): void {
    if (this.cache.has(key)) {
      this.cache.delete(key);
      this.order = this.order.filter((k) => k !== key);
    } else if (this.cache.size >= this.maxSize) {
      // Evict LRU
      const lru = this.order.shift();
      if (lru !== undefined) this.cache.delete(lru);
    }

    const now = Date.now();
    const expiresAt = this.defaultTtl !== null ? now + this.defaultTtl : null;
    this.cache.set(key, {
      value,
      createdAt: now,
      expiresAt,
      accessCount: 0,
    });
    this.order.push(key);
  }

  get size(): number {
    return this.cache.size;
  }

  clear(): void {
    this.cache.clear();
    this.order = [];
  }
}