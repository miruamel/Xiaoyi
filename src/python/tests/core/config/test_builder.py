"""
Test suite for xiaoyi.core.config.builder module.

@package xiaoyi.tests.core.config
@brief Tests for ConfigBuilder
@since 0.1.0
"""

import pytest
from xiaoyi.core.config.builder import ConfigBuilder
from xiaoyi.core.config.source.file import FileSource, FileSourceOptions, ConfigFormat
from xiaoyi.core.config.source.env import EnvSource, EnvSourceOptions


class TestConfigBuilder:
    """Tests for ConfigBuilder class."""

    def test_builder_creation(self):
        """Test creating a ConfigBuilder."""
        builder = ConfigBuilder()
        assert builder._sources == []

    def test_add_source(self):
        """Test adding a single source."""
        builder = ConfigBuilder()
        source = FileSource(FileSourceOptions(path="./config.toml"))
        result = builder.add_source(source)
        assert result is builder  # Returns self for chaining
        assert len(builder._sources) == 1
        assert builder._sources[0] is source

    def test_add_sources(self):
        """Test adding multiple sources."""
        builder = ConfigBuilder()
        source1 = FileSource(FileSourceOptions(path="./config1.toml"))
        source2 = FileSource(FileSourceOptions(path="./config2.toml"))
        result = builder.add_sources([source1, source2])
        assert result is builder  # Returns self for chaining
        assert len(builder._sources) == 2

    def test_add_sources_empty(self):
        """Test adding empty list of sources."""
        builder = ConfigBuilder()
        builder.add_sources([])
        assert builder._sources == []

    def test_chaining(self):
        """Test method chaining."""
        builder = ConfigBuilder()
        source1 = FileSource(FileSourceOptions(path="./config1.toml"))
        source2 = FileSource(FileSourceOptions(path="./config2.toml"))
        result = builder.add_source(source1).add_source(source2)
        assert result is builder
        assert len(builder._sources) == 2

    @pytest.mark.asyncio
    async def test_build_empty(self):
        """Test building with no sources."""
        builder = ConfigBuilder()
        config = await builder.build()
        assert config.data == {}
        assert config.sources == []

    @pytest.mark.asyncio
    async def test_build_single_source(self, tmp_path):
        """Test building with a single file source."""
        config_file = tmp_path / "config.toml"
        config_file.write_text('app = { name = "test", version = "1.0" }')

        builder = ConfigBuilder()
        builder.add_source(FileSource(FileSourceOptions(path=str(config_file))))

        config = await builder.build()
        assert config.data == {"app": {"name": "test", "version": "1.0"}}
        assert len(config.sources) == 1

    @pytest.mark.asyncio
    async def test_build_priority_order(self, tmp_path):
        """Test that sources are merged by priority (lower first)."""
        # Priority 100 (lower = higher priority in sort, but wait - lower priority number means higher priority?)
        # Looking at builder: sorted by priority, lower first, so higher priority number overrides
        # Actually: "Sort by priority (lower first, so higher priority overrides)"
        # This means priority 100 comes before 200, and 200 overrides 100
        config1 = tmp_path / "config1.toml"
        config1.write_text('app = { name = "config1" }')

        config2 = tmp_path / "config2.toml"
        config2.write_text('app = { name = "config2" }')

        builder = ConfigBuilder()
        builder.add_source(FileSource(FileSourceOptions(path=str(config1), priority=100)))
        builder.add_source(FileSource(FileSourceOptions(path=str(config2), priority=200)))

        config = await builder.build()
        # Priority 200 overrides 100, so config2 wins
        assert config.data == {"app": {"name": "config2"}}

    @pytest.mark.asyncio
    async def test_build_multiple_sources(self, tmp_path, monkeypatch):
        """Test building with multiple source types."""
        config_file = tmp_path / "config.toml"
        config_file.write_text('app = { name = "file" }')

        monkeypatch.setenv("XIAOYI_APP_NAME", "env")

        builder = ConfigBuilder()
        builder.add_source(FileSource(FileSourceOptions(path=str(config_file), priority=100)))
        builder.add_source(EnvSource(EnvSourceOptions(prefix="XIAOYI_", priority=200)))

        config = await builder.build()
        # Env source has higher priority (200 > 100), so it overrides
        assert config.data.get("app") == "env"
        assert len(config.sources) == 2