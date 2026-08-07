//! # Lexer Scanner
//!
//! `scanner` implements character-level scanning for the lexer.
//!
//! Path: `xiaoyi::lexer::scanner`
//!
//! @module lexer::scanner
//! @brief Character scanner for tokenization
//! @group Language Processing
//! @since 0.1.0
//! @author Miruamel
//! @see crate::lexer::token

/// Character scanner state.
///
/// @brief Scanner position and input
/// @group Language Processing
/// @since 0.1.0
#[derive(Debug, Clone)]
pub struct Scanner {
    input: Vec<char>,
    position: usize,
    line: usize,
    column: usize,
}

impl Scanner {
    /// Create new scanner.
    ///
    /// @param source Input source
    /// @return Scanner instance
    /// @since 0.1.0
    pub fn new(source: &str) -> Self {
        Self {
            input: source.chars().collect(),
            position: 0,
            line: 1,
            column: 1,
        }
    }
}
