//! # Builder Template Module
//!
//! `template` provides template system for agent generation.
//!
//! Path: `xiaoyi::builder::template`
//!
//! @module builder::template
//! @brief Template system for agent construction
//! @group Agent Composition
//! @since 0.1.0
//! @author Miruamel
//! @see crate::builder::ast

/// Template for agent generation.
///
/// @brief Agent template definition
/// @group Agent Composition
/// @since 0.1.0
#[derive(Debug, Clone)]
pub struct Template {
    pub name: String,
    pub content: String,
}

impl Template {
    /// Create new template.
    ///
    /// @param name Template name
    /// @param content Template content
    /// @return Template instance
    /// @since 0.1.0
    pub fn new(name: impl Into<String>, content: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            content: content.into(),
        }
    }
}
