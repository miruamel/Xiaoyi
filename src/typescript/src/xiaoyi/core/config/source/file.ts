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

  /** Simple YAML parser (subset with nested support). */
  private parseYaml(content: string): Record<string, unknown> {
    const result: Record<string, unknown> = {};
    const lines = content.split("\n");
    const stack: Array<{ indent: number; obj: Record<string, unknown> }> = [
      { indent: -1, obj: result },
    ];

    for (const line of lines) {
      const trimmed = line.trim();
      if (!trimmed || trimmed.startsWith("#")) continue;

      // Calculate indent
      const indent = line.length - line.trimStart().length;
      const idx = trimmed.indexOf(":");
      if (idx <= 0) continue;

      const key = trimmed.slice(0, idx).trim();
      const value = trimmed.slice(idx + 1).trim();

      // Find parent based on indent
      while (stack.length > 1 && stack[stack.length - 1].indent >= indent) {
        stack.pop();
      }

      const parent = stack[stack.length - 1].obj;
      if (value === "" || value === "|" || value === ">") {
        // Nested object
        const nested: Record<string, unknown> = {};
        parent[key] = nested;
        stack.push({ indent, obj: nested });
      } else {
        parent[key] = this.parseYamlValue(value);
      }
    }
    return result;
  }

  /** Simple TOML parser (subset with nested support). */
  private parseToml(content: string): Record<string, unknown> {
    const result: Record<string, unknown> = {};
    const lines = content.split("\n");
    let currentTable: Record<string, unknown> = result;

    for (const line of lines) {
      const trimmed = line.trim();
      if (!trimmed || trimmed.startsWith("#")) continue;

      // Check for table headers [section] or [section.subsection]
      const tableMatch = trimmed.match(/^\[(.+)\]$/);
      if (tableMatch) {
        const tablePath = tableMatch[1].split(".");
        currentTable = result;
        for (const part of tablePath) {
          if (!(part in currentTable) || typeof currentTable[part] !== "object") {
            currentTable[part] = {};
          }
          currentTable = currentTable[part] as Record<string, unknown>;
        }
        continue;
      }

      const idx = trimmed.indexOf("=");
      if (idx > 0) {
        const key = trimmed.slice(0, idx).trim();
        const value = trimmed.slice(idx + 1).trim();
        currentTable[key] = this.parseYamlValue(value);
      }
    }
    return result;
  }

  /** Parse YAML/TOML value. */
  private parseYamlValue(value: string): unknown {
    if (value === "true") return true;
    if (value === "false") return false;
    if (value === "null" || value === "~") return null;
    const num = Number(value);
    if (!Number.isNaN(num)) return num;
    return value.replace(/^["']|["']$/g, "");
  }

  /**

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