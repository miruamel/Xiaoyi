//! Syntax primitive float token: F32.
//!
//! Layer hierarchy:
//! - 1 syntax
//! - 2 primitive
//! - 3 f32
//!
//! Represents a 32-bit floating-point primitive syntax token with optional suffix.

#[derive(Debug, Clone, Copy)]
pub struct F32Token {
    pub value: f32,
    pub suffix: Option<char>,
}

impl F32Token {
    pub fn new(value: f32, suffix: Option<char>) -> Self {
        Self { value, suffix }
    }

    /// Render token into source-level representation.
    pub fn display(&self) -> String {
        match self.suffix {
            Some(s) => format!("{}{}", self.value, s),
            None => format!("{}", self.value),
        }
    }
}
