/**
 * # Configuration
 *
 * `config` provides configuration management with multiple sources
 * (file, environment, vault) and a builder pattern.
 *
 * Path: `xiaoyi.core.config`
 *
 * - Layer 0: `core`
 * - Layer 1: `config` — configuration management.
 * - Layer 2: `source` — configuration sources.
 * - Layer 3: `file`/`env`/`vault` — concrete sources.
 *
 * @module core.config
 * @brief Configuration management with multiple sources
 * @group Core
 * @since 0.1.0
 * @author Miruamel
 * @see core.config.source
 * @see core.config.source.file
 * @see core.config.source.env
export * from "./source";
export * from "./config";
export * from "./builder";
export { ConfigBuilder } from "./builder";
export { Config } from "./config";
export { ConfigSource } from "./source";
export { ConfigSourceError } from "./config";

/**
 * Configuration value type.
 *
 * @brief Supported config value types
 * @group Core
 * @since 0.1.0
 */
export type ConfigValue = string | number | boolean | null | ConfigValue[] | { [key: string]: ConfigValue };

/**
 * Configuration merge strategy.
 *
 * @brief How to merge config layers
 * @group Core
 * @since 0.1.0
 */
export enum ConfigMergeStrategy {
  /** Deep merge objects. */
  Deep = "deep",
  /** Shallow merge (replace). */
  Shallow = "shallow",
  /** Replace entirely. */
  Replace = "replace",
}