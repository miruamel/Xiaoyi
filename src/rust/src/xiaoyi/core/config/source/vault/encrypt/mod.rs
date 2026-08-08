//! # Vault Encryption
//!
//! `encrypt` provides AES-256-GCM encryption for vault secrets.
//!
//! Path: `xiaoyi::core::config::source::vault::encrypt`
//!
//! - Layer 0: `core`
//! - Layer 1: `config`
//! - Layer 2: `source`
//! - Layer 3: `vault`
//! - Layer 4: `encrypt`
//!
//! @module core::config::source::vault::encrypt
//! @brief AES-256-GCM encryption for vault
//! @group Core Runtime
//! @since 0.1.0
//! @author Miruamel
//! @see crate::core::config::source::vault::decrypt
//! @see crate::core::config::source::vault::key
//!
//! # Format
//!
//! Ciphertext: `nonce(12 bytes) || ciphertext || tag(16 bytes)`
//!
//! @security
//!   - Uses random 96-bit nonce per encryption.
//!   - GCM provides authenticated encryption.
//!   - Never reuse nonce with same key.
use crate::xiaoyi::core::error::{ErrorKind, Result, XiaoyiError};
use aes_gcm::aead::{Aead, KeyInit, OsRng};
use aes_gcm::{Aes256Gcm, Key, Nonce};
use rand::RngCore;

/// Encrypt plaintext with AES-256-GCM.
///
/// @param plaintext Data to encrypt
/// @param key 32-byte encryption key
/// @return Encrypted bytes (nonce || ciphertext || tag)
/// @throw Config error on encryption failure
/// @since 0.1.0
/// @security Random nonce per invocation
pub fn encrypt(plaintext: &[u8], key: &[u8; 32]) -> Result<Vec<u8>> {
    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(key));

    let mut nonce = [0u8; 12];
    OsRng.fill_bytes(&mut nonce);

    let ciphertext = cipher
        .encrypt(Nonce::from_slice(&nonce), plaintext)
        .map_err(|e| {
            XiaoyiError::new(ErrorKind::Config, "encryption failed")
                .with_meta("error", &e.to_string())
        })?;

    let mut result = Vec::with_capacity(12 + ciphertext.len());
    result.extend_from_slice(&nonce);
    result.extend_from_slice(&ciphertext);
    Ok(result)
}
