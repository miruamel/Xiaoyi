use crate::xiaoyi::core::config::Config;

/// REST controller registration.
///
/// @brief Register REST controller handlers
/// @since 0.1.0
/// @author Miruamel
#[derive(Debug, Clone, Default)]
pub struct RestControllerRegistry;

impl RestControllerRegistry {
    /// Create a registry.
    ///
    /// @param config Gateway configuration
    /// @return RestControllerRegistry instance
    /// @since 0.1.0
    pub fn new(_config: &Config) -> Self {
        Self
    }
}
