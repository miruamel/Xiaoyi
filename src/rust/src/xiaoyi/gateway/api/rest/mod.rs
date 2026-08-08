pub mod controller;
pub mod middleware;

use crate::xiaoyi::core::config::Config;

/// REST API controller.
///
/// @brief REST API controller
/// @since 0.1.0
/// @author Miruamel
#[derive(Debug, Clone, Default)]
pub struct RestController;

impl RestController {
    /// Create controller from config.
    ///
    /// @param config Gateway configuration
    /// @return RestController instance
    /// @since 0.1.0
    pub fn new(_config: &Config) -> Self {
        Self
    }
}
