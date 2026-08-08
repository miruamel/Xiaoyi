use crate::xiaoyi::workflow::dag::Dag;

/// DAG scheduler.
///
/// @brief Schedule execution over a DAG
/// @since 0.1.0
/// @author Miruamel
/// @see Dag
pub struct DagScheduler;

impl DagScheduler {
    /// Schedule a DAG for execution.
    ///
    /// @param dag Workflow DAG
    /// @since 0.1.0
    pub fn schedule(&self, _dag: &Dag) {
        // scheduler placeholder
    }
}
