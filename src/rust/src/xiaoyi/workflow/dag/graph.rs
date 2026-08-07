//! # Workflow DAG Graph
//!
//! `graph` implements a directed acyclic graph using petgraph with
//! topological sorting for workflow execution ordering.
//!
//! Path: `xiaoyi::workflow::dag::graph`
//!
//! - Layer 0: `workflow`
//! - Layer 1: `dag`
//! - Layer 2: `graph` — graph with toposort.
//! - Layer 3: `node`/`edge`/`cycle` — primitives.
//!
//! @module workflow::dag::graph
//! @brief DAG graph with topological execution ordering
//! @group Orchestration
//! @since 0.1.0
//! @author Miruamel
//! @see crate::workflow::dag
//! @see crate::orchestrator::executor
//!
//! # Example
//!
//! ```rust
//! use xiaoyi::workflow::dag::graph::{DagGraph, DagNode, DagEdge, NodeId, NodeKind, EdgeKind};
//!
//! let mut graph = DagGraph::new();
//! let n1 = graph.add_node(DagNode::new(NodeId("task1".into()), "Task 1", NodeKind::Task));
//! let n2 = graph.add_node(DagNode::new(NodeId("task2".into()), "Task 2", NodeKind::Task));
//! graph.add_edge(DagEdge::new(NodeId("task1".into()), NodeId("task2".into()), EdgeKind::Sequential)).unwrap();
//!
//! let order = graph.topological_order().unwrap();
//! assert_eq!(order.len(), 2);
//! ```
//!
//! # Errors
//!
//! - Returns error if cycle detected during topological sort.
//! - Returns error if edge references non-existent node.
//! - NodeId must be unique within graph.
use petgraph::algo::toposort;
use petgraph::graph::{DiGraph, NodeIndex};
use std::collections::{HashMap, HashSet};

/// Unique node identifier.
///
/// @brief Unique identifier for DAG nodes
/// @group Orchestration
/// @since 0.1.0
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NodeId(pub String);

impl NodeId {
    /// Create new NodeId.
    ///
    /// @param id Unique string identifier
    /// @return NodeId instance
    /// @since 0.1.0
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }
}

/// DAG node with metadata.
///
/// @brief Workflow node with kind and metadata
/// @group Orchestration
/// @since 0.1.0
/// @see NodeKind
#[derive(Debug, Clone)]
pub struct DagNode {
    /// Unique node ID.
    pub id: NodeId,
    /// Human-readable label.
    pub label: String,
    /// Node type/kind.
    pub kind: NodeKind,
    /// Arbitrary metadata.
    pub metadata: HashMap<String, String>,
}

impl DagNode {
    /// Create a new node.
    ///
    /// @param id Unique identifier
    /// @param label Human-readable label
    /// @param kind Node kind
    /// @return New DagNode
    /// @since 0.1.0
    pub fn new(id: NodeId, label: impl Into<String>, kind: NodeKind) -> Self {
        Self {
            id,
            label: label.into(),
            kind,
            metadata: HashMap::new(),
        }
    }
}

/// Node kind classification.
///
/// @brief Node type in workflow
/// @group Orchestration
/// @since 0.1.0
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeKind {
    /// Single task unit.
    Task,
    /// Autonomous agent.
    Agent,
    /// Sequential chain.
    Chain,
    /// Conditional branch.
    Conditional,
    /// Conditional branch (Python binding compatibility).
    Condition,
    /// Parallel fan-out.
    Parallel,
    /// Merge node (Python binding compatibility).
    Merge,
}

/// DAG edge connecting nodes.
///
/// @brief Dependency edge between nodes
/// @group Orchestration
/// @since 0.1.0
/// @see EdgeKind
#[derive(Debug, Clone)]
pub struct DagEdge {
    /// Source node ID.
    pub from: NodeId,
    /// Target node ID.
    pub to: NodeId,
    /// Edge type.
    pub kind: EdgeKind,
}

impl DagEdge {
    /// Create a new edge.
    ///
    /// @param from Source node ID
    /// @param to Target node ID
    /// @param kind Edge kind
    /// @return New DagEdge
    /// @since 0.1.0
    pub fn new(from: NodeId, to: NodeId, kind: EdgeKind) -> Self {
        Self { from, to, kind }
    }
}

/// Edge kind classification.
///
/// @brief Dependency edge type
/// @group Orchestration
/// @since 0.1.0
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EdgeKind {
    /// Sequential dependency.
    Sequential,
    /// Conditional dependency.
    Conditional,
    /// Parallel fan-out.
    Parallel,
}

/// Directed acyclic graph for workflow execution.
///
/// @brief Workflow DAG with topological ordering
/// @group Orchestration
/// @since 0.1.0
/// @see DagNode
/// @see DagEdge
#[derive(Debug, Default)]
pub struct DagGraph {
    graph: DiGraph<DagNode, DagEdge>,
    node_indices: HashMap<NodeId, NodeIndex>,
}

impl DagGraph {
    /// Create a new empty DAG.
    ///
    /// @return Empty DagGraph
    /// @since 0.1.0
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a node to the graph.
    ///
    /// @param node Node to add
    /// @return Internal node index
    /// @since 0.1.0
    pub fn add_node(&mut self, node: DagNode) -> NodeIndex {
        let idx = self.graph.add_node(node.clone());
        self.node_indices.insert(node.id.clone(), idx);
        idx
    }

    /// Get NodeId from NodeIndex.
    ///
    /// @param idx Internal node index
    /// @return NodeId if found
    /// @since 0.1.0
    pub fn node_id(&self, idx: NodeIndex) -> Option<NodeId> {
        self.node_indices
            .iter()
            .find(|(_, &v)| v == idx)
            .map(|(k, _)| k.clone())
    }

    /// Add an edge between nodes.
    ///
    /// @param edge Edge to add
    /// @return Ok(()) or error if nodes not found
    /// @throw Error if source or target node missing
    /// @since 0.1.0
    pub fn add_edge(&mut self, edge: DagEdge) -> Result<(), String> {
        let from_idx = self
            .node_indices
            .get(&edge.from)
            .ok_or("source node not found")?;
        let to_idx = self
            .node_indices
            .get(&edge.to)
            .ok_or("target node not found")?;
        self.graph.add_edge(*from_idx, *to_idx, edge);
        Ok(())
    }

    /// Get topological execution order.
    ///
    /// @return Vector of NodeIds in execution order
    /// @throw Error if graph contains cycles
    /// @since 0.1.0
    pub fn topological_order(&self) -> Result<Vec<NodeId>, String> {
        let order = toposort(&self.graph, None).map_err(|_| "graph contains cycles")?;
        Ok(order
            .into_iter()
            .map(|idx| self.graph[idx].id.clone())
            .collect())
    }

    /// Get node by ID.
    ///
    /// @param id Node ID to lookup
    /// @return Optional reference to node
    /// @since 0.1.0
    pub fn get_node(&self, id: &NodeId) -> Option<&DagNode> {
        self.node_indices.get(id).map(|&idx| &self.graph[idx])
    }

    /// Iterate all nodes.
    ///
    /// @return Iterator over all nodes
    /// @since 0.1.0
    pub fn nodes(&self) -> impl Iterator<Item = &DagNode> {
        self.graph.node_weights()
    }

    /// Check if graph has cycles.
    ///
    /// @return true if cycle exists
    /// @since 0.1.0
    pub fn has_cycles(&self) -> bool {
        toposort(&self.graph, None).is_err()
    }

    /// Get node count.
    ///
    /// @return Number of nodes
    /// @since 0.1.0
    pub fn node_count(&self) -> usize {
        self.graph.node_count()
    }

    /// Get edge count.
    ///
    /// @return Number of edges
    /// @since 0.1.0
    pub fn edge_count(&self) -> usize {
        self.graph.edge_count()
    }
}

/// Result type for DAG operations.
type Result<T, E = String> = std::result::Result<T, E>;
