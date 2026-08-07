//! # 32-bit Float (f32)
//!
//! `f32` provides IEEE 754 single-precision floating-point type.
//!
//! Path: `xiaoyi::domain::token::primitive::float::f32`
//!
//! - Layer 0: `domain`
//! - Layer 1: `token`
//! - Layer 2: `primitive`
//! - Layer 3: `float`
//! - Layer 4: `f32`
//!
//! @module domain::token::primitive::float::f32
//! @brief IEEE 754 single-precision float
//! @group Domain
//! @since 0.1.0
//! @author Miruamel
//! @see crate::domain::token::primitive::float
//! @see crate::domain::token::primitive::float::f64

/// 32-bit float type alias.
///
/// @brief f32 type
/// @group Domain
/// @since 0.1.0
pub type F32 = f32;

/// f32 bit pattern.
///
/// @brief Raw bits of f32
/// @group Domain
/// @since 0.1.0
pub type F32Bits = u32;

/// f32 constants.
pub mod consts {
    /// Positive infinity.
    pub const INFINITY: f32 = f32::INFINITY;
    /// Negative infinity.
    pub const NEG_INFINITY: f32 = f32::NEG_INFINITY;
    /// Not a Number.
    pub const NAN: f32 = f32::NAN;
    /// Minimum positive normal value.
    pub const MIN_POSITIVE: f32 = f32::MIN_POSITIVE;
    /// Maximum finite value.
    pub const MAX: f32 = f32::MAX;
    /// Minimum finite value.
    pub const MIN: f32 = f32::MIN;
    /// Epsilon (difference between 1.0 and next representable).
    pub const EPSILON: f32 = f32::EPSILON;
}

/// Check if value is finite.
///
/// @param value f32 value
/// @return true if finite
/// @since 0.1.0
pub fn is_finite(value: f32) -> bool {
    value.is_finite()
}

/// Check if value is NaN.
///
/// @param value f32 value
/// @return true if NaN
/// @since 0.1.0
pub fn is_nan(value: f32) -> bool {
    value.is_nan()
}

/// Check if value is infinite.
///
/// @param value f32 value
/// @return true if infinite
/// @since 0.1.0
pub fn is_infinite(value: f32) -> bool {
    value.is_infinite()
}