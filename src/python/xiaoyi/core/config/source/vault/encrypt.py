"""
# Vault Encryption

`encrypt` provides high-level encryption for vault configuration.

Path: `xiaoyi.core.config.source.vault.encrypt`

@module xiaoyi.core.config.source.vault.encrypt
@brief High-level vault encryption
@group Core
@security Encrypts configuration data. Handle keys securely.
@since 0.1.0
@author Miruamel
@see xiaoyi.core.config.source.vault.aes
@see xiaoyi.core.config.source.vault.key
"""

import json
from .aes import encrypt_aes, EncryptedData
from .key import derive_key, generate_salt, KeyDerivationOptions


async def encrypt_config(
    config: dict,
    password: str
) -> tuple[bytes, EncryptedData]:
    """
    Encrypt configuration object to vault format.

    @param config Configuration to encrypt
    @param password Password for key derivation
    @return Tuple of (salt, encrypted data)
    @since 0.1.0
    @group Core
    @security Uses PBKDF2 + AES-256-GCM. Salt is randomly generated.
    @example
    ```python
    salt, encrypted = await encrypt_config({"api_key": "secret"}, "my-password")
    ```
    """
    salt = generate_salt()
    key = await derive_key(KeyDerivationOptions(password=password, salt=salt))
    plaintext = json.dumps(config).encode("utf-8")
    data = await encrypt_aes(key, plaintext)
    return salt, data


def serialize_vault(salt: bytes, data: EncryptedData) -> bytes:
    """
    Serialize encrypted vault data to bytes.

    @param salt Derivation salt
    @param data Encrypted data
    @return Serialized bytes (salt || nonce || tag || ciphertext)
    @since 0.1.0
    @group Core
    """
    return salt + data.nonce + data.tag + data.ciphertext


__all__ = [
    "encrypt_config",
    "serialize_vault",
]