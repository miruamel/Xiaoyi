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
use xiaoyi::domain::token::{PrimitiveKind, IntKind, IntWidth, IntType, FloatKind, SyntaxKind};

#[test]
fn test_primitive_kind_variants() {
    assert_eq!(PrimitiveKind::Int, PrimitiveKind::Int);
    assert_eq!(PrimitiveKind::Float, PrimitiveKind::Float);
    assert_eq!(PrimitiveKind::Bool, PrimitiveKind::Bool);
    assert_eq!(PrimitiveKind::String, PrimitiveKind::String);

    assert_ne!(PrimitiveKind::Int, PrimitiveKind::Float);
    assert_ne!(PrimitiveKind::Bool, PrimitiveKind::String);
}

#[test]
fn test_primitive_kind_debug() {
    let debug = format!("{:?}", PrimitiveKind::Int);
    assert_eq!(debug, "Int");

    let debug = format!("{:?}", PrimitiveKind::Float);
    assert_eq!(debug, "Float");

    let debug = format!("{:?}", PrimitiveKind::Bool);
    assert_eq!(debug, "Bool");

    let debug = format!("{:?}", PrimitiveKind::String);
    assert_eq!(debug, "String");
}

#[test]
fn test_int_kind_variants() {
    assert_eq!(IntKind::Signed, IntKind::Signed);
    assert_eq!(IntKind::Unsigned, IntKind::Unsigned);
    assert_ne!(IntKind::Signed, IntKind::Unsigned);
}

#[test]
fn test_int_kind_debug() {
    assert_eq!(format!("{:?}", IntKind::Signed), "Signed");
    assert_eq!(format!("{:?}", IntKind::Unsigned), "Unsigned");
}

#[test]
fn test_int_width_variants() {
    // Test that expected widths exist
    let widths = [IntWidth::U8, IntWidth::U16, IntWidth::U32, IntWidth::U64, IntWidth::U128, IntWidth::Usize];
    for i in 0..widths.len() {
        for j in 0..widths.len() {
            if i == j {
                assert_eq!(widths[i], widths[j]);
            } else {
                assert_ne!(widths[i], widths[j]);
            }
        }
    }
}

#[test]
fn test_int_width_debug() {
    assert_eq!(format!("{:?}", IntWidth::U8), "U8");
    assert_eq!(format!("{:?}", IntWidth::U64), "U64");
    assert_eq!(format!("{:?}", IntWidth::Usize), "Usize");
}

#[test]
fn test_int_type_construction() {
    let int_type = IntType::new(IntKind::Signed, IntWidth::I32);
    assert_eq!(int_type.kind(), IntKind::Signed);
    assert_eq!(int_type.width(), IntWidth::I32);
}

#[test]
fn test_int_type_equality() {
    assert_eq!(IntType::new(IntKind::Signed, IntWidth::I32), IntType::new(IntKind::Signed, IntWidth::I32));
    assert_ne!(IntType::new(IntKind::Signed, IntWidth::I32), IntType::new(IntKind::Unsigned, IntWidth::I32));
    assert_ne!(IntType::new(IntKind::Signed, IntWidth::I32), IntType::new(IntKind::Signed, IntWidth::I64));
}

#[test]
fn test_int_type_debug() {
    let debug = format!("{:?}", IntType::new(IntKind::Signed, IntWidth::I32));
    assert!(debug.contains("Signed"));
    assert!(debug.contains("I32"));
}

#[test]
fn test_float_kind_variants() {
    assert_eq!(FloatKind::F32, FloatKind::F32);
    assert_eq!(FloatKind::F64, FloatKind::F64);
    assert_ne!(FloatKind::F32, FloatKind::F64);
}

#[test]
fn test_float_kind_debug() {
    assert_eq!(format!("{:?}", FloatKind::F32), "F32");
    assert_eq!(format!("{:?}", FloatKind::F64), "F64");
}

#[test]
fn test_syntax_kind_variants() {
    let kinds = [
        SyntaxKind::Keyword,
        SyntaxKind::Operator,
        SyntaxKind::Delimiter,
        SyntaxKind::Literal,
        SyntaxKind::Identifier,
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
    assert_eq!(format!("{:?}", SyntaxKind::Keyword), "Keyword");
    assert_eq!(format!("{:?}", SyntaxKind::Operator), "Operator");
    assert_eq!(format!("{:?}", SyntaxKind::Delimiter), "Delimiter");
    assert_eq!(format!("{:?}", SyntaxKind::Literal), "Literal");
    assert_eq!(format!("{:?}", SyntaxKind::Identifier), "Identifier");
    assert_eq!(format!("{:?}", SyntaxKind::Eof), "Eof");
}

#[test]
fn test_token_reexports_work() {
    // Verify re-exports from token module
    use xiaoyi::domain::token::{PrimitiveKind as PK, IntKind as IK, FloatKind as FK, SyntaxKind as SK};

    let _ = PK::Int;
    let _ = IK::Signed;
    let _ = FK::F64;
    let _ = SK::Keyword;
}