use std::time::Duration;

/// Retry configuration.
///
/// @brief Configuration for retry behavior
/// @group Core Runtime
/// @since 0.1.0
/// @author Miruamel
#[derive(Debug, Clone)]
pub struct RetryConfig {
    /// Maximum number of retry attempts.
    pub max_attempts: u32,
    /// Base delay between retries.
    pub base_delay: Duration,
    /// Maximum delay between retries.
    pub max_delay: Duration,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_attempts: 3,
            base_delay: Duration::from_millis(100),
            max_delay: Duration::from_secs(2),
        }
    }
}

impl RetryConfig {
    /// Create a new retry configuration.
    ///
    /// @brief Initialize retry config with custom values
    /// @param max_attempts Maximum retry attempts
    /// @param base_delay Minimum delay between retries
    /// @param max_delay Maximum delay between retries
    /// @return RetryConfig instance
    /// @since 0.1.0
    pub fn new(max_attempts: u32, base_delay: Duration, max_delay: Duration) -> Self {
        Self {
            max_attempts,
            base_delay,
            max_delay,
        }
    }

    /// Calculate delay for a given attempt number using exponential backoff.
    ///
    /// @brief Calculate backoff delay for attempt
    /// @param attempt Current attempt number (0-indexed)
    /// @return Delay duration
    /// @since 0.1.0
    pub fn delay_for_attempt(&self, attempt: u32) -> Duration {
        let exp = attempt.min(10) as u32;
        let delay = self.base_delay * 2u32.pow(exp);
        std::cmp::min(delay, self.max_delay)
    }
}
