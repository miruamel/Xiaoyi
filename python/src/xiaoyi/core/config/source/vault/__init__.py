"""Configuration vault source for encrypted secrets.

Path: xiaoyi.core.config.source.vault

Layer hierarchy:
- 0: core
- 1: config
- 2: source
- 3: vault
- 4: encrypt/decrypt/aes/key
"""

from __future__ import annotations

import base64
import os
from dataclasses import dataclass
from pathlib import Path
from typing import Optional

from xiaoyi.core.error import XiaoyiError, ErrorKind


@dataclass
class Vault:
    """Vault configuration containing encrypted secrets."""
    path: str
    key: bytes

    @classmethod
    def new(cls, path: str, key: bytes) -> Vault:
        return cls(path=path, key=key)

    def decrypt(self, ciphertext: bytes) -> bytes:
        """Decrypt a value from the vault."""
        if not self.key:
            raise XiaoyiError(ErrorKind.CONFIG, "vault key is empty")
        # Placeholder: integrate with AES-GCM
        return ciphertext

    def encrypt(self, plaintext: bytes) -> bytes:
        """Encrypt a value for the vault."""
        if not self.key:
            raise XiaoyiError(ErrorKind.CONFIG, "vault key is empty")
        # Placeholder: integrate with AES-GCM
        return plaintext


# AES key management
def generate_key() -> bytes:
    """Generate a new AES-256 key."""
    return os.urandom(32)


def derive_key(password: str, salt: bytes) -> bytes:
    """Derive key from password using PBKDF2."""
    import hashlib
    import binascii
    # PBKDF2 with SHA256
    key = hashlib.pbkdf2_hmac('sha256', password.encode(), salt, 100_000, dklen=32)
    return key