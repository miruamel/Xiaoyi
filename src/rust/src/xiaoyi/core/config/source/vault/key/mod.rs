use crate::xiaoyi::core::error::XiaoyiError;

/// Vault key derivation.
///
/// @brief Derive encryption key from passphrase
/// @since 0.1.0
/// @author Miruamel
pub struct VaultKey;

impl VaultKey {
    /// Derive key from passphrase.
    ///
    /// @param passphrase Passphrase bytes
    /// @return Derived key bytes
    /// @since 0.1.0
    pub fn derive(&self, passphrase: &[u8]) -> Result<[u8; 32], XiaoyiError> {
        use sha2::Digest;
        let hash = sha2::Sha256::digest(passphrase);
        let mut key = [0u8; 32];
        key.copy_from_slice(&hash);
        Ok(key)
    }
}
