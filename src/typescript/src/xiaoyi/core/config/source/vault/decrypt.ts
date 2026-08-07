/**
 * # Vault Decryption
 *
 * `decrypt` provides high-level decryption for vault configuration.
 *
 * Path: `xiaoyi.core.config.source.vault.decrypt`
 *
 * @module core.config.source.vault.decrypt
 * @brief High-level vault decryption
 * @group Core
 * @security Decrypts configuration data. Handle keys securely.
 * @since 0.1.0
 * @author Miruamel
 * @see core.config.source.vault.aes
 * @see core.config.source.vault.key
 */
import { decrypt as aesDecrypt, EncryptedData } from "./aes";
import { deriveKey } from "./key";

/**
 * Deserialize vault data from bytes.
 *
 * @param bytes - Serialized vault data
 * @param saltLength - Salt length in bytes (default 16)
 * @returns Salt and encrypted data
 * @since 0.1.0
 * @group Core
 */
export function deserialize(
  bytes: Uint8Array,
  saltLength: number = 16
): { salt: Uint8Array; data: EncryptedData } {
  const salt = bytes.slice(0, saltLength);
  let offset = saltLength;

  const nonce = bytes.slice(offset, offset + 12);
  offset += 12;

  const tag = bytes.slice(offset, offset + 16);
  offset += 16;

  const ciphertext = bytes.slice(offset);

  return {
    salt,
    data: {
      ciphertext,
      tag,
      nonce,
      algorithm: "AES-256-GCM",
    },
  };
}

/**
 * Decrypt vault data to configuration object.
 *
 * @param salt - Derivation salt
 * @param data - Encrypted data
 * @param password - Password for key derivation
 * @returns Decrypted configuration
 * @throws {Error} If decryption fails
 * @since 0.1.0
 * @group Core
 * @security Validates authentication tag. Throws on tampering or wrong password.
 * @example
 * ```typescript
 * const config = await decrypt(salt, data, "my-password");
 * ```
 */
export async function decrypt(
  salt: Uint8Array,
  data: EncryptedData,
  password: string
): Promise<Record<string, unknown>> {
  const key = await deriveKey({ password, salt });
  const plaintext = await aesDecrypt(key, data);
  return JSON.parse(new TextDecoder().decode(plaintext));
}

/**
 * Decrypt serialized vault bytes.
 *
 * @param bytes - Serialized vault data
 * @param password - Password for key derivation
 * @returns Decrypted configuration
 * @since 0.1.0
 * @group Core
 */
export async function decryptBytes(
  bytes: Uint8Array,
  password: string
): Promise<Record<string, unknown>> {
  const { salt, data } = deserialize(bytes);
  return decrypt(salt, data, password);
}