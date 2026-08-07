//! # Workflow DAG Tests
//!
//! Tests for `xiaoyi::workflow::dag::graph` DAG operations.
//!
//! @module tests::workflow_dag
//! @brief Unit tests for workflow DAG
//! @group Orchestration
//! @since 0.1.0
//! @author Miruamel
//! @see crate::workflow::dag::graph

use pretty_assertions::assert_eq;
use xiaoyi::workflow::dag::graph::{DagGraph, DagNode, DagEdge, NodeId, NodeKind, EdgeKind};

#[test]
fn test_node_id() {
    let id = NodeId("task1".to_string());
    assert_eq!(id.as_str(), "task1");
    assert_eq!(id.0, "task1");
}

#[test]
fn test_node_id_equality() {
    assert_eq!(NodeId("a".to_string()), NodeId("a".to_string()));
    assert_ne!(NodeId("a".to_string()), NodeId("b".to_string()));
}

#[test]
fn test_dag_node_creation() {
    let node = DagNode::new(NodeId("task1".into()), "Task 1", NodeKind::Task);
    assert_eq!(node.id().as_str(), "task1");
    assert_eq!(node.name(), "Task 1");
    assert_eq!(node.kind(), NodeKind::Task);
}

#[test]
fn test_dag_node_kinds() {
    assert_eq!(NodeKind::Task, NodeKind::Task);
    assert_eq!(NodeKind::Start, NodeKind::Start);
    assert_eq!(NodeKind::End, NodeKind::End);
    assert_ne!(NodeKind::Task, NodeKind::Start);
}

#[test]
fn test_dag_edge_creation() {
    let edge = DagEdge::new(NodeId("a".into()), NodeId("b".into()), EdgeKind::Sequential);
    assert_eq!(edge.from().as_str(), "a");
    assert_eq!(edge.to().as_str(), "b");
    assert_eq!(edge.kind(), EdgeKind::Sequential);
}

#[test]
fn test_dag_edge_kinds() {
    assert_eq!(EdgeKind::Sequential, EdgeKind::Sequential);
    assert_eq!(EdgeKind::Parallel, EdgeKind::Parallel);
    assert_eq!(EdgeKind::Conditional, EdgeKind::Conditional);
    assert_ne!(EdgeKind::Sequential, EdgeKind::Parallel);
}

#[test]
fn test_dag_graph_new() {
    let graph = DagGraph::new();
    assert!(graph.is_empty());
    assert_eq!(graph.node_count(), 0);
    assert_eq!(graph.edge_count(), 0);
}

#[test]
fn test_dag_graph_add_node() {
    let mut graph = DagGraph::new();
    let idx = graph.add_node(DagNode::new(NodeId("task1".into()), "Task 1", NodeKind::Task));

    assert_eq!(graph.node_count(), 1);
    assert_eq!(graph.edge_count(), 0);
    assert!(!graph.is_empty());

    let node = graph.get_node(&NodeId("task1".into())).unwrap();
    assert_eq!(node.id().as_str(), "task1");
}

#[test]
fn test_dag_graph_duplicate_node() {
    let mut graph = DagGraph::new();
    graph.add_node(DagNode::new(NodeId("task1".into()), "Task 1", NodeKind::Task));

    // Adding duplicate should not increase count (or return existing index)
    let idx2 = graph.add_node(DagNode::new(NodeId("task1".into()), "Task 1 Updated", NodeKind::Task));
    // Implementation may replace or keep first - verify count doesn't exceed 1
    assert_eq!(graph.node_count(), 1);
}

#[test]
fn test_dag_graph_add_edge() {
    let mut graph = DagGraph::new();
    graph.add_node(DagNode::new(NodeId("a".into()), "A", NodeKind::Task));
    graph.add_node(DagNode::new(NodeId("b".into()), "B", NodeKind::Task));

    let result = graph.add_edge(DagEdge::new(NodeId("a".into()), NodeId("b".into()), EdgeKind::Sequential));
    assert!(result.is_ok());
    assert_eq!(graph.edge_count(), 1);
}

#[test]
fn test_dag_graph_add_edge_missing_node() {
    let mut graph = DagGraph::new();
    graph.add_node(DagNode::new(NodeId("a".into()), "A", NodeKind::Task));

    // Edge to non-existent node
    let result = graph.add_edge(DagEdge::new(NodeId("a".into()), NodeId("b".into()), EdgeKind::Sequential));
    assert!(result.is_err());
}

#[test]
fn test_dag_graph_topological_order_simple() {
    let mut graph = DagGraph::new();
    graph.add_node(DagNode::new(NodeId("a".into()), "A", NodeKind::Task));
    graph.add_node(DagNode::new(NodeId("b".into()), "B", NodeKind::Task));
    graph.add_node(DagNode::new(NodeId("c".into()), "C", NodeKind::Task));

    graph.add_edge(DagEdge::new(NodeId("a".into()), NodeId("b".into()), EdgeKind::Sequential)).unwrap();
    graph.add_edge(DagEdge::new(NodeId("b".into()), NodeId("c".into()), EdgeKind::Sequential)).unwrap();

    let order = graph.topological_order().unwrap();
    assert_eq!(order.len(), 3);

    // a must come before b, b before c
    let a_pos = order.iter().position(|n| n.as_str() == "a").unwrap();
    let b_pos = order.iter().position(|n| n.as_str() == "b").unwrap();
    let c_pos = order.iter().position(|n| n.as_str() == "c").unwrap();

    assert!(a_pos < b_pos);
    assert!(b_pos < c_pos);
}

#[test]
fn test_dag_graph_topological_order_branching() {
    let mut graph = DagGraph::new();
    graph.add_node(DagNode::new(NodeId("start".into()), "Start", NodeKind::Start));
    graph.add_node(DagNode::new(NodeId("a".into()), "A", NodeKind::Task));
    graph.add_node(DagNode::new(NodeId("b".into()), "B", NodeKind::Task));
    graph.add_node(DagNode::new(NodeId("end".into()), "End", NodeKind::End));

    // start -> a, start -> b
    graph.add_edge(DagEdge::new(NodeId("start".into()), NodeId("a".into()), EdgeKind::Parallel)).unwrap();
    graph.add_edge(DagEdge::new(NodeId("start".into()), NodeId("b".into()), EdgeKind::Parallel)).unwrap();
    // a -> end, b -> end
    graph.add_edge(DagEdge::new(NodeId("a".into()), NodeId("end".into()), EdgeKind::Sequential)).unwrap();
    graph.add_edge(DagEdge::new(NodeId("b".into()), NodeId("end".into()), EdgeKind::Sequential)).unwrap();

    let order = graph.topological_order().unwrap();
    assert_eq!(order.len(), 4);

    let start_pos = order.iter().position(|n| n.as_str() == "start").unwrap();
    let a_pos = order.iter().position(|n| n.as_str() == "a").unwrap();
    let b_pos = order.iter().position(|n| n.as_str() == "b").unwrap();
    let end_pos = order.iter().position(|n| n.as_str() == "end").unwrap();

    assert!(start_pos < a_pos);
    assert!(start_pos < b_pos);
    assert!(a_pos < end_pos);
    assert!(b_pos < end_pos);
}

#[test]
fn test_dag_graph_cycle_detection() {
    let mut graph = DagGraph::new();
    graph.add_node(DagNode::new(NodeId("a".into()), "A", NodeKind::Task));
    graph.add_node(DagNode::new(NodeId("b".into()), "B", NodeKind::Task));
    graph.add_node(DagNode::new(NodeId("c".into()), "C", NodeKind::Task));

    graph.add_edge(DagEdge::new(NodeId("a".into()), NodeId("b".into()), EdgeKind::Sequential)).unwrap();
    graph.add_edge(DagEdge::new(NodeId("b".into()), NodeId("c".into()), EdgeKind::Sequential)).unwrap();
    graph.add_edge(DagEdge::new(NodeId("c".into()), NodeId("a".into()), EdgeKind::Sequential)).unwrap(); // Cycle!

    let result = graph.topological_order();
    assert!(result.is_err());
}

#[test]
fn test_dag_graph_self_cycle() {
    let mut graph = DagGraph::new();
    graph.add_node(DagNode::new(NodeId("a".into()), "A", NodeKind::Task));

    let result = graph.add_edge(DagEdge::new(NodeId("a".into()), NodeId("a".into()), EdgeKind::Sequential));
    // Self-edge may be allowed at add_edge but caught at toposort
    let order = graph.topological_order();
    assert!(order.is_err());
}

#[test]
fn test_dag_graph_isolated_nodes() {
    let mut graph = DagGraph::new();
    graph.add_node(DagNode::new(NodeId("a".into()), "A", NodeKind::Task));
    graph.add_node(DagNode::new(NodeId("b".into()), "B", NodeKind::Task));
    graph.add_node(DagNode::new(NodeId("c".into()), "C", NodeKind::Task));

    // Only connect a -> b, c is isolated
    graph.add_edge(DagEdge::new(NodeId("a".into()), NodeId("b".into()), EdgeKind::Sequential)).unwrap();

    let order = graph.topological_order().unwrap();
    assert_eq!(order.len(), 3); // All nodes should be in order
    assert!(order.contains(&NodeId("c".into())));
}

#[test]
fn test_dag_graph_get_node() {
    let mut graph = DagGraph::new();
    graph.add_node(DagNode::new(NodeId("task1".into()), "Task 1", NodeKind::Task));

    let node = graph.get_node(&NodeId("task1".into()));
    assert!(node.is_some());
    assert_eq!(node.unwrap().name(), "Task 1");

    let missing = graph.get_node(&NodeId("missing".into()));
    assert!(missing.is_none());
}

#[test]
fn test_dag_graph_multiple_edges_same_nodes() {
    let mut graph = DagGraph::new();
    graph.add_node(DagNode::new(NodeId("a".into()), "A", NodeKind::Task));
    graph.add_node(DagNode::new(NodeId("b".into()), "B", NodeKind::Task));

    graph.add_edge(DagEdge::new(NodeId("a".into()), NodeId("b".into()), EdgeKind::Sequential)).unwrap();
    // Add another edge between same nodes with different kind
    graph.add_edge(DagEdge::new(NodeId("a".into()), NodeId("b".into()), EdgeKind::Parallel)).unwrap();

    assert_eq!(graph.edge_count(), 2);
}

#[test]
fn test_dag_graph_complex_dag() {
    let mut graph = DagGraph::new();

    // Build a diamond DAG:
    //     start
    //    /    \
    //   a      b
    //    \    /
    //     end

    graph.add_node(DagNode::new(NodeId("start".into()), "Start", NodeKind::Start));
    graph.add_node(DagNode::new(NodeId("a".into()), "A", NodeKind::Task));
    graph.add_node(DagNode::new(NodeId("b".into()), "B", NodeKind::Task));
    graph.add_node(DagNode::new(NodeId("end".into()), "End", NodeKind::End));

    graph.add_edge(DagEdge::new(NodeId("start".into()), NodeId("a".into()), EdgeKind::Parallel)).unwrap();
    graph.add_edge(DagEdge::new(NodeId("start".into()), NodeId("b".into()), EdgeKind::Parallel)).unwrap();
    graph.add_edge(DagEdge::new(NodeId("a".into()), NodeId("end".into()), EdgeKind::Sequential)).unwrap();
    graph.add_edge(DagEdge::new(NodeId("b".into()), NodeId("end".into()), EdgeKind::Sequential)).unwrap();

    let order = graph.topological_order().unwrap();
    assert_eq!(order.len(), 4);

    let start = order.iter().position(|n| n.as_str() == "start").unwrap();
    let a = order.iter().position(|n| n.as_str() == "a").unwrap();
    let b = order.iter().position(|n| n.as_str() == "b").unwrap();
    let end = order.iter().position(|n| n.as_str() == "end").unwrap();

    assert!(start < a);
    assert!(start < b);
    assert!(a < end);
    assert!(b < end);
}