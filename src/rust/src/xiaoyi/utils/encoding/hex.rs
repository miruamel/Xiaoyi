/// Hex encoding utilities.
///
/// @brief Encode and decode hex strings
/// @since 0.1.0
/// @author Miruamel
pub struct HexCodec;

impl HexCodec {
    /// Encode bytes as hex.
    ///
    /// @param bytes Input bytes
    /// @return Hex string
    /// @since 0.1.0
    pub fn encode(&self, bytes: &[u8]) -> String {
        bytes.iter().map(|b| format!("{b:02x}")).collect()
    }

    /// Decode hex string to bytes.
    ///
    /// @param input Hex string
    /// @return Decoded bytes or error
    /// @since 0.1.0
    pub fn decode(&self, input: &str) -> Result<Vec<u8>, crate::xiaoyi::core::error::XiaoyiError> {
        if input.len() % 2 != 0 {
            return Err(crate::xiaoyi::core::error::XiaoyiError::new(
                crate::xiaoyi::core::error::ErrorKind::Parse,
                "invalid hex input length",
            ));
        }
        (0..input.len())
            .step_by(2)
            .map(|i| {
                u8::from_str_radix(&input[i..i + 2], 16).map_err(|err| {
                    crate::xiaoyi::core::error::XiaoyiError::new(
                        crate::xiaoyi::core::error::ErrorKind::Parse,
                        format!("invalid hex input: {err}"),
                    )
                })
            })
            .collect()
    }
}
