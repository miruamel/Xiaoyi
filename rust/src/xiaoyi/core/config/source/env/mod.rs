//! # Layer 0 - Foundation / Core Config Source Env
//!
//! Environment config source reads values from process environment variables.
//! It is the lowest-priority source so that explicit file/vault values override it.
//!
//! Path: `xiaoyi::core::config::source::env`
//!
//! Layer hierarchy:
//! - 0: `core` — foundational types shared by SDK and runtime.
//! - 1: `config` — unified configuration model.
//! - 2: `source` — provider trait and loaders.
//! - 3: `env` — environment variable provider.

use std::collections::HashMap;
use std::env;

#[derive(Debug, Default, Clone)]
pub struct EnvSource {
    pub prefix: Option<String>,
    pub vars: HashMap<String, String>,
}

impl EnvSource {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_prefix(mut self, prefix: impl Into<String>) -> Self {
        self.prefix = Some(prefix.into());
        self
    }

    pub fn load(&mut self) {
        self.vars = env::vars().collect();
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        let lookup = if let Some(prefix) = &self.prefix {
            format!("{}_{}", prefix.to_uppercase(), key.to_uppercase())
        } else {
            key.to_uppercase()
        };
        self.vars.get(&lookup).map(|s| s.as_str())
    }
}
