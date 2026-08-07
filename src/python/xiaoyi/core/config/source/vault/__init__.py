"""
# Vault Configuration Source

`vault` provides encrypted configuration storage with AES-256-GCM.

Path: `xiaoyi.core.config.source.vault`

- Layer 0: `core`
- Layer 1: `config`
- Layer 2: `source`
- Layer 3: `vault` — encrypted configuration.
- Layer 4: `key`/`encrypt`/`decrypt`/`aes` — crypto primitives.

@module xiaoyi.core.config.source.vault
@brief Encrypted configuration source with AES-256-GCM
@group Core
@security This module handles sensitive configuration data. Keys must be protected.
@since 0.1.0
@author Miruamel
@see xiaoyi.core.config.source
@see xiaoyi.core.config.source.vault.key
@see xiaoyi.core.config.source.vault.encrypt
@see xiaoyi.core.config.source.vault.decrypt
"""

from .key import *
from .encrypt import *
from .decrypt import *
from .aes import *
from .vault_source import *

__all__ = [
    "derive_key",
    "generate_salt",
    "KeyDerivationOptions",
    "encrypt_config",
    "serialize_vault",
    "decrypt_config",
    "deserialize_vault",
    "decrypt_vault_bytes",
    "encrypt_aes",
    "decrypt_aes",
    "EncryptedData",
    "VaultSource",
    "VaultSourceOptions",
]