//! # Vault Decryption
//!
//! `decrypt` provides AES-256-GCM decryption for vault secrets.
//!
//! Path: `xiaoyi::core::config::source::vault::decrypt`
//!
//! - Layer 0: `core`
//! - Layer 1: `config`
//! - Layer 2: `source`
//! - Layer 3: `vault`
//! - Layer 4: `decrypt`
//!
//! @module core::config::source::vault::decrypt
//! @brief AES-256-GCM decryption for vault
//! @group Core Runtime
//! @since 0.1.0
//! @author Miruamel
//! @see crate::core::config::source::vault::encrypt
//! @see crate::core::config::source::vault::key
//!
//! # Format
//!
//! Expects: `nonce(12 bytes) || ciphertext || tag(16 bytes)`
//!
//! @security
//!   - Validates GCM authentication tag.
//!   - Returns error on auth failure (tampering).
use crate::xiaoyi::core::error::{ErrorKind, Result, XiaoyiError};
use aes_gcm::{Aes256Gcm, Key, Nonce};
use aes_gcm::aead::{Aead, KeyInit};

/// Decrypt ciphertext with AES-256-GCM.
///
/// @param ciphertext Encrypted bytes (nonce || ciphertext || tag)
/// @param key 32-byte encryption key
/// @return Decrypted plaintext
/// @throw Config error on decryption/auth failure
/// @since 0.1.0
/// @security Validates GCM authentication tag
pub fn decrypt(ciphertext: &[u8], key: &[u8; 32]) -> Result<Vec<u8>> {
    if ciphertext.len() < 12 + 16 {
        return Err(XiaoyiError::new(
            ErrorKind::Config,
            "ciphertext too short",
        ));
    }

    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(key));
    let nonce = Nonce::from_slice(&ciphertext[..12]);
    let encrypted = &ciphertext[12..];

    cipher.decrypt(nonce, encrypted)
        .map_err(|e| XiaoyiError::new(
            ErrorKind::Config,
            "decryption failed (auth tag mismatch?)",
        ).with_meta("error", &e.to_string()))
}