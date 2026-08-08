//! # Model Router
//!
//! `model_router` routes review requests to appropriate LLM models
//! based on complexity and cost (Heavy/Light/Micro).
//!
//! Path: `xiaoyi::critic::model_router`
//!
//! @module critic::model_router
//! @brief Model router for critic pipeline
//! @group AI Review
//! @since 0.1.0
//! @author Miruamel
//! @see crate::critic
//! @see crate::llm::client

use crate::xiaoyi::llm::client::LlmClient;

/// Model tier for routing.
///
/// @brief LLM model tier classification
/// @group AI Review
/// @since 0.1.0
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ModelTier {
    /// Heavy models (GPT-4, Claude Opus) - for complex analysis
    Heavy,
    /// Light models (Llama 3 70B, GPT-3.5) - for standard analysis
    Light,
    /// Micro models (Phi-3, Llama 3 8B) - for fast/cheap analysis
    Micro,
}

/// Model router configuration.
///
/// @brief Router configuration
/// @group AI Review
/// @since 0.1.0
// #[derive(Debug, Clone)]
pub struct RouterConfig {
    /// Heavy model client
    pub heavy: Option<Box<dyn LlmClient>>,
    /// Light model client
    pub light: Option<Box<dyn LlmClient>>,
    /// Micro model client
    pub micro: Option<Box<dyn LlmClient>>,
}

impl Default for RouterConfig {
    fn default() -> Self {
        Self {
            heavy: None,
            light: None,
            micro: None,
        }
    }
}

/// Model router for selecting appropriate LLM.
///
/// @brief Routes review tasks to optimal model tier
/// @group AI Review
/// @since 0.1.0
// #[derive(Debug, Clone)]
pub struct ModelRouter {
    config: RouterConfig,
}

impl ModelRouter {
    /// Create new model router.
    ///
    /// @param config Router configuration
    /// @return ModelRouter instance
    /// @since 0.1.0
    pub fn new(config: RouterConfig) -> Self {
        Self { config }
    }

    /// Get client for tier.
    ///
    /// @param tier Model tier
    /// @return LLM client or None if not configured
    /// @since 0.1.0
    pub fn get_client(&self, tier: ModelTier) -> Option<&dyn LlmClient> {
        match tier {
            ModelTier::Heavy => self.config.heavy.as_deref(),
            ModelTier::Light => self.config.light.as_deref(),
            ModelTier::Micro => self.config.micro.as_deref(),
        }
    }

    /// Determine tier based on task complexity.
    ///
    /// @param complexity Task complexity (0.0 - 1.0)
    /// @return Recommended model tier
    /// @since 0.1.0
    pub fn select_tier(&self, complexity: f32) -> ModelTier {
        if complexity > 0.7 {
            ModelTier::Heavy
        } else if complexity > 0.3 {
            ModelTier::Light
        } else {
            ModelTier::Micro
        }
    }
}