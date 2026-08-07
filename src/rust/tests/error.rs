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
use xiaoyi::core::error::{ErrorKind, XiaoyiError, Result};

#[test]
fn test_error_kind_variants() {
    // Test that all ErrorKind variants exist and can be compared
    let kinds = [
        ErrorKind::Syntax,
        ErrorKind::Parse,
        ErrorKind::Runtime,
        ErrorKind::Io,
        ErrorKind::Auth,
        ErrorKind::Policy,
        ErrorKind::Llm,
        ErrorKind::Memory,
        ErrorKind::Tool,
        ErrorKind::Workflow,
        ErrorKind::Config,
        ErrorKind::State,
    ];

    // Verify they are all distinct
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
fn test_error_creation_basic() {
    let err = XiaoyiError::new(ErrorKind::Config, "missing api key");
    assert_eq!(err.kind(), ErrorKind::Config);
    assert_eq!(err.message(), "missing api key");
    assert!(err.meta().is_empty());
}

#[test]
fn test_error_creation_with_metadata() {
    let err = XiaoyiError::new(ErrorKind::Config, "missing api key")
        .with_meta("path", "/etc/xiaoyi/config.toml")
        .with_meta("line", "42");
    assert_eq!(err.kind(), ErrorKind::Config);
    assert_eq!(err.message(), "missing api key");
    assert_eq!(err.meta().get("path"), Some(&"/etc/xiaoyi/config.toml".to_string()));
    assert_eq!(err.meta().get("line"), Some(&"42".to_string()));
}

#[test]
fn test_error_display_format() {
    let err = XiaoyiError::new(ErrorKind::Config, "missing api key");
    let display = format!("{}", err);
    assert_eq!(display, "[Config] missing api key");
}

#[test]
fn test_error_display_with_metadata() {
    let err = XiaoyiError::new(ErrorKind::Config, "missing api key")
        .with_meta("path", "/etc/xiaoyi/config.toml");
    let display = format!("{}", err);
    // Display should still be kind + message, metadata not in default Display
    assert_eq!(display, "[Config] missing api key");
}

#[test]
fn test_error_chain() {
    let cause = XiaoyiError::new(ErrorKind::Io, "file not found");
    let err = XiaoyiError::new(ErrorKind::Config, "failed to load config")
        .with_cause(cause);
    assert!(err.source().is_some());
}

#[test]
fn test_error_result_type_alias() {
    fn returns_result() -> Result<i32> {
        Err(XiaoyiError::new(ErrorKind::Runtime, "oops"))
    }
    let result: Result<i32> = returns_result();
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.kind(), ErrorKind::Runtime);
}

#[test]
fn test_error_equality() {
    let err1 = XiaoyiError::new(ErrorKind::Config, "msg");
    let err2 = XiaoyiError::new(ErrorKind::Config, "msg");
    let err3 = XiaoyiError::new(ErrorKind::Runtime, "msg");

    // Same kind and message, but different metadata -> not equal
    assert_ne!(err1, err2);

    // Different kind -> not equal
    assert_ne!(err1, err3);
}

#[test]
fn test_error_from_status() {
    use xiaoyi::core::result::Status;
    let err: XiaoyiError = Status::FailedPrecondition.into();
    assert_eq!(err.kind(), ErrorKind::Runtime); // Status maps to Runtime kind
}

#[test]
fn test_error_kind_non_exhaustive() {
    // ErrorKind is #[non_exhaustive] - this test ensures we can match on known variants
    let kind = ErrorKind::Config;
    match kind {
        ErrorKind::Syntax => panic!("unexpected"),
        ErrorKind::Parse => panic!("unexpected"),
        ErrorKind::Runtime => panic!("unexpected"),
        ErrorKind::Io => panic!("unexpected"),
        ErrorKind::Auth => panic!("unexpected"),
        ErrorKind::Policy => panic!("unexpected"),
        ErrorKind::Llm => panic!("unexpected"),
        ErrorKind::Memory => panic!("unexpected"),
        ErrorKind::Tool => panic!("unexpected"),
        ErrorKind::Workflow => panic!("unexpected"),
        ErrorKind::Config => {}
        ErrorKind::State => panic!("unexpected"),
        _ => panic!("unexpected variant"),
    }
}

#[test]
fn test_error_debug_format() {
    let err = XiaoyiError::new(ErrorKind::Config, "test");
    let debug = format!("{:?}", err);
    assert!(debug.contains("Config"));
    assert!(debug.contains("test"));
}

#[test]
fn test_error_multiple_metadata_entries() {
    let err = XiaoyiError::new(ErrorKind::Llm, "rate limited")
        .with_meta("provider", "openai")
        .with_meta("retry_after", "60")
        .with_meta("limit", "100");
    assert_eq!(err.meta().len(), 3);
    assert_eq!(err.meta().get("provider"), Some(&"openai".to_string()));
    assert_eq!(err.meta().get("retry_after"), Some(&"60".to_string()));
    assert_eq!(err.meta().get("limit"), Some(&"100".to_string()));
}