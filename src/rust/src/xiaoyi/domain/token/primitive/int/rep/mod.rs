//! # Integer Representation
//!
//! `rep` defines integer representation details (endianness, encoding).
//!
//! Path: `xiaoyi::domain::token::primitive::int::rep`
//!
//! - Layer 0: `domain`
//! - Layer 1: `token`
//! - Layer 2: `primitive`
//! - Layer 3: `int`
//! - Layer 4: `rep`
//!
//! @module domain::token::primitive::int::rep
//! @brief Integer representation details
//! @group Domain
//! @since 0.1.0
//! @author Miruamel
//! @see crate::domain::token::primitive::int
//! @see crate::domain::token::primitive::int::kind

/// Integer endianness.
///
/// @brief Byte order for serialization
/// @group Domain
/// @since 0.1.0
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Endianness {
    /// Little-endian (least significant byte first).
    Little,
    /// Big-endian (most significant byte first).
    Big,
    /// Native endianness.
    Native,
}

impl Endianness {
    /// Get native endianness.
    ///
    /// @return Native endianness
    /// @since 0.1.0
    pub fn native() -> Self {
        if cfg!(target_endian = "little") {
            Endianness::Little
        } else {
            Endianness::Big
        }
    }
}

/// Default integer representation.
///
/// @brief Default: signed, 64-bit, little-endian
/// @group Domain
/// @since 0.1.0
pub const DEFAULT_REP: (IntKind, IntWidth, Endianness) =
    (IntKind::Signed, IntWidth::W64, Endianness::Little);

use crate::xiaoyi::domain::token::primitive::int::{IntKind, IntWidth};
