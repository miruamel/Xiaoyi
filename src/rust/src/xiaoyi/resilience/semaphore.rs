use std::sync::Arc;
use tokio::sync::Semaphore;

/// Limits concurrency for protected operations.
///
/// @brief Concurrency limiter
/// @since 0.1.0
/// @author Miruamel
/// @see crate::xiaoyi::resilience
#[derive(Debug, Clone)]
pub struct ConcurrencyLimiter {
    permit: Arc<Semaphore>,
}

impl ConcurrencyLimiter {
    /// Create a new limiter.
    ///
    /// @param max Maximum concurrent operations
    /// @return ConcurrencyLimiter instance
    /// @since 0.1.0
    pub fn new(max: usize) -> Self {
        Self {
            permit: Arc::new(Semaphore::new(max)),
        }
    }

    /// Acquire a permit.
    ///
    /// @return Permit guard
    /// @since 0.1.0
    pub async fn acquire(&self) -> tokio::sync::OwnedSemaphorePermit {
        self.permit.clone().acquire_owned().await.unwrap()
    }
}
