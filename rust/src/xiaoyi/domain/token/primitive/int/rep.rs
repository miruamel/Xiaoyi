//! # Layer 1 - Domain / Token Primitive Int Rep
//!
//! Rep presents the concrete integer primitive representation as a typed summary
//! used by lexer, parser, and codegen layers without exposing raw encoding bytes.
//!
//! Path: `xiaoyi::domain::token::primitive::int::rep`
//!
//! Layer hierarchy:
//! - 1: `domain`
//! - 2: `token`
//! - 3: `primitive`
//! - 4: `int`
//! - 5: `rep`

use super::{IntKind, IntWidth};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IntRep {
    pub width: IntWidth,
    pub kind: IntKind,
}

impl IntRep {
    pub const fn new(width: IntWidth, kind: IntKind) -> Self {
        Self { width, kind }
    }

    pub const fn is_signed(&self) -> bool {
        matches!(self.kind, IntKind::Signed)
    }

    pub const fn bits(&self) -> u8 {
        match self.width {
            IntWidth::Int8 => 8,
            IntWidth::Int16 => 16,
            IntWidth::Int32 => 32,
            IntWidth::Int64 => 64,
        }
    }
}
