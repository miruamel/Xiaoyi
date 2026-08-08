//! # 64-bit Float (f64)
//!
//! `f64` provides IEEE 754 double-precision floating-point type.
//!
//! Path: `xiaoyi::domain::token::primitive::float::f64`
//!
//! - Layer 0: `domain`
//! - Layer 1: `token`
//! - Layer 2: `primitive`
//! - Layer 3: `float`
//! - Layer 4: `f64`
//!
//! @module domain::token::primitive::float::f64
//! @brief IEEE 754 double-precision float
//! @group Domain
//! @since 0.1.0
//! @author Miruamel
//! @see crate::domain::token::primitive::float
//! @see crate::domain::token::primitive::float::f32

/// 64-bit float type alias.
///
/// @brief f64 type
/// @group Domain
/// @since 0.1.0
pub type F64 = f64;

/// f64 bit pattern.
///
/// @brief Raw bits of f64
/// @group Domain
/// @since 0.1.0
pub type F64Bits = u64;

/// f64 constants.
pub mod consts {
    /// Positive infinity.
    pub const INFINITY: f64 = f64::INFINITY;
    /// Negative infinity.
    pub const NEG_INFINITY: f64 = f64::NEG_INFINITY;
    /// Not a Number.
    pub const NAN: f64 = f64::NAN;
    /// Minimum positive normal value.
    pub const MIN_POSITIVE: f64 = f64::MIN_POSITIVE;
    /// Maximum finite value.
    pub const MAX: f64 = f64::MAX;
    /// Minimum finite value.
    pub const MIN: f64 = f64::MIN;
    /// Epsilon (difference between 1.0 and next representable).
    pub const EPSILON: f64 = f64::EPSILON;
}

/// Check if value is finite.
///
/// @param value f64 value
/// @return true if finite
/// @since 0.1.0
pub fn is_finite(value: f64) -> bool {
    value.is_finite()
}

/// Check if value is NaN.
///
/// @param value f64 value
/// @return true if NaN
/// @since 0.1.0
pub fn is_nan(value: f64) -> bool {
    value.is_nan()
}

/// Check if value is infinite.
///
/// @param value f64 value
/// @return true if infinite
/// @since 0.1.0
pub fn is_infinite(value: f64) -> bool {
    value.is_infinite()
}
