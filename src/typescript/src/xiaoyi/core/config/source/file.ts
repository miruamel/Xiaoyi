/**
 * # File Configuration Source
 *
 * `file` provides file-based configuration loading (JSON, YAML, TOML).
 *
 * Path: `xiaoyi.core.config.source.file`
 *
 * - Layer 0: `core`
 * - Layer 1: `config`
 * - Layer 2: `source`
 * - Layer 3: `file`
 *
 * @module core.config.source.file
 * @brief File-based configuration source
 * @group Core
 * @since 0.1.0
 * @author Miruamel
 * @see core.config.source
 * @see core.config.source.env
 * @see core.config.source.vault
 */
import { ConfigSource, ConfigSourceError } from ".";
import * as fs from "fs/promises";
import * as path from "path";

/** Supported config file formats. */
export enum ConfigFormat {
  /** JSON format. */
  Json = "json",
  /** YAML format. */
  Yaml = "yaml",
  /** TOML format. */
  Toml = "toml",
}

/** File source options. */
export interface FileSourceOptions {
  /** File path. */
  path: string;
  /** File format (auto-detected from extension if not specified). */
  format?: ConfigFormat;
  /** Watch for changes. */
  watch?: boolean;
  /** Source priority. */
  priority?: number;
}

/** File-based configuration source. */
export class FileSource implements ConfigSource {
  public readonly name: string;
  public readonly priority: number;
  private readonly filePath: string;
  private readonly format: ConfigFormat;
  private readonly watchEnabled: boolean;
  private watchers: Map<string, () => void> = new Map();

  /**
   * Create file source.
   *
   * @param options - Source options
   * @since 0.1.0
   */
  constructor(options: FileSourceOptions) {
    this.filePath = path.resolve(options.path);
    this.format = options.format ?? this.detectFormat();
    this.name = `file:${this.filePath}`;
    this.priority = options.priority ?? 100;
    this.watchEnabled = options.watch ?? false;
  }

  /** Detect format from extension. */
  private detectFormat(): ConfigFormat {
    const ext = path.extname(this.filePath).toLowerCase();
    switch (ext) {
      case ".json": return ConfigFormat.Json;
      case ".yaml":
      case ".yml": return ConfigFormat.Yaml;
      case ".toml": return ConfigFormat.Toml;
      default: return ConfigFormat.Json;
    }
  }

  /**
   * Load configuration from file.
   *
   * @returns Parsed configuration
   * @throws {ConfigSourceError} If loading or parsing fails
   * @since 0.1.0
   */
  async load(): Promise<Record<string, unknown>> {
    try {
      const content = await fs.readFile(this.filePath, "utf-8");
      return this.parse(content);
    } catch (error) {
      throw new ConfigSourceError(this.name, `Failed to load: ${error}`, error as Error);
    }
  }

  /** Parse content based on format. */
  private parse(content: string): Record<string, unknown> {
    switch (this.format) {
      case ConfigFormat.Json:
        return JSON.parse(content);
      case ConfigFormat.Yaml:
        // Simple YAML parsing - in production use a proper YAML library
        return this.parseYaml(content);
      case ConfigFormat.Toml:
        // Simple TOML parsing - in production use a proper TOML library
        return this.parseToml(content);
    }
  }

  /** Simple YAML parser (subset). */
  private parseYaml(content: string): Record<string, unknown> {
    const result: Record<string, unknown> = {};
    for (const line of content.split("\n")) {
      const trimmed = line.trim();
      if (!trimmed || trimmed.startsWith("#")) continue;
      const idx = trimmed.indexOf(":");
      if (idx > 0) {
        const key = trimmed.slice(0, idx).trim();
        const value = trimmed.slice(idx + 1).trim();
        result[key] = this.parseYamlValue(value);
      }
    }
    return result;
  }

  /** Parse YAML value. */
  private parseYamlValue(value: string): unknown {
    if (value === "true") return true;
    if (value === "false") return false;
    if (value === "null" || value === "~") return null;
    const num = Number(value);
    if (!Number.isNaN(num)) return num;
    return value.replace(/^["']|["']$/g, "");
  }

  /** Simple TOML parser (subset). */
  private parseToml(content: string): Record<string, unknown> {
    const result: Record<string, unknown> = {};
    for (const line of content.split("\n")) {
      const trimmed = line.trim();
      if (!trimmed || trimmed.startsWith("#")) continue;
      const idx = trimmed.indexOf("=");
      if (idx > 0) {
        const key = trimmed.slice(0, idx).trim();
        const value = trimmed.slice(idx + 1).trim();
        result[key] = this.parseYamlValue(value);
      }
    }
    return result;
  }

  /**
   * Watch for file changes.
   *
   * @param callback - Change callback
   * @returns Unsubscribe function
   * @since 0.1.0
   */
  watch(callback: (config: Record<string, unknown>) => void): () => void {
    if (!this.watchEnabled) {
      return () => {};
    }
    // In production, use fs.watch or chokidar
    const interval = setInterval(async () => {
      try {
        const config = await this.load();
        callback(config);
      } catch {
        // Ignore watch errors
      }
    }, 5000);

    return () => clearInterval(interval);
  }
}