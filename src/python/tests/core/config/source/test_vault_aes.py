"""
Test suite for xiaoyi.core.config.source.vault.aes module.

@package xiaoyi.tests.core.config.source.vault
@brief Tests for AES-256-GCM encryption round-trip
@since 0.1.0
"""

import pytest
import os
from xiaoyi.core.config.source.vault.aes import (
    encrypt_aes,
    decrypt_aes,
    EncryptedData,
)
from xiaoyi.core.config.source.vault.key import (
    derive_key,
    generate_salt,
    KeyDerivationOptions,
)
from xiaoyi.core.config.source.vault.encrypt import (
    encrypt_config,
    serialize_vault,
)
from xiaoyi.core.config.source.vault.decrypt import (
    decrypt_config,
    deserialize_vault,
    decrypt_vault_bytes,
)


class TestEncryptedData:
    """Tests for EncryptedData dataclass."""

    def test_encrypted_data_creation(self):
        """Test creating EncryptedData instance."""
        data = EncryptedData(
            ciphertext=b"ciphertext",
            tag=b"tag12345678901234",
            nonce=b"nonce12345678",
            algorithm="AES-256-GCM"
        )
        assert data.ciphertext == b"ciphertext"
        assert data.tag == b"tag12345678901234"
        assert data.nonce == b"nonce12345678"
        assert data.algorithm == "AES-256-GCM"

    def test_encrypted_data_default_algorithm(self):
        """Test default algorithm value."""
        data = EncryptedData(
            ciphertext=b"ciphertext",
            tag=b"tag12345678901234",
            nonce=b"nonce12345678"
        )
        assert data.algorithm == "AES-256-GCM"


class TestGenerateSalt:
    """Tests for generate_salt function."""

    def test_generate_salt_default_length(self):
        """Test generating salt with default length."""
        salt = generate_salt()
        assert len(salt) == 16
        assert isinstance(salt, bytes)

    def test_generate_salt_custom_length(self):
        """Test generating salt with custom length."""
        salt = generate_salt(32)
        assert len(salt) == 32

    def test_generate_salt_uniqueness(self):
        """Test that generated salts are unique."""
        salts = {generate_salt() for _ in range(100)}
        assert len(salts) == 100


class TestKeyDerivationOptions:
    """Tests for KeyDerivationOptions."""

    def test_key_derivation_options_defaults(self):
        """Test default values."""
        options = KeyDerivationOptions(password="secret", salt=b"salt123456789012")
        assert options.password == "secret"
        assert options.salt == b"salt123456789012"
        assert options.iterations == 100000
        assert options.key_length == 32

    def test_key_derivation_options_custom(self):
        """Test custom values."""
        options = KeyDerivationOptions(
            password="secret",
            salt=b"salt123456789012",
            iterations=200000,
            key_length=32
        )
        assert options.iterations == 200000
        assert options.key_length == 32


class TestDeriveKey:
    """Tests for derive_key function."""

    @pytest.mark.asyncio
    async def test_derive_key_basic(self):
        """Test basic key derivation."""
        salt = os.urandom(16)
        options = KeyDerivationOptions(password="test-password", salt=salt)
        key = await derive_key(options)

        assert isinstance(key, bytes)
        assert len(key) == 32

    @pytest.mark.asyncio
    async def test_derive_key_deterministic(self):
        """Test that same inputs produce same key."""
        salt = os.urandom(16)
        options1 = KeyDerivationOptions(password="test-password", salt=salt)
        options2 = KeyDerivationOptions(password="test-password", salt=salt)

        key1 = await derive_key(options1)
        key2 = await derive_key(options2)

        assert key1 == key2

    @pytest.mark.asyncio
    async def test_derive_key_different_password(self):
        """Test that different passwords produce different keys."""
        salt = os.urandom(16)
        options1 = KeyDerivationOptions(password="password1", salt=salt)
        options2 = KeyDerivationOptions(password="password2", salt=salt)

        key1 = await derive_key(options1)
        key2 = await derive_key(options2)

        assert key1 != key2

    @pytest.mark.asyncio
    async def test_derive_key_different_salt(self):
        """Test that different salts produce different keys."""
        options1 = KeyDerivationOptions(password="password", salt=os.urandom(16))
        options2 = KeyDerivationOptions(password="password", salt=os.urandom(16))

        key1 = await derive_key(options1)
        key2 = await derive_key(options2)

        assert key1 != key2

    @pytest.mark.asyncio
    async def test_derive_key_custom_iterations(self):
        """Test key derivation with custom iterations."""
        salt = os.urandom(16)
        options = KeyDerivationOptions(password="password", salt=salt, iterations=50000)
        key = await derive_key(options)

        assert len(key) == 32

    @pytest.mark.asyncio
    async def test_derive_key_custom_key_length(self):
        """Test key derivation with custom key length."""
        salt = os.urandom(16)
        options = KeyDerivationOptions(password="password", salt=salt, key_length=16)
        key = await derive_key(options)

        assert len(key) == 16


class TestEncryptDecryptAES:
    """Tests for AES encryption/decryption round-trip."""

    @pytest.mark.asyncio
    async def test_encrypt_decrypt_roundtrip(self):
        """Test encrypt/decrypt round-trip."""
        key = os.urandom(32)
        plaintext = b"Hello, World!"
        associated_data = b"additional-data"

        encrypted = await encrypt_aes(key, plaintext, associated_data)
        decrypted = await decrypt_aes(key, encrypted, associated_data)

        assert decrypted == plaintext

    @pytest.mark.asyncio
    async def test_encrypt_decrypt_without_aad(self):
        """Test encrypt/decrypt without associated data."""
        key = os.urandom(32)
        plaintext = b"Secret message"

        encrypted = await encrypt_aes(key, plaintext)
        decrypted = await decrypt_aes(key, encrypted)

        assert decrypted == plaintext

    @pytest.mark.asyncio
    async def test_encrypt_decrypt_large_data(self):
        """Test encrypt/decrypt with large data."""
        key = os.urandom(32)
        plaintext = os.urandom(10000)  # 10KB

        encrypted = await encrypt_aes(key, plaintext)
        decrypted = await decrypt_aes(key, encrypted)

        assert decrypted == plaintext

    @pytest.mark.asyncio
    async def test_encrypt_decrypt_empty_data(self):
        """Test encrypt/decrypt with empty data."""
        key = os.urandom(32)
        plaintext = b""

        encrypted = await encrypt_aes(key, plaintext)
        decrypted = await decrypt_aes(key, encrypted)

        assert decrypted == plaintext

    @pytest.mark.asyncio
    async def test_encrypt_decrypt_unicode(self):
        """Test encrypt/decrypt with unicode data."""
        key = os.urandom(32)
        plaintext = "Hello, 世界! 🌍".encode("utf-8")

        encrypted = await encrypt_aes(key, plaintext)
        decrypted = await decrypt_aes(key, encrypted)

        assert decrypted.decode("utf-8") == "Hello, 世界! 🌍"

    @pytest.mark.asyncio
    async def test_decrypt_wrong_key_fails(self):
        """Test decryption with wrong key fails."""
        key1 = os.urandom(32)
        key2 = os.urandom(32)
        plaintext = b"Secret"

        encrypted = await encrypt_aes(key1, plaintext)

        with pytest.raises(ValueError, match="Decryption failed"):
            await decrypt_aes(key2, encrypted)

    @pytest.mark.asyncio
    async def test_decrypt_tampered_ciphertext_fails(self):
        """Test decryption with tampered ciphertext fails."""
        key = os.urandom(32)
        plaintext = b"Secret message"

        encrypted = await encrypt_aes(key, plaintext)
        # Tamper with ciphertext
        tampered = EncryptedData(
            ciphertext=bytes([b ^ 0xFF for b in encrypted.ciphertext]),
            tag=encrypted.tag,
            nonce=encrypted.nonce,
        )

        with pytest.raises(ValueError, match="Decryption failed"):
            await decrypt_aes(key, tampered)

    @pytest.mark.asyncio
    async def test_decrypt_tampered_tag_fails(self):
        """Test decryption with tampered tag fails."""
        key = os.urandom(32)
        plaintext = b"Secret message"

        encrypted = await encrypt_aes(key, plaintext)
        # Tamper with tag
        tampered = EncryptedData(
            ciphertext=encrypted.ciphertext,
            tag=bytes([b ^ 0xFF for b in encrypted.tag]),
            nonce=encrypted.nonce,
        )

        with pytest.raises(ValueError, match="Decryption failed"):
            await decrypt_aes(key, tampered)

    @pytest.mark.asyncio
    async def test_decrypt_tampered_nonce_fails(self):
        """Test decryption with tampered nonce fails."""
        key = os.urandom(32)
        plaintext = b"Secret message"

        encrypted = await encrypt_aes(key, plaintext)
        # Tamper with nonce
        tampered = EncryptedData(
            ciphertext=encrypted.ciphertext,
            tag=encrypted.tag,
            nonce=bytes([b ^ 0xFF for b in encrypted.nonce]),
        )

        with pytest.raises(ValueError, match="Decryption failed"):
            await decrypt_aes(key, tampered)

    @pytest.mark.asyncio
    async def test_decrypt_wrong_aad_fails(self):
        """Test decryption with wrong associated data fails."""
        key = os.urandom(32)
        plaintext = b"Secret message"
        aad1 = b"correct-aad"
        aad2 = b"wrong-aad"

        encrypted = await encrypt_aes(key, plaintext, aad1)

        with pytest.raises(ValueError, match="Decryption failed"):
            await decrypt_aes(key, encrypted, aad2)


class TestEncryptConfig:
    """Tests for high-level encrypt_config function."""

    @pytest.mark.asyncio
    async def test_encrypt_config_roundtrip(self):
        """Test encrypt_config with decrypt_config round-trip."""
        config = {"api_key": "secret", "database": {"password": "db-pass"}}
        password = "test-password"

        salt, encrypted = await encrypt_config(config, password)
        decrypted = await decrypt_config(salt, encrypted, password)

        assert decrypted == config

    @pytest.mark.asyncio
    async def test_encrypt_config_generates_salt(self):
        """Test that encrypt_config generates random salt."""
        config = {"key": "value"}
        password = "password"

        salt1, _ = await encrypt_config(config, password)
        salt2, _ = await encrypt_config(config, password)

        assert salt1 != salt2
        assert len(salt1) == 16
        assert len(salt2) == 16


class TestSerializeDeserializeVault:
    """Tests for vault serialization/deserialization."""

    @pytest.mark.asyncio
    async def test_serialize_deserialize_roundtrip(self):
        """Test serialize_vault and deserialize_vault round-trip."""
        salt = os.urandom(16)
        encrypted = await encrypt_aes(os.urandom(32), b"plaintext")

        serialized = serialize_vault(salt, encrypted)
        deserialized_salt, deserialized_encrypted = deserialize_vault(serialized)

        assert deserialized_salt == salt
        assert deserialized_encrypted.ciphertext == encrypted.ciphertext
        assert deserialized_encrypted.tag == encrypted.tag
        assert deserialized_encrypted.nonce == encrypted.nonce

    @pytest.mark.asyncio
    async def test_decrypt_vault_bytes_roundtrip(self):
        """Test decrypt_vault_bytes round-trip."""
        config = {"key": "value"}
        password = "password"

        salt, encrypted = await encrypt_config(config, password)
        vault_bytes = serialize_vault(salt, encrypted)

        decrypted = await decrypt_vault_bytes(vault_bytes, password)

        assert decrypted == config

    @pytest.mark.asyncio
    async def test_vault_format_structure(self):
        """Test that vault format has correct structure: salt || nonce || tag || ciphertext."""
        salt = b"0123456789abcdef"  # 16 bytes
        key = os.urandom(32)
        plaintext = b"test"
        encrypted = await encrypt_aes(key, plaintext)

        vault_bytes = serialize_vault(salt, encrypted)

        # Check structure
        assert vault_bytes[:16] == salt
        assert vault_bytes[16:28] == encrypted.nonce  # 12 bytes
        assert vault_bytes[28:44] == encrypted.tag    # 16 bytes
        assert vault_bytes[44:] == encrypted.ciphertext


class TestAESIntegration:
    """Integration tests for AES encryption."""

    @pytest.mark.asyncio
    async def test_full_encryption_workflow(self):
        """Test complete encryption workflow: config -> encrypt -> serialize -> deserialize -> decrypt."""
        config = {
            "app": {"name": "xiaoyi", "version": "0.1.0"},
            "secrets": {"api_key": "sk-12345", "db_password": "super-secret"},
        }
        password = "workflow-password"

        # Encrypt
        salt, encrypted = await encrypt_config(config, password)
        vault_bytes = serialize_vault(salt, encrypted)

        # Decrypt
        decrypted = await decrypt_vault_bytes(vault_bytes, password)

        assert decrypted == config

    @pytest.mark.asyncio
    async def test_multiple_encryptions_produce_different_ciphertext(self):
        """Test that multiple encryptions of same data produce different ciphertext."""
        config = {"key": "value"}
        password = "password"

        salt1, encrypted1 = await encrypt_config(config, password)
        salt2, encrypted2 = await encrypt_config(config, password)

        # Different salts should produce different ciphertexts
        assert salt1 != salt2
        assert encrypted1.ciphertext != encrypted2.ciphertext
        assert encrypted1.nonce != encrypted2.nonce
        assert encrypted1.tag != encrypted2.tag