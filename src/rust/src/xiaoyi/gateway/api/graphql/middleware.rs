use crate::xiaoyi::core::config::Config;

/// GraphQL middleware stack.
///
/// @brief GraphQL middleware configuration
/// @since 0.1.0
/// @author Miruamel
#[derive(Debug, Clone, Default)]
pub struct GraphQLMiddleware;

impl GraphQLMiddleware {
    /// Create middleware stack.
    ///
    /// @param config Gateway configuration
    /// @return GraphQLMiddleware instance
    /// @since 0.1.0
    pub fn new(_config: &Config) -> Self {
        Self
    }
}
