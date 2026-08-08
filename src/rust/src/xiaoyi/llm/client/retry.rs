use crate::xiaoyi::utils::retry::RetryConfig;

/// LLM client retry policy.
///
/// @brief Retry policy for LLM requests
/// @since 0.1.0
/// @author Miruamel
/// @see crate::xiaoyi::llm::client
#[derive(Debug, Clone)]
pub struct ClientRetryPolicy {
    pub config: RetryConfig,
}

impl Default for ClientRetryPolicy {
    fn default() -> Self {
        Self {
            config: RetryConfig::default(),
        }
    }
}
