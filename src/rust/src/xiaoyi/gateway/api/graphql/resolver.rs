use crate::xiaoyi::core::config::Config;

/// GraphQL resolver registry.
///
/// @brief Register GraphQL resolvers
/// @since 0.1.0
/// @author Miruamel
#[derive(Debug, Clone, Default)]
pub struct GraphQLResolverRegistry;

impl GraphQLResolverRegistry {
    /// Create resolver registry.
    ///
    /// @param config Gateway configuration
    /// @return GraphQLResolverRegistry instance
    /// @since 0.1.0
    pub fn new(_config: &Config) -> Self {
        Self
    }
}
