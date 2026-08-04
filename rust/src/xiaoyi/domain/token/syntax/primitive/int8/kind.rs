//! Syntax primitive integer token kind: INT8.
//!
//! Layer hierarchy:
//! - 1 syntax
//! - 2 primitive
//! - 3 int8
//! - 4 kind
//!
//! Concrete variant taxonomy for INT8 syntax nodes before rendering.

#[derive(Debug, Clone, Copy)]
pub enum Int8Kind {
    Literal,
    Variable,
    Cast,
}

impl Int8Kind {
    /// Human-readable label used by parser diagnostics and codegen backends.
    pub fn label(&self) -> &'static str {
        match self {
            Int8Kind::Literal => "INT8_LITERAL",
            Int8Kind::Variable => "INT8_VAR",
            Int8Kind::Cast => "INT8_CAST",
        }
    }
}
