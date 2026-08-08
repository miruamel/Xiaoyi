//! @module knowledge::graph::ast_graph::graph
//! @brief Directed graph over AST nodes with dependency analysis
//! @group Knowledge
//! @since 0.1.0
//! @author Miruamel
//! @see crate::knowledge::graph::ast_graph

use petgraph::algo::is_cyclic_directed;
use petgraph::graph::{Graph, NodeIndex};
use std::collections::HashMap;

use crate::xiaoyi::core::error::{ErrorKind, Result, XiaoyiError};

use super::{AstEdge, AstEdgeKind, AstNode};

/// Directed graph over abstract syntax tree nodes.
#[derive(Debug, Clone, Default)]
pub struct AstGraph {
    /// The underlying petgraph structure.
    inner: Graph<AstNode, AstEdgeKind>,
    /// Mapping from node IDs to graph node indices.
    ids: HashMap<u64, NodeIndex>,
    /// Next available node ID for new nodes.
    next: u64,
}

impl AstGraph {
    /// Create a new, empty AST graph.
    pub fn new() -> Self {
        Self {
            inner: Graph::new(),
            ids: HashMap::new(),
            next: 1,
        }
    }

    /// Add a node to the graph and return its ID.
    pub fn add_node(&mut self, node: AstNode) -> u64 {
        let id = self.next;
        self.next += 1;
        let idx = self.inner.add_node(node);
        self.ids.insert(id, idx);
        id
    }

    /// Add an edge to the graph.
    pub fn add_edge(&mut self, edge: AstEdge) -> Result<()> {
        let from_idx = self.ids.get(&edge.from).ok_or_else(|| {
            XiaoyiError::new(
                ErrorKind::Tool,
                format!("Source node ID {} not found in graph", edge.from),
            )
        })?;
        let to_idx = self.ids.get(&edge.to).ok_or_else(|| {
            XiaoyiError::new(
                ErrorKind::Tool,
                format!("Target node ID {} not found in graph", edge.to),
            )
        })?;
        self.inner.add_edge(*from_idx, *to_idx, edge.kind);
        Ok(())
    }

    /// Look up a node by its ID.
    pub fn node(&self, id: u64) -> Option<&AstNode> {
        self.ids
            .get(&id)
            .and_then(|idx| self.inner.node_weight(*idx))
    }

    /// Return references to all nodes in the graph.
    pub fn nodes(&self) -> Vec<&AstNode> {
        self.inner
            .node_indices()
            .filter_map(|idx| self.inner.node_weight(idx))
            .collect()
    }

    /// Reconstruct edges from the underlying petgraph and return them.
    pub fn edges(&self) -> Vec<AstEdge> {
        self.inner
            .edge_indices()
            .filter_map(|edge_idx| {
                let (from, to) = self.inner.edge_endpoints(edge_idx)?;
                let from_id = self.node_id(from)?;
                let to_id = self.node_id(to)?;
                let kind = self.inner.edge_weight(edge_idx)?;
                Some(AstEdge::new(from_id, to_id, *kind))
            })
            .collect()
    }

    /// Return node IDs of all callees reachable via a Calls edge from the given node.
    pub fn callees(&self, id: u64) -> Vec<u64> {
        self.node_indices_by_id(id)
            .map(|idx| {
                self.inner
                    .neighbors_directed(idx, petgraph::Direction::Outgoing)
                    .filter_map(|neighbor| self.node_id(neighbor))
                    .filter(|&callee_id| self.edge_kind(id, callee_id) == Some(AstEdgeKind::Calls))
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default()
    }

    /// Return node IDs of all callers that call the given node via a Calls edge.
    pub fn callers(&self, id: u64) -> Vec<u64> {
        self.node_indices_by_id(id)
            .map(|idx| {
                self.inner
                    .neighbors_directed(idx, petgraph::Direction::Incoming)
                    .filter_map(|caller| self.node_id(caller))
                    .filter(|&caller_id| self.edge_kind(caller_id, id) == Some(AstEdgeKind::Calls))
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default()
    }

    /// Return true if the graph contains a directed cycle.
    pub fn has_cycle(&self) -> bool {
        is_cyclic_directed(&self.inner)
    }

    /// Helper: get node ID from a NodeIndex if possible.
    fn node_id(&self, idx: NodeIndex) -> Option<u64> {
        self.ids
            .iter()
            .find_map(|(&id, &node_idx)| (node_idx == idx).then_some(id))
    }

    /// Helper: get edge kind between two nodes by their IDs.
    fn edge_kind(&self, from_id: u64, to_id: u64) -> Option<AstEdgeKind> {
        let from_idx = self.ids.get(&from_id)?;
        let to_idx = self.ids.get(&to_id)?;
        self.inner
            .find_edge(*from_idx, *to_idx)
            .and_then(|edge_idx| self.inner.edge_weight(edge_idx).cloned())
    }

    /// Helper: get NodeIndex for a node ID.
    fn node_indices_by_id(&self, id: u64) -> Option<NodeIndex> {
        self.ids.get(&id).copied()
    }
}
