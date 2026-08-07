"""
Test suite for xiaoyi.core.config.source.vault.vault_source module.

@package xiaoyi.tests.core.config.source
@brief Tests for VaultSource, VaultSourceOptions
@since 0.1.0
"""

import pytest
import os
from pathlib import Path
from xiaoyi.core.config.source.vault.vault_source import (
    VaultSource,
    VaultSourceOptions,
)
from xiaoyi.core.config.source.vault.encrypt import encrypt_config, serialize_vault
from xiaoyi.core.config.config import ConfigSourceError


class TestVaultSourceOptions:
    """Tests for VaultSourceOptions."""

    def test_vault_source_options_required(self):
        """Test required parameters."""
        options = VaultSourceOptions(path="./vault.bin", password="secret")
        assert options.path == "./vault.bin"
        assert options.password == "secret"
        assert options.priority == 300

    def test_vault_source_options_custom_priority(self):
        """Test custom priority."""
        options = VaultSourceOptions(path="./vault.bin", password="secret", priority=250)
        assert options.priority == 250


class TestVaultSource:
    """Tests for VaultSource class."""

    def test_vault_source_name(self):
        """Test source name property."""
        source = VaultSource(VaultSourceOptions(path="./vault.bin", password="secret"))
        assert source.name == "vault:./vault.bin"

    def test_vault_source_priority(self):
        """Test source priority property."""
        source = VaultSource(VaultSourceOptions(path="./vault.bin", password="secret", priority=250))
        assert source.priority == 250

    @pytest.mark.asyncio
    async def test_load_valid_vault(self, tmp_path):
        """Test loading a valid encrypted vault."""
        config = {"api_key": "secret-key", "database": {"password": "db-pass"}}
        password = "test-password"

        # Encrypt the config
        salt, encrypted = await encrypt_config(config, password)
        vault_data = serialize_vault(salt, encrypted)

        # Write to file
        vault_file = tmp_path / "vault.bin"
        vault_file.write_bytes(vault_data)

        # Load via VaultSource
        source = VaultSource(VaultSourceOptions(path=str(vault_file), password=password))
        data = await source.load()

        assert data == config

    @pytest.mark.asyncio
    async def test_load_wrong_password(self, tmp_path):
        """Test loading with wrong password raises error."""
        config = {"api_key": "secret-key"}
        password = "correct-password"

        salt, encrypted = await encrypt_config(config, password)
        vault_data = serialize_vault(salt, encrypted)

        vault_file = tmp_path / "vault.bin"
        vault_file.write_bytes(vault_data)

        # Try with wrong password
        source = VaultSource(VaultSourceOptions(path=str(vault_file), password="wrong-password"))

        with pytest.raises(ConfigSourceError) as exc_info:
            await source.load()

        assert "Decryption failed" in str(exc_info.value)

    @pytest.mark.asyncio
    async def test_load_nonexistent_file(self):
        """Test loading non-existent file raises error."""
        source = VaultSource(VaultSourceOptions(path="/nonexistent/vault.bin", password="secret"))

        with pytest.raises(ConfigSourceError) as exc_info:
            await source.load()

        assert "File not found" in exc_info.value.message

    @pytest.mark.asyncio
    async def test_load_corrupted_vault(self, tmp_path):
        """Test loading corrupted vault data raises error."""
        vault_file = tmp_path / "vault.bin"
        vault_file.write_bytes(b"corrupted data")

        source = VaultSource(VaultSourceOptions(path=str(vault_file), password="secret"))

        with pytest.raises(ConfigSourceError):
            await source.load()

    @pytest.mark.asyncio
    async def test_load_empty_vault(self, tmp_path):
        """Test loading empty vault file raises error."""
        vault_file = tmp_path / "vault.bin"
        vault_file.write_bytes(b"")

        source = VaultSource(VaultSourceOptions(path=str(vault_file), password="secret"))

        with pytest.raises(ConfigSourceError):
            await source.load()

    def test_watch_returns_unsubscribe(self):
        """Test watch returns an unsubscribe function."""
        source = VaultSource(VaultSourceOptions(path="./vault.bin", password="secret"))
        callback_called = []

        def callback(data):
            callback_called.append(data)

        unsubscribe = source.watch(callback)
        assert callable(unsubscribe)

        # Call unsubscribe
        unsubscribe()
        # Should not raise

    @pytest.mark.asyncio
    async def test_watch_callback_called_on_load(self, tmp_path):
        """Test that watch callback is called on load."""
        config = {"key": "value"}
        password = "test-password"

        salt, encrypted = await encrypt_config(config, password)
        vault_data = serialize_vault(salt, encrypted)

        vault_file = tmp_path / "vault.bin"
        vault_file.write_bytes(vault_data)

        source = VaultSource(VaultSourceOptions(path=str(vault_file), password=password))
        callback_called = []

        def callback(data):
            callback_called.append(data)

        unsubscribe = source.watch(callback)
        await source.load()
        unsubscribe()

        assert len(callback_called) == 1
        assert callback_called[0] == config

    @pytest.mark.asyncio
    async def test_watch_multiple_callbacks(self, tmp_path):
        """Test multiple watch callbacks."""
        config = {"key": "value"}
        password = "test-password"

        salt, encrypted = await encrypt_config(config, password)
        vault_data = serialize_vault(salt, encrypted)

        vault_file = tmp_path / "vault.bin"
        vault_file.write_bytes(vault_data)

        source = VaultSource(VaultSourceOptions(path=str(vault_file), password=password))
        calls1 = []
        calls2 = []

        def callback1(data):
            calls1.append(data)

        def callback2(data):
            calls2.append(data)

        unsubscribe1 = source.watch(callback1)
        unsubscribe2 = source.watch(callback2)

        await source.load()

        unsubscribe1()
        unsubscribe2()

        assert calls1 == [config]
        assert calls2 == [config]


class TestVaultSourceEdgeCases:
    """Edge case tests for VaultSource."""

    @pytest.mark.asyncio
    async def test_load_complex_config(self, tmp_path):
        """Test loading complex nested configuration."""
        config = {
            "app": {
                "name": "xiaoyi",
                "version": "0.1.0",
                "settings": {
                    "debug": True,
                    "log_level": "info",
                }
            },
            "database": {
                "host": "localhost",
                "port": 5432,
                "credentials": {
                    "username": "admin",
                    "password": "secret123"
                }
            },
            "features": ["auth", "logging", "metrics"],
        }
        password = "complex-password"

        salt, encrypted = await encrypt_config(config, password)
        vault_data = serialize_vault(salt, encrypted)

        vault_file = tmp_path / "vault.bin"
        vault_file.write_bytes(vault_data)

        source = VaultSource(VaultSourceOptions(path=str(vault_file), password=password))
        data = await source.load()

        assert data == config

    @pytest.mark.asyncio
    async def test_load_unicode_config(self, tmp_path):
        """Test loading config with unicode characters."""
        config = {
            "message": "Hello, 世界! 🌍",
            "emoji": "🚀",
            "chinese": "中文",
        }
        password = "unicode-password"

        salt, encrypted = await encrypt_config(config, password)
        vault_data = serialize_vault(salt, encrypted)

        vault_file = tmp_path / "vault.bin"
        vault_file.write_bytes(vault_data)

        source = VaultSource(VaultSourceOptions(path=str(vault_file), password=password))
        data = await source.load()

        assert data == config