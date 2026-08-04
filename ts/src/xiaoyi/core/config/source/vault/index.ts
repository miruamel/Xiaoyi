/**
 * Configuration vault source for encrypted secrets.
 *
 * Path: xiaoyi.core.config.source.vault
 *
 * Layer hierarchy:
 * - 0: core
 * - 1: config
 * - 2: source
 * - 3: vault
 * - 4: encrypt/decrypt/aes/key
 */

export interface VaultConfig {
  path: string;
  key: Uint8Array;
}

export class Vault {
  readonly path: string;
  readonly key: Uint8Array;

  constructor(config: VaultConfig) {
    this.path = config.path;
    this.key = config.key;
  }

  static new(path: string, key: Uint8Array): Vault {
    return new Vault({ path, key });
  }

  decrypt(ciphertext: Uint8Array): Uint8Array {
    if (this.key.length === 0) {
      throw new Error("[config] vault key is empty");
    }
    // Placeholder: integrate with AES-GCM
    return ciphertext;
  }

  encrypt(plaintext: Uint8Array): Uint8Array {
    if (this.key.length === 0) {
      throw new Error("[config] vault key is empty");
    }
    // Placeholder: integrate with AES-GCM
    return plaintext;
  }
}

// AES key management
export namespace aes {
  export function generateKey(): Uint8Array {
    // In real implementation, use Web Crypto API
    const key = new Uint8Array(32);
    crypto.getRandomValues(key);
    return key;
  }
}

// Key derivation from password
export namespace key {
  export async function deriveKey(password: string, salt: Uint8Array): Promise<Uint8Array> {
    const encoder = new TextEncoder();
    const keyMaterial = await crypto.subtle.importKey(
      "raw",
      encoder.encode(password),
      { name: "PBKDF2" },
      false,
      ["deriveBits"]
    );
    const derived = await crypto.subtle.deriveBits(
      { name: "PBKDF2", salt, iterations: 100_000, hash: "SHA-256" },
      keyMaterial,
      256
    );
    return new Uint8Array(derived);
  }
}