//! # Domain Token Tests
//!
//! Tests for `xiaoyi::domain::token` token representations.
//!
//! @module tests::domain_token
//! @brief Unit tests for domain tokens
//! @group Domain
//! @since 0.1.0
//! @author Miruamel
//! @see crate::domain::token

use pretty_assertions::assert_eq;
use xiaoyi::{PrimitiveKind, IntKind, IntWidth, IntType, FloatKind, SyntaxKind};

#[test]
fn test_primitive_kind_variants() {
    assert_eq!(PrimitiveKind::Int, PrimitiveKind::Int);
    assert_eq!(PrimitiveKind::Float, PrimitiveKind::Float);
    assert_eq!(PrimitiveKind::Bool, PrimitiveKind::Bool);
    assert_eq!(PrimitiveKind::String, PrimitiveKind::String);

    assert_ne!(PrimitiveKind::Int, PrimitiveKind::Float);
}

#[test]
fn test_primitive_kind_debug() {
    let debug = format!("{:?}", PrimitiveKind::Int);
    assert!(debug.contains("Int"));
}

#[test]
fn test_int_kind_variants() {
    assert_eq!(IntKind::Signed, IntKind::Signed);
    assert_eq!(IntKind::Unsigned, IntKind::Unsigned);
    assert_ne!(IntKind::Signed, IntKind::Unsigned);
}

#[test]
fn test_int_kind_debug() {
    let debug = format!("{:?}", IntKind::Signed);
    assert!(debug.contains("Signed"));
}

#[test]
fn test_int_width_variants() {
    assert_eq!(IntWidth::W8, IntWidth::W8);
    assert_eq!(IntWidth::W16, IntWidth::W16);
    assert_eq!(IntWidth::W32, IntWidth::W32);
    assert_eq!(IntWidth::W64, IntWidth::W64);
    assert_eq!(IntWidth::W128, IntWidth::W128);

    assert_ne!(IntWidth::W32, IntWidth::W64);
}

#[test]
fn test_int_width_debug() {
    let debug = format!("{:?}", IntWidth::W32);
    assert!(debug.contains("W32"));
}

#[test]
fn test_int_type_construction() {
    let signed_32 = IntType::new(IntKind::Signed, IntWidth::W32);
    let unsigned_64 = IntType::new(IntKind::Unsigned, IntWidth::W64);

    assert_eq!(signed_32.kind, IntKind::Signed);
    assert_eq!(signed_32.width, IntWidth::W32);
    assert_eq!(unsigned_64.kind, IntKind::Unsigned);
    assert_eq!(unsigned_64.width, IntWidth::W64);
}

#[test]
fn test_int_type_equality() {
    let a = IntType::new(IntKind::Signed, IntWidth::W32);
    let b = IntType::new(IntKind::Signed, IntWidth::W32);
    let c = IntType::new(IntKind::Unsigned, IntWidth::W32);

    assert_eq!(a, b);
    assert_ne!(a, c);
}

#[test]
fn test_int_type_debug() {
    let t = IntType::new(IntKind::Signed, IntWidth::W32);
    let debug = format!("{:?}", t);
    assert!(debug.contains("Signed"));
    assert!(debug.contains("W32"));
}

#[test]
fn test_float_kind_variants() {
    assert_eq!(FloatKind::F32, FloatKind::F32);
    assert_eq!(FloatKind::F64, FloatKind::F64);
    assert_ne!(FloatKind::F32, FloatKind::F64);
}

#[test]
fn test_float_kind_debug() {
    let debug = format!("{:?}", FloatKind::F32);
    assert!(debug.contains("F32"));
}

#[test]
fn test_syntax_kind_variants() {
    let kinds = [
        SyntaxKind::Identifier,
        SyntaxKind::Literal,
        SyntaxKind::Keyword,
        SyntaxKind::Operator,
        SyntaxKind::Delimiter,
        SyntaxKind::Eof,
    ];

    for i in 0..kinds.len() {
        for j in 0..kinds.len() {
            if i == j {
                assert_eq!(kinds[i], kinds[j]);
            } else {
                assert_ne!(kinds[i], kinds[j]);
            }
        }
    }
}

#[test]
fn test_syntax_kind_debug() {
    let debug = format!("{:?}", SyntaxKind::Keyword);
    assert!(debug.contains("Keyword"));
}

#[test]
fn test_token_reexports_work() {
    // Verify all re-exports from lib.rs work
    use xiaoyi::{PrimitiveKind, IntKind, IntWidth, IntType, FloatKind, SyntaxKind};
    let _pk = PrimitiveKind::Int;
    let _ik = IntKind::Signed;
    let _iw = IntWidth::W32;
    let _it = IntType::new(IntKind::Signed, IntWidth::W32);
    let _fk = FloatKind::F64;
    let _sk = SyntaxKind::Keyword;
}