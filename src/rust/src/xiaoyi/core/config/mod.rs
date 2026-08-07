//! # Core Configuration System
//!
//! `config` provides a layered configuration system supporting multiple sources
//! with priority-based merging: defaults < file < env < vault.
//!
//! Path: `xiaoyi::core::config`
//!
//! - Layer 0: `core` — foundational cross-cutting types.
//! - Layer 1: `config` — configuration contract.
//! - Layer 2: `source` — source implementations.
//! - Layer 3: `file`/`env`/`vault` — concrete sources.
//! - Layer 4-5: normalization, encryption, path resolution.
//!
//! @module core::config
//! @brief Layered configuration with multiple sources
//! @group Core Runtime
//! @since 0.1.0
//! @author Miruamel
//! @see crate::core::config::source
//! @see crate::core::error
//!
//! # Example
//!
//! ```no_run
//! use xiaoyi::core::config::{Config, source::FileSource};
//!
//! let config = Config::builder()
//!     .add_source(FileSource::new("./config.toml"))
//!     .build()?;
//! let port: u16 = config.get("server.port")?;
//! ```
//!
//! # Configuration Sources
//!
//! Sources are evaluated in order; later sources override earlier ones.
//!
//! 1. **Defaults** — Built-in defaults for each setting.
//! 2. **File** — TOML, JSON, YAML files (layer 3).
//! 3. **Environment** — `XIAOYI_` prefixed env vars (layer 3).
//! 4. **Vault** — Encrypted secrets with AES-GCM (layer 3).
//!
//! @security
//!   - Vault sources encrypt secrets at rest with AES-256-GCM.
//!   - Keys derived from `XIAOYI_VAULT_KEY` env var (32 bytes).
//!   - Never log vault contents; metadata only.
pub mod source;

use crate::xiaoyi::core::error::{ErrorKind, Result, XiaoyiError};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Configuration builder for composing multiple sources.
///
/// @brief Fluent builder for multi-source configuration
/// @group Core Runtime
/// @since 0.1.0
#[derive(Default)]
pub struct ConfigBuilder {
    sources: Vec<Box<dyn ConfigSource>>,
}

impl ConfigBuilder {
    /// Create a new configuration builder.
    ///
    /// @return Empty ConfigBuilder
    /// @since 0.1.0
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a configuration source.
    ///
    /// @param source Configuration source implementation
    /// @return Self for chaining
    /// @since 0.1.0
    pub fn add_source(mut self, source: impl ConfigSource + 'static) -> Self {
        self.sources.push(Box::new(source));
        self
    }

    /// Build the final configuration by merging all sources.
    ///
    /// @return Merged Config or error
    /// @throw Config error if any required source fails
    /// @since 0.1.0
    pub fn build(self) -> Result<Config> {
        let mut merged = HashMap::new();
        for source in self.sources {
            let data = source.load()?;
            merged.extend(data);
        }
        Ok(Config { data: merged })
    }
}

/// Trait for configuration sources.
///
/// @brief Configuration source contract
/// @group Core Runtime
/// @since 0.1.0
/// @see FileSource
/// @see EnvSource
/// @see VaultSource
pub trait ConfigSource: Send + Sync {
    /// Load configuration from this source.
    ///
    /// @return Key-value map or error
    /// @throw Config error on load failure
    /// @since 0.1.0
    fn load(&self) -> Result<HashMap<String, serde_json::Value>>;
}

/// Merged configuration from all sources.
///
/// @brief Immutable merged configuration
/// @group Core Runtime
/// @since 0.1.0
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    data: HashMap<String, serde_json::Value>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            data: HashMap::new(),
        }
    }
}

impl Config {
    /// Get a typed value by key.
    ///
    /// @param key Dot-notation key (e.g., "server.port")
    /// @return Deserialized value or error if missing/invalid
    /// @throw Config error if key not found or type mismatch
    /// @since 0.1.0
    pub fn get<T: for<'de> Deserialize<'de>>(&self, key: &str) -> Result<T> {
        let mut current: &serde_json::Value = self
            .data
            .get(key.split('.').next().unwrap_or(""))
            .ok_or_else(|| XiaoyiError::new(ErrorKind::Config, format!("config key not found: {}", key)))?;
        for part in key.split('.').skip(1) {
            current = current
                .get(part)
                .ok_or_else(|| XiaoyiError::new(ErrorKind::Config, format!("config key not found: {}", key)))?;
        }
        serde_json::from_value(current.clone())
            .map_err(|e| XiaoyiError::new(ErrorKind::Config, format!("type mismatch for {}: {}", key, e)))
    }
    /// Check if a key exists.
    ///
    /// @param key Dot-notation key
    /// @return true if key exists in any source
    pub fn has(&self, key: &str) -> bool {
        let mut current: &serde_json::Value = self
            .data
            .get(key.split('.').next().unwrap_or(""))
            .unwrap_or(&serde_json::Value::Null);
        for part in key.split('.').skip(1) {
            match current.get(part) {
                Some(v) => current = v,
                None => return false,
            }
        }
        !current.is_null()
    }
}