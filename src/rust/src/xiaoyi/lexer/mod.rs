//! # Lexer Module
//!
//! `lexer` provides lexical analysis and tokenization for the Xiaoyi framework.
//!
//! Path: `xiaoyi::lexer`
//!
//! - Layer 0: `lexer` — Lexical analysis layer.
//! - Layer 1: `token` — Token types and classification.
//! - Layer 2: `scanner` — Character scanning and token production.
//! - Layer 3: `error` — Lexical error types.
//!
//! @module lexer
//! @brief Lexical analysis and tokenization
//! @group Language Processing
//! @since 0.1.0
//! @author Miruamel
//! @see crate::domain::token
//!
//! # Example
//!
//! ```rust
//! use xiaoyi::lexer::Lexer;
//!
//! let lexer = Lexer::new("fn main() { }");
//! for token in lexer {
//!     println!("{:?}", token);
//! }
//! ```
pub mod token;
pub mod scanner;
pub mod error;

/// Lexer for tokenizing source code.
///
/// @brief Source code tokenizer
/// @group Language Processing
/// @since 0.1.0
/// @threadsafe
#[derive(Debug, Clone)]
pub struct Lexer {
    input: String,
    position: usize,
}

impl Lexer {
    /// Create a new lexer from source code.
    ///
    /// @param source Source code to tokenize
    /// @return Lexer instance
    /// @since 0.1.0
    pub fn new(source: impl Into<String>) -> Self {
        Self {
            input: source.into(),
            position: 0,
        }
    }
}