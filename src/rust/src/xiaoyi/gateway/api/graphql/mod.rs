pub mod middleware;
pub mod resolver;

use crate::xiaoyi::core::config::Config;

/// GraphQL API controller.
///
/// @brief GraphQL API controller
/// @since 0.1.0
/// @author Miruamel
#[derive(Debug, Clone, Default)]
pub struct GraphQLController;

impl GraphQLController {
    /// Create controller from config.
    ///
    /// @param config Gateway configuration
    /// @return GraphQLController instance
    /// @since 0.1.0
    pub fn new(_config: &Config) -> Self {
        Self
    }
}
