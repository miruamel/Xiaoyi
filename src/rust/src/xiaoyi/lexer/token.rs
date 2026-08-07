//! # Lexer Token Types
//!
//! `token` defines token types for the Xiaoyi lexer.
//!
//! Path: `xiaoyi::lexer::token`
//!
//! @module lexer::token
//! @brief Lexer token definitions
//! @group Language Processing
//! @since 0.1.0
//! @author Miruamel
//! @see crate::domain::token::SyntaxKind

use crate::xiaoyi::domain::token::SyntaxKind;

/// Token with kind and position.
///
/// @brief Token with position info
/// @group Language Processing
/// @since 0.1.0
#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: SyntaxKind,
    pub text: String,
    pub offset: usize,
    pub line: usize,
    pub column: usize,
}