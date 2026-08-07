//! # Configuration Sources
//!
//! `source` provides concrete implementations for loading configuration
//! from files, environment variables, and encrypted vaults.
//!
//! Path: `xiaoyi::core::config::source`
//!
//! - Layer 0: `core`
//! - Layer 1: `config`
//! - Layer 2: `source` — source contract and implementations.
//! - Layer 3: `file`/`env`/`vault` — concrete sources.
//!
//! @module core::config::source
//! @brief Configuration source implementations
//! @group Core Runtime
//! @since 0.1.0
//! @author Miruamel
//! @see crate::core::config
//! @see crate::core::config::source::file
//! @see crate::core::config::source::env
//! @see crate::core::config::source::vault
pub mod env;
pub mod file;
pub mod vault;

use crate::xiaoyi::core::error::Result;
use async_trait::async_trait;
use std::collections::HashMap;

/// Async trait for configuration sources.
///
/// @brief Async configuration source contract
/// @group Core Runtime
/// @since 0.1.0
#[async_trait]
pub trait ConfigSource: Send + Sync {
    /// Load configuration asynchronously.
    ///
    /// @return Key-value map or error
    /// @throw Config error on load failure
    /// @since 0.1.0
    async fn load(&self) -> Result<HashMap<String, serde_json::Value>>;
}
