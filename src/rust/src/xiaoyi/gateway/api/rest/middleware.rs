use crate::xiaoyi::core::config::Config;

/// REST middleware stack.
///
/// @brief REST middleware configuration
/// @since 0.1.0
/// @author Miruamel
#[derive(Debug, Clone, Default)]
pub struct RestMiddleware;

impl RestMiddleware {
    /// Create middleware stack.
    ///
    /// @param config Gateway configuration
    /// @return RestMiddleware instance
    /// @since 0.1.0
    pub fn new(_config: &Config) -> Self {
        Self
    }
}
