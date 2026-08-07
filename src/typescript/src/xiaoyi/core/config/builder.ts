/**
 * # Configuration Builder
 *
 * `builder` provides a fluent builder pattern for constructing configurations.
 *
 * Path: `xiaoyi.core.config.builder`
 *
 * @module core.config.builder
 * @brief Fluent configuration builder
 * @group Core
 * @since 0.1.0
 * @author Miruamel
 * @see core.config
 * @see core.config.source
 */
import { Config, ConfigSource } from "./config";

/**
 * Configuration builder for composing multiple sources.
 *
 * @brief Build configuration from multiple sources
 * @group Core
 * @since 0.1.0
 * @example
 * ```typescript
 * const config = await new ConfigBuilder()
 *   .addSource(new FileSource({ path: "./config.toml" }))
 *   .addSource(new EnvSource({ prefix: "XIAOYI_" }))
 *   .build();
 * ```
 */
export class ConfigBuilder {
  private sources: ConfigSource[] = [];

  /**
   * Add a configuration source.
   *
   * @param source - Configuration source to add
   * @returns this (for chaining)
   * @since 0.1.0
   */
  addSource(source: ConfigSource): this {
    this.sources.push(source);
    return this;
  }

  /**
   * Add multiple configuration sources.
   *
   * @param sources - Configuration sources to add
   * @returns this (for chaining)
   * @since 0.1.0
   */
  addSources(sources: ConfigSource[]): this {
    this.sources.push(...sources);
    return this;
  }

  /**
   * Build configuration by loading all sources in priority order.
   *
   * @returns Merged configuration
   * @since 0.1.0
   */
  async build(): Promise<Config> {
    // Sort by priority (lower first, so higher priority overrides)
    const sorted = [...this.sources].sort((a, b) => a.priority - b.priority);

    const data: Record<string, unknown> = {};
    const sourceNames: string[] = [];

    for (const source of sorted) {
      const loaded = await source.load();
      Object.assign(data, loaded);
      sourceNames.push(source.name);
    }

    return { data, sources: sourceNames };
  }
}