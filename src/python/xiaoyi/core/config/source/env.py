"""
# Environment Configuration Source

`env` provides environment variable-based configuration loading.

Path: `xiaoyi.core.config.source.env`

- Layer 0: `core`
- Layer 1: `config`
- Layer 2: `source`
- Layer 3: `env`

@module xiaoyi.core.config.source.env
@brief Environment variable configuration source
@group Core
@since 0.1.0
@author Miruamel
@see xiaoyi.core.config.source
@see xiaoyi.core.config.source.file
@see xiaoyi.core.config.source.vault
"""

import json
import os
from typing import Any, Callable, Dict, Optional

from ..config import ConfigSource, ConfigSourceError


class EnvSourceOptions:
    """
    Environment source options.

    @brief Options for environment configuration source
    @group Core
    @since 0.1.0
    """

    def __init__(
        self,
        prefix: str = "XIAOYI_",
        priority: int = 200,
        parser: Optional[Callable[[str], Any]] = None,
    ):
        """
        Create environment source options.

        @param prefix Environment variable prefix (e.g., "XIAOYI_")
        @param priority Source priority
        @param parser Custom parser for values
        @since 0.1.0
        """
        self.prefix = prefix
        self.priority = priority
        self.parser = parser


class EnvSource(ConfigSource):
    """
    Environment variable configuration source.

    @brief Environment variable configuration source
    @group Core
    @since 0.1.0
    @example
    ```python
    source = EnvSource(EnvSourceOptions(prefix="XIAOYI_"))
    config = await source.load()
    ```
    """

    def __init__(self, options: EnvSourceOptions):
        self._prefix = options.prefix
        self._priority = options.priority
        self._parser = options.parser or self._default_parser
        self._name = f"env:{self._prefix}"

    @property
    def name(self) -> str:
        return self._name

    @property
    def priority(self) -> int:
        return self._priority

    def _default_parser(self, value: str) -> Any:
        """Default value parser - tries JSON, falls back to string."""
        try:
            return json.loads(value)
        except (json.JSONDecodeError, ValueError):
            return value

    async def load(self) -> Dict[str, Any]:
        """
        Load configuration from environment variables.

        @return Configuration object
        @since 0.1.0
        """
        result: Dict[str, Any] = {}

        for key, value in os.environ.items():
            if key.startswith(self._prefix):
                config_key = key[len(self._prefix) :].lower().replace("_", ".")
                result[config_key] = self._parser(value)

        return result

    def watch(self, callback) -> None:
        """
        Watch for environment changes (not supported).

        @param callback Change callback
        @returns No-op unsubscribe
        @since 0.1.0
        """
        # Environment variables cannot be watched reliably
        pass


__all__ = [
    "EnvSourceOptions",
    "EnvSource",
]