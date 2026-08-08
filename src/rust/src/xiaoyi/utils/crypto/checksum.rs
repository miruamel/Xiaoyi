use crate::xiaoyi::utils::encoding::hex::HexCodec;
use crate::xiaoyi::utils::string::truncate;

/// Crypto checksum utilities.
///
/// @brief Compute checksums and hashes
/// @since 0.1.0
/// @author Miruamel
pub struct Checksum;

impl Checksum {
    /// Compute SHA-256 hex digest.
    ///
    /// @param input Input bytes
    /// @return Hex digest
    /// @since 0.1.0
    pub fn sha256(&self, input: &[u8]) -> String {
        use sha2::Digest;
        let bytes = sha2::Sha256::digest(input).to_vec();
        truncate(&HexCodec.encode(&bytes), 32)
    }
}
