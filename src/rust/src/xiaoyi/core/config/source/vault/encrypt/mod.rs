use crate::xiaoyi::core::error::{ErrorKind, XiaoyiError};

/// Vault encryption helper.
///
/// @brief Encrypt secrets with AES-256-GCM
/// @since 0.1.0
/// @author Miruamel
pub struct VaultEncryptor;

impl VaultEncryptor {
    /// Encrypt plaintext bytes.
    ///
    /// @param plaintext Plaintext bytes
    /// @param key Encryption key
    /// @return Ciphertext bytes or error
    /// @since 0.1.0
    pub fn encrypt(&self, plaintext: &[u8], key: &[u8; 32]) -> Result<Vec<u8>, XiaoyiError> {
        use aes_gcm::aead::{Aead, KeyInit};
        use aes_gcm::{Aes256Gcm, Nonce};
        let cipher = Aes256Gcm::new(key.into());
        let nonce = Nonce::from([0u8; 12]);
        cipher
            .encrypt(&nonce, plaintext)
            .map_err(|_| XiaoyiError::new(ErrorKind::Config, "vault encryption failed"))
    }
}
