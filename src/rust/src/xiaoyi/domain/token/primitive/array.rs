/// Token array abstraction.
///
/// @brief Token array type
/// @since 0.1.0
/// @author Miruamel
#[derive(Debug, Clone)]
pub struct TokenArray {
    pub bytes: Vec<u8>,
}

impl TokenArray {
    /// Create a token array from bytes.
    ///
    /// @param bytes Raw bytes
    /// @return TokenArray instance
    /// @since 0.1.0
    pub fn new(bytes: Vec<u8>) -> Self {
        Self { bytes }
    }
}
