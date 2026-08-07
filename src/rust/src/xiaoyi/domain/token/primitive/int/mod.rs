//! # Integer Primitives
//!
//! `int` provides signed and unsigned integer types with configurable
//! width, representation, and normalization.
//!
//! Path: `xiaoyi::domain::token::primitive::int`
//!
//! - Layer 0: `domain`
//! - Layer 1: `token`
//! - Layer 2: `primitive`
//! - Layer 3: `int` — integer type family.
//! - Layer 4: `kind`/`width`/`rep`/`normalize` — details.
//!
//! @module domain::token::primitive::int
//! @brief Integer primitive types with width and representation
//! @group Domain
//! @since 0.1.0
//! @author Miruamel
//! @see crate::domain::token::primitive
//! @see crate::domain::token::primitive::int::kind
//! @see crate::domain::token::primitive::int::width
pub mod kind;
pub mod normalize;
pub mod rep;
pub mod width;

/// Integer type with configurable signedness and width.
///
/// @brief Parameterized integer type
/// @group Domain
/// @since 0.1.0
/// @see IntKind
/// @see IntWidth
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct IntType {
    /// Signed or unsigned.
    pub kind: IntKind,
    /// Bit width.
    pub width: IntWidth,
}

impl IntType {
    /// Create new integer type.
    ///
    /// @param kind Signedness
    /// @param width Bit width
    /// @return IntType instance
    /// @since 0.1.0
    pub fn new(kind: IntKind, width: IntWidth) -> Self {
        Self { kind, width }
    }

    /// Get size in bytes.
    ///
    /// @return Byte size
    /// @since 0.1.0
    pub fn byte_size(&self) -> usize {
        self.width.bits() / 8
    }

    /// Check if signed.
    ///
    /// @return true if signed
    /// @since 0.1.0
    pub fn is_signed(&self) -> bool {
        matches!(self.kind, IntKind::Signed)
    }
}

/// Integer signedness.
///
/// @brief Signed or unsigned classification
/// @group Domain
/// @since 0.1.0
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IntKind {
    /// Signed integer (two's complement).
    Signed,
    /// Unsigned integer.
    Unsigned,
}

/// Integer bit width.
///
/// @brief Supported integer widths
/// @group Domain
/// @since 0.1.0
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IntWidth {
    /// 8-bit.
    W8,
    /// 16-bit.
    W16,
    /// 32-bit.
    W32,
    /// 64-bit.
    W64,
    /// 128-bit.
    W128,
}

impl IntWidth {
    /// Get bit width.
    ///
    /// @return Bits
    /// @since 0.1.0
    pub fn bits(&self) -> usize {
        match self {
            IntWidth::W8 => 8,
            IntWidth::W16 => 16,
            IntWidth::W32 => 32,
            IntWidth::W64 => 64,
            IntWidth::W128 => 128,
        }
    }
}
