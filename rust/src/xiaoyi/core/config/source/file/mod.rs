//! Configuration file source with path normalization.
//!
//! Path: `xiaoyi::core::config::source::file`
//!
//! Layer hierarchy:
//! - 0: core
//! - 1: config
//! - 2: source
//! - 3: file
//! - 4: path/absolute/unix/norm

use std::path::{Path, PathBuf};
use crate::xiaoyi::core::error::{ErrorKind, Result, XiaoyiError};

/// File-based configuration source.
#[derive(Debug, Clone)]
pub struct FileSource {
    pub path: PathBuf,
    pub required: bool,
}

impl FileSource {
    pub fn new(path: impl AsRef<Path>) -> Self {
        Self {
            path: path.as_ref().to_path_buf(),
            required: true,
        }
    }

    pub fn optional(mut self) -> Self {
        self.required = false;
        self
    }

    /// Load and parse configuration file (TOML, JSON, YAML).
    pub fn load(&self) -> Result<serde_json::Value> {
        let content = std::fs::read_to_string(&self.path).map_err(|e| {
            XiaoyiError::new(ErrorKind::Config, "failed to read config file")
                .with_meta("path", self.path.display().to_string())
                .with_meta("error", e.to_string())
        })?;

        let ext = self.path.extension().and_then(|e| e.to_str()).unwrap_or("");
        match ext {
            "toml" => toml::from_str(&content).map_err(|e| {
                XiaoyiError::new(ErrorKind::Config, "failed to parse TOML")
                    .with_meta("path", self.path.display().to_string())
                    .with_meta("error", e.to_string())
            }),
            "json" => serde_json::from_str(&content).map_err(|e| {
                XiaoyiError::new(ErrorKind::Config, "failed to parse JSON")
                    .with_meta("path", self.path.display().to_string())
                    .with_meta("error", e.to_string())
            }),
            "yaml" | "yml" => serde_yaml::from_str(&content).map_err(|e| {
                XiaoyiError::new(ErrorKind::Config, "failed to parse YAML")
                    .with_meta("path", self.path.display().to_string())
                    .with_meta("error", e.to_string())
            }),
            _ => Err(XiaoyiError::new(
                ErrorKind::Config,
                "unsupported config file format",
            )
            .with_meta("path", self.path.display().to_string())
            .with_meta("extension", ext)),
        }
    }
}

/// Path normalization utilities.
pub mod norm {
    use std::path::{Path, PathBuf};

    /// Normalize a path (resolve . and .., remove redundant separators).
    pub fn normalize(path: &Path) -> PathBuf {
        let mut components = Vec::new();
        for comp in path.components() {
            match comp {
                std::path::Component::ParentDir => {
                    if !components.is_empty() && !matches!(components.last(), Some(std::path::Component::ParentDir)) {
                        components.pop();
                    } else {
                        components.push(comp);
                    }
                }
                std::path::Component::CurDir => {}
                _ => components.push(comp),
            }
        }
        PathBuf::from_iter(components)
    }

    /// Convert to absolute path.
    pub fn absolute(path: &Path) -> PathBuf {
        if path.is_absolute() {
            path.to_path_buf()
        } else {
            std::env::current_dir().unwrap_or_default().join(path)
        }
    }

    /// Convert to Unix-style path (forward slashes).
    pub fn unix(path: &Path) -> String {
        path.to_string_lossy().replace('\\', "/")
    }
}