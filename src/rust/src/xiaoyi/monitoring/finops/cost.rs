use crate::xiaoyi::core::error::Result;
use crate::xiaoyi::core::error::{ErrorKind, XiaoyiError};
use serde::{Deserialize, Serialize};

/// Token and cost estimation for LLM usage.
///
/// @brief Cost estimation data structure for tracking token consumption and calculated costs
/// @group Monitoring
/// @since 0.1.0
/// @author Miruamel
/// @see CostTracker::record
/// @see crate::xiaoyi::core::error
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CostEstimate {
    /// Number of prompt tokens consumed.
    pub prompt_tokens: u64,
    /// Number of completion tokens generated.
    pub completion_tokens: u64,
    /// LLM model identifier (e.g., "gpt-4o", "claude-3-opus").
    pub model: String,
    /// Calculated cost in USD for this estimate.
    pub cost_usd: f64,
}

impl CostEstimate {
    /// Creates a new cost estimate from model, prompt tokens, and completion tokens.
    ///
    /// @brief Construct a cost estimate with calculated USD value based on model pricing
    /// @param model LLM model name used for the request
    /// @param prompt_tokens Number of input tokens
    /// @param completion_tokens Number of output tokens
    /// @return `Result<CostEstimate>` New cost estimate or error if pricing not found
    /// @throw ErrorKind::Llm "Unknown model pricing"
    /// @group Monitoring
    /// @since 0.1.0
    /// @author Miruamel
    pub fn estimate(
        model: &str,
        prompt_tokens: u64,
        completion_tokens: u64,
    ) -> Result<CostEstimate> {
        // Hardcoded per-1k-token pricing table (USD)
        let price_per_1k_tokens = match model {
            "gpt-4o" => 0.005,
            "gpt-4-turbo" => 0.01,
            "claude-3-opus" => 0.015,
            "claude-3-sonnet" => 0.008,
            "claude-3-haiku" => 0.0025,
            "gpt-3.5-turbo" => 0.002,
            "llama2-70b" => 0.0015,
            "mistral-7b" => 0.001,
            _ => {
                return Err(XiaoyiError::new(
                    ErrorKind::Llm,
                    format!("Unknown model pricing: {}", model),
                ));
            }
        };

        let total_tokens = prompt_tokens + completion_tokens;
        let cost_usd = (total_tokens as f64 / 1000.0) * price_per_1k_tokens;

        Ok(Self {
            prompt_tokens,
            completion_tokens,
            model: model.to_string(),
            cost_usd,
        })
    }
}
