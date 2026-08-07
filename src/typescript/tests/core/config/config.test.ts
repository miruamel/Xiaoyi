import { describe, it, expect, vi, beforeEach } from "vitest";
import { ConfigBuilder, Config, ConfigSource, ConfigSourceError, ConfigMergeStrategy, ConfigValue } from "@xiaoyi/xiaoyi/core/config";

describe("core/config", () => {
  describe("ConfigBuilder", () => {
    it("should build config from single source", async () => {
      const mockSource: ConfigSource = {
        name: "test-source",
        priority: 100,
        load: vi.fn().mockResolvedValue({ key1: "value1", key2: 42 }),
      };

      const builder = new ConfigBuilder().addSource(mockSource);
      const config = await builder.build();

      expect(config.data).toEqual({ key1: "value1", key2: 42 });
      expect(config.sources).toEqual(["test-source"]);
      expect(mockSource.load).toHaveBeenCalledOnce();
    });

    it("should build config from multiple sources with priority order", async () => {
      const lowPriority: ConfigSource = {
        name: "low",
        priority: 100,
        load: vi.fn().mockResolvedValue({ a: 1, b: 2 }),
      };
      const highPriority: ConfigSource = {
        name: "high",
        priority: 200,
        load: vi.fn().mockResolvedValue({ b: 20, c: 30 }),
      };

      const builder = new ConfigBuilder().addSources([lowPriority, highPriority]);
      const config = await builder.build();

      // Higher priority (larger number) should override
      expect(config.data).toEqual({ a: 1, b: 20, c: 30 });
      expect(config.sources).toEqual(["low", "high"]);
    });

    it("should support chaining addSource", async () => {
      const source1: ConfigSource = {
        name: "s1",
        priority: 10,
        load: vi.fn().mockResolvedValue({ a: 1 }),
      };
      const source2: ConfigSource = {
        name: "s2",
        priority: 20,
        load: vi.fn().mockResolvedValue({ b: 2 }),
      };

      const config = await new ConfigBuilder().addSource(source1).addSource(source2).build();

      expect(config.data).toEqual({ a: 1, b: 2 });
      expect(config.sources).toEqual(["s1", "s2"]);
    });

    it("should sort sources by priority (lower first, so higher priority wins)", async () => {
      const source1: ConfigSource = { name: "s1", priority: 500, load: vi.fn().mockResolvedValue({ x: 1 }) };
      const source2: ConfigSource = { name: "s2", priority: 100, load: vi.fn().mockResolvedValue({ x: 2, y: 2 }) };

      const config = await new ConfigBuilder().addSources([source1, source2]).build();

      // s1 has higher priority (500), should override s1's x
      expect(config.data.x).toBe(1);
      expect(config.data.y).toBe(2);
    });

    it("should handle empty sources", async () => {
      const config = await new ConfigBuilder().build();

      expect(config.data).toEqual({});
      expect(config.sources).toEqual([]);
    });

    it("should return new builder instance for chaining", () => {
      const builder = new ConfigBuilder();
      const builder2 = builder.addSource({ name: "test", priority: 1, load: vi.fn() });

      expect(builder2).toBe(builder);
    });
  });

  describe("Config interface", () => {
    it("should have data and sources properties", () => {
      const config: Config = {
        data: { key: "value" },
        sources: ["source1"],
      };

      expect(config.data).toBeDefined();
      expect(config.sources).toBeDefined();
      expect(Array.isArray(config.sources)).toBe(true);
    });
  });

  describe("ConfigMergeStrategy enum", () => {
    it("should have expected values", () => {
      expect(ConfigMergeStrategy.Deep).toBe("deep");
      expect(ConfigMergeStrategy.Shallow).toBe("shallow");
      expect(ConfigMergeStrategy.Replace).toBe("replace");
    });
  });

  describe("ConfigValue type", () => {
    it("should accept string", () => {
      const val: ConfigValue = "string";
      expect(val).toBe("string");
    });

    it("should accept number", () => {
      const val: ConfigValue = 42;
      expect(val).toBe(42);
    });

    it("should accept boolean", () => {
      const val: ConfigValue = true;
      expect(val).toBe(true);
    });

    it("should accept null", () => {
      const val: ConfigValue = null;
      expect(val).toBeNull();
    });

    it("should accept array", () => {
      const val: ConfigValue = [1, "two", true];
      expect(val).toEqual([1, "two", true]);
    });

    it("should accept object", () => {
      const val: ConfigValue = { nested: { key: "value" } };
      expect(val).toEqual({ nested: { key: "value" } });
    });
  });

  describe("ConfigSourceError", () => {
    it("should create error with source prefix", () => {
      const error = new ConfigSourceError("file", "Not found");

      expect(error.message).toBe("[file] Not found");
      expect(error.name).toBe("ConfigSourceError");
      expect(error.source).toBe("file");
    });

    it("should be instanceof Error", () => {
      const error = new ConfigSourceError("env", "Invalid");
      expect(error instanceof Error).toBe(true);
    });
  });
});