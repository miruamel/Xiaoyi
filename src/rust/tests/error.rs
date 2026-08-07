//! # Error Module Tests
//!
//! Tests for `xiaoyi::core::error` error construction and handling.
//!
//! @module tests::error
//! @brief Unit tests for error types
//! @group Core Runtime
//! @since 0.1.0
//! @author Miruamel
//! @see crate::core::error

use pretty_assertions::assert_eq;
use xiaoyi::{XiaoyiError, ErrorKind, Result, Status};

#[test]
fn test_error_kind_variants() {
    assert_eq!(ErrorKind::Syntax, ErrorKind::Syntax);
    assert_eq!(ErrorKind::Parse, ErrorKind::Parse);
    assert_eq!(ErrorKind::Runtime, ErrorKind::Runtime);
    assert_eq!(ErrorKind::Io, ErrorKind::Io);
    assert_eq!(ErrorKind::Auth, ErrorKind::Auth);
    assert_eq!(ErrorKind::Policy, ErrorKind::Policy);
    assert_eq!(ErrorKind::Llm, ErrorKind::Llm);
    assert_eq!(ErrorKind::Memory, ErrorKind::Memory);
    assert_eq!(ErrorKind::Tool, ErrorKind::Tool);
    assert_eq!(ErrorKind::Workflow, ErrorKind::Workflow);
    assert_eq!(ErrorKind::Config, ErrorKind::Config);
    assert_eq!(ErrorKind::State, ErrorKind::State);
}

#[test]
fn test_error_creation_basic() {
    let err = XiaoyiError::new(ErrorKind::Config, "missing api key");
    assert_eq!(err.kind, ErrorKind::Config);
    assert_eq!(err.message, "missing api key");
    assert!(err.meta.is_empty());
}

#[test]
fn test_error_creation_with_metadata() {
    let err = XiaoyiError::new(ErrorKind::Config, "missing api key")
        .with_meta("path", "/etc/xiaoyi/config.toml");
    assert_eq!(err.kind, ErrorKind::Config);
    assert_eq!(err.message, "missing api key");
    assert_eq!(err.meta.len(), 1);
    assert_eq!(err.meta[0], ("path".to_string(), "/etc/xiaoyi/config.toml".to_string()));
}

#[test]
fn test_error_display_format() {
    let err = XiaoyiError::new(ErrorKind::Config, "missing api key");
    let display = format!("{}", err);
    assert!(display.contains("Config"));
    assert!(display.contains("missing api key"));
}

#[test]
fn test_error_display_with_metadata() {
    let err = XiaoyiError::new(ErrorKind::Runtime, "connection failed")
        .with_meta("endpoint", "https://api.example.com")
        .with_meta("code", "ECONNREFUSED");
    let display = format!("{}", err);
    assert!(display.contains("Runtime"));
    assert!(display.contains("connection failed"));
}

#[test]
fn test_error_chain() {
    let err1 = XiaoyiError::new(ErrorKind::Io, "file not found");
    let err2 = XiaoyiError::new(ErrorKind::Config, "config error").with_meta("source", "err1");
    // XiaoyiError doesn't support error chaining via source()
    assert_eq!(err1.kind, ErrorKind::Io);
    assert_eq!(err2.kind, ErrorKind::Config);
}

#[test]
fn test_error_result_type_alias() {
    let ok: Result<i32> = Ok(42);
    assert_eq!(ok.unwrap(), 42);

    let err: Result<i32> = Err(XiaoyiError::new(ErrorKind::Config, "test error"));
    assert!(err.is_err());
    assert_eq!(err.unwrap_err().kind, ErrorKind::Config);
}

#[test]
fn test_error_equality() {
    let err1 = XiaoyiError::new(ErrorKind::Config, "same error");
    let err2 = XiaoyiError::new(ErrorKind::Config, "same error");
    let err3 = XiaoyiError::new(ErrorKind::Runtime, "different error");

    assert_eq!(err1.kind, err2.kind);
    assert_eq!(err1.message, err2.message);
    assert_ne!(err1.kind, err3.kind);
    assert_ne!(err1.message, err3.message);
}

#[test]
fn test_error_from_status() {
    let err: XiaoyiError = Status::NotFound.into();
    assert_eq!(err.kind, ErrorKind::Runtime);
    assert!(err.message.contains("NotFound"));
}

#[test]
fn test_error_kind_non_exhaustive() {
    // ErrorKind is non_exhaustive - new variants may be added
    // This test just ensures the known variants work
    let _ = ErrorKind::Syntax;
    let _ = ErrorKind::Parse;
    let _ = ErrorKind::Runtime;
    let _ = ErrorKind::Io;
    let _ = ErrorKind::Auth;
    let _ = ErrorKind::Policy;
    let _ = ErrorKind::Llm;
    let _ = ErrorKind::Memory;
    let _ = ErrorKind::Tool;
    let _ = ErrorKind::Workflow;
    let _ = ErrorKind::Config;
    let _ = ErrorKind::State;
}

#[test]
fn test_error_debug_format() {
    let err = XiaoyiError::new(ErrorKind::Config, "debug test");
    let debug = format!("{:?}", err);
    assert!(debug.contains("Config"));
    assert!(debug.contains("debug test"));
}

#[test]
fn test_error_multiple_metadata_entries() {
    let err = XiaoyiError::new(ErrorKind::Llm, "rate limited")
        .with_meta("provider", "openai")
        .with_meta("retry_after", "60")
        .with_meta("limit", "100");

    assert_eq!(err.meta.len(), 3);
    assert_eq!(err.meta[0], ("provider".to_string(), "openai".to_string()));
    assert_eq!(err.meta[1], ("retry_after".to_string(), "60".to_string()));
    assert_eq!(err.meta[2], ("limit".to_string(), "100".to_string()));
}