//! # Builder AST Module
//!
//! `ast` provides AST manipulation and sanitization for agent building.
//!
//! Path: `xiaoyi::builder::ast`
//!
//! @module builder::ast
//! @brief AST manipulation for agent construction
//! @group Agent Composition
//! @since 0.1.0
//! @author Miruamel
//! @see crate::builder

/// AST node for agent definition.
///
/// @brief Agent AST node
/// @group Agent Composition
/// @since 0.1.0
#[derive(Debug, Clone)]
pub struct AgentAst {
    pub name: String,
    pub model: String,
    pub tools: Vec<String>,
}

/// Sanitize AST for safety.
///
/// @brief Remove unsafe constructs
/// @group Agent Composition
/// @since 0.1.0
pub fn sanitize(ast: &mut AgentAst) {
    // Remove potentially dangerous constructs
    ast.tools.retain(|t| !t.starts_with("dangerous_"));
}
