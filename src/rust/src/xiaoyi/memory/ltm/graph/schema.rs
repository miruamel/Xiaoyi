use crate::xiaoyi::memory::ltm::graph::Graph;

/// Graph schema rules.
///
/// @brief Schema constraints for knowledge graph
/// @since 0.1.0
/// @author Miruamel
/// @see Graph
pub struct GraphSchema;

impl GraphSchema {
    /// Validate graph structure.
    ///
    /// @param graph Knowledge graph
    /// @return Validation result
    /// @since 0.1.0
    pub fn validate(&self, graph: &Graph) -> Result<(), crate::xiaoyi::core::error::XiaoyiError> {
        if graph.nodes.is_empty() {
            return Err(crate::xiaoyi::core::error::XiaoyiError::new(
                crate::xiaoyi::core::error::ErrorKind::Config,
                "graph must contain nodes",
            ));
        }
        Ok(())
    }
}
