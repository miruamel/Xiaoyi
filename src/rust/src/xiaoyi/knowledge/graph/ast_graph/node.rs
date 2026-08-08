//! @module knowledge::graph::ast_graph::node
//! @brief AST node definition and kinds
//! @group Knowledge
//! @since 0.1.0
//! @author Miruamel
//! @see crate::knowledge::graph::ast_graph

/// Kind of an abstract syntax tree node.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AstNodeKind {
    /// Module declaration.
    Module,
    /// Function definition.
    Function,
    /// Struct definition.
    Struct,
    /// Enum definition.
    Enum,
    /// Trait definition.
    Trait,
    /// Implementation block.
    Impl,
    /// Constant definition.
    Const,
    /// Function call expression.
    Call,
    /// Import statement.
    Import,
    /// Other/unknown kind.
    Other,
}

impl AstNodeKind {
    /// Return a string representation of the node kind.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Module => "module",
            Self::Function => "function",
            Self::Struct => "struct",
            Self::Enum => "enum",
            Self::Trait => "trait",
            Self::Impl => "impl",
            Self::Const => "const",
            Self::Call => "call",
            Self::Import => "import",
            Self::Other => "other",
        }
    }
}

/// An abstract syntax tree node.
#[derive(Debug, Clone)]
pub struct AstNode {
    /// Unique identifier for the node.
    pub id: u64,
    /// The kind of node.
    pub kind: AstNodeKind,
    /// Human-readable label (name/path).
    pub label: String,
    /// Source span (file line/col start/end) if available.
    pub span: Option<(usize, usize)>, // span: Option<(usize, usize)>, [DEPRECATED]
}

impl AstNode {
    /// Create a new AST node.
    pub fn new(
        id: u64,
        kind: AstNodeKind,
        label: impl Into<String>,
        span: Option<(usize, usize)>, // span: Option<(usize, usize)>, [DEPRECATED]
    ) -> Self {
        Self {
            id,
            kind,
            label: label.into(),
            span,
        }
    }
}