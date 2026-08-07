//! # File-based Configuration Source
//!
//! `file` loads configuration from TOML, JSON, or YAML files with
//! path normalization utilities.
//!
//! Path: `xiaoyi::core::config::source::file`
//!
//! - Layer 0: `core`
//! - Layer 1: `config`
//! - Layer 2: `source`
//! - Layer 3: `file`
//! - Layer 4: `path`/`absolute`/`unix`/`norm` — path utilities.
//!
//! @module core::config::source::file
//! @brief File configuration source with path utilities
//! @group Core Runtime
//! @since 0.1.0
//! @author Miruamel
//! @see crate::core::config::source
//! @see crate::core::config::source::file::path
//! @see crate::core::config::source::file::norm
//!
//! # Example
//!
//! ```no_run
//! use xiaoyi::core::config::source::file::FileSource;
//!
//! let source = FileSource::new("./config.toml").optional();
//! let config = source.load().await?;
//! ```
//!
//! # Supported Formats
//!
//! | Extension | Parser |
//! |-----------|--------|
//! | `.toml`   | `toml` |
//! | `.json`   | `serde_json` |
//! | `.yaml`, `.yml` | `serde_yaml` |
//!
//! # Errors
//!
//! - Returns error if required file not found.
//! - Returns error if format unsupported.
//! - Returns error if parse fails.

use crate::xiaoyi::core::config::source::ConfigSource;
use crate::xiaoyi::core::error::{ErrorKind, Result, XiaoyiError};
use async_trait::async_trait;
use std::collections::HashMap;
use std::path::Path;

/// File-based configuration source.
///
/// @brief Loads config from TOML/JSON/YAML files
/// @group Core Runtime
/// @since 0.1.0
#[derive(Debug, Clone)]
pub struct FileSource {
    path: String,
    required: bool,
}

impl FileSource {
    /// Create a new file source.
    ///
    /// @param path Path to configuration file
    /// @return FileSource instance
    /// @since 0.1.0
    pub fn new(path: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            required: true,
        }
    }

    /// Mark this source as optional (won't error if missing).
    ///
    /// @return Self for chaining
    /// @since 0.1.0
    pub fn optional(mut self) -> Self {
        self.required = false;
        self
    }
}
#[async_trait]
impl ConfigSource for FileSource {
    async fn load(&self) -> Result<HashMap<String, serde_json::Value>> {
        let path = Path::new(&self.path);
        if !path.exists() {
            if self.required {
                return Err(XiaoyiError::new(
                    ErrorKind::Config,
                    "config file not found",
                ).with_meta("path", &self.path));
            }
            return Ok(HashMap::new());
        }

        let content = tokio::fs::read_to_string(path).await
            .map_err(|e| XiaoyiError::new(ErrorKind::Config, "failed to read config file")
                .with_meta("path", &self.path)
                .with_meta("error", &e.to_string()))?;
        let ext = path.extension()
            .and_then(|s| s.to_str())
            .unwrap_or("")
            .to_lowercase();
        let value = match ext.as_str() {
            "toml" => toml::from_str(&content).map_err(|e| XiaoyiError::new(ErrorKind::Config, "failed to parse TOML").with_meta("error", &e.to_string())),
            "json" => serde_json::from_str(&content).map_err(|e| XiaoyiError::new(ErrorKind::Config, "failed to parse JSON").with_meta("error", &e.to_string())),
            "yaml" | "yml" => serde_yaml::from_str(&content).map_err(|e| XiaoyiError::new(ErrorKind::Config, "failed to parse YAML").with_meta("error", &e.to_string())),
            _ => return Err(XiaoyiError::new(
                ErrorKind::Config,
                "unsupported config file format",
            ).with_meta("path", &self.path).with_meta("extension", &ext)),
        };

        value.map_err(|e| XiaoyiError::new(ErrorKind::Config, "failed to parse config")
            .with_meta("path", &self.path)
            .with_meta("error", &e.to_string()))
    }
}

/// Path normalization utilities.
///
/// @brief Cross-platform path normalization
/// @group Core Runtime
/// @since 0.1.0
pub mod norm {
    use std::path::Path;
    ///
    /// @param path Input path
    /// @return Normalized path
    /// @since 0.1.0
    pub fn normalize(path: &Path) -> std::path::PathBuf {
        let mut components = Vec::new();
        for component in path.components() {
            match component {
                std::path::Component::ParentDir => {
                    if !components.is_empty() && !matches!(components.last(), Some(std::path::Component::ParentDir)) {
                        components.pop();
                    } else {
                        components.push(component);
                    }
                }
                std::path::Component::CurDir => {}
                _ => components.push(component),
            }
        }
        components.iter().collect()
    }
}

/// Convert path to absolute.
///
/// @param path Input path
/// @return Absolute path
/// @since 0.1.0
pub fn absolute(path: &Path) -> std::path::PathBuf {
    if path.is_absolute() {
        path.to_path_buf()
    } else {
        std::env::current_dir().unwrap_or_default().join(path)
    }
}

/// Convert path to Unix-style (forward slashes).
///
/// @param path Input path
/// @return Unix-style path string
/// @since 0.1.0
pub fn unix(path: &Path) -> String {
    path.to_string_lossy().replace('\\', "/")
}