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

use std::fmt;

/// Unified error kind for Xiaoyi runtime.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ErrorKind {
    Syntax,
    Parse,
    Runtime,
    Io,
    Auth,
    Policy,
    Llm,
    Memory,
    Tool,
    Workflow,
    Config,
    State,
}

/// Structured error value with kind, source location, and metadata map.
#[derive(Debug, Clone)]
pub struct XiaoyiError {
    pub kind: ErrorKind,
    pub message: String,
    pub meta: Vec<(String, String)>,
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

/// Helper to construct errors with metadata.
impl XiaoyiError {
    pub fn new(kind: ErrorKind, message: impl Into<String>) -> Self {
        Self {
            kind,
            message: message.into(),
            meta: Vec::new(),
        }
    }

    pub fn with_meta(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.meta.push((key.into(), value.into()));
        self
    }
}

pub type Result<T> = std::result::Result<T, XiaoyiError>;
