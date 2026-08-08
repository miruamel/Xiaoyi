use crate::xiaoyi::workflow::dag::graph::DagGraph;

/// Splits a goal into high-level execution steps.
///
/// @brief Decompose goal into planner steps
/// @param goal Goal description
/// @return DAG representing planned steps
/// @since 0.1.0
/// @author Miruamel
/// @see crate::xiaoyi::orchestrator
pub fn plan(goal: &str) -> DagGraph {
    use crate::xiaoyi::workflow::dag::graph::{DagNode, NodeId, NodeKind};
    let mut graph = DagGraph::new();
    let root = graph.add_node(DagNode::new(
        NodeId("plan_root".into()),
        goal,
        NodeKind::Task,
    ));
    let _ = root;
    graph
}
