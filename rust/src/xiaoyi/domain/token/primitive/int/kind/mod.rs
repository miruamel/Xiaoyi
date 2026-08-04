//! # Layer 1 - Domain / Token Primitive Int Kind
//!
//! Signedness classification for integer token primitives. Every concrete integer
//! token maps to one of these kinds before value encoding is applied.
//!
//! Path: `xiaoyi::domain::token::primitive::int::kind`
//!
//! Layer hierarchy:
//! - 1: `domain`
//! - 2: `token`
//! - 3: `primitive`
//! - 4: `int`
//! - 5: `kind`

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IntKind {
    Signed,
    Unsigned,
}
