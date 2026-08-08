//! # Primitive Types
//!
//! `primitive` defines the fundamental primitive types: integers,
//! floats, booleans, and strings with their representations.
//!
//! Path: `xiaoyi::domain::token::primitive`
//!
//! - Layer 0: `domain`
//! - Layer 1: `token`
//! - Layer 2: `primitive` — primitive type system.
//! - Layer 3: `int`/`float`/`bool`/`string` — type families.
//! - Layer 4: `kind`/`width`/`rep`/`normalize` — type details.
//!
//! @module domain::token::primitive
//! @brief Fundamental primitive type definitions
//! @group Domain
//! @since 0.1.0
//! @author Miruamel
//! @see crate::domain::token
//! @see crate::domain::token::primitive::int
//! @see crate::domain::token::primitive::float
pub mod array;
pub mod bool;
pub mod bytes;
pub mod float;
pub mod int;
pub mod string;

// Re-exports from int
pub use int::{IntKind, IntType, IntWidth};
// Re-exports from float
pub use float::FloatKind;

/// Primitive type kind.
///
/// @brief Classification of primitive types
/// @group Domain
/// @since 0.1.0
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PrimitiveKind {
    /// Signed/unsigned integer.
    Int,
    /// Floating point.
    Float,
    /// Boolean.
    Bool,
    /// UTF-8 string.
    String,
}
