//! # Layer 1 - Domain / Token Primitive Int Width
//!
//! Width describes the storage bit width of an integer token primitive.
//! Width combines with signedness to create the concrete primitive type.
//!
//! Path: `xiaoyi::domain::token::primitive::int::width`
//!
//! Layer hierarchy:
//! - 1: `domain`
//! - 2: `token`
//! - 3: `primitive`
//! - 4: `int`
//! - 5: `width`

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IntWidth {
    Int8,
    Int16,
    Int32,
    Int64,
}
