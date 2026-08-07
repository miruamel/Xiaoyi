//! # Integer Normalization
//!
//! `normalize` provides integer value normalization (clamping, wrapping).
//!
//! Path: `xiaoyi::domain::token::primitive::int::normalize`
//!
//! - Layer 0: `domain`
//! - Layer 1: `token`
//! - Layer 2: `primitive`
//! - Layer 3: `int`
//! - Layer 4: `normalize`
//!
//! @module domain::token::primitive::int::normalize
//! @brief Integer value normalization
//! @group Domain
//! @since 0.1.0
//! @author Miruamel
//! @see crate::domain::token::primitive::int
//! @see crate::domain::token::primitive::int::width

use crate::xiaoyi::domain::token::primitive::int::IntType;

/// Normalize integer value to fit within type bounds.
///
/// @param value Input value
/// @param int_type Target integer type
/// @return Normalized value (clamped or wrapped)
/// @since 0.1.0
pub fn normalize_i128(value: i128, int_type: IntType) -> i128 {
    let bits = int_type.width.bits();
    let max = if int_type.is_signed() {
        (1i128 << (bits - 1)) - 1
    } else {
        (1i128 << bits) - 1
    };
    let min = if int_type.is_signed() {
        -(1i128 << (bits - 1))
    } else {
        0
    };

    if value > max {
        max
    } else if value < min {
        min
    } else {
        value
    }
}

/// Wrap integer value to fit within type bounds (modulo).
///
/// @param value Input value
/// @param int_type Target integer type
/// @return Wrapped value
/// @since 0.1.0
pub fn wrap_i128(value: i128, int_type: IntType) -> i128 {
    let bits = int_type.width.bits();
    let range = 1i128 << bits;

    if int_type.is_signed() {
        let half = 1i128 << (bits - 1);
        let wrapped = ((value + half) % range + range) % range;
        wrapped - half
    } else {
        (value % range + range) % range
    }
}

/// Convert between integer types with overflow check.
///
/// @param value Source value
/// @param from Source type
/// @param to Target type
/// @return Ok(value) or Err if overflow
/// @since 0.1.0
pub fn convert_checked(value: i128, from: IntType, to: IntType) -> Result<i128, &'static str> {
    let normalized = normalize_i128(value, to);
    if normalized != value && from.width.bits() <= to.width.bits() {
        Err("overflow")
    } else {
        Ok(normalized)
    }
}

type Result<T, E> = std::result::Result<T, E>;
