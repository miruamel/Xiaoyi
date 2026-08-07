"""
# Core Module

`core` provides configuration, error handling, and result types for the Xiaoyi framework.

Path: `xiaoyi.core`

- Layer 0: `core`
- Layer 1: `config` — configuration management.
- Layer 2: `error` — error types and handling.
- Layer 3: `result` — result/status types.

@package xiaoyi.core
@brief Core infrastructure: config, error, result
@group Core
@since 0.1.0
@author Miruamel
@see xiaoyi.core.config
@see xiaoyi.core.error
@see xiaoyi.core.result
"""

from .error import *
from .config import *
from .result import *

__all__ = [
    "ErrorKind",
    "XiaoyiError",
    "create_error",
    "is_xiaoyi_error",
    "Config",
    "ConfigSource",
    "ConfigSourceError",
    "ConfigBuilder",
    "ConfigValue",
    "ConfigMergeStrategy",
    "Result",
    "ok",
    "err",
    "is_ok",
    "is_err",
    "unwrap",
    "unwrap_err",
    "map",
    "map_err",
    "and_then",
    "or_else",
]