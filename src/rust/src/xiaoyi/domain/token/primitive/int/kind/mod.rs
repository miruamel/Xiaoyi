//! # Integer Kind
//!
//! `kind` defines signed vs unsigned integer classification.
//!
//! Path: `xiaoyi::domain::token::primitive::int::kind`
//!
//! - Layer 0: `domain`
//! - Layer 1: `token`
//! - Layer 2: `primitive`
//! - Layer 3: `int`
//! - Layer 4: `kind`
//!
//! @module domain::token::primitive::int::kind
//! @brief Integer signedness classification
//! @group Domain
//! @since 0.1.0
//! @author Miruamel
//! @see crate::domain::token::primitive::int
//! @see crate::domain::token::primitive::int::width

/// Signed integer (two's complement representation).
///
/// @brief Signed integer kind
/// @group Domain
/// @since 0.1.0
pub const SIGNED: IntKind = IntKind::Signed;

/// Unsigned integer.
///
/// @brief Unsigned integer kind
/// @group Domain
/// @since 0.1.0
pub const UNSIGNED: IntKind = IntKind::Unsigned;

use crate::xiaoyi::domain::token::primitive::int::{IntKind, IntType, IntWidth};

/// Get default integer type (signed 64-bit).
///
/// @return Default IntType
/// @since 0.1.0
pub fn default_int_type() -> IntType {
    IntType::new(IntKind::Signed, IntWidth::W64)
}
