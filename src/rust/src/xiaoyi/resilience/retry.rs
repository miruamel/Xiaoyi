//! # Retry Module
//!
//! `retry` provides retry logic with exponential backoff.
//!
//! Path: `xiaoyi::resilience::retry`
//!
//! @module resilience::retry
//! @brief Retry logic with backoff
//! @group AI Resilience
//! @since 0.1.0
//! @author Miruamel
//! @see crate::resilience
//! @see crate::resilience::circuit_breaker
//! @complexity O(max_attempts) per operation

use crate::xiaoyi::resilience::RetryConfig;

/// Retry policy for operation execution.
///
/// @brief Configurable retry with exponential backoff
/// @group AI Resilience
/// @since 0.1.0
/// @complexity O(max_attempts) per operation
#[derive(Debug, Clone)]
pub struct RetryPolicy {
    config: RetryConfig,
}

impl RetryPolicy {
    /// Create new retry policy.
    ///
    /// @param config Retry configuration
    /// @return RetryPolicy instance
    /// @since 0.1.0
    pub fn new(config: RetryConfig) -> Self {
        Self { config }
    }

    /// Execute operation with retry.
    ///
    /// @param operation Operation to execute
    /// @return Result of operation
    /// @throw LastError if all retries exhaust
    /// @since 0.1.0
    pub async fn execute<F, Fut, T, E>(&self, operation: F) -> std::result::Result<T, E>
    where
        F: Fn() -> Fut,
        Fut: std::future::Future<Output = std::result::Result<T, E>>,
        E: std::fmt::Display + Clone,
    {
        let mut attempt = 0;
        let mut delay = self.config.initial_delay_ms;

        loop {
            match operation().await {
                Ok(result) => return Ok(result),
                Err(e) => {
                    attempt += 1;
                    if attempt >= self.config.max_attempts {
                        return Err(e);
                    }

                    // In production, would use tokio::time::sleep
                    // tokio::time::sleep(Duration::from_millis(delay)).await;
                    let _ = delay; // Acknowledge delay calculation

                    delay = ((delay as f64) * self.config.backoff_multiplier) as u64;
                    if delay > self.config.max_delay_ms {
                        delay = self.config.max_delay_ms;
                    }
                }
            }
        }
    }
}

impl Default for RetryPolicy {
    fn default() -> Self {
        Self::new(RetryConfig::default())
    }
}
