//! # Detailed Status Codes
//!
//! `status` provides granular status codes for fine-grained error handling
//! and observability.
//!
//! Path: `xiaoyi::core::result::status`
//!
//! - Layer 0: `core`
//! - Layer 1: `result`
//! - Layer 2: `status`
//!
//! @module core::result::status
//! @brief Granular status codes for observability
//! @group Core Runtime
//! @since 0.1.0
//! @author Miruamel
//! @see crate::core::result
//!
/// # Usage
///
/// Use for structured logging, metrics, and retry decisions.
///
/// ```rust
/// use xiaoyi::core::result::status::{RetryClass, StatusCode};
///
/// let status = StatusCode::ResourceExhausted;
/// match status.retry_class() {
///     RetryClass::Transient => println!("retry"),
///     RetryClass::Permanent => println!("alert"),
///     RetryClass::Unknown => println!("unknown"),
/// }
/// ```
/// Retryable vs permanent classification.
///
/// @brief Error retryability classification
/// @group Core Runtime
/// @since 0.1.0
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RetryClass {
    /// Error is transient; retry with backoff.
    Transient,
    /// Error is permanent; do not retry.
    Permanent,
    /// Uncertain; retry once then escalate.
    Unknown,
}

/// Detailed status codes aligned with gRPC/HTTP standards.
///
/// @brief Granular status for structured error handling
/// @group Core Runtime
/// @since 0.1.0
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StatusCode {
    /// Success.
    Ok = 0,
    /// Cancelled.
    Cancelled = 1,
    /// Unknown.
    Unknown = 2,
    /// Invalid argument.
    InvalidArgument = 3,
    /// Deadline exceeded.
    DeadlineExceeded = 4,
    /// Not found.
    NotFound = 5,
    /// Already exists.
    AlreadyExists = 6,
    /// Permission denied.
    PermissionDenied = 7,
    /// Resource exhausted.
    ResourceExhausted = 8,
    /// Failed precondition.
    FailedPrecondition = 9,
    /// Aborted.
    Aborted = 10,
    /// Out of range.
    OutOfRange = 11,
    /// Unimplemented.
    Unimplemented = 12,
    /// Internal.
    Internal = 13,
    /// Unavailable.
    Unavailable = 14,
    /// Data loss.
    DataLoss = 15,
    /// Unauthenticated.
    Unauthenticated = 16,
}

impl StatusCode {
    /// Get retry classification for this status.
    ///
    /// @return RetryClass for this status
    /// @since 0.1.0
    pub fn retry_class(&self) -> RetryClass {
        use RetryClass::*;
        match self {
            StatusCode::Ok
            | StatusCode::Cancelled
            | StatusCode::InvalidArgument
            | StatusCode::NotFound
            | StatusCode::AlreadyExists
            | StatusCode::PermissionDenied
            | StatusCode::FailedPrecondition
            | StatusCode::OutOfRange
            | StatusCode::Unimplemented
            | StatusCode::DataLoss
            | StatusCode::Unauthenticated => Permanent,
            StatusCode::DeadlineExceeded
            | StatusCode::ResourceExhausted
            | StatusCode::Aborted
            | StatusCode::Internal
            | StatusCode::Unavailable => Transient,
            StatusCode::Unknown => Unknown,
        }
    }

    /// Convert to HTTP status code.
    ///
    /// @return HTTP status code
    /// @since 0.1.0
    pub fn to_http(&self) -> u16 {
        match self {
            StatusCode::Ok => 200,
            StatusCode::Cancelled => 499,
            StatusCode::Unknown => 500,
            StatusCode::InvalidArgument => 400,
            StatusCode::DeadlineExceeded => 504,
            StatusCode::NotFound => 404,
            StatusCode::AlreadyExists => 409,
            StatusCode::PermissionDenied => 403,
            StatusCode::ResourceExhausted => 429,
            StatusCode::FailedPrecondition => 412,
            StatusCode::Aborted => 409,
            StatusCode::OutOfRange => 400,
            StatusCode::Unimplemented => 501,
            StatusCode::Internal => 500,
            StatusCode::Unavailable => 503,
            StatusCode::DataLoss => 500,
            StatusCode::Unauthenticated => 401,
        }
    }
}
