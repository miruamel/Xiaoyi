//! # Vault Key Management
//!
//! `key` handles encryption key derivation and loading for the vault.
//!
//! Path: `xiaoyi::core::config::source::vault::key`
//!
//! - Layer 0: `core`
//! - Layer 1: `config`
//! - Layer 2: `source`
//! - Layer 3: `vault`
//! - Layer 4: `key`
//!
//! @module core::config::source::vault::key
//! @brief Encryption key loading and derivation
//! @group Core Runtime
//! @since 0.1.0
//! @author Miruamel
//! @see crate::core::config::source::vault
//! @see crate::core::config::source::vault::encrypt
//! @see crate::core::config::source::vault::decrypt
//!
//! # Key Source
//!
//! Key is loaded from `XIAOYI_VAULT_KEY` environment variable.
//! The value must be a base64-encoded 32-byte (256-bit) key.
//!
//! # Example
//!
//! ```bash
//! # Generate a key
//! openssl rand -base64 32
//! # Set in environment
//! export XIAOYI_VAULT_KEY="base64_encoded_32_byte_key"
//! ```
//!
//! @security
//!   - Key MUST be 32 bytes after base64 decode.
//!   - Use hardware security module (HSM) in production.
//!   - Rotate keys periodically; re-encrypt vault on rotation.
//!   - Never commit keys to version control.
use crate::xiaoyi::core::error::{ErrorKind, Result, XiaoyiError};

/// Load encryption key from environment.
///
/// @return 32-byte key array
/// @throw Config error if key missing or invalid
/// @since 0.1.0
pub fn load_key() -> Result<[u8; 32]> {
    let key_b64 = std::env::var("XIAOYI_VAULT_KEY")
        .map_err(|_| XiaoyiError::new(ErrorKind::Config, "XIAOYI_VAULT_KEY not set"))?;

    let key_bytes = base64::decode(&key_b64).map_err(|e| {
        XiaoyiError::new(ErrorKind::Config, "XIAOYI_VAULT_KEY invalid base64")
            .with_meta("error", &e.to_string())
    })?;

    if key_bytes.len() != 32 {
        return Err(XiaoyiError::new(
            ErrorKind::Config,
            "XIAOYI_VAULT_KEY must be 32 bytes (256 bits)",
        )
        .with_meta("length", &key_bytes.len().to_string()));
    }

    let mut key = [0u8; 32];
    key.copy_from_slice(&key_bytes);
    Ok(key)
}

/// Derive key from passphrase using PBKDF2.
///
/// @param passphrase User passphrase
/// @param salt Salt bytes (16+ bytes recommended)
/// @param iterations Iteration count (100000+ recommended)
/// @return 32-byte derived key
/// @since 0.1.0
/// @security Use high iteration count; store salt with vault
pub fn derive_key(passphrase: &str, salt: &[u8], iterations: u32) -> [u8; 32] {
    use pbkdf2::pbkdf2_hmac;
    use sha2::Sha256;

    let mut key = [0u8; 32];
    pbkdf2_hmac::<Sha256>(passphrase.as_bytes(), salt, iterations, &mut key);
    key
}
