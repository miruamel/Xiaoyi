/**
 * # Configuration Core Types
 *
 * `config` provides core configuration types and interfaces.
 *
 * Path: `xiaoyi.core.config`
 *
 * @module core.config
 * @brief Core configuration types
 * @group Core
 * @since 0.1.0
 * @author Miruamel
 * @see core.config.source
 * @see core.config.builder
 */

/**
 * Configuration object type.
 *
 * @brief Configuration data structure
 * @group Core
 * @since 0.1.0
 */
export interface Config {
  /** Configuration data as key-value pairs. */
  data: Record<string, unknown>;
  /** Source metadata. */
  sources: string[];
}

/**
 * Configuration source interface.
 *
 * @brief Abstract configuration source
 * @group Core
 * @since 0.1.0
 */
export interface ConfigSource {
  /** Source name. */
  readonly name: string;
  /** Source priority (higher = loaded later, overrides earlier). */
  readonly priority: number;
  /** Load configuration from source. */
  load(): Promise<Record<string, unknown>>;
  /** Watch for changes. */
  watch?(callback: (config: Record<string, unknown>) => void): void;
}

/**
 * Configuration source error.
 *
 * @brief Error from configuration source
 * @group Core
 * @since 0.1.0
 */
export class ConfigSourceError extends Error {
  public readonly source: string;

  constructor(source: string, message: string) {
    super(`[${source}] ${message}`);
    this.name = "ConfigSourceError";
    this.source = source;
  }
}