"""
# File Configuration Source

`file` provides file-based configuration loading (JSON, YAML, TOML).

Path: `xiaoyi.core.config.source.file`

- Layer 0: `core`
- Layer 1: `config`
- Layer 2: `source`
- Layer 3: `file`

@module xiaoyi.core.config.source.file
@brief File-based configuration source
@group Core
@since 0.1.0
@author Miruamel
@see xiaoyi.core.config.source
@see xiaoyi.core.config.source.env
@see xiaoyi.core.config.source.vault
"""

import json
import os
from pathlib import Path
from typing import Any, Dict, Optional
from enum import Enum

from ..config import ConfigSource, ConfigSourceError


class ConfigFormat(str, Enum):
    """Supported config file formats."""

    #: JSON format.
    JSON = "json"
    #: YAML format.
    YAML = "yaml"
    #: TOML format.
    TOML = "toml"


class FileSourceOptions:
    """
    File source options.

    @brief Options for file configuration source
    @group Core
    @since 0.1.0
    """

    def __init__(
        self,
        path: str,
        format: Optional[ConfigFormat] = None,
        watch: bool = False,
        priority: int = 100,
    ):
        """
        Create file source options.

        @param path File path
        @param format File format (auto-detected from extension if not specified)
        @param watch Watch for changes
        @param priority Source priority
        @since 0.1.0
        """
        self.path = path
        self.format = format
        self.watch = watch
        self.priority = priority


class FileSource(ConfigSource):
    """
    File-based configuration source.

    @brief File-based configuration source
    @group Core
    @since 0.1.0
    @example
    ```python
    source = FileSource(FileSourceOptions(path="./config.toml"))
    config = await source.load()
    ```
    """

    def __init__(self, options: FileSourceOptions):
        self._path = options.path
        self._format = options.format or self._detect_format()
        self._watch = options.watch
        self._priority = options.priority
        self._name = f"file:{self._path}"

    @property
    def name(self) -> str:
        return self._name

    @property
    def priority(self) -> int:
        return self._priority

    def _detect_format(self) -> ConfigFormat:
        ext = Path(self._path).suffix.lower()
        if ext == ".json":
            return ConfigFormat.JSON
        elif ext in (".yaml", ".yml"):
            return ConfigFormat.YAML
        elif ext == ".toml":
            return ConfigFormat.TOML
        return ConfigFormat.JSON

    async def load(self) -> Dict[str, Any]:
        """
        Load configuration from file.

        @return Configuration object
        @throws ConfigSourceError If loading fails
        @since 0.1.0
        """
        try:
            path = Path(self._path)
            if not path.exists():
                raise ConfigSourceError(self.name, f"File not found: {self._path}")

            content = path.read_text(encoding="utf-8")

            if self._format == ConfigFormat.JSON:
                return json.loads(content)
            elif self._format == ConfigFormat.YAML:
                try:
                    import yaml
                    return yaml.safe_load(content) or {}
                except ImportError:
                    raise ConfigSourceError(
                        self.name, "PyYAML required for YAML support"
                    )
            elif self._format == ConfigFormat.TOML:
                try:
                    import tomllib
                    return tomllib.loads(content)
                except ImportError:
                    try:
                        import tomli
                        return tomli.loads(content)
                    except ImportError:
                        raise ConfigSourceError(
                            self.name, "tomllib or tomli required for TOML support"
                        )
            return {}
        except ConfigSourceError:
            raise
        except Exception as e:
            raise ConfigSourceError(self.name, str(e), e)

    def watch(self, callback) -> None:
        """
        Watch for file changes (not implemented in async version).

        @param callback Change callback
        @returns No-op unsubscribe
        @since 0.1.0
        """
        # File watching would require async file system events
        # This is a placeholder for future implementation
        pass


__all__ = [
    "ConfigFormat",
    "FileSourceOptions",
    "FileSource",
]