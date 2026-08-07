//! # Boolean Primitive
//!
//! `bool` provides the boolean type with true/false values.
//!
//! Path: `xiaoyi::domain::token::primitive::bool`
//!
//! - Layer 0: `domain`
//! - Layer 1: `token`
//! - Layer 2: `primitive`
//! - Layer 3: `bool`
//!
//! @module domain::token::primitive::bool
//! @brief Boolean primitive type
//! @group Domain
//! @since 0.1.0
//! @author Miruamel
//! @see crate::domain::token::primitive
//! @see crate::domain::token::primitive::int

/// Boolean type alias.
///
/// @brief bool type
/// @group Domain
/// @since 0.1.0
pub type Bool = bool;

/// True value.
///
/// @brief Boolean true
/// @group Domain
/// @since 0.1.0
pub const TRUE: bool = true;

/// False value.
///
/// @brief Boolean false
/// @group Domain
/// @since 0.1.0
pub const FALSE: bool = false;

/// Logical NOT.
///
/// @param value Boolean value
/// @return Negated value
/// @since 0.1.0
pub fn not(value: bool) -> bool {
    !value
}

/// Logical AND.
///
/// @param a First value
/// @param b Second value
/// @return a && b
/// @since 0.1.0
pub fn and(a: bool, b: bool) -> bool {
    a && b
}

/// Logical OR.
///
/// @param a First value
/// @param b Second value
/// @return a || b
/// @since 0.1.0
pub fn or(a: bool, b: bool) -> bool {
    a || b
}