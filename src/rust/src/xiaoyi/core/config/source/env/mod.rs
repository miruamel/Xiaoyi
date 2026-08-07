//! # Environment Variable Configuration Source
//!
//! `env` loads configuration from environment variables with `XIAOYI_` prefix.
//!
//! Path: `xiaoyi::core::config::source::env`
//!
//! - Layer 0: `core`
//! - Layer 1: `config`
//! - Layer 2: `source`
//! - Layer 3: `env`
//!
//! @module core::config::source::env
//! @brief Environment variable configuration source
//! @group Core Runtime
//! @since 0.1.0
//! @author Miruamel
//! @see crate::core::config::source
//! @see crate::core::config::source::file
//! @see crate::core::config::source::vault
//!
//! # Variable Format
//!
//! Environment variables must be prefixed with `XIAOYI_` and use double underscore
//! for nesting: `XIAOYI_SERVER__PORT=3000` → `server.port`.
//!
//! # Example
//!
//! ```bash
//! export XIAOYI_SERVER__PORT=3000
//! export XIAOYI_DATABASE__URL=postgres://...
//! export XIAOYI_LOG__LEVEL=debug
//! ```
//!
//! @security
//!   - Environment variables may be visible in process listings.
//!   - Use vault for sensitive values (API keys, passwords).
//!   - Never log full environment.
use crate::xiaoyi::core::config::source::ConfigSource;
use crate::xiaoyi::core::error::Result;
use async_trait::async_trait;
use std::collections::HashMap;

/// Environment variable configuration source.
///
/// @brief Loads config from XIAOYI_* environment variables
/// @group Core Runtime
/// @since 0.1.0
#[derive(Debug, Default, Clone)]
pub struct EnvSource {
    prefix: String,
}

impl EnvSource {
    /// Create a new env source with default prefix `XIAOYI_`.
    ///
    /// @return EnvSource instance
    /// @since 0.1.0
    pub fn new() -> Self {
        Self {
            prefix: "XIAOYI_".to_string(),
        }
    }

    /// Create with custom prefix.
    ///
    /// @param prefix Custom environment variable prefix
    /// @return EnvSource instance
    /// @since 0.1.0
    pub fn with_prefix(prefix: impl Into<String>) -> Self {
        Self {
            prefix: prefix.into(),
        }
    }
}

#[async_trait]
impl ConfigSource for EnvSource {
    async fn load(&self) -> Result<HashMap<String, serde_json::Value>> {
        let mut result = HashMap::new();

        for (key, value) in std::env::vars() {
            if let Some(stripped) = key.strip_prefix(&self.prefix) {
                let config_key = stripped.to_lowercase().replace("__", ".");
                result.insert(config_key, serde_json::Value::String(value));
            }
        }

        Ok(result)
    }
}
