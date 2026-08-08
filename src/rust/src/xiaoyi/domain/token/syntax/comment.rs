/// Comment token.
///
/// @brief Comment token type
/// @since 0.1.0
/// @author Miruamel
/// @see Keyword
#[derive(Debug, Clone)]
pub struct Comment {
    pub text: String,
}

impl Comment {
    /// Create a comment token.
    ///
    /// @param text Comment text
    /// @return Comment instance
    /// @since 0.1.0
    pub fn new(text: impl Into<String>) -> Self {
        Self { text: text.into() }
    }
}
