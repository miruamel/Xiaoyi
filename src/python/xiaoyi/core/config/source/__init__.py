"""
# Configuration Sources

`source` defines the configuration source trait and implementations.

Path: `xiaoyi.core.config.source`

- Layer 0: `core`
- Layer 1: `config`
- Layer 2: `source` — configuration source abstraction.
- Layer 3: `file`/`env`/`vault` — concrete sources.

@package xiaoyi.core.config.source
@brief Configuration source abstraction
@group Core
@since 0.1.0
@author Miruamel
@see xiaoyi.core.config
@see xiaoyi.core.config.source.file
@see xiaoyi.core.config.source.env
@see xiaoyi.core.config.source.vault
"""

from .file import *
from .env import *
from .vault import *
from ..config import ConfigSource, ConfigSourceError

__all__ = [
    "ConfigSource",
    "ConfigSourceError",
    "FileSource",
    "FileSourceOptions",
    "ConfigFormat",
    "EnvSource",
    "EnvSourceOptions",
    "VaultSource",
    "VaultSourceOptions",
]