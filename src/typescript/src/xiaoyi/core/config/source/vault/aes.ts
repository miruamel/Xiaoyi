/**
 * # AES Encryption
 *
 * `aes` provides AES-256-GCM encryption primitives.
 *
 * Path: `xiaoyi.core.config.source.vault.aes`
 *
 * @module core.config.source.vault.aes
 * @brief AES-256-GCM encryption primitives
 * @group Core
 * @security Uses AES-256-GCM. Nonce must be unique per encryption.
 * @since 0.1.0
 * @author Miruamel
 * @see core.config.source.vault.key
 * @see core.config.source.vault.encrypt
 * @see core.config.source.vault.decrypt
 */

/**
 * Encrypted data structure.
 *
 * @brief Encrypted payload with metadata
 * @group Core
 * @since 0.1.0
 */
export interface EncryptedData {
  /** Ciphertext bytes. */
  ciphertext: Uint8Array;
  /** Authentication tag (16 bytes). */
  tag: Uint8Array;
  /** Nonce/IV (12 bytes for GCM). */
  nonce: Uint8Array;
  /** Algorithm identifier. */
  algorithm: string;
}

/**
 * Encrypt data using AES-256-GCM.
 *
 * @param key - 32-byte encryption key
 * @param plaintext - Data to encrypt
 * @param associatedData - Optional AAD for authentication
 * @returns Encrypted data with nonce and tag
 * @since 0.1.0
 * @group Core
 * @security Nonce must be unique per key. Never reuse nonce with same key.
 * @example
 * ```typescript
 * const encrypted = await encrypt(key, new TextEncoder().encode("secret"));
 * ```
 */
export async function encrypt(
  key: Uint8Array,
  plaintext: Uint8Array,
  associatedData?: Uint8Array
): Promise<EncryptedData> {
  if (key.length !== 32) {
    throw new Error("AES-256 key must be 32 bytes");
  }

  const nonce = crypto.getRandomValues(new Uint8Array(12));
  const cryptoKey = await crypto.subtle.importKey(
    "raw",
    key,
    { name: "AES-GCM" },
    false,
    ["encrypt"]
  );

  const ciphertext = await crypto.subtle.encrypt(
    { name: "AES-GCM", iv: nonce, additionalData: associatedData },
    cryptoKey,
    plaintext
  );

  // GCM returns ciphertext || tag concatenated
  const combined = new Uint8Array(ciphertext);
  const tag = combined.slice(combined.length - 16);
  const actualCiphertext = combined.slice(0, combined.length - 16);

  return {
    ciphertext: actualCiphertext,
    tag,
    nonce,
    algorithm: "AES-256-GCM",
  };
}

/**
 * Decrypt data using AES-256-GCM.
 *
 * @param key - 32-byte encryption key
 * @param data - Encrypted data to decrypt
 * @param associatedData - Optional AAD for authentication
 * @returns Decrypted plaintext
 * @throws {Error} If decryption fails (authentication tag mismatch)
 * @since 0.1.0
 * @group Core
 * @security Validates authentication tag. Throws on tampering.
 */
export async function decrypt(
  key: Uint8Array,
  data: EncryptedData,
  associatedData?: Uint8Array
): Promise<Uint8Array> {
  if (key.length !== 32) {
    throw new Error("AES-256 key must be 32 bytes");
  }

  const cryptoKey = await crypto.subtle.importKey(
    "raw",
    key,
    { name: "AES-GCM" },
    false,
    ["decrypt"]
  );

  // Combine ciphertext and tag for Web Crypto API
  const combined = new Uint8Array(data.ciphertext.length + data.tag.length);
  combined.set(data.ciphertext);
  combined.set(data.tag, data.ciphertext.length);

  try {
    const plaintext = await crypto.subtle.decrypt(
      { name: "AES-GCM", iv: data.nonce, additionalData: associatedData },
      cryptoKey,
      combined
    );
    return new Uint8Array(plaintext);
  } catch {
    throw new Error("Decryption failed: authentication tag mismatch");
  }
}