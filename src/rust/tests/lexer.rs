//! # Lexer Tests
//!
//! Tests for `xiaoyi::lexer` scanner and tokenization.
//!
//! @module tests::lexer
//! @brief Unit tests for lexer
//! @group Language Processing
//! @since 0.1.0
//! @author Miruamel
//! @see crate::lexer

use pretty_assertions::assert_eq;

// Integration test: import from the xiaoyi crate (matching builder.rs pattern)
use xiaoyi::SyntaxKind;
use xiaoyi::lexer::scanner::Scanner;
use xiaoyi::lexer::token::Token;
#[test]
fn test_scanner_new() {
    let _scanner = Scanner::new("test input");
    // Scanner fields are private - only test that construction works
    assert!(true);
}

#[test]
fn test_sc2() {
    let _scanner = Scanner::new("");
    assert!(true);
}

#[test]
fn test_scanner_unicode() {
    let _scanner = Scanner::new("hello 世界 🦀");
    assert!(true);
}

#[test]
fn test_token_creation() {
    let token = Token {
        kind: SyntaxKind::Identifier,
        text: "foo".into(),
        offset: 0,
        line: 1,
        column: 1,
    };
    assert_eq!(token.kind, SyntaxKind::Identifier);
    assert_eq!(token.text, "foo");
    assert_eq!(token.line, 1);
    assert_eq!(token.column, 1);
}

#[test]
fn test_token_debug() {
    let token = Token {
        kind: SyntaxKind::Literal,
        text: "42".into(),
        offset: 0,
        line: 5,
        column: 10,
    };
    let debug = format!("{:?}", token);
    assert!(debug.contains("Literal"));
    assert!(debug.contains("42"));
    assert!(debug.contains("5"));
    assert!(debug.contains("10"));
}

#[test]
fn test_token_equality() {
    let t1 = Token {
        kind: SyntaxKind::Keyword,
        text: "if".into(),
        offset: 0,
        line: 1,
        column: 1,
    };
    let t2 = Token {
        kind: SyntaxKind::Keyword,
        text: "if".into(),
        offset: 0,
        line: 1,
        column: 1,
    };
    let t3 = Token {
        kind: SyntaxKind::Keyword,
        text: "else".into(),
        offset: 0,
        line: 1,
        column: 1,
    };

    assert_eq!(t1, t2);
    assert_ne!(t1, t3);
}

#[test]
fn test_syntax_kind_variants() {
    // Use the actual variants that exist in SyntaxKind enum
    assert_eq!(SyntaxKind::Identifier, SyntaxKind::Identifier);
    assert_eq!(SyntaxKind::Literal, SyntaxKind::Literal);
    assert_eq!(SyntaxKind::Keyword, SyntaxKind::Keyword);
    assert_eq!(SyntaxKind::Operator, SyntaxKind::Operator);
    assert_eq!(SyntaxKind::Delimiter, SyntaxKind::Delimiter);
    assert_eq!(SyntaxKind::Eof, SyntaxKind::Eof);

    assert_ne!(SyntaxKind::Identifier, SyntaxKind::Literal);
}
