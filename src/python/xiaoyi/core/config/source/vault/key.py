"""
# Vault Key Management

`key` provides key derivation and management for vault encryption.

Path: `xiaoyi.core.config.source.vault.key`

@module xiaoyi.core.config.source.vault.key
@brief Key derivation and management for vault
@group Core
@security This module handles encryption keys. Keys must be protected.
@since 0.1.0
@author Miruamel
@see xiaoyi.core.config.source.vault
@see xiaoyi.core.config.source.vault.aes
"""

import os
from dataclasses import dataclass
from typing import Optional
import hashlib
import hmac


@dataclass
class KeyDerivationOptions:
    """
    Key derivation options.

    @brief Options for key derivation
    @group Core
    @since 0.1.0
    @security Use strong passwords and high iteration counts.
    """

    #: Password to derive key from.
    password: str
    #: Salt (random bytes, 16+ bytes recommended).
    salt: bytes
    #: Iteration count (higher = more secure, slower).
    iterations: int = 100000
    #: Key length in bytes (32 for AES-256).
    key_length: int = 32


async def derive_key(options: KeyDerivationOptions) -> bytes:
    """
    Derive encryption key from password using PBKDF2.

    @param options Derivation options
    @return Derived key (bytes)
    @since 0.1.0
    @group Core
    @security Uses PBKDF2 with SHA-256. Ensure sufficient iterations.
    @example
    ```python
    import os
    salt = os.urandom(16)
    key = await derive_key(KeyDerivationOptions(password="secret", salt=salt, iterations=100000))
    ```
    """
    # Use PBKDF2 with SHA-256
    key = hashlib.pbkdf2_hmac(
        "sha256",
        options.password.encode("utf-8"),
        options.salt,
        options.iterations,
        dklen=options.key_length,
    )
    return key


def generate_salt(length: int = 16) -> bytes:
    """
    Generate random salt for key derivation.

    @param length Salt length in bytes (default 16)
    @return Random salt
    @since 0.1.0
    @group Core
    """
    return os.urandom(length)


__all__ = [
    "KeyDerivationOptions",
    "derive_key",
    "generate_salt",
]