"""
Test suite for xiaoyi.core.config.source.env module.

@package xiaoyi.tests.core.config.source
@brief Tests for EnvSource, EnvSourceOptions
@since 0.1.0
"""

import pytest
import os
from xiaoyi.core.config.source.env import (
    EnvSource,
    EnvSourceOptions,
)
from xiaoyi.core.config.config import ConfigSourceError


class TestEnvSourceOptions:
    """Tests for EnvSourceOptions."""

    def test_env_source_options_defaults(self):
        """Test default values."""
        options = EnvSourceOptions()
        assert options.prefix == "XIAOYI_"
        assert options.priority == 200
        assert options.parser is None

    def test_env_source_options_custom(self):
        """Test custom values."""
        def custom_parser(value: str):
            return int(value)
        options = EnvSourceOptions(
            prefix="APP_",
            priority=150,
            parser=custom_parser
        )
        assert options.prefix == "APP_"
        assert options.priority == 150
        assert options.parser == custom_parser


class TestEnvSource:
    """Tests for EnvSource class."""

    def test_env_source_name(self):
        """Test source name property."""
        source = EnvSource(EnvSourceOptions(prefix="APP_"))
        assert source.name == "env:APP_"

    def test_env_source_priority(self):
        """Test source priority property."""
        source = EnvSource(EnvSourceOptions(priority=150))
        assert source.priority == 150

    @pytest.mark.asyncio
    async def test_load_with_prefix(self, monkeypatch):
        """Test loading env vars with prefix."""
        monkeypatch.setenv("XIAOYI_APP_NAME", "test")
        monkeypatch.setenv("XIAOYI_APP_VERSION", "1.0")
        monkeypatch.setenv("OTHER_VAR", "ignored")

        source = EnvSource(EnvSourceOptions(prefix="XIAOYI_"))
        data = await source.load()

        assert data == {"app_name": "test", "app_version": "1.0"}

    @pytest.mark.asyncio
    async def test_load_nested_keys(self, monkeypatch):
        """Test loading nested keys with double underscore."""
        monkeypatch.setenv("XIAOYI_DATABASE__HOST", "localhost")
        monkeypatch.setenv("XIAOYI_DATABASE__PORT", "5432")

        source = EnvSource(EnvSourceOptions(prefix="XIAOYI_"))
        data = await source.load()

        assert data == {"database": {"host": "localhost", "port": "5432"}}

    @pytest.mark.asyncio
    async def test_load_no_matching_vars(self, monkeypatch):
        """Test loading when no vars match prefix."""
        monkeypatch.setenv("OTHER_VAR", "value")

        source = EnvSource(EnvSourceOptions(prefix="XIAOYI_"))
        data = await source.load()

        assert data == {}

    @pytest.mark.asyncio
    async def test_load_custom_parser(self, monkeypatch):
        """Test loading with custom parser."""
        monkeypatch.setenv("XIAOYI_PORT", "8080")
        monkeypatch.setenv("XIAOYI_DEBUG", "true")

        def parse_bool(value: str):
            return value.lower() == "true"

        source = EnvSource(EnvSourceOptions(
            prefix="XIAOYI_",
            parser=lambda v: parse_bool(v) if v.lower() in ("true", "false") else int(v)
        ))
        data = await source.load()

        assert data == {"port": 8080, "debug": True}

    @pytest.mark.asyncio
    async def test_load_default_parser_json(self, monkeypatch):
        """Test default parser handles JSON values."""
        monkeypatch.setenv("XIAOYI_CONFIG", '{"key": "value"}')
        monkeypatch.setenv("XIAOYI_NUMBER", "42")
        monkeypatch.setenv("XIAOYI_BOOL", "true")
        monkeypatch.setenv("XIAOYI_STRING", "hello")

        source = EnvSource(EnvSourceOptions(prefix="XIAOYI_"))
        data = await source.load()

        assert data == {
            "config": {"key": "value"},
            "number": 42,
            "bool": True,
            "string": "hello",
        }

    @pytest.mark.asyncio
    async def test_load_default_parser_invalid_json(self, monkeypatch):
        """Test default parser falls back to string for invalid JSON."""
        monkeypatch.setenv("XIAOYI_INVALID", "{ not json }")

        source = EnvSource(EnvSourceOptions(prefix="XIAOYI_"))
        data = await source.load()

        assert data == {"invalid": "{ not json }"}

    def test_watch_returns_none(self):
        """Test watch returns None (not implemented)."""
        source = EnvSource(EnvSourceOptions())
        result = source.watch(lambda x: None)
        assert result is None


class TestEnvSourceEdgeCases:
    """Edge case tests for EnvSource."""

    @pytest.mark.asyncio
    async def test_load_empty_prefix(self, monkeypatch):
        """Test loading with empty prefix loads all env vars (not recommended)."""
        monkeypatch.setenv("TEST_VAR", "value")

        source = EnvSource(EnvSourceOptions(prefix=""))
        data = await source.load()

        # Empty prefix would match everything, but let's test it doesn't crash
        assert "test_var" in data

    @pytest.mark.asyncio
    async def test_load_case_insensitive_keys(self, monkeypatch):
        """Test that keys are converted to lowercase."""
        monkeypatch.setenv("XIAOYI_API_KEY", "secret")

        source = EnvSource(EnvSourceOptions(prefix="XIAOYI_"))
        data = await source.load()

        assert "api_key" in data
        assert "API_KEY" not in data