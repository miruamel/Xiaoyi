//! # Builder Codegen Module
//!
//! `codegen` generates code from sanitized AST.
//!
//! Path: `xiaoyi::builder::codegen`
//!
//! @module builder::codegen
//! @brief Code generation from agent AST
//! @group Agent Composition
//! @since 0.1.0
//! @author Miruamel
//! @see crate::builder::ast

use crate::xiaoyi::builder::ast::AgentAst;

/// Generate Rust code from AST.
///
/// @param ast Agent AST
/// @return Generated code
/// @since 0.1.0
pub fn generate_rust(ast: &AgentAst) -> String {
    format!(
        r#"// Generated agent: {}
struct {}({});
"#,
        ast.name, ast.name, ast.model
    )
}
