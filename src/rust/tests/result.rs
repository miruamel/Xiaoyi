//! # Result Module Tests
//!
//! Tests for `xiaoyi::core::result` status codes and ResultExt.
//!
//! @module tests::result
//! @brief Unit tests for result types
//! @group Core Runtime
//! @since 0.1.0
//! @author Miruamel
//! @see crate::core::result

use pretty_assertions::assert_eq;
use xiaoyi::core::result::{Status, Result, ResultExt};

#[test]
fn test_status_variants() {
    let statuses = [
        Status::Ok,
        Status::Cancelled,
        Status::Unknown,
        Status::InvalidArgument,
        Status::DeadlineExceeded,
        Status::NotFound,
        Status::AlreadyExists,
        Status::PermissionDenied,
        Status::ResourceExhausted,
        Status::FailedPrecondition,
        Status::Aborted,
        Status::OutOfRange,
        Status::Unimplemented,
        Status::Internal,
        Status::Unavailable,
        Status::DataLoss,
        Status::Unauthenticated,
    ];

    for i in 0..statuses.len() {
        for j in 0..statuses.len() {
            if i == j {
                assert_eq!(statuses[i], statuses[j]);
            } else {
                assert_ne!(statuses[i], statuses[j]);
            }
        }
    }
}

#[test]
fn test_status_display() {
    assert_eq!(format!("{}", Status::Ok), "OK");
    assert_eq!(format!("{}", Status::Cancelled), "CANCELLED");
    assert_eq!(format!("{}", Status::Unknown), "UNKNOWN");
    assert_eq!(format!("{}", Status::InvalidArgument), "INVALID_ARGUMENT");
    assert_eq!(format!("{}", Status::DeadlineExceeded), "DEADLINE_EXCEEDED");
    assert_eq!(format!("{}", Status::NotFound), "NOT_FOUND");
    assert_eq!(format!("{}", Status::AlreadyExists), "ALREADY_EXISTS");
    assert_eq!(format!("{}", Status::PermissionDenied), "PERMISSION_DENIED");
    assert_eq!(format!("{}", Status::ResourceExhausted), "RESOURCE_EXHAUSTED");
    assert_eq!(format!("{}", Status::FailedPrecondition), "FAILED_PRECONDITION");
    assert_eq!(format!("{}", Status::Aborted), "ABORTED");
    assert_eq!(format!("{}", Status::OutOfRange), "OUT_OF_RANGE");
    assert_eq!(format!("{}", Status::Unimplemented), "UNIMPLEMENTED");
    assert_eq!(format!("{}", Status::Internal), "INTERNAL");
    assert_eq!(format!("{}", Status::Unavailable), "UNAVAILABLE");
    assert_eq!(format!("{}", Status::DataLoss), "DATA_LOSS");
    assert_eq!(format!("{}", Status::Unauthenticated), "UNAUTHENTICATED");
}

#[test]
fn test_status_from_into_error() {
    use xiaoyi::core::error::XiaoyiError;
    let err: XiaoyiError = Status::FailedPrecondition.into();
    assert_eq!(err.kind(), xiaoyi::core::error::ErrorKind::Runtime);
    assert!(err.message().contains("FAILED_PRECONDITION"));
}

#[test]
fn test_result_type_alias() {
    fn returns_ok() -> Result<i32> {
        Ok(42)
    }
    fn returns_err() -> Result<i32> {
        Err(xiaoyi::core::error::XiaoyiError::new(
            xiaoyi::core::error::ErrorKind::Runtime,
            "failed",
        ))
    }

    assert_eq!(returns_ok().unwrap(), 42);
    assert!(returns_err().is_err());
}

#[test]
fn test_result_ext_methods() {
    let ok_result: Result<i32, &str> = Ok(10);
    let err_result: Result<i32, &str> = Err("error");

    // Test is_ok / is_err
    assert!(ok_result.is_ok());
    assert!(!ok_result.is_err());
    assert!(!err_result.is_ok());
    assert!(err_result.is_err());

    // Test unwrap
    assert_eq!(ok_result.unwrap(), 10);

    // Test unwrap_err
    assert_eq!(err_result.unwrap_err(), "error");
}

#[test]
fn test_result_ext_map() {
    let ok_result: Result<i32, &str> = Ok(5);
    let mapped = ok_result.map(|x| x * 2);
    assert_eq!(mapped.unwrap(), 10);

    let err_result: Result<i32, &str> = Err("error");
    let mapped = err_result.map(|x| x * 2);
    assert_eq!(mapped.unwrap_err(), "error");
}

#[test]
fn test_result_ext_map_err() {
    let ok_result: Result<i32, &str> = Ok(5);
    let mapped = ok_result.map_err(|e| format!("wrapped: {}", e));
    assert_eq!(mapped.unwrap(), 5);

    let err_result: Result<i32, &str> = Err("error");
    let mapped = err_result.map_err(|e| format!("wrapped: {}", e));
    assert_eq!(mapped.unwrap_err(), "wrapped: error");
}

#[test]
fn test_result_ext_and_then() {
    let ok_result: Result<i32, &str> = Ok(5);
    let chained = ok_result.and_then(|x| Ok::<i32, &str>(x + 3));
    assert_eq!(chained.unwrap(), 8);

    let err_result: Result<i32, &str> = Err("error");
    let chained = err_result.and_then(|x| Ok::<i32, &str>(x + 3));
    assert_eq!(chained.unwrap_err(), "error");
}

#[test]
fn test_result_ext_or_else() {
    let ok_result: Result<i32, &str> = Ok(5);
    let recovered = ok_result.or_else(|_| Ok(99));
    assert_eq!(recovered.unwrap(), 5);

    let err_result: Result<i32, &str> = Err("error");
    let recovered = err_result.or_else(|_| Ok(99));
    assert_eq!(recovered.unwrap(), 99);
}

#[test]
fn test_status_equality() {
    assert_eq!(Status::Ok, Status::Ok);
    assert_ne!(Status::Ok, Status::Cancelled);
}

#[test]
fn test_status_debug() {
    let debug = format!("{:?}", Status::NotFound);
    assert_eq!(debug, "NotFound");
}

#[test]
fn test_result_works_with_xiaoyi_error() {
    let result: Result<String> = Err(xiaoyi::core::error::XiaoyiError::new(
        xiaoyi::core::error::ErrorKind::Config,
        "bad config",
    ));
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.kind(), xiaoyi::core::error::ErrorKind::Config);
    assert_eq!(err.message(), "bad config");
}