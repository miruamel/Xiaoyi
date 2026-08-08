//! # Lexer Errors
//!
//! `error` defines lexical analysis error types.
//!
//! Path: `xiaoyi::lexer::error`
//!
//! @module lexer::error
//! @brief Lexical error types
//! @group Language Processing
//! @since 0.1.0
//! @author Miruamel
//! @see crate::core::error

use crate::xiaoyi::core::error::{ErrorKind, XiaoyiError};

/// Lexical error type.
///
/// @brief Lexical error with position
/// @group Language Processing
/// @since 0.1.0
#[derive(Debug, Clone, thiserror::Error)]
#[error("Lexical error at line {line}, column {column}: {msg}")]
pub struct LexError {
    pub line: usize,
    pub column: usize,
    pub msg: String,
}

impl LexError {
    /// Create new lexical error.
    ///
    /// @param line Line number
    /// @param column Column number
    /// @param msg Error message
    /// @return LexError
    /// @since 0.1.0
    pub fn new(line: usize, column: usize, msg: impl Into<String>) -> Self {
        Self {
            line,
            column,
            msg: msg.into(),
        }
    }
}

/// Convert to XiaoyiError.
///
/// @brief Convert to core error
/// @group Language Processing
/// @since 0.1.0
impl From<LexError> for XiaoyiError {
    fn from(err: LexError) -> Self {
        XiaoyiError::new(ErrorKind::Syntax, err.msg)
            .with_meta("line", err.line.to_string())
            .with_meta("column", err.column.to_string())
    }
}
