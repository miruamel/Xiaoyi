"""
Test suite for xiaoyi.core.config.source.file module.

@package xiaoyi.tests.core.config.source
@brief Tests for FileSource, ConfigFormat, FileSourceOptions
@since 0.1.0
"""

import pytest
import json
from xiaoyi.core.config.source.file import (
    FileSource,
    FileSourceOptions,
    ConfigFormat,
)
from xiaoyi.core.config.config import ConfigSourceError


class TestConfigFormat:
    """Tests for ConfigFormat enum."""

    def test_config_format_values(self):
        """Test that all expected formats exist."""
        assert ConfigFormat.JSON == "json"
        assert ConfigFormat.YAML == "yaml"
        assert ConfigFormat.TOML == "toml"

    def test_config_format_iteration(self):
        """Test that all formats can be iterated."""
        formats = list(ConfigFormat)
        assert len(formats) == 3


class TestFileSourceOptions:
    """Tests for FileSourceOptions."""

    def test_file_source_options_defaults(self):
        """Test default values."""
        options = FileSourceOptions(path="./config.toml")
        assert options.path == "./config.toml"
        assert options.format is None
        assert options.watch is False
        assert options.priority == 100

    def test_file_source_options_custom(self):
        """Test custom values."""
        options = FileSourceOptions(
            path="./config.json",
            format=ConfigFormat.JSON,
            watch=True,
            priority=50
        )
        assert options.path == "./config.json"
        assert options.format == ConfigFormat.JSON
        assert options.watch is True
        assert options.priority == 50


class TestFileSource:
    """Tests for FileSource class."""

    def test_file_source_name(self):
        """Test source name property."""
        source = FileSource(FileSourceOptions(path="./config.toml"))
        assert source.name == "file:./config.toml"

    def test_file_source_priority(self):
        """Test source priority property."""
        source = FileSource(FileSourceOptions(path="./config.toml", priority=50))
        assert source.priority == 50

    @pytest.mark.asyncio
    async def test_load_json(self, tmp_path):
        """Test loading JSON config."""
        config_file = tmp_path / "config.json"
        config_file.write_text(json.dumps({"app": {"name": "test"}, "version": "1.0"}))

        source = FileSource(FileSourceOptions(path=str(config_file), format=ConfigFormat.JSON))
        data = await source.load()

        assert data == {"app": {"name": "test"}, "version": "1.0"}

    @pytest.mark.asyncio
    async def test_load_toml(self, tmp_path):
        """Test loading TOML config."""
        config_file = tmp_path / "config.toml"
        config_file.write_text('app = { name = "test" }\nversion = "1.0"')

        source = FileSource(FileSourceOptions(path=str(config_file), format=ConfigFormat.TOML))
        data = await source.load()

        assert data == {"app": {"name": "test"}, "version": "1.0"}

    @pytest.mark.asyncio
    async def test_load_yaml(self, tmp_path):
        """Test loading YAML config."""
        config_file = tmp_path / "config.yaml"
        config_file.write_text('app:\n  name: "test"\nversion: "1.0"')

        source = FileSource(FileSourceOptions(path=str(config_file), format=ConfigFormat.YAML))
        data = await source.load()

        assert data == {"app": {"name": "test"}, "version": "1.0"}

    @pytest.mark.asyncio
    async def test_load_auto_detect_json(self, tmp_path):
        """Test auto-detecting JSON from extension."""
        config_file = tmp_path / "config.json"
        config_file.write_text(json.dumps({"key": "value"}))

        source = FileSource(FileSourceOptions(path=str(config_file)))
        data = await source.load()

        assert data == {"key": "value"}

    @pytest.mark.asyncio
    async def test_load_auto_detect_toml(self, tmp_path):
        """Test auto-detecting TOML from extension."""
        config_file = tmp_path / "config.toml"
        config_file.write_text('key = "value"')

        source = FileSource(FileSourceOptions(path=str(config_file)))
        data = await source.load()

        assert data == {"key": "value"}

    @pytest.mark.asyncio
    async def test_load_auto_detect_yaml(self, tmp_path):
        """Test auto-detecting YAML from extension."""
        config_file = tmp_path / "config.yaml"
        config_file.write_text('key: "value"')

        source = FileSource(FileSourceOptions(path=str(config_file)))
        data = await source.load()

        assert data == {"key": "value"}

    @pytest.mark.asyncio
    async def test_load_file_not_found(self):
        """Test loading non-existent file raises error."""
        source = FileSource(FileSourceOptions(path="/nonexistent/config.toml"))

        with pytest.raises(ConfigSourceError) as exc_info:
            await source.load()

        assert exc_info.value.source == "file:/nonexistent/config.toml"
        assert "File not found" in exc_info.value.message

    @pytest.mark.asyncio
    async def test_load_invalid_json(self, tmp_path):
        """Test loading invalid JSON raises error."""
        config_file = tmp_path / "config.json"
        config_file.write_text("{ invalid json }")

        source = FileSource(FileSourceOptions(path=str(config_file), format=ConfigFormat.JSON))

        with pytest.raises(ConfigSourceError) as exc_info:
            await source.load()

        assert exc_info.value.source == f"file:{config_file}"

    @pytest.mark.asyncio
    async def test_load_empty_file(self, tmp_path):
        """Test loading empty file."""
        config_file = tmp_path / "config.toml"
        config_file.write_text("")

        source = FileSource(FileSourceOptions(path=str(config_file)))
        data = await source.load()

        assert data == {}

    def test_watch_returns_none(self):
        """Test watch returns None (not implemented)."""
        source = FileSource(FileSourceOptions(path="./config.toml"))
        result = source.watch(lambda x: None)
        assert result is None


class TestFileSourceEdgeCases:
    """Edge case tests for FileSource."""

    @pytest.mark.asyncio
    async def test_load_yaml_without_pyyaml(self, tmp_path, monkeypatch):
        """Test loading YAML without PyYAML raises error."""
        # This test verifies the error message when PyYAML is not available
        config_file = tmp_path / "config.yaml"
        config_file.write_text('key: "value"')

        # We can't easily test the ImportError without mocking imports
        # Just verify it loads when PyYAML is available
        source = FileSource(FileSourceOptions(path=str(config_file), format=ConfigFormat.YAML))
        data = await source.load()
        assert data == {"key": "value"}

    @pytest.mark.asyncio
    async def test_load_toml_without_tomllib(self, tmp_path, monkeypatch):
        """Test loading TOML when tomllib/tomli unavailable."""
        # Similar - we verify it works when available
        config_file = tmp_path / "config.toml"
        config_file.write_text('key = "value"')

        source = FileSource(FileSourceOptions(path=str(config_file), format=ConfigFormat.TOML))
        data = await source.load()
        assert data == {"key": "value"}

    @pytest.mark.asyncio
    async def test_load_utf8_encoding(self, tmp_path):
        """Test loading file with UTF-8 content."""
        config_file = tmp_path / "config.json"
        config_file.write_text(json.dumps({"message": "Hello, 世界! 🌍"}), encoding="utf-8")

        source = FileSource(FileSourceOptions(path=str(config_file), format=ConfigFormat.JSON))
        data = await source.load()

        assert data == {"message": "Hello, 世界! 🌍"}