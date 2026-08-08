//! # Workflow DAG
//!
//! `dag` provides the core DAG data structure and execution logic.
//!
//! Path: `xiaoyi::workflow::dag`
//!
//! - Layer 0: `workflow`
//! - Layer 1: `dag` — DAG implementation.
//! - Layer 2: `graph` — graph with topological sorting.
//! - Layer 3: `node`/`edge`/`cycle` — primitives.
//!
//! @module workflow::dag
//! @brief Directed acyclic graph for workflow execution
//! @group Orchestration
//! @since 0.1.0
//! @author Miruamel
//! @see crate::workflow
//! @see crate::workflow::dag::graph
pub mod graph;
pub mod scheduler;

// Re-exports from graph
pub use graph::{DagEdge, DagGraph as Dag, DagGraph, DagNode, EdgeKind, NodeId, NodeKind};
