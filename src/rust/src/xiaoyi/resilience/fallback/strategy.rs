use crate::xiaoyi::resilience::fallback::FallbackHandler;

/// Fallback strategy registry.
///
/// @brief Manage named fallback strategies
/// @since 0.1.0
/// @author Miruamel
/// @see FallbackHandler
#[derive(Debug, Clone, Default)]
pub struct FallbackStrategyRegistry;

impl FallbackStrategyRegistry {
    /// Register a fallback handler by name.
    ///
    /// @param name Strategy name
    /// @param handler Fallback handler
    /// @since 0.1.0
    pub fn register(&mut self, _name: impl Into<String>, _handler: FallbackHandler) {
        // registry placeholder
    }

    /// Get fallback handler by name.
    ///
    /// @param name Strategy name
    /// @return Fallback handler if registered
    /// @since 0.1.0
    pub fn get(&self, _name: &str) -> Option<FallbackHandler> {
        None
    }
}
