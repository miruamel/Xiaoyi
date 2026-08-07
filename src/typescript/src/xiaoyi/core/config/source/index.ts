/**
 * # Configuration Sources
 *
 * `source` defines the configuration source trait and implementations.
 *
 * Path: `xiaoyi.core.config.source`
 *
 * - Layer 0: `core`
 * - Layer 1: `config`
 * - Layer 2: `source` — configuration source abstraction.
 * - Layer 3: `file`/`env`/`vault` — concrete sources.
 *
 * @module core.config.source
 * @brief Configuration source abstraction
 * @group Core
 * @since 0.1.0
 * @author Miruamel
 * @see core.config
 * @see core.config.source.file
 * @see core.config.source.env
 * @see core.config.source.vault
 */
export * from "./file";
export * from "./env";
export * from "./vault";

/**
 * Configuration source interface.
 *
 * @brief Async configuration source
 * @group Core
 * @since 0.1.0
 * @see core.config.source.FileSource
 * @see core.config.source.EnvSource
 * @see core.config.source.VaultSource
 */
export interface ConfigSource {
  /** Source name. */
  readonly name: string;
  /** Source priority (higher = overrides lower). */
  readonly priority: number;

  /**
   * Load configuration from source.
   *
   * @returns Configuration object
   * @throws {ConfigSourceError} If loading fails
   * @since 0.1.0
   */
  load(): Promise<Record<string, unknown>>;

  /**
   * Watch for configuration changes.
   *
   * @param callback - Change callback
   * @returns Unsubscribe function
   * @since 0.1.0
   */
  watch(callback: (config: Record<string, unknown>) => void): () => void;
}

/**
 * Configuration source error.
 *
 * @brief Error from config source
 * @group Core
 * @since 0.1.0
 */
export class ConfigSourceError extends Error {
  /** Source name. */
  public readonly source: string;

  /**
   * Create config source error.
   *
   * @param source - Source name
   * @param message - Error message
   * @param cause - Original error
   * @since 0.1.0
   */
  constructor(source: string, message: string, cause?: Error) {
    super(`Config source '${source}': ${message}`);
    this.name = "ConfigSourceError";
    this.source = source;
    this.cause = cause;
  }
}