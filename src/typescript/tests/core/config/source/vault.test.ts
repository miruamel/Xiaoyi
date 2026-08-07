import { describe, it, expect, vi, beforeEach, afterEach } from "vitest";
import { VaultSource, VaultSourceOptions } from "@xiaoyi/xiaoyi/core/config/source/vault-source";
import * as fs from "fs/promises";
import * as path from "path";
import { encrypt as encryptConfig } from "@xiaoyi/xiaoyi/core/config/source/vault/encrypt";
import { deriveKey } from "@xiaoyi/xiaoyi/core/config/source/vault/key";

describe("core/config/source/vault", () => {
  const testDir = "/tmp/xiaoyi-test-vault";
  const password = "test-password-123";

  beforeEach(async () => {
    await fs.mkdir(testDir, { recursive: true });
  });

  afterEach(async () => {
    await fs.rm(testDir, { recursive: true, force: true });
  });

  describe("VaultSource constructor", () => {
    it("should create source with path and password", () => {
      const options: VaultSourceOptions = { path: "/tmp/vault.bin", password };
      const source = new VaultSource(options);

      expect(source.name).toBe("vault:/tmp/vault.bin");
      expect(source.priority).toBe(300); // default
    });

    it("should create source with custom priority", () => {
      const options: VaultSourceOptions = { path: "/tmp/vault.bin", password, priority: 400 };
      const source = new VaultSource(options);

      expect(source.priority).toBe(400);
    });
  });

  describe("load()", () => {
    it("should load and decrypt vault file", async () => {
      const vaultPath = path.join(testDir, "config.vault");
      const testConfig = { secret: "value", apiKey: "sk-12345" };

      // Create encrypted vault file
      const encrypted = await encryptConfig(testConfig, password);
      await fs.writeFile(vaultPath, encrypted);

      const source = new VaultSource({ path: vaultPath, password });
      const config = await source.load();

      expect(config).toEqual(testConfig);
    });

    it("should handle nested configuration", async () => {
      const vaultPath = path.join(testDir, "nested.vault");
      const testConfig = {
        database: { host: "localhost", port: 5432, credentials: { user: "admin", pass: "secret" } },
        features: ["auth", "logging"],
      };

      const encrypted = await encryptConfig(testConfig, password);
      await fs.writeFile(vaultPath, encrypted);

      const source = new VaultSource({ path: vaultPath, password });
      const config = await source.load();

      expect(config).toEqual(testConfig);
    });

    it("should throw on wrong password", async () => {
      const vaultPath = path.join(testDir, "config.vault");
      const testConfig = { secret: "value" };

      const encrypted = await encryptConfig(testConfig, password);
      await fs.writeFile(vaultPath, encrypted);

      const source = new VaultSource({ path: vaultPath, password: "wrong-password" });
      await expect(source.load()).rejects.toThrow();
    });

    it("should throw on corrupted vault file", async () => {
      const vaultPath = path.join(testDir, "corrupted.vault");
      await fs.writeFile(vaultPath, "corrupted data");

      const source = new VaultSource({ path: vaultPath, password });
      await expect(source.load()).rejects.toThrow();
    });

    it("should throw on missing vault file", async () => {
      const source = new VaultSource({ path: "/nonexistent/vault.bin", password });
      await expect(source.load()).rejects.toThrow();
    });

    it("should throw on empty vault file", async () => {
      const vaultPath = path.join(testDir, "empty.vault");
      await fs.writeFile(vaultPath, "");

      const source = new VaultSource({ path: vaultPath, password });
      await expect(source.load()).rejects.toThrow();
    });
  });

  describe("watch()", () => {
    it("should return unsubscribe function", async () => {
      const vaultPath = path.join(testDir, "config.vault");
      const testConfig = { watched: true };
      const encrypted = await encryptConfig(testConfig, password);
      await fs.writeFile(vaultPath, encrypted);

      const source = new VaultSource({ path: vaultPath, password });
      const callback = vi.fn();
      const unsubscribe = source.watch(callback);

      expect(typeof unsubscribe).toBe("function");
    });

    it("should call callback on file change", async () => {
      const vaultPath = path.join(testDir, "watch.vault");
      const initialConfig = { version: 1 };
      const encrypted = await encryptConfig(initialConfig, password);
      await fs.writeFile(vaultPath, encrypted);

      const source = new VaultSource({ path: vaultPath, password });
      const callback = vi.fn();
      const unsubscribe = source.watch(callback);

      // Wait a bit for watcher to settle
      await new Promise((resolve) => setTimeout(resolve, 100));

      // Modify the file
      const updatedConfig = { version: 2 };
      const updatedEncrypted = await encryptConfig(updatedConfig, password);
      await fs.writeFile(vaultPath, updatedEncrypted);

      // Wait for watch to trigger
      await new Promise((resolve) => setTimeout(resolve, 500));

      expect(callback).toHaveBeenCalled();
      const calledConfig = callback.mock.calls[0][0];
      expect(calledConfig).toEqual(updatedConfig);

      unsubscribe();
    });

    it("should not call callback after unsubscribe", async () => {
      const vaultPath = path.join(testDir, "unsub.vault");
      const initialConfig = { version: 1 };
      const encrypted = await encryptConfig(initialConfig, password);
      await fs.writeFile(vaultPath, encrypted);

      const source = new VaultSource({ path: vaultPath, password });
      const callback = vi.fn();
      const unsubscribe = source.watch(callback);

      await new Promise((resolve) => setTimeout(resolve, 100));

      unsubscribe();

      const updatedConfig = { version: 2 };
      const updatedEncrypted = await encryptConfig(updatedConfig, password);
      await fs.writeFile(vaultPath, updatedEncrypted);

      await new Promise((resolve) => setTimeout(resolve, 500));

      expect(callback).not.toHaveBeenCalled();
    });
  });

  describe("integration with ConfigBuilder", () => {
    it("should work as config source in builder", async () => {
      const vaultPath = path.join(testDir, "builder.vault");
      const testConfig = { fromVault: true, secret: "vault-secret" };
      const encrypted = await encryptConfig(testConfig, password);
      await fs.writeFile(vaultPath, encrypted);

      const { ConfigBuilder } = await import("@xiaoyi/xiaoyi/core/config");
      const builder = new ConfigBuilder().addSource(new VaultSource({ path: vaultPath, password }));
      const config = await builder.build();

      expect(config.data).toEqual(testConfig);
      expect(config.sources).toContain(`vault:${vaultPath}`);
    });
  });
});