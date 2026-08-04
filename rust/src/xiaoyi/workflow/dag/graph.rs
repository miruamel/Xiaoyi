//! Workflow DAG graph structure.
//!
//! Path: `xiaoyi::workflow::dag::graph`
//!
//! Layer hierarchy:
//! - 0: workflow
//! - 1: dag
//! - 2: graph
//! - 3: node/edge/cycle
//!
//! Directed acyclic graph for workflow execution ordering.

use std::collections::{HashMap, HashSet, VecDeque};
use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::algo::toposort;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NodeId(pub String);

#[derive(Debug, Clone)]
pub struct DagNode {
    pub id: NodeId,
    pub label: String,
    pub kind: NodeKind,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeKind {
    Task,
    Agent,
    Chain,
    Conditional,
    Parallel,
}

#[derive(Debug, Clone)]
pub struct DagEdge {
    pub from: NodeId,
    pub to: NodeId,
    pub kind: EdgeKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EdgeKind {
    Sequential,
    Conditional,
    Parallel,
}

#[derive(Debug, Default)]
pub struct DagGraph {
    graph: DiGraph<DagNode, DagEdge>,
    node_indices: HashMap<NodeId, NodeIndex>,
}

impl DagGraph {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_node(&mut self, node: DagNode) -> NodeIndex {
        let idx = self.graph.add_node(node.clone());
        self.node_indices.insert(node.id, idx);
        idx
    }

    pub fn add_edge(&mut self, edge: DagEdge) -> Result<(), String> {
        let from_idx = self.node_indices.get(&edge.from).ok_or("source node not found")?;
        let to_idx = self.node_indices.get(&edge.to).ok_or("target node not found")?;
        self.graph.add_edge(*from_idx, *to_idx, edge);
        Ok(())
    }

    pub fn topological_order(&self) -> Result<Vec<NodeId>, String> {
        let order = toposort(&self.graph, None).map_err(|_| "graph contains cycles")?;
        Ok(order.into_iter().map(|idx| self.graph[idx].id.clone()).collect())
    }

    pub fn get_node(&self, id: &NodeId) -> Option<&DagNode> {
        self.node_indices.get(id).map(|&idx| &self.graph[idx])
    }

    pub fn nodes(&self) -> impl Iterator<Item = &DagNode> {
        self.graph.node_weights()
    }
}