/**
 * # Vault Configuration Source
 *
 * `vault-source` provides encrypted configuration loading from vault files.
 *
 * Path: `xiaoyi.core.config.source.vault-source`
 *
 * @module core.config.source.vault-source
 * @brief Encrypted file-based configuration source
 * @group Core
 * @security Loads encrypted configuration. Key must be provided securely.
 * @since 0.1.0
 * @author Miruamel
 * @see core.config.source
 * @see core.config.source.vault
 * @see core.config.source.vault.aes
 * @see core.config.source.vault.decrypt
 */
import { ConfigSource, ConfigSourceError } from ".";
import * as fs from "fs/promises";
import { decryptBytes } from "./vault/decrypt";

/** Vault source options. */
export interface VaultSourceOptions {
  /** Vault file path. */
  path: string;
  /** Encryption password. */
  password: string;
  /** Source priority. */
  priority?: number;
}

/** Encrypted configuration source. */
export class VaultSource implements ConfigSource {
  public readonly name: string;
  public readonly priority: number;
  private readonly path: string;
  private readonly password: string;

  /**
   * Create vault source.
   *
   * @param options - Source options
   * @since 0.1.0
   * @security Password should be provided via secure means (env var, secret manager).
   */
  constructor(options: VaultSourceOptions) {
    this.path = options.path;
    this.password = options.password;
    this.name = `vault:${options.path}`;
    this.priority = options.priority ?? 300;
  }

  /**
   * Load and decrypt configuration from vault file.
   *
   * @returns Decrypted configuration object
   * @throws {ConfigSourceError} If file not found or decryption fails
   * @since 0.1.0
   * @security Validates authentication tag. Throws on tampering.
   */
  async load(): Promise<Record<string, unknown>> {
    try {
      const bytes = await fs.readFile(this.path);
      return await decryptBytes(bytes, this.password);
    } catch (error) {
      if (error instanceof Error) {
        throw new ConfigSourceError(this.name, error.message, error);
      }
      throw new ConfigSourceError(this.name, "Unknown error");
    }
  }

  /**
   * Watch for vault file changes.
   *
   * @param callback - Change callback
   * @returns Unsubscribe function
   * @since 0.1.0
   */
  watch(callback: (config: Record<string, unknown>) => void): () => void {
    let stopped = false;
    let lastMtime = 0;

    const check = async () => {
      if (stopped) return;
      try {
        const stat = await fs.stat(this.path);
        if (stat.mtimeMs > lastMtime) {
          lastMtime = stat.mtimeMs;
          const config = await this.load();
          callback(config);
        }
      } catch {
        // Ignore errors during watch
      }
      if (!stopped) {
        setTimeout(check, 1000);
      }
    };

    check();

    return () => {
      stopped = true;
    };
  }
}