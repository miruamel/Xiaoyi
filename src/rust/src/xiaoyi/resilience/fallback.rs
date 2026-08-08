//! # Fallback Module
//!
//! `fallback` provides fallback execution paths.
//!
//! Path: `xiaoyi::resilience::fallback`
//!
//! @module resilience::fallback
//! @brief Fallback execution paths
//! @group AI Resilience
//! @since 0.1.0
//! @author Miruamel
//! @see crate::resilience
//! @see crate::resilience::circuit_breaker


/// Fallback handler for degraded operation.
///
/// @brief Provides fallback when primary fails
/// @group AI Resilience
/// @since 0.1.0
#[derive(Debug, Clone)]
pub struct FallbackHandler {
    fallbacks: Vec<String>,
}

impl FallbackHandler {
    /// Create new fallback handler.
    ///
    /// @param fallbacks List of fallback handlers
    /// @return FallbackHandler instance
    /// @since 0.1.0
    pub fn new(fallbacks: Vec<String>) -> Self {
        Self { fallbacks }
    }

    /// Get fallback options.
    ///
    /// @return Vector of fallback options
    /// @since 0.1.0
    pub fn get_fallbacks(&self) -> &[String] {
        &self.fallbacks
    }

    /// Add fallback.
    ///
    /// @param fallback Fallback description
    /// @since 0.1.0
    pub fn add_fallback(&mut self, fallback: impl Into<String>) {
        self.fallbacks.push(fallback.into());
    }
}

impl Default for FallbackHandler {
    fn default() -> Self {
        Self {
            fallbacks: vec!["cached_result".to_string()],
        }
    }
}