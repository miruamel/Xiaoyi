use crate::xiaoyi::core::error::{ErrorKind, XiaoyiError};

/// Vault decryption helper.
///
/// @brief Decrypt secrets with AES-256-GCM
/// @since 0.1.0
/// @author Miruamel
pub struct VaultDecryptor;

impl VaultDecryptor {
    /// Decrypt ciphertext bytes.
    ///
    /// @param ciphertext Ciphertext bytes
    /// @param key Encryption key
    /// @return Plaintext bytes or error
    /// @since 0.1.0
    pub fn decrypt(&self, ciphertext: &[u8], key: &[u8; 32]) -> Result<Vec<u8>, XiaoyiError> {
        use aes_gcm::aead::{Aead, KeyInit};
        use aes_gcm::{Aes256Gcm, Nonce};
        let cipher = Aes256Gcm::new(key.into());
        let nonce = Nonce::from([0u8; 12]);
        cipher
            .decrypt(&nonce, ciphertext)
            .map_err(|_| XiaoyiError::new(ErrorKind::Config, "vault decryption failed"))
    }
}
