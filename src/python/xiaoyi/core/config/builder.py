"""
# Configuration Builder

`builder` provides a fluent builder pattern for constructing configurations.

Path: `xiaoyi.core.config.builder`

@module xiaoyi.core.config.builder
@brief Fluent configuration builder
@group Core
@since 0.1.0
@author Miruamel
@see xiaoyi.core.config
@see xiaoyi.core.config.source
"""

from typing import List
from .config import Config, ConfigSource


class ConfigBuilder:
    """
    Configuration builder for composing multiple sources.

    @brief Build configuration from multiple sources
    @group Core
    @since 0.1.0
    @example
    ```python
    config = await (ConfigBuilder()
        .add_source(FileSource(path="./config.toml"))
        .add_source(EnvSource(prefix="XIAOYI_"))
        .build())
    ```
    """

    def __init__(self):
        self._sources: List[ConfigSource] = []

    def add_source(self, source: ConfigSource) -> "ConfigBuilder":
        """
        Add a configuration source.

        @param source Configuration source to add
        @return this (for chaining)
        @since 0.1.0
        """
        self._sources.append(source)
        return self

    def add_sources(self, sources: List[ConfigSource]) -> "ConfigBuilder":
        """
        Add multiple configuration sources.

        @param sources Configuration sources to add
        @return this (for chaining)
        @since 0.1.0
        """
        self._sources.extend(sources)
        return self

    async def build(self) -> Config:
        """
        Build configuration by loading all sources in priority order.

        @return Merged configuration
        @since 0.1.0
        """
        # Sort by priority (lower first, so higher priority overrides)
        sorted_sources = sorted(self._sources, key=lambda s: s.priority)

        data: Dict[str, Any] = {}
        source_names: List[str] = []

        for source in sorted_sources:
            loaded = await source.load()
            data.update(loaded)
            source_names.append(source.name)

        return Config(data=data, sources=source_names)


__all__ = ["ConfigBuilder"]