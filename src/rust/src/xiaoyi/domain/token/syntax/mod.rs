//! # Syntax Tokens
//!
//! `syntax` provides syntax-level tokens (keywords, operators, delimiters)
//! for the Xiaoyi language parser.
//!
//! Path: `xiaoyi::domain::token::syntax`
//!
//! - Layer 0: `domain`
//! - Layer 1: `token`
//! - Layer 2: `syntax` — syntax token definitions.
//! - Layer 3: `keyword`/`operator`/`delimiter` — token categories.
//!
//! @module domain::token::syntax
//! @brief Syntax-level tokens for parsing
//! @group Domain
//! @since 0.1.0
//! @author Miruamel
//! @see crate::domain::token
//! @see crate::domain::token::primitive
//! @see crate::lexer
pub mod keyword;
pub mod operator;
pub mod punctuation;

/// Syntax token kind.
///
/// @brief Classification of syntax tokens
/// @group Domain
/// @since 0.1.0
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SyntaxKind {
    /// Keyword (if, else, while, etc.)
    Keyword,
    /// Operator (+, -, *, /, etc.)
    Operator,
    /// Delimiter ((), {}, [], etc.)
    Delimiter,
    /// Literal (number, string, bool)
    Literal,
    /// Identifier.
    Identifier,
    /// End of input.
    Eof,
}
