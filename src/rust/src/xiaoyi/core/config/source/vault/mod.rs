//! # Vault Configuration Source
//!
//! `vault` provides encrypted secret storage with AES-256-GCM encryption.
//! Secrets are encrypted at rest and decrypted at runtime.
//!
//! Path: `xiaoyi::core::config::source::vault`
//!
//! - Layer 0: `core`
//! - Layer 1: `config`
//! - Layer 2: `source`
//! - Layer 3: `vault`
//! - Layer 4: `encrypt`/`decrypt`/`aes`/`key` — crypto primitives.
//!
//! @module core::config::source::vault
//! @brief Encrypted secrets vault with AES-256-GCM
//! @group Core Runtime
//! @since 0.1.0
//! @author Miruamel
//! @see crate::core::config::source
//! @see crate::core::config::source::vault::encrypt
//! @see crate::core::config::source::vault::decrypt
//!
//! # Security
//!
//! - Uses AES-256-GCM for authenticated encryption.
//! - Key derived from `XIAOYI_VAULT_KEY` (32 bytes base64).
//! - Nonce generated randomly per encryption.
//! - Ciphertext format: `nonce(12) || ciphertext || tag(16)`.
//!
//! @security
//!   - Key MUST be 32 bytes (256 bits).
//!   - Never log plaintext secrets.
//!   - Rotate keys periodically via key derivation.
//!   - Vault files should have restricted permissions (600).
pub mod aes;
pub mod decrypt;
pub mod encrypt;
pub mod key;

use crate::xiaoyi::core::config::source::ConfigSource;
use crate::xiaoyi::core::config::source::vault::decrypt::decrypt as vault_decrypt;
use crate::xiaoyi::core::config::source::vault::key::load_key;
use crate::xiaoyi::core::error::{ErrorKind, Result, XiaoyiError};
use std::collections::HashMap;
use std::fmt::Debug;
use std::path::Path;

/// Vault configuration source (encrypted file).
///
/// @brief Encrypted secrets file source
/// @group Core Runtime
/// @since 0.1.0
#[derive(Debug, Clone)]
pub struct VaultSource {
    path: String,
    required: bool,
}

impl VaultSource {
    /// Create a new vault source.
    ///
    /// @param path Path to encrypted vault file
    /// @return VaultSource instance
    /// @since 0.1.0
    pub fn new(path: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            required: true,
        }
    }

    /// Mark as optional.
    ///
    /// @return Self for chaining
    /// @since 0.1.0
    pub fn optional(mut self) -> Self {
        self.required = false;
        self
    }
}

impl ConfigSource for VaultSource {
    fn load(&self) -> Result<HashMap<String, serde_json::Value>> {
        let rt = tokio::runtime::Runtime::new().map_err(|e| {
            XiaoyiError::new(ErrorKind::Config, "failed to create runtime")
                .with_meta("error", &e.to_string())
        })?;
        rt.block_on(async {
            let path = Path::new(&self.path);
            if !path.exists() {
                if self.required {
                    return Err(XiaoyiError::new(ErrorKind::Config, "vault file not found")
                        .with_meta("path", &self.path));
                }
                return Ok(HashMap::new());
            }

            let encrypted = tokio::fs::read(path).await.map_err(|e| {
                XiaoyiError::new(ErrorKind::Config, "failed to read vault")
                    .with_meta("path", &self.path)
                    .with_meta("error", &e.to_string())
            })?;

            let key = load_key()?;
            let decrypted = vault_decrypt(&encrypted, &key).map_err(|e| {
                XiaoyiError::new(ErrorKind::Config, "vault decryption failed")
                    .with_meta("path", &self.path)
                    .with_meta("error", &e.to_string())
            })?;

            let content = String::from_utf8(decrypted).map_err(|e| {
                XiaoyiError::new(ErrorKind::Config, "vault content not valid UTF-8")
                    .with_meta("error", &e.to_string())
            })?;

            toml::from_str(&content).map_err(|e| {
                XiaoyiError::new(ErrorKind::Config, "vault content parse failed")
                    .with_meta("error", &e.to_string())
            })?
        })
    }

    fn clone_box(&self) -> Box<dyn ConfigSource> {
        Box::new(self.clone())
    }
}

/// Encrypt data for vault storage.
///
/// @param plaintext Plaintext bytes
/// @param key 32-byte encryption key
/// @return Encrypted bytes (nonce || ciphertext || tag)
/// @throw Config error on encryption failure
/// @since 0.1.0
/// @security Uses AES-256-GCM with random nonce
pub fn encrypt(plaintext: &[u8], key: &[u8; 32]) -> Result<Vec<u8>> {
    crate::xiaoyi::core::config::source::vault::encrypt::encrypt(plaintext, key)
}

/// Decrypt vault data.
///
/// @param ciphertext Encrypted bytes (nonce || ciphertext || tag)
/// @param key 32-byte encryption key
/// @return Decrypted plaintext
/// @throw Config error on decryption/auth failure
/// @since 0.1.0
/// @security Validates GCM authentication tag
pub fn decrypt(ciphertext: &[u8], key: &[u8; 32]) -> Result<Vec<u8>> {
    crate::xiaoyi::core::config::source::vault::decrypt::decrypt(ciphertext, key)
}
