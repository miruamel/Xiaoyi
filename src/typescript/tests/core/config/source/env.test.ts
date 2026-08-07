import { describe, it, expect, vi, beforeEach, afterEach } from "vitest";
import { EnvSource, EnvSourceOptions } from "../../src/xiaoyi/core/config/source/env";

describe("core/config/source/env", () => {
  const originalEnv = { ...process.env };

  beforeEach(() => {
    vi.resetModules();
    process.env = { ...originalEnv };
  });

  afterEach(() => {
    process.env = originalEnv;
  });

  describe("EnvSource constructor", () => {
    it("should create source with default prefix XIAOYI_", () => {
      const source = new EnvSource();

      expect(source.name).toBe("env:XIAOYI_");
      expect(source.priority).toBe(200); // default
    });

    it("should create source with custom prefix", () => {
      const options: EnvSourceOptions = { prefix: "MYAPP_" };
      const source = new EnvSource(options);

      expect(source.name).toBe("env:MYAPP_");
    });

    it("should create source with custom priority", () => {
      const options: EnvSourceOptions = { prefix: "TEST_", priority: 50 };
      const source = new EnvSource(options);

      expect(source.priority).toBe(50);
    });

    it("should create source with custom parser", () => {
      const customParser = vi.fn((v: string) => `parsed:${v}`);
      const options: EnvSourceOptions = { prefix: "PARSER_", parser: customParser };
      const source = new EnvSource(options);

      expect(source).toBeInstanceOf(EnvSource);
    });
  });

  describe("load()", () => {
    it("should load environment variables with prefix", async () => {
      process.env.XIAOYI_DB_HOST = "localhost";
      process.env.XIAOYI_DB_PORT = "5432";
      process.env.XIAOYI_DEBUG = "true";

      const source = new EnvSource();
      const config = await source.load();

      expect(config).toEqual({
        db: { host: "localhost", port: "5432" },
        debug: "true",
      });
    });

    it("should convert underscore to dot for nested keys", async () => {
      process.env.XIAOYI_NESTED_DEEP_KEY = "value";

      const source = new EnvSource();
      const config = await source.load();

      expect(config).toEqual({ nested: { deep: { key: "value" } } });
    });

    it("should lowercase keys", async () => {
      process.env.XIAOYI_UPPERCASE_KEY = "value";

      const source = new EnvSource();
      const config = await source.load();

      expect(config).toEqual({ uppercase: { key: "value" } });
    });

    it("should parse JSON values", async () => {
      process.env.XIAOYI_JSON_CONFIG = '{"key": "value", "num": 42}';
      process.env.XIAOYI_JSON_ARRAY = '[1, 2, 3]';

      const source = new EnvSource();
      const config = await source.load();

      expect(config).toEqual({
        json: { config: { key: "value", num: 42 } },
        json: { array: [1, 2, 3] },
      });
    });

    it("should keep non-JSON values as strings", async () => {
      process.env.XIAOYI_STRING_VALUE = "hello world";
      process.env.XIAOYI_NUMBER_STRING = "123";
      process.env.XIAOYI_BOOL_STRING = "true";

      const source = new EnvSource();
      const config = await source.load();

      expect(config).toEqual({
        string: { value: "hello world" },
        number: { string: "123" },
        bool: { string: "true" },
      });
    });

    it("should ignore variables without prefix", async () => {
      process.env.OTHER_VAR = "ignored";
      process.env.XIAOYI_MY_VAR = "included";

      const source = new EnvSource();
      const config = await source.load();

      expect(config).toEqual({ my: { var: "included" } });
      expect(config.other).toBeUndefined();
    });

    it("should ignore undefined values", async () => {
      process.env.XIAOYI_DEFINED = "value";
      // XIAOYI_UNDEFINED is not set

      const source = new EnvSource();
      const config = await source.load();

      expect(config).toEqual({ defined: "value" });
    });

    it("should work with custom prefix", async () => {
      process.env.MYAPP_DB_URL = "postgres://localhost";
      process.env.MYAPP_CACHE_TTL = "300";

      const source = new EnvSource({ prefix: "MYAPP_" });
      const config = await source.load();

      expect(config).toEqual({
        db: { url: "postgres://localhost" },
        cache: { ttl: "300" },
      });
    });

    it("should use custom parser when provided", async () => {
      const customParser = vi.fn((value: string) => `custom:${value}`);
      process.env.CUSTOM_PREFIX_VALUE = "test";

      const source = new EnvSource({ prefix: "CUSTOM_PREFIX_", parser: customParser });
      const config = await source.load();

      expect(customParser).toHaveBeenCalledWith("test");
      expect(config).toEqual({ value: "custom:test" });
    });

    it("should return empty object when no matching vars", async () => {
      process.env.OTHER_PREFIX_VAR = "value";

      const source = new EnvSource({ prefix: "NONEXISTENT_" });
      const config = await source.load();

      expect(config).toEqual({});
    });
  });

  describe("watch()", () => {
    it("should return no-op unsubscribe function", () => {
      const source = new EnvSource();
      const callback = vi.fn();

      const unsubscribe = source.watch(callback);

      expect(typeof unsubscribe).toBe("function");
      unsubscribe(); // Should not throw
    });

    it("should not call callback immediately", () => {
      const source = new EnvSource();
      const callback = vi.fn();

      source.watch(callback);

      expect(callback).not.toHaveBeenCalled();
    });
  });

  describe("defaultParser", () => {
    it("should parse JSON objects", async () => {
      const source = new EnvSource();
      // Access private method via load with JSON env var
      process.env.XIAOYI_TEST_JSON = '{"a": 1}';

      const config = await source.load();
      expect(config.test).toEqual({ json: { a: 1 } });
    });

    it("should parse JSON arrays", async () => {
      process.env.XIAOYI_TEST_ARRAY = '[1, 2, 3]';

      const source = new EnvSource();
      const config = await source.load();

      expect(config.test).toEqual({ array: [1, 2, 3] });
    });

    it("should parse JSON primitives", async () => {
      process.env.XIAOYI_TEST_NUMBER = "42";
      process.env.XIAOYI_TEST_BOOL = "true";
      process.env.XIAOYI_TEST_NULL = "null";

      const source = new EnvSource();
      const config = await source.load();

      expect(config.test).toEqual({
        number: 42,
        bool: true,
        null: null,
      });
    });

    it("should return string for non-JSON", async () => {
      process.env.XIAOYI_TEST_STRING = "plain string";

      const source = new EnvSource();
      const config = await source.load();

      expect(config.test).toEqual({ string: "plain string" });
    });
  });
});