//! # Layer 0 - Foundation / Core Error
//!
//! `error` defines the unified error model used across the entire Xiaoyi runtime.
//! Errors are categorized by kind: syntax, parse, runtime, io, auth, policy, llm,
//! memory, tool, workflow, config, and state.
//!
//! Path: `xiaoyi::core::error`
//!
//! - Layer 0: `core` — foundational cross-cutting types.
//! - Layer 1: `error` — error contract and display logic.
//!
//! Each error variant carries structured metadata so upstream layers can map
//! failures to retryable / recoverable / fatal decisions without string matching.
//!
//! @module core::error
//! @brief Unified error model for Xiaoyi runtime
//! @group Core Runtime
//! @since 0.1.0
//! @author Miruamel
//! @see crate::core::result
//! @see crate::core::config
//!
//! # Example
//!
//! ```rust
//! use xiaoyi::core::error::{ErrorKind, XiaoyiError};
//!
//! let err = XiaoyiError::new(ErrorKind::Config, "missing api key")
//!     .with_meta("path", "/etc/xiaoyi/config.toml");
//! println!("{}", err); // [Config] missing api key
//! ```
//!
//! # Errors
//!
//! - All error kinds are non-exhaustive; new variants may be added in minor versions.
//! - Use `ErrorKind::*` to match on categories.
//! - Metadata is opaque key-value pairs for structured logging and recovery.
use serde::{Deserialize, Serialize};
use std::fmt;

/// Unified error kind for Xiaoyi runtime.
///
/// @brief Categorical failure model for runtime errors
/// @group Core Runtime
/// @since 0.1.0
/// @author Miruamel
/// @see XiaoyiError
/// @see crate::core::result::status::StatusCode
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum ErrorKind {
    /// Syntax error during parsing or compilation.
    ///
    /// @brief Syntax parsing failure
    /// @since 0.1.0
    Syntax,
    /// Parse error for structured data (JSON, TOML, etc.).
    ///
    /// @brief Structured data parse failure
    /// @since 0.1.0
    Parse,
    /// Runtime execution error.
    ///
    /// @brief Runtime execution failure
    /// @since 0.1.0
    Runtime,
    /// I/O error (file, network, etc.).
    ///
    /// @brief I/O operation failure
    /// @since 0.1.0
    Io,
    /// Authentication/authorization failure.
    ///
    /// @brief Authentication or authorization failure
    /// @since 0.1.0
    Auth,
    /// Policy violation (rate limit, quota, etc.).
    ///
    /// @brief Policy enforcement violation
    /// @since 0.1.0
    Policy,
    /// LLM provider error.
    ///
    /// @brief LLM provider communication failure
    /// @since 0.1.0
    Llm,
    /// Memory system error (STM/LTM).
    ///
    /// @brief Memory system operation failure
    /// @since 0.1.0
    Memory,
    /// Tool execution error.
    ///
    /// @brief Tool execution failure
    /// @since 0.1.0
    Tool,
    /// Workflow DAG execution error.
    ///
    /// @brief Workflow DAG execution failure
    /// @since 0.1.0
    Workflow,
    /// Configuration error.
    ///
    /// @brief Configuration loading or validation failure
    /// @since 0.1.0
    Config,
    /// State management error.
    ///
    /// @brief State management failure
    /// @since 0.1.0
    State,
}

/// Structured error value with kind, message, and metadata map.
///
/// @brief Structured error with metadata for recovery decisions
/// @group Core Runtime
/// @since 0.1.0
/// @author Miruamel
/// @see ErrorKind
/// @see crate::core::result::Result
/// @threadsafe
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XiaoyiError {
    /// Error category.
    ///
    /// @brief Error classification for recovery logic
    /// @since 0.1.0
    pub kind: ErrorKind,
    /// Human-readable error message.
    ///
    /// @brief User-facing error description
    /// @since 0.1.0
    pub message: String,
    /// Structured metadata for recovery/logging.
    ///
    /// @brief Key-value pairs for structured logging and recovery
    /// @since 0.1.0
    pub meta: Vec<(String, String)>,
}

/// Helper to construct errors with metadata.
impl XiaoyiError {
    /// Create a new error with kind and message.
    ///
    /// @param kind Error category
    /// @param message Human-readable description
    /// @return New XiaoyiError instance
    /// @since 0.1.0
    /// @example
    /// ```rust
    /// use xiaoyi::core::error::{ErrorKind, XiaoyiError};
    /// let err = XiaoyiError::new(ErrorKind::Config, "missing api key");
    /// ```
    /// @see ErrorKind
    /// @see with_meta
    pub fn new(kind: ErrorKind, message: impl Into<String>) -> Self {
        Self {
            kind,
            message: message.into(),
            meta: Vec::new(),
        }
    }

    /// Add a metadata key-value pair.
    ///
    /// @param key Metadata key
    /// @param value Metadata value
    /// @return Self for chaining
    /// @since 0.1.0
    /// @example
    /// ```rust
    /// use xiaoyi::core::error::{ErrorKind, XiaoyiError};
    /// let err = XiaoyiError::new(ErrorKind::Config, "missing api key")
    ///     .with_meta("path", "/etc/xiaoyi/config.toml");
    /// ```
    /// @see new
    pub fn with_meta(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.meta.push((key.into(), value.into()));
        self
    }
}
impl fmt::Display for XiaoyiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{:?}] {}", self.kind, self.message)
    }
}

impl std::error::Error for XiaoyiError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

/// Type alias for Results using XiaoyiError.
///
/// @brief Standard Result type for Xiaoyi operations
/// @since 0.1.0
/// @author Miruamel
pub type Result<T> = std::result::Result<T, XiaoyiError>;
