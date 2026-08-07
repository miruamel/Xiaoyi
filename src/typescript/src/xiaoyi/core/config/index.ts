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
 */
import { ConfigBuilder } from "./builder";
import { ConfigSourceError } from "./config";
import { FileSource } from "./source/file";
import { EnvSource } from "./source/env";
import { VaultSource } from "./source/vault";

export { ConfigBuilder };
export { ConfigSourceError };
export { FileSource };
export { EnvSource };
export { VaultSource };
export type { Config, ConfigSource } from "./config";
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