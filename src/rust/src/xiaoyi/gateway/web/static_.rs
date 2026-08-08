use crate::xiaoyi::core::config::Config;

/// Static asset serving for Web UI.
///
/// @brief Static asset serving for Web UI
/// @since 0.1.0
/// @author Miruamel
#[derive(Debug, Clone, Default)]
pub struct StaticAssetServer;

impl StaticAssetServer {
    /// Create asset server.
    ///
    /// @param config Gateway configuration
    /// @return StaticAssetServer instance
    /// @since 0.1.0
    pub fn new(_config: &Config) -> Self {
        Self
    }
}
