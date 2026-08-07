import { describe, it, expect, vi, beforeEach, afterEach } from "vitest";
import { FileSource, ConfigFormat, FileSourceOptions } from "@xiaoyi/xiaoyi/core/config/source/file";
import * as fs from "fs/promises";
import * as path from "path";

describe("core/config/source/file", () => {
  const testDir = "/tmp/xiaoyi-test-config";

  beforeEach(async () => {
    await fs.mkdir(testDir, { recursive: true });
  });

  afterEach(async () => {
    await fs.rm(testDir, { recursive: true, force: true });
  });

  describe("FileSource constructor", () => {
    it("should create source with required path", () => {
      const options: FileSourceOptions = { path: "/tmp/config.json" };
      const source = new FileSource(options);

      expect(source.name).toBe("file:/tmp/config.json");
      expect(source.priority).toBe(100); // default
    });

    it("should create source with custom priority", () => {
      const options: FileSourceOptions = { path: "/tmp/config.json", priority: 500 };
      const source = new FileSource(options);

      expect(source.priority).toBe(500);
    });

    it("should create source with explicit format", () => {
      const options: FileSourceOptions = { path: "/tmp/config", format: ConfigFormat.Yaml };
      const source = new FileSource(options);

      // Format is used internally, not exposed directly
      expect(source).toBeInstanceOf(FileSource);
    });

    it("should create source with watch option", () => {
      const options: FileSourceOptions = { path: "/tmp/config.json", watch: true };
      const source = new FileSource(options);

      expect(typeof source.watch).toBe("function");
    });
  });

  describe("load() - JSON format", () => {
    it("should load JSON file", async () => {
      const filePath = path.join(testDir, "config.json");
      await fs.writeFile(filePath, JSON.stringify({ key1: "value1", num: 42, bool: true }));

      const source = new FileSource({ path: filePath });
      const config = await source.load();

      expect(config).toEqual({ key1: "value1", num: 42, bool: true });
    });

    it("should handle nested JSON objects", async () => {
      const filePath = path.join(testDir, "nested.json");
      await fs.writeFile(filePath, JSON.stringify({ outer: { inner: { value: "deep" } } }));

      const source = new FileSource({ path: filePath });
      const config = await source.load();

      expect(config).toEqual({ outer: { inner: { value: "deep" } } });
    });

    it("should throw on invalid JSON", async () => {
      const filePath = path.join(testDir, "invalid.json");
      await fs.writeFile(filePath, "{ invalid json }");

      const source = new FileSource({ path: filePath });
      await expect(source.load()).rejects.toThrow();
    });
  });

  describe("load() - YAML format", () => {
    it("should load YAML file with .yaml extension", async () => {
      const filePath = path.join(testDir, "config.yaml");
      await fs.writeFile(filePath, "key1: value1\nnum: 42\nbool: true\n");

      const source = new FileSource({ path: filePath });
      const config = await source.load();

      expect(config).toEqual({ key1: "value1", num: 42, bool: true });
    });

    it("should load YAML file with .yml extension", async () => {
      const filePath = path.join(testDir, "config.yml");
      await fs.writeFile(filePath, "key1: value1\nnum: 42\n");

      const source = new FileSource({ path: filePath });
      const config = await source.load();

      expect(config).toEqual({ key1: "value1", num: 42 });
    });

    it("should handle nested YAML", async () => {
      const filePath = path.join(testDir, "nested.yaml");
      await fs.writeFile(filePath, "outer:\n  inner:\n    value: deep\n");

      const source = new FileSource({ path: filePath });
      const config = await source.load();

      expect(config).toEqual({ outer: { inner: { value: "deep" } } });
    });
  });

  describe("load() - TOML format", () => {
    it("should load TOML file", async () => {
      const filePath = path.join(testDir, "config.toml");
      await fs.writeFile(filePath, 'key1 = "value1"\nnum = 42\nbool = true\n');

      const source = new FileSource({ path: filePath });
      const config = await source.load();

      expect(config).toEqual({ key1: "value1", num: 42, bool: true });
    });

    it("should handle nested TOML tables", async () => {
      const filePath = path.join(testDir, "nested.toml");
      await fs.writeFile(filePath, "[outer]\n[outer.inner]\nvalue = \"deep\"\n");

      const source = new FileSource({ path: filePath });
      const config = await source.load();

      expect(config).toEqual({ outer: { inner: { value: "deep" } } });
    });
  });

  describe("format auto-detection", () => {
    it("should auto-detect JSON from .json extension", async () => {
      const filePath = path.join(testDir, "auto.json");
      await fs.writeFile(filePath, '{"auto": true}');

      const source = new FileSource({ path: filePath }); // no format specified
      const config = await source.load();

      expect(config).toEqual({ auto: true });
    });

    it("should auto-detect YAML from .yaml extension", async () => {
      const filePath = path.join(testDir, "auto.yaml");
      await fs.writeFile(filePath, "auto: true\n");

      const source = new FileSource({ path: filePath });
      const config = await source.load();

      expect(config).toEqual({ auto: true });
    });

    it("should auto-detect TOML from .toml extension", async () => {
      const filePath = path.join(testDir, "auto.toml");
      await fs.writeFile(filePath, "auto = true\n");

      const source = new FileSource({ path: filePath });
      const config = await source.load();

      expect(config).toEqual({ auto: true });
    });

    it("should allow explicit format override", async () => {
      const filePath = path.join(testDir, "override.json");
      await fs.writeFile(filePath, "override = true\n"); // TOML content but .json extension

      const source = new FileSource({ path: filePath, format: ConfigFormat.Toml });
      const config = await source.load();

      expect(config).toEqual({ override: true });
    });
  });

  describe("watch()", () => {
    it("should return unsubscribe function when watch is enabled", async () => {
      const filePath = path.join(testDir, "watch.json");
      await fs.writeFile(filePath, '{"watched": true}');

      const source = new FileSource({ path: filePath, watch: true });
      const unsubscribe = source.watch(vi.fn());

      expect(typeof unsubscribe).toBe("function");
      unsubscribe(); // Should not throw
    });

    it("should return no-op unsubscribe when watch is disabled", () => {
      const source = new FileSource({ path: "/nonexistent.json", watch: false });
      const unsubscribe = source.watch(vi.fn());

      expect(typeof unsubscribe).toBe("function");
      unsubscribe(); // Should not throw
    });

    it("should return no-op unsubscribe by default", () => {
      const source = new FileSource({ path: "/nonexistent.json" });
      const unsubscribe = source.watch(vi.fn());

      expect(typeof unsubscribe).toBe("function");
    });
  });

  describe("error handling", () => {
    it("should throw on missing file", async () => {
      const source = new FileSource({ path: "/nonexistent/path/config.json" });
      await expect(source.load()).rejects.toThrow();
    });

    it("should throw on directory instead of file", async () => {
      const source = new FileSource({ path: testDir });
      await expect(source.load()).rejects.toThrow();
    });
  });

  describe("ConfigFormat enum", () => {
    it("should have expected values", () => {
      expect(ConfigFormat.Json).toBe("json");
      expect(ConfigFormat.Yaml).toBe("yaml");
      expect(ConfigFormat.Toml).toBe("toml");
    });
  });
});