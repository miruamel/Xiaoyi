/**
 * # Vault Key Management
 *
 * `key` provides key derivation and management for vault encryption.
 *
 * Path: `xiaoyi.core.config.source.vault.key`
 *
 * @module core.config.source.vault.key
 * @brief Key derivation and management for vault
 * @group Core
 * @security This module handles encryption keys. Keys must be protected.
 * @since 0.1.0
 * @author Miruamel
 * @see core.config.source.vault
 * @see core.config.source.vault.aes
 */

/**
 * Key derivation options.
 *
 * @brief Options for key derivation
 * @group Core
 * @since 0.1.0
 * @security Use strong passwords and high iteration counts.
 */
export interface KeyDerivationOptions {
  /** Password to derive key from. */
  password: string;
  /** Salt (random bytes, 16+ bytes recommended). */
  salt: Uint8Array;
  /** Iteration count (higher = more secure, slower). */
  iterations?: number;
  /** Key length in bytes (32 for AES-256). */
  keyLength?: number;
}

/**
 * Derive encryption key from password using PBKDF2.
 *
 * @param options - Derivation options
 * @returns Derived key (Uint8Array)
 * @since 0.1.0
 * @group Core
 * @security Uses PBKDF2 with SHA-256. Ensure sufficient iterations.
 * @example
 * ```typescript
 * const salt = crypto.getRandomValues(new Uint8Array(16));
 * const key = await deriveKey({ password: "secret", salt, iterations: 100000 });
 * ```
 */
export async function deriveKey(options: KeyDerivationOptions): Promise<Uint8Array> {
  const { password, salt, iterations = 100000, keyLength = 32 } = options;

  const encoder = new TextEncoder();
  const keyMaterial = await crypto.subtle.importKey(
    "raw",
    encoder.encode(password),
    { name: "PBKDF2" },
    false,
    ["deriveBits"]
  );

  const derived = await crypto.subtle.deriveBits(
    {
      name: "PBKDF2",
      salt,
      iterations,
      hash: "SHA-256",
    },
    keyMaterial,
    keyLength * 8
  );

  return new Uint8Array(derived);
}

/**
 * Generate random salt for key derivation.
 *
 * @param length - Salt length in bytes (default 16)
 * @returns Random salt
 * @since 0.1.0
 * @group Core
 */
export function generateSalt(length: number = 16): Uint8Array {
  return crypto.getRandomValues(new Uint8Array(length));
}