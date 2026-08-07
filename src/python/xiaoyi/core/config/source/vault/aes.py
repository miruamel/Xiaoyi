"""
# AES Encryption

`aes` provides AES-256-GCM encryption primitives.

Path: `xiaoyi.core.config.source.vault.aes`

@module xiaoyi.core.config.source.vault.aes
@brief AES-256-GCM encryption primitives
@group Core
@security Uses AES-256-GCM. Nonce must be unique per encryption.
@since 0.1.0
@author Miruamel
@see xiaoyi.core.config.source.vault.key
@see xiaoyi.core.config.source.vault.encrypt
@see xiaoyi.core.config.source.vault.decrypt
"""

import os
from dataclasses import dataclass
from typing import Optional
from cryptography.hazmat.primitives.ciphers.aead import AESGCM


@dataclass
class EncryptedData:
    """
    Encrypted data structure.

    @brief Encrypted payload with metadata
    @group Core
    @since 0.1.0
    """

    #: Ciphertext bytes.
    ciphertext: bytes
    #: Authentication tag (16 bytes).
    tag: bytes
    #: Nonce/IV (12 bytes for GCM).
    nonce: bytes
    #: Algorithm identifier.
    algorithm: str = "AES-256-GCM"


async def encrypt_aes(
    key: bytes,
    plaintext: bytes,
    associated_data: Optional[bytes] = None
) -> EncryptedData:
    """
    Encrypt data using AES-256-GCM.

    @param key 32-byte encryption key
    @param plaintext Data to encrypt
    @param associated_data Optional AAD for authentication
    @return Encrypted data with nonce and tag
    @since 0.1.0
    @group Core
    @security Nonce must be unique per key. Never reuse nonce with same key.
    @example
    ```python
    encrypted = await encrypt_aes(key, b"secret data")
    ```
    """
    if len(key) != 32:
        raise ValueError("AES-256 key must be 32 bytes")

    nonce = os.urandom(12)
    aesgcm = AESGCM(key)

    # AESGCM.encrypt returns ciphertext + tag concatenated
    ciphertext_with_tag = aesgcm.encrypt(nonce, plaintext, associated_data)

    # Split ciphertext and tag (tag is last 16 bytes)
    tag = ciphertext_with_tag[-16:]
    ciphertext = ciphertext_with_tag[:-16]

    return EncryptedData(
        ciphertext=ciphertext,
        tag=tag,
        nonce=nonce,
        algorithm="AES-256-GCM",
    )


async def decrypt_aes(
    key: bytes,
    data: EncryptedData,
    associated_data: Optional[bytes] = None
) -> bytes:
    """
    Decrypt data using AES-256-GCM.

    @param key 32-byte encryption key
    @param data Encrypted data to decrypt
    @param associated_data Optional AAD for authentication
    @return Decrypted plaintext
    @throws ValueError If decryption fails (authentication tag mismatch)
    @since 0.1.0
    @group Core
    @security Validates authentication tag. Throws on tampering.
    """
    if len(key) != 32:
        raise ValueError("AES-256 key must be 32 bytes")

    aesgcm = AESGCM(key)

    # Combine ciphertext and tag for cryptography library
    combined = data.ciphertext + data.tag

    try:
        plaintext = aesgcm.decrypt(data.nonce, combined, associated_data)
        return plaintext
    except Exception as e:
        raise ValueError("Decryption failed: authentication tag mismatch") from e


__all__ = [
    "EncryptedData",
    "encrypt_aes",
    "decrypt_aes",
]