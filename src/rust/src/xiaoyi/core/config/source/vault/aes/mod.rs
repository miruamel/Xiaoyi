use crate::xiaoyi::core::error::{ErrorKind, XiaoyiError};

/// AES-GCM primitive helpers.
///
/// @brief Low-level AES-256-GCM operations
/// @since 0.1.0
/// @author Miruamel
pub struct AesGcm;

impl AesGcm {
    /// Validate key length.
    ///
    /// @param key Encryption key
    /// @return Success or error
    /// @since 0.1.0
    pub fn validate_key(key: &[u8]) -> Result<(), XiaoyiError> {
        if key.len() == 32 {
            Ok(())
        } else {
            Err(XiaoyiError::new(
                ErrorKind::Config,
                format!("invalid AES-256-GCM key length: {}", key.len()),
            ))
        }
    }
}
