/**
 * # Vault Encryption
 *
 * `encrypt` provides high-level encryption for vault configuration.
 *
 * Path: `xiaoyi.core.config.source.vault.encrypt`
 *
 * @module core.config.source.vault.encrypt
 * @brief High-level vault encryption
 * @group Core
 * @security Encrypts configuration data. Handle keys securely.
 * @since 0.1.0
 * @author Miruamel
 * @see core.config.source.vault.aes
 * @see core.config.source.vault.key
 */
import { encrypt as aesEncrypt, EncryptedData } from "./aes";
import { deriveKey, generateSalt } from "./key";

/**
 * Encrypt configuration object to vault format.
 *
 * @param config - Configuration to encrypt
 * @param password - Password for key derivation
 * @returns Encrypted vault data (salt + encrypted config)
 * @since 0.1.0
 * @group Core
 * @security Uses PBKDF2 + AES-256-GCM. Salt is randomly generated.
 * @example
 * ```typescript
 * const vaultData = await encrypt({ apiKey: "secret" }, "my-password");
 * ```
 */
export async function encrypt(
  config: Record<string, unknown>,
  password: string
): Promise<Uint8Array> {
  const salt = generateSalt();
  const key = await deriveKey({ password, salt });
  const plaintext = new TextEncoder().encode(JSON.stringify(config));
  const data = await aesEncrypt(key, plaintext);
  return serialize(salt, data);
}

/**
 * Serialize encrypted vault data to bytes.
 *
 * @param salt - Derivation salt
 * @param data - Encrypted data
 * @returns Serialized bytes (salt || nonce || tag || ciphertext)
 * @since 0.1.0
 * @group Core
 */
export function serialize(salt: Uint8Array, data: EncryptedData): Uint8Array {
  const totalLength = salt.length + data.nonce.length + data.tag.length + data.ciphertext.length;
  const result = new Uint8Array(totalLength);
  let offset = 0;
  result.set(salt, offset);
  offset += salt.length;
  result.set(data.nonce, offset);
  offset += data.nonce.length;
  result.set(data.tag, offset);
  offset += data.tag.length;
  result.set(data.ciphertext, offset);
  return result;
}