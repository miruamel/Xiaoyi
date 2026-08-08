//! # Builder Module
//!
//! `builder` provides agent construction with AST sanitization and code generation.
//!
//! Path: `xiaoyi::builder`
//!
//! - Layer 0: `builder` — Agent builder layer.
//! - Layer 1: `ast` — AST manipulation and sanitization.
//! - Layer 2: `codegen` — Code generation from AST.
//! - Layer 3: `template` — Template system for agents.
//!
//! @module builder
//! @brief Agent builder with AST sanitization
//! @group Agent Composition
//! @since 0.1.0
//! @author Miruamel
//! @see crate::orchestrator
//! @see crate::gateway
//!
/// # Example
///
/// ```rust
/// use xiaoyi::builder::AgentBuilder;
/// use xiaoyi::core::config::Config;
///
/// let config = Config::default();
/// let agent = AgentBuilder::new(config)
///     .name("assistant")
///     .model("gpt-4")
///     .build()?;
/// ```
pub mod ast;
pub mod codegen;
pub mod formatter;
pub mod template;
pub mod validator;

use crate::xiaoyi::core::config::Config;
use crate::xiaoyi::core::error::Result;

/// Agent builder for constructing agents.
///
/// @brief Fluent agent construction
/// @group Agent Composition
/// @since 0.1.0
#[derive(Debug, Clone)]
pub struct AgentBuilder {
    config: Config,
    name: Option<String>,
    model: Option<String>,
}

impl AgentBuilder {
    /// Create new agent builder.
    ///
    /// @param config Runtime configuration
    /// @return AgentBuilder instance
    /// @since 0.1.0
    pub fn new(config: Config) -> Self {
        Self {
            config,
            name: None,
            model: None,
        }
    }

    /// Set agent name.
    ///
    /// @param name Agent name
    /// @return Self for chaining
    /// @since 0.1.0
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Set model name.
    ///
    /// @param model Model identifier
    /// @return Self for chaining
    /// @since 0.1.0
    pub fn model(mut self, model: impl Into<String>) -> Self {
        self.model = Some(model.into());
        self
    }

    pub fn build(self) -> Result<AgentHandle> {
        Ok(AgentHandle {
            name: self.name.unwrap_or_default(),
            model: self.model.unwrap_or_default(),
        })
    }
}

/// Built agent handle.
///
/// @brief Constructed agent reference
/// @group Agent Composition
/// @since 0.1.0
#[derive(Debug, Clone)]
pub struct AgentHandle {
    pub name: String,
    pub model: String,
}
