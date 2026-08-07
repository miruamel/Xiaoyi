"""
# Vault Decryption

`decrypt` provides high-level decryption for vault configuration.

Path: `xiaoyi.core.config.source.vault.decrypt`

@module xiaoyi.core.config.source.vault.decrypt
@brief High-level vault decryption
@group Core
@security Decrypts configuration data. Handle keys securely.
@since 0.1.0
@author Miruamel
@see xiaoyi.core.config.source.vault.aes
@see xiaoyi.core.config.source.vault.key
"""

import json
from .aes import decrypt_aes, EncryptedData
from .key import derive_key, KeyDerivationOptions


def deserialize_vault(
    data: bytes,
    salt_length: int = 16
) -> tuple[bytes, EncryptedData]:
    """
    Deserialize vault data from bytes.

    @param data Serialized vault data
    @param salt_length Salt length in bytes (default 16)
    @return Tuple of (salt, encrypted data)
    @since 0.1.0
    @group Core
    """
    salt = data[:salt_length]
    offset = salt_length

    nonce = data[offset : offset + 12]
    offset += 12

    tag = data[offset : offset + 16]
    offset += 16

    ciphertext = data[offset:]

    return salt, EncryptedData(
        ciphertext=ciphertext,
        tag=tag,
        nonce=nonce,
        algorithm="AES-256-GCM",
    )


async def decrypt_config(
    salt: bytes,
    data: EncryptedData,
    password: str
) -> dict:
    """
    Decrypt vault data to configuration object.

    @param salt Derivation salt
    @param data Encrypted data
    @param password Password for key derivation
    @return Decrypted configuration
    @throws ValueError If decryption fails
    @since 0.1.0
    @group Core
    @security Validates authentication tag. Throws on tampering or wrong password.
    @example
    ```python
    config = await decrypt_config(salt, data, "my-password")
    ```
    """
    key = await derive_key(KeyDerivationOptions(password=password, salt=salt))
    plaintext = await decrypt_aes(key, data)
    return json.loads(plaintext.decode("utf-8"))


async def decrypt_vault_bytes(
    data: bytes,
    password: str
) -> dict:
    """
    Decrypt serialized vault bytes.

    @param data Serialized vault data
    @param password Password for key derivation
    @return Decrypted configuration
    @since 0.1.0
    @group Core
    """
    salt, encrypted = deserialize_vault(data)
    return await decrypt_config(salt, encrypted, password)


__all__ = [
    "deserialize_vault",
    "decrypt_config",
    "decrypt_vault_bytes",
]