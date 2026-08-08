//! # Core Result Types
//!
//! `result` provides the standard Result type alias and status codes
//! used throughout the Xiaoyi runtime.
//!
//! Path: `xiaoyi::core::result`
//!
//! - Layer 0: `core`
//! - Layer 1: `result` — result type and status.
//! - Layer 2: `status` — detailed status codes.
//!
//! @module core::result
//! @brief Standard Result type and status codes
//! @group Core Runtime
//! @since 0.1.0
//! @author Miruamel
//! @see crate::core::error
//! @see crate::core::result::status
//!
//! # Example
//!
//! ```rust
//! use xiaoyi::core::result::{Result, Status};
//!
//! fn do_work() -> Result<()> {
//!     if check_fails() {
//!         return Err(Status::FailedPrecondition.into());
//!     }
//!     Ok(())
//! }
//! ```
pub mod status;

use crate::xiaoyi::core::error::{ErrorKind, XiaoyiError};

/// Standard Result type for Xiaoyi operations.
///
/// @brief Result type with XiaoyiError
/// @since 0.1.0
pub type Result<T, E = XiaoyiError> = std::result::Result<T, E>;

/// Extension trait for Result with XiaoyiError.
///
/// @brief Adds convenience methods to Result
/// @since 0.1.0
pub trait ResultExt<T> {
    /// Convert error to XiaoyiError.
    ///
    /// @return Result with XiaoyiError
    /// @since 0.1.0
    fn into_xiaoyi_error(self) -> Result<T>;
}

impl<T, E: std::error::Error + Send + Sync + 'static> ResultExt<T> for std::result::Result<T, E> {
    fn into_xiaoyi_error(self) -> Result<T> {
        self.map_err(|e| XiaoyiError::new(ErrorKind::Runtime, e.to_string()))
    }
}

/// Convert status into error.
///
/// @param status Status code
/// @return XiaoyiError
/// @since 0.1.0
impl From<Status> for XiaoyiError {
    fn from(status: Status) -> Self {
        XiaoyiError::new(ErrorKind::Runtime, status.to_string())
    }
}

/// High-level operation status codes.
///
/// @brief Operation outcome classification
/// @group Core Runtime
/// @since 0.1.0
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    /// Operation completed successfully.
    Ok,
    /// Operation was cancelled.
    Cancelled,
    /// Unknown error.
    Unknown,
    /// Invalid argument provided.
    InvalidArgument,
    /// Deadline exceeded before completion.
    DeadlineExceeded,
    /// Requested entity not found.
    NotFound,
    /// Entity already exists.
    AlreadyExists,
    /// Permission denied.
    PermissionDenied,
    /// Resource exhausted.
    ResourceExhausted,
    /// Precondition check failed.
    FailedPrecondition,
    /// Operation aborted (conflict).
    Aborted,
    /// Operation out of valid range.
    OutOfRange,
    /// Operation not implemented.
    Unimplemented,
    /// Internal system error.
    Internal,
    /// Service unavailable.
    Unavailable,
    /// Data loss or corruption.
    DataLoss,
    /// Unauthenticated request.
    Unauthenticated,
}

impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
