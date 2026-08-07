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
use xiaoyi::workflow::dag::graph::{EdgeKind, NodeId, NodeKind};
use xiaoyi::{DagEdge, DagGraph, DagNode};

#[test]
fn test_node_id() {
    let id = NodeId::new("task1");
    assert_eq!(id.0, "task1");
}

#[test]
fn test_node_id_equality() {
    let id1 = NodeId::new("task1");
    let id2 = NodeId::new("task1");
    let id3 = NodeId::new("task2");

    assert_eq!(id1, id2);
    assert_ne!(id1, id3);
}

#[test]
fn test_dag_node_creation() {
    let node = DagNode::new(NodeId::new("task1"), "Task 1", NodeKind::Task);
    assert_eq!(node.id.0, "task1");
    assert_eq!(node.label, "Task 1");
    assert_eq!(node.kind, NodeKind::Task);
}

#[test]
fn test_dag_node_kinds() {
    assert_eq!(NodeKind::Task, NodeKind::Task);
    assert_eq!(NodeKind::Agent, NodeKind::Agent);
    assert_eq!(NodeKind::Chain, NodeKind::Chain);
    assert_eq!(NodeKind::Conditional, NodeKind::Conditional);
    assert_eq!(NodeKind::Parallel, NodeKind::Parallel);

    assert_ne!(NodeKind::Task, NodeKind::Agent);
}

#[test]
fn test_dag_edge_creation() {
    let edge = DagEdge::new(
        NodeId::new("task1"),
        NodeId::new("task2"),
        EdgeKind::Sequential,
    );
    assert_eq!(edge.from.0, "task1");
    assert_eq!(edge.to.0, "task2");
    assert_eq!(edge.kind, EdgeKind::Sequential);
}

#[test]
fn test_dag_edge_kinds() {
    assert_eq!(EdgeKind::Sequential, EdgeKind::Sequential);
    assert_eq!(EdgeKind::Conditional, EdgeKind::Conditional);
    assert_eq!(EdgeKind::Parallel, EdgeKind::Parallel);

    assert_ne!(EdgeKind::Sequential, EdgeKind::Conditional);
}

#[test]
fn test_dag_graph_new() {
    let graph = DagGraph::new();
    assert_eq!(graph.node_count(), 0);
    assert_eq!(graph.edge_count(), 0);
}

#[test]
fn test_dag_graph_add_node() {
    let mut graph = DagGraph::new();
    let node = DagNode::new(NodeId::new("task1"), "Task 1", NodeKind::Task);
    let _idx = graph.add_node(node);

    assert_eq!(graph.node_count(), 1);
    assert!(graph.get_node(&NodeId::new("task1")).is_some());
}

#[test]
fn test_dag_graph_duplicate_node() {
    let mut graph = DagGraph::new();
    let node1 = DagNode::new(NodeId::new("task1"), "Task 1", NodeKind::Task);
    let node2 = DagNode::new(NodeId::new("task1"), "Task 1 Duplicate", NodeKind::Task);

    graph.add_node(node1);
    graph.add_node(node2); // Second node with same ID is added (current behavior)

    // Both nodes are stored (current implementation doesn't deduplicate by ID)
    assert_eq!(graph.node_count(), 2);
}

#[test]
fn test_dag_graph_add_edge() {
    let mut graph = DagGraph::new();
    graph.add_node(DagNode::new(NodeId::new("task1"), "Task 1", NodeKind::Task));
    graph.add_node(DagNode::new(NodeId::new("task2"), "Task 2", NodeKind::Task));

    let result = graph.add_edge(DagEdge::new(
        NodeId::new("task1"),
        NodeId::new("task2"),
        EdgeKind::Sequential,
    ));
    assert!(result.is_ok());
    assert_eq!(graph.edge_count(), 1);
}

#[test]
fn test_dag_graph_add_edge_missing_node() {
    let mut graph = DagGraph::new();
    graph.add_node(DagNode::new(NodeId::new("task1"), "Task 1", NodeKind::Task));

    let result = graph.add_edge(DagEdge::new(
        NodeId::new("task1"),
        NodeId::new("nonexistent"),
        EdgeKind::Sequential,
    ));
    assert!(result.is_err());
}

#[test]
fn test_dag_graph_topological_order_simple() {
    let mut graph = DagGraph::new();
    graph.add_node(DagNode::new(NodeId::new("task1"), "Task 1", NodeKind::Task));
    graph.add_node(DagNode::new(NodeId::new("task2"), "Task 2", NodeKind::Task));
    graph
        .add_edge(DagEdge::new(
            NodeId::new("task1"),
            NodeId::new("task2"),
            EdgeKind::Sequential,
        ))
        .unwrap();

    let order = graph.topological_order().unwrap();
    assert_eq!(order.len(), 2);
    assert_eq!(order[0].0, "task1");
    assert_eq!(order[1].0, "task2");
}

#[test]
fn test_dag_graph_topological_order_branching() {
    let mut graph = DagGraph::new();
    graph.add_node(DagNode::new(NodeId::new("start"), "Start", NodeKind::Task));
    graph.add_node(DagNode::new(NodeId::new("task1"), "Task 1", NodeKind::Task));
    graph.add_node(DagNode::new(NodeId::new("task2"), "Task 2", NodeKind::Task));
    graph.add_node(DagNode::new(NodeId::new("end"), "End", NodeKind::Task));

    graph
        .add_edge(DagEdge::new(
            NodeId::new("start"),
            NodeId::new("task1"),
            EdgeKind::Sequential,
        ))
        .unwrap();
    graph
        .add_edge(DagEdge::new(
            NodeId::new("start"),
            NodeId::new("task2"),
            EdgeKind::Sequential,
        ))
        .unwrap();
    graph
        .add_edge(DagEdge::new(
            NodeId::new("task1"),
            NodeId::new("end"),
            EdgeKind::Sequential,
        ))
        .unwrap();
    graph
        .add_edge(DagEdge::new(
            NodeId::new("task2"),
            NodeId::new("end"),
            EdgeKind::Sequential,
        ))
        .unwrap();

    let order = graph.topological_order().unwrap();
    assert_eq!(order.len(), 4);
    assert_eq!(order[0].0, "start");
    assert_eq!(order[3].0, "end");
}

#[test]
fn test_dag_graph_cycle_detection() {
    let mut graph = DagGraph::new();
    graph.add_node(DagNode::new(NodeId::new("a"), "A", NodeKind::Task));
    graph.add_node(DagNode::new(NodeId::new("b"), "B", NodeKind::Task));
    graph
        .add_edge(DagEdge::new(
            NodeId::new("a"),
            NodeId::new("b"),
            EdgeKind::Sequential,
        ))
        .unwrap();
    graph
        .add_edge(DagEdge::new(
            NodeId::new("b"),
            NodeId::new("a"),
            EdgeKind::Sequential,
        ))
        .unwrap();

    let result = graph.topological_order();
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("cycle"));
}
#[test]
fn test_dag_graph_self_cycle() {
    let mut graph = DagGraph::new();
    graph.add_node(DagNode::new(NodeId::new("a"), "A", NodeKind::Task));
    let result = graph.add_edge(DagEdge::new(
        NodeId::new("a"),
        NodeId::new("a"),
        EdgeKind::Sequential,
    ));
    // Self-loop is added without error (current implementation doesn't check)
    assert!(result.is_ok());

    // But topological sort should fail
    let order = graph.topological_order();
    assert!(order.is_err());
    assert!(order.unwrap_err().contains("cycle"));
}

#[test]
fn test_dag_graph_isolated_nodes() {
    let mut graph = DagGraph::new();
    graph.add_node(DagNode::new(NodeId::new("task1"), "Task 1", NodeKind::Task));
    graph.add_node(DagNode::new(NodeId::new("task2"), "Task 2", NodeKind::Task));
    graph.add_node(DagNode::new(NodeId::new("task3"), "Task 3", NodeKind::Task));

    let order = graph.topological_order().unwrap();
    assert_eq!(order.len(), 3);
}

#[test]
fn test_dag_graph_get_node() {
    let mut graph = DagGraph::new();
    graph.add_node(DagNode::new(NodeId::new("task1"), "Task 1", NodeKind::Task));

    let node = graph.get_node(&NodeId::new("task1"));
    assert!(node.is_some());
    assert_eq!(node.unwrap().label, "Task 1");

    let missing = graph.get_node(&NodeId::new("nonexistent"));
    assert!(missing.is_none());
}

#[test]
fn test_dag_graph_multiple_edges_same_nodes() {
    let mut graph = DagGraph::new();
    graph.add_node(DagNode::new(NodeId::new("task1"), "Task 1", NodeKind::Task));
    graph.add_node(DagNode::new(NodeId::new("task2"), "Task 2", NodeKind::Task));

    graph
        .add_edge(DagEdge::new(
            NodeId::new("task1"),
            NodeId::new("task2"),
            EdgeKind::Sequential,
        ))
        .unwrap();
    let result = graph.add_edge(DagEdge::new(
        NodeId::new("task1"),
        NodeId::new("task2"),
        EdgeKind::Conditional,
    ));
    assert!(result.is_ok()); // Multiple edges between same nodes allowed
    assert_eq!(graph.edge_count(), 2);
}

#[test]
fn test_dag_graph_complex_dag() {
    let mut graph = DagGraph::new();

    // Build a complex DAG
    let nodes = ["a", "b", "c", "d", "e", "f"];
    for n in &nodes {
        graph.add_node(DagNode::new(
            NodeId::new(*n),
            (*n).to_uppercase(),
            NodeKind::Task,
        ));
    }

    // a -> b, a -> c, b -> d, c -> d, d -> e, d -> f
    graph
        .add_edge(DagEdge::new(
            NodeId::new("a"),
            NodeId::new("b"),
            EdgeKind::Sequential,
        ))
        .unwrap();
    graph
        .add_edge(DagEdge::new(
            NodeId::new("a"),
            NodeId::new("c"),
            EdgeKind::Sequential,
        ))
        .unwrap();
    graph
        .add_edge(DagEdge::new(
            NodeId::new("b"),
            NodeId::new("d"),
            EdgeKind::Sequential,
        ))
        .unwrap();
    graph
        .add_edge(DagEdge::new(
            NodeId::new("c"),
            NodeId::new("d"),
            EdgeKind::Sequential,
        ))
        .unwrap();
    graph
        .add_edge(DagEdge::new(
            NodeId::new("d"),
            NodeId::new("e"),
            EdgeKind::Sequential,
        ))
        .unwrap();
    graph
        .add_edge(DagEdge::new(
            NodeId::new("d"),
            NodeId::new("f"),
            EdgeKind::Sequential,
        ))
        .unwrap();

    let order = graph.topological_order().unwrap();
    assert_eq!(order.len(), 6);
    assert_eq!(order[0].0, "a");
    assert!(
        order.iter().position(|n| n.0 == "d").unwrap()
            > order.iter().position(|n| n.0 == "b").unwrap()
    );
    assert!(
        order.iter().position(|n| n.0 == "d").unwrap()
            > order.iter().position(|n| n.0 == "c").unwrap()
    );
    assert!(order[5].0 == "e" || order[5].0 == "f"); // e or f last (parallel)
}
