"""
# Configuration Core Types

`config` provides core configuration types and interfaces.

Path: `xiaoyi.core.config`

@module xiaoyi.core.config.config
@brief Core configuration types
@group Core
@since 0.1.0
@author Miruamel
@see xiaoyi.core.config.source
@see xiaoyi.core.config.builder
"""

from typing import Any, Dict, Optional, Protocol, runtime_checkable
from dataclasses import dataclass, field


@dataclass
class Config:
    """
    Configuration object type.

    @brief Configuration data structure
    @group Core
    @since 0.1.0
    """

    #: Configuration data as key-value pairs.
    data: Dict[str, Any] = field(default_factory=dict)
    #: Source metadata.
    sources: list[str] = field(default_factory=list)


@runtime_checkable
class ConfigSource(Protocol):
    """
    Configuration source interface.

    @brief Abstract configuration source
    @group Core
    @since 0.1.0
    """

    #: Source name.
    @property
    def name(self) -> str: ...

    #: Source priority (higher = loaded later, overrides earlier).
    @property
    def priority(self) -> int: ...

    def load(self) -> Dict[str, Any]: ...

    def watch(self, callback) -> None: ...


class ConfigSourceError(Exception):
    """
    Configuration source error.

    @brief Error from configuration source
    @group Core
    @since 0.1.0
    """

    def __init__(self, source: str, message: str, cause: Optional[Exception] = None):
        super().__init__(f"Config source '{source}': {message}")
        self.source = source
        self.cause = cause


# Type alias for configuration values
ConfigValue = str | int | float | bool | None | list["ConfigValue"] | Dict[str, "ConfigValue"]


class ConfigMergeStrategy(str):
    """
    Configuration merge strategy.

    @brief How to merge config layers
    @group Core
    @since 0.1.0
    """

    #: Deep merge objects.
    DEEP = "deep"
    #: Shallow merge (replace).
    SHALLOW = "shallow"
    #: Replace entirely.
    REPLACE = "replace"


__all__ = [
    "Config",
    "ConfigSource",
    "ConfigSourceError",
    "ConfigValue",
    "ConfigMergeStrategy",
]