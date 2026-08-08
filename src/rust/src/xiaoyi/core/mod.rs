//! # Core Module
//!
//! `core` provides foundational cross-cutting types for the Xiaoyi runtime.
//!
//! Path: `xiaoyi::core`
//!
//! - Layer 0: `core` — Foundational cross-cutting types.
//! - Layer 1: `config` — Configuration system.
//! - Layer 1: `error` — Error handling.
//! - Layer 1: `result` — Result extensions.
//!
//! @module core
//! @brief Foundational cross-cutting types
//! @group Core Runtime
//! @since 0.1.0
//! @author Miruamel
//! @see crate::core::config
//! @see crate::core::error
//! @see crate::core::result
//!
//! # Example
//!
//! ```rust
//! use xiaoyi::core::{Config, ErrorKind, XiaoyiError, Result, Status, ResultExt};
//!
//! let config = Config::default();
//! let result: Result<i32> = Ok(42);
//! ```
pub mod config;
pub mod error;
pub mod result;
