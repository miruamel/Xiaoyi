import { describe, it, expect, vi, beforeEach } from "vitest";
import { LruCache, CacheEntry, createStmCache, StmEntry } from "../../src/xiaoyi/memory/stm/cache";

describe("memory/stm/cache", () => {
  describe("LruCache", () => {
    let cache: LruCache<string>;

    beforeEach(() => {
      cache = new LruCache<string>({ maxSize: 3 });
    });

    describe("constructor", () => {
      it("should create cache with default maxSize", () => {
        const defaultCache = new LruCache<string>();
        expect(defaultCache).toBeInstanceOf(LruCache);
      });

      it("should create cache with custom maxSize", () => {
        const customCache = new LruCache<string>({ maxSize: 100 });
        expect(customCache).toBeInstanceOf(LruCache);
      });

      it("should start empty", () => {
        expect(cache.size).toBe(0);
        expect(cache.has("any")).toBe(false);
      });
    });

    describe("set / get", () => {
      it("should store and retrieve value", () => {
        cache.set("key1", "value1");

        expect(cache.get("key1")).toBe("value1");
        expect(cache.size).toBe(1);
      });

      it("should return undefined for missing key", () => {
        expect(cache.get("missing")).toBeUndefined();
      });

      it("should update existing key", () => {
        cache.set("key1", "value1");
        cache.set("key1", "value2");

        expect(cache.get("key1")).toBe("value2");
        expect(cache.size).toBe(1);
      });

      it("should track access count", () => {
        cache.set("key1", "value1");
        cache.get("key1");
        cache.get("key1");

        const entry = (cache as any).cache.get("key1");
        expect(entry.accessCount).toBe(3); // 1 set + 2 gets
      });

      it("should update lastAccess on get", () => {
        cache.set("key1", "value1");
        const firstAccess = (cache as any).cache.get("key1").lastAccess;

        // Small delay to ensure different timestamp
        const start = Date.now();
        while (Date.now() === start) {}

        cache.get("key1");
        const secondAccess = (cache as any).cache.get("key1").lastAccess;

        expect(secondAccess).toBeGreaterThan(firstAccess);
      });
    });

    describe("has", () => {
      it("should return true for existing key", () => {
        cache.set("key1", "value1");
        expect(cache.has("key1")).toBe(true);
      });

      it("should return false for missing key", () => {
        expect(cache.has("missing")).toBe(false);
      });

      it("should not count as access", () => {
        cache.set("key1", "value1");
        const beforeAccess = (cache as any).cache.get("key1").accessCount;

        cache.has("key1");

        const afterAccess = (cache as any).cache.get("key1").accessCount;
        expect(afterAccess).toBe(beforeAccess);
      });
    });

    describe("delete", () => {
      it("should remove key and return true", () => {
        cache.set("key1", "value1");
        const result = cache.delete("key1");

        expect(result).toBe(true);
        expect(cache.has("key1")).toBe(false);
        expect(cache.size).toBe(0);
      });

      it("should return false for missing key", () => {
        const result = cache.delete("missing");
        expect(result).toBe(false);
      });
    });

    describe("clear", () => {
      it("should remove all entries", () => {
        cache.set("key1", "value1");
        cache.set("key2", "value2");
        cache.clear();

        expect(cache.size).toBe(0);
        expect(cache.has("key1")).toBe(false);
        expect(cache.has("key2")).toBe(false);
      });

      it("should work on empty cache", () => {
        cache.clear();
        expect(cache.size).toBe(0);
      });
    });

    describe("LRU eviction", () => {
      it("should evict least recently used when at capacity", () => {
        cache.set("a", "1");
        cache.set("b", "2");
        cache.set("c", "3"); // cache full: a, b, c

        cache.set("d", "4"); // should evict 'a' (least recently used)

        expect(cache.has("a")).toBe(false);
        expect(cache.has("b")).toBe(true);
        expect(cache.has("c")).toBe(true);
        expect(cache.has("d")).toBe(true);
        expect(cache.size).toBe(3);
      });

      it("should not evict recently accessed items", () => {
        cache.set("a", "1");
        cache.set("b", "2");
        cache.set("c", "3");

        cache.get("a"); // access 'a' making it most recent
        cache.set("d", "4"); // should evict 'b' (least recent)

        expect(cache.has("a")).toBe(true);
        expect(cache.has("b")).toBe(false);
        expect(cache.has("c")).toBe(true);
        expect(cache.has("d")).toBe(true);
      });

      it("should handle get after eviction", () => {
        cache.set("a", "1");
        cache.set("b", "2");
        cache.set("c", "3");

        cache.get("a");
        cache.set("d", "4"); // evicts b

        expect(cache.get("a")).toBe("1");
        expect(cache.get("d")).toBe("4");
        expect(cache.get("b")).toBeUndefined();
      });
    });

    describe("size property", () => {
      it("should return current size", () => {
        expect(cache.size).toBe(0);
        cache.set("a", "1");
        expect(cache.size).toBe(1);
        cache.set("b", "2");
        expect(cache.size).toBe(2);
        cache.delete("a");
        expect(cache.size).toBe(1);
      });

      it("should not exceed maxSize", () => {
        cache.set("a", "1");
        cache.set("b", "2");
        cache.set("c", "3");
        cache.set("d", "4"); // evicts one
        cache.set("e", "5"); // evicts another

        expect(cache.size).toBeLessThanOrEqual(3);
      });
    });

    describe("entries / keys / values", () => {
      it("should iterate entries in LRU order (most recent first)", () => {
        cache.set("a", "1");
        cache.set("b", "2");
        cache.set("c", "3");
        cache.get("a"); // a becomes most recent

        const entries = Array.from(cache.entries());
        expect(entries[0][0]).toBe("a"); // most recent
        expect(entries[1][0]).toBe("c");
        expect(entries[2][0]).toBe("b"); // least recent
      });

      it("should return keys", () => {
        cache.set("a", "1");
        cache.set("b", "2");

        const keys = Array.from(cache.keys());
        expect(keys).toContain("a");
        expect(keys).toContain("b");
      });

      it("should return values", () => {
        cache.set("a", "1");
        cache.set("b", "2");

        const values = Array.from(cache.values());
        expect(values).toContain("1");
        expect(values).toContain("2");
      });
    });

    describe("with generics", () => {
      it("should work with object values", () => {
        const objCache = new LruCache<{ id: number; name: string }>({ maxSize: 2 });
        objCache.set("user1", { id: 1, name: "Alice" });

        const user = objCache.get("user1");
        expect(user).toEqual({ id: 1, name: "Alice" });
      });

      it("should work with StmEntry type", () => {
        const stmCache = new LruCache<StmEntry>({ maxSize: 2 });
        const entry: StmEntry = { id: "1", role: "user", content: "Hello", timestamp: Date.now() };

        stmCache.set("entry1", entry);
        expect(stmCache.get("entry1")).toEqual(entry);
      });
    });
  });

  describe("CacheEntry interface", () => {
    it("should have required properties", () => {
      const entry: CacheEntry<string> = {
        value: "test",
        lastAccess: Date.now(),
        accessCount: 1,
      };

      expect(entry.value).toBe("test");
      expect(typeof entry.lastAccess).toBe("number");
      expect(entry.accessCount).toBe(1);
    });
  });

  describe("createStmCache", () => {
    it("should create cache with default maxSize 1000", () => {
      const cache = createStmCache();

      expect(cache).toBeInstanceOf(LruCache);
      // Default maxSize is 1000
      // Add 1001 items, first should be evicted
      for (let i = 0; i < 1001; i++) {
        cache.set(`key${i}`, { id: String(i), role: "user", content: "test", timestamp: Date.now() } as StmEntry);
      }

      expect(cache.size).toBeLessThanOrEqual(1000);
    });

    it("should be usable as STM cache", () => {
      const cache = createStmCache();
      const entry: StmEntry = { id: "1", role: "assistant", content: "Response", timestamp: Date.now() };

      cache.set("conv1", entry);
      expect(cache.get("conv1")).toEqual(entry);
    });
  });

  describe("edge cases", () => {
    it("should handle maxSize of 1", () => {
      const smallCache = new LruCache<string>({ maxSize: 1 });
      smallCache.set("a", "1");
      smallCache.set("b", "2");

      expect(smallCache.has("a")).toBe(false);
      expect(smallCache.has("b")).toBe(true);
      expect(smallCache.size).toBe(1);
    });

    it("should handle maxSize of 0", () => {
      const zeroCache = new LruCache<string>({ maxSize: 0 });
      zeroCache.set("a", "1");

      expect(zeroCache.size).toBe(0);
      expect(zeroCache.has("a")).toBe(false);
    });

    it("should handle rapid set/get cycles", () => {
      for (let i = 0; i < 100; i++) {
        cache.set(`key${i}`, `value${i}`);
        cache.get(`key${i}`);
      }

      expect(cache.size).toBeLessThanOrEqual(3);
    });
  });
});