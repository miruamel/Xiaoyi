use base64::{Engine, engine::general_purpose::STANDARD};

/// Encodes bytes as base64.
///
/// @brief Encode bytes to base64 string
/// @param bytes Input bytes
/// @return Base64 string
/// @since 0.1.0
/// @author Miruamel
pub fn encode_base64(bytes: &[u8]) -> String {
    STANDARD.encode(bytes)
}

/// Decodes a base64 string to bytes.
///
/// @brief Decode base64 string to bytes
/// @param input Base64 string
/// @return Decoded bytes or error
/// @since 0.1.0
/// @author Miruamel
pub fn decode_base64(input: &str) -> Result<Vec<u8>, base64::DecodeError> {
    STANDARD.decode(input)
}
