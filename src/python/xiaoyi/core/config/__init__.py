"""
# Configuration Module

`config` provides configuration management with multiple sources
(file, environment, vault) and a builder pattern.

Path: `xiaoyi.core.config`

- Layer 0: `core`
- Layer 1: `config` — configuration management.
- Layer 2: `source` — configuration sources.
- Layer 3: `file`/`env`/`vault` — concrete sources.

@package xiaoyi.core.config
@brief Configuration management with multiple sources
@group Core
@since 0.1.0
@author Miruamel
@see xiaoyi.core.config.source
@see xiaoyi.core.config.source.file
@see xiaoyi.core.config.source.env
@see xiaoyi.core.config.source.vault
"""

from .config import *
from .builder import *
from .source import *

__all__ = [
    "Config",
    "ConfigSource",
    "ConfigSourceError",
    "ConfigBuilder",
    "ConfigValue",
    "ConfigMergeStrategy",
]