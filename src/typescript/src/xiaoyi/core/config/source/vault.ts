/**
 * # Vault Configuration Source
 *
 * `vault` provides encrypted configuration storage with AES-256-GCM.
 *
 * Path: `xiaoyi.core.config.source.vault`
 *
 * - Layer 0: `core`
 * - Layer 1: `config`
 * - Layer 2: `source`
 * - Layer 3: `vault` — encrypted configuration.
 * - Layer 4: `key`/`encrypt`/`decrypt`/`aes` — crypto primitives.
 *
 * @module core.config.source.vault
 * @brief Encrypted configuration source with AES-256-GCM
 * @group Core
 * @security This module handles sensitive configuration data. Keys must be protected.
 * @since 0.1.0
 * @author Miruamel
 * @see core.config.source
 * @see core.config.source.vault.key
 * @see core.config.source.vault.encrypt
 * @see core.config.source.vault.decrypt
 */
export { deriveKey, generateSalt, type KeyDerivationOptions } from "./vault/key";
export { encrypt as encryptConfig, serialize as serializeVault } from "./vault/encrypt";
export { deserialize, decrypt as decryptConfig, decryptBytes } from "./vault/decrypt";
export { type EncryptedData, encrypt as encryptAes, decrypt as decryptAes } from "./vault/aes";
export { VaultSource, type VaultSourceOptions } from "./vault-source";