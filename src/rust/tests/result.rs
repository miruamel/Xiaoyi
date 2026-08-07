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
use xiaoyi::{ErrorKind, Result, ResultExt, Status, XiaoyiError};

#[test]
fn test_status_variants() {
    assert_eq!(Status::Ok, Status::Ok);
    assert_eq!(Status::Cancelled, Status::Cancelled);
    assert_eq!(Status::Unknown, Status::Unknown);
    assert_eq!(Status::InvalidArgument, Status::InvalidArgument);
    assert_eq!(Status::DeadlineExceeded, Status::DeadlineExceeded);
    assert_eq!(Status::NotFound, Status::NotFound);
    assert_eq!(Status::AlreadyExists, Status::AlreadyExists);
    assert_eq!(Status::PermissionDenied, Status::PermissionDenied);
    assert_eq!(Status::ResourceExhausted, Status::ResourceExhausted);
    assert_eq!(Status::FailedPrecondition, Status::FailedPrecondition);
    assert_eq!(Status::Aborted, Status::Aborted);
    assert_eq!(Status::OutOfRange, Status::OutOfRange);
    assert_eq!(Status::Unimplemented, Status::Unimplemented);
    assert_eq!(Status::Internal, Status::Internal);
    assert_eq!(Status::Unavailable, Status::Unavailable);
    assert_eq!(Status::DataLoss, Status::DataLoss);
    assert_eq!(Status::Unauthenticated, Status::Unauthenticated);
}

#[test]
fn test_status_display() {
    assert_eq!(format!("{}", Status::Ok), "Ok");
    assert_eq!(format!("{}", Status::Cancelled), "Cancelled");
    assert_eq!(format!("{}", Status::Unknown), "Unknown");
    assert_eq!(format!("{}", Status::InvalidArgument), "InvalidArgument");
    assert_eq!(format!("{}", Status::DeadlineExceeded), "DeadlineExceeded");
    assert_eq!(format!("{}", Status::NotFound), "NotFound");
    assert_eq!(format!("{}", Status::AlreadyExists), "AlreadyExists");
    assert_eq!(format!("{}", Status::PermissionDenied), "PermissionDenied");
    assert_eq!(
        format!("{}", Status::ResourceExhausted),
        "ResourceExhausted"
    );
    assert_eq!(
        format!("{}", Status::FailedPrecondition),
        "FailedPrecondition"
    );
    assert_eq!(format!("{}", Status::Aborted), "Aborted");
    assert_eq!(format!("{}", Status::OutOfRange), "OutOfRange");
    assert_eq!(format!("{}", Status::Unimplemented), "Unimplemented");
    assert_eq!(format!("{}", Status::Internal), "Internal");
    assert_eq!(format!("{}", Status::Unavailable), "Unavailable");
    assert_eq!(format!("{}", Status::DataLoss), "DataLoss");
    assert_eq!(format!("{}", Status::Unauthenticated), "Unauthenticated");
}

#[test]
fn test_status_from_into_error() {
    let err: XiaoyiError = Status::NotFound.into();
    assert_eq!(err.kind, ErrorKind::Runtime);
    assert!(err.message.contains("NotFound"));

    let err: XiaoyiError = Status::PermissionDenied.into();
    assert_eq!(err.kind, ErrorKind::Runtime);
    assert!(err.message.contains("PermissionDenied"));
}

#[test]
fn test_result_type_alias() {
    let ok: Result<i32> = Ok(42);
    assert_eq!(ok.unwrap(), 42);

    let err: Result<i32> = Err(XiaoyiError::new(ErrorKind::Config, "test error"));
    assert!(err.is_err());
    assert_eq!(err.unwrap_err().kind, ErrorKind::Config);
}

#[test]
fn test_result_ext_methods() {
    let ok: Result<i32> = Ok(42);
    assert_eq!(ok.unwrap_or(0), 42);

    let ok2: Result<i32> = Ok(42);
    assert_eq!(ok2.unwrap_or_default(), 42);

    let ok3: Result<i32> = Ok(42);
    assert_eq!(ok3.unwrap_or_else(|_| 0), 42);

    let err: Result<i32> = Err(XiaoyiError::new(ErrorKind::Runtime, "fail"));
    assert_eq!(err.unwrap_or(100), 100);

    let err2: Result<i32> = Err(XiaoyiError::new(ErrorKind::Runtime, "fail"));
    assert_eq!(err2.unwrap_or_default(), 0);

    let err3: Result<i32> = Err(XiaoyiError::new(ErrorKind::Runtime, "fail"));
    assert_eq!(err3.unwrap_or_else(|_| 200), 200);
}

#[test]
fn test_result_ext_map() {
    let ok: Result<i32> = Ok(10);
    let mapped = ok.map(|x| x * 2);
    assert_eq!(mapped.unwrap(), 20);

    let err: Result<i32> = Err(XiaoyiError::new(ErrorKind::Config, "fail"));
    let mapped = err.map(|x| x * 2);
    assert!(mapped.is_err());
}

#[test]
fn test_result_ext_map_err() {
    let err: Result<i32> = Err(XiaoyiError::new(ErrorKind::Config, "original"));
    let mapped = err.map_err(|e| XiaoyiError::new(ErrorKind::Runtime, e.message));
    assert!(mapped.is_err());
    assert_eq!(mapped.unwrap_err().kind, ErrorKind::Runtime);
}

#[test]
fn test_result_ext_and_then() {
    let ok: Result<i32> = Ok(5);
    let chained = ok.and_then(|x| Ok(x * 3));
    assert_eq!(chained.unwrap(), 15);

    let err: Result<i32> = Err(XiaoyiError::new(ErrorKind::Config, "fail"));
    let chained = err.and_then(|x| Ok(x * 3));
    assert!(chained.is_err());
}

#[test]
fn test_result_ext_or_else() {
    let ok: Result<i32> = Ok(42);
    let recovered: Result<i32> = ok.or_else(|_| Ok(100));
    assert_eq!(recovered.unwrap(), 42);

    let err: Result<i32> = Err(XiaoyiError::new(ErrorKind::Config, "fail"));
    let recovered: Result<i32> = err.or_else(|_| Ok(100));
    assert_eq!(recovered.unwrap(), 100);
}

#[test]
fn test_status_equality() {
    assert_eq!(Status::Ok, Status::Ok);
    assert_ne!(Status::Ok, Status::Cancelled);
    assert_ne!(Status::NotFound, Status::PermissionDenied);
}

#[test]
fn test_status_debug() {
    let debug = format!("{:?}", Status::NotFound);
    assert!(debug.contains("NotFound"));
}

#[test]
fn test_result_works_with_xiaoyi_error() {
    let result: Result<String> = Err(XiaoyiError::new(ErrorKind::Config, "config error"));
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().kind, ErrorKind::Config);
}

#[test]
fn test_result_ext_into_xiaoyi_error() {
    // Test the custom ResultExt method
    let std_result: std::result::Result<i32, std::io::Error> = Err(std::io::Error::new(
        std::io::ErrorKind::NotFound,
        "file not found",
    ));
    let xiaoyi_result: Result<i32> = std_result.into_xiaoyi_error();
    assert!(xiaoyi_result.is_err());
    assert_eq!(xiaoyi_result.unwrap_err().kind, ErrorKind::Runtime);
}
