//! @module knowledge::graph::ast_graph::edge
//! @brief AST edge definition and kinds
//! @group Knowledge
//! @since 0.1.0
//! @author Miruamel
//! @see crate::knowledge::graph::ast_graph

/// Kind of an abstract syntax tree edge.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AstEdgeKind {
    /// Contains relationship (parent/child).
    Contains,
    /// Call relationship.
    Calls,
    /// Import relationship.
    Imports,
    /// Implementation relationship.
    Implements,
    /// Reference relationship.
    References,
}

impl AstEdgeKind {
    /// Return a string representation of the edge kind.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Contains => "contains",
            Self::Calls => "calls",
            Self::Imports => "imports",
            Self::Implements => "implements",
            Self::References => "references",
        }
    }
}

/// An edge in the abstract syntax tree graph.
#[derive(Debug, Clone)]
pub struct AstEdge {
    /// Source node ID.
    pub from: u64,
    /// Target node ID.
    pub to: u64,
    /// The kind of relationship.
    pub kind: AstEdgeKind,
}

impl AstEdge {
    /// Create a new AST edge.
    pub fn new(from: u64, to: u64, kind: AstEdgeKind) -> Self {
        Self { from, to, kind }
    }
}