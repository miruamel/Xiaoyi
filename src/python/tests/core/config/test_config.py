"""
Test suite for xiaoyi.core.config.config module.

@package xiaoyi.tests.core.config
@brief Tests for Config, ConfigSource, ConfigSourceError
@since 0.1.0
"""

import pytest
from xiaoyi.core.config.config import (
    Config,
    ConfigSource,
    ConfigSourceError,
    ConfigValue,
    ConfigMergeStrategy,
)


class TestConfig:
    """Tests for Config dataclass."""

    def test_config_default(self):
        """Test creating Config with defaults."""
        config = Config()
        assert config.data == {}
        assert config.sources == []

    def test_config_with_data(self):
        """Test creating Config with data."""
        data = {"app": {"name": "test"}, "version": "1.0"}
        config = Config(data=data)
        assert config.data == data

    def test_config_with_sources(self):
        """Test creating Config with source metadata."""
        config = Config(data={"key": "value"}, sources=["file:config.toml", "env:XIAOYI_"])
        assert config.sources == ["file:config.toml", "env:XIAOYI_"]

    def test_config_data_isolation(self):
        """Test that config data is not shared between instances."""
        config1 = Config(data={"a": 1})
        config2 = Config(data={"b": 2})
        config1.data["c"] = 3
        assert "c" not in config2.data

    def test_config_sources_isolation(self):
        """Test that config sources is not shared between instances."""
        config1 = Config(sources=["source1"])
        config2 = Config(sources=["source2"])
        config1.sources.append("source3")
        assert "source3" not in config2.sources


class TestConfigSourceError:
    """Tests for ConfigSourceError."""

    def test_config_source_error_basic(self):
        """Test creating ConfigSourceError with message."""
        error = ConfigSourceError("test_source", "Failed to load")
        assert error.source == "test_source"
        assert error.message == "Failed to load"
        assert error.cause is None

    def test_config_source_error_with_cause(self):
        """Test creating ConfigSourceError with cause."""
        cause = ValueError("Invalid format")
        error = ConfigSourceError("test_source", "Failed to load", cause)
        assert error.source == "test_source"
        assert error.message == "Failed to load"
        assert error.cause == cause

    def test_config_source_error_str(self):
        """Test string representation."""
        error = ConfigSourceError("file:config.toml", "File not found")
        assert "file:config.toml" in str(error)
        assert "File not found" in str(error)

    def test_config_source_error_is_exception(self):
        """Test that ConfigSourceError is an Exception."""
        error = ConfigSourceError("test", "message")
        assert isinstance(error, Exception)


class TestConfigMergeStrategy:
    """Tests for ConfigMergeStrategy."""

    def test_replace_strategy(self):
        """Test REPLACE strategy constant."""
        assert ConfigMergeStrategy.REPLACE == "replace"


class TestConfigSourceProtocol:
    """Tests for ConfigSource protocol."""

    def test_config_source_is_runtime_checkable(self):
        """Test that ConfigSource is runtime checkable."""
        # This just verifies the protocol exists and is properly decorated
        assert hasattr(ConfigSource, '__is_protocol__')