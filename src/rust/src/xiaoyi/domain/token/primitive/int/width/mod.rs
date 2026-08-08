//! # Integer Width
//!
//! `width` defines supported integer bit widths.
//!
//! Path: `xiaoyi::domain::token::primitive::int::width`
//!
//! - Layer 0: `domain`
//! - Layer 1: `token`
//! - Layer 2: `primitive`
//! - Layer 3: `int`
//! - Layer 4: `width`
//!
//! @module domain::token::primitive::int::width
//! @brief Integer bit width definitions
//! @group Domain
//! @since 0.1.0
//! @author Miruamel
//! @see crate::domain::token::primitive::int
//! @see crate::domain::token::primitive::int::kind

use crate::xiaoyi::domain::token::primitive::int::IntWidth;

/// 8-bit integer width.
///
/// @brief 8-bit width (1 byte)
/// @group Domain
/// @since 0.1.0
pub const W8: IntWidth = IntWidth::W8;

/// 16-bit integer width.
///
/// @brief 16-bit width (2 bytes)
/// @group Domain
/// @since 0.1.0
pub const W16: IntWidth = IntWidth::W16;

/// 32-bit integer width.
///
/// @brief 32-bit width (4 bytes)
/// @group Domain
/// @since 0.1.0
pub const W32: IntWidth = IntWidth::W32;

/// 64-bit integer width.
///
/// @brief 64-bit width (8 bytes)
/// @group Domain
/// @since 0.1.0
pub const W64: IntWidth = IntWidth::W64;

/// 128-bit integer width.
///
/// @brief 128-bit width (16 bytes)
/// @group Domain
/// @since 0.1.0
pub const W128: IntWidth = IntWidth::W128;

/// Get default width (64-bit).
///
/// @return Default IntWidth
/// @since 0.1.0
pub fn default_width() -> IntWidth {
    IntWidth::W64
}
