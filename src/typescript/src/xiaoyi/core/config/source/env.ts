/**
 * # Environment Configuration Source
 *
 * `env` provides environment variable-based configuration loading.
 *
 * Path: `xiaoyi.core.config.source.env`
 *
 * - Layer 0: `core`
 * - Layer 1: `config`
 * - Layer 2: `source`
 * - Layer 3: `env`
 *
 * @module core.config.source.env
 * @brief Environment variable configuration source
 * @group Core
 * @since 0.1.0
 * @author Miruamel
 * @see core.config.source
 * @see core.config.source.file
 * @see core.config.source.vault
 */
import { ConfigSource, ConfigSourceError } from ".";

/** Environment source options. */
export interface EnvSourceOptions {
  /** Environment variable prefix (e.g., "XIAOYI_"). */
  prefix?: string;
  /** Source priority. */
  priority?: number;
  /** Custom parser for values. */
  parser?: (value: string) => unknown;
}

/** Environment variable configuration source. */
export class EnvSource implements ConfigSource {
  public readonly name: string;
  public readonly priority: number;
  private readonly prefix: string;
  private readonly parser: (value: string) => unknown;

  /**
   * Create environment source.
   *
   * @param options - Source options
   * @since 0.1.0
   */
  constructor(options: EnvSourceOptions = {}) {
    this.prefix = options.prefix ?? "XIAOYI_";
    this.name = `env:${this.prefix}`;
    this.priority = options.priority ?? 200;
    this.parser = options.parser ?? this.defaultParser.bind(this);
  }

  /** Default value parser - parses JSON objects and arrays, keeps primitives as strings. */
  private defaultParser(value: string): unknown {
    const trimmed = value.trim();
    // Only parse if it looks like an object or array
    if ((trimmed.startsWith("{") && trimmed.endsWith("}")) || (trimmed.startsWith("[") && trimmed.endsWith("]"))) {
      try {
        return JSON.parse(value);
      } catch {
        return value;
      }
    }
    // Keep primitives (numbers, booleans, null) and other strings as-is
    return value;
  }

  /**
   * Set nested value in object using dot notation.
   */
  private setNested(obj: Record<string, unknown>, path: string, value: unknown): void {
    const keys = path.split(".");
    let current = obj;
    for (let i = 0; i < keys.length - 1; i++) {
      const key = keys[i];
      if (!(key in current) || typeof current[key] !== "object" || current[key] === null) {
        current[key] = {};
      }
      current = current[key] as Record<string, unknown>;
    }
    current[keys[keys.length - 1]] = value;
  }

  /**
   * Load configuration from environment variables.
   *
   * @returns Configuration object
   * @since 0.1.0
   */
  async load(): Promise<Record<string, unknown>> {
    const result: Record<string, unknown> = {};

    for (const [key, value] of Object.entries(process.env)) {
      if (key.startsWith(this.prefix) && value !== undefined) {
        const configKey = key.slice(this.prefix.length).toLowerCase().replace(/_/g, ".");
        const parsedValue = this.parser(value);
        this.setNested(result, configKey, parsedValue);
      }
    }

    return result;
  }

  /**
   * Watch for environment changes (not supported).
   *
   * @param _callback - Change callback
   * @returns No-op unsubscribe
   * @since 0.1.0
   */
  watch(_callback: (config: Record<string, unknown>) => void): () => void {
    // Environment variables cannot be watched reliably
    return () => {};
  }
}