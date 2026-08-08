use crate::xiaoyi::workflow::dag::Graph;

/// Executor capability description.
///
/// @brief Executor capabilities
/// @since 0.1.0
/// @author Miruamel
#[derive(Debug, Clone)]
pub struct ExecutorCapability {
    pub max_concurrency: usize,
    pub supports_streaming: bool,
}
