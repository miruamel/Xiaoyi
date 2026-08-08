use std::sync::Arc;
use tokio::sync::Mutex;

/// Isolates failures to a bounded pool.
///
/// @brief Failure isolation boundary
/// @since 0.1.0
/// @author Miruamel
/// @see crate::xiaoyi::resilience
#[derive(Debug, Clone)]
pub struct IsolationBoundary {
    failures: Arc<Mutex<u64>>,
}

impl IsolationBoundary {
    /// Create a new isolation boundary.
    ///
    /// @return IsolationBoundary instance
    /// @since 0.1.0
    pub fn new() -> Self {
        Self {
            failures: Arc::new(Mutex::new(0)),
        }
    }

    /// Record a failure.
    ///
    /// @since 0.1.0
    pub async fn record_failure(&self) {
        let mut count = self.failures.lock().await;
        *count += 1;
    }

    /// Return current failure count.
    ///
    /// @since 0.1.0
    pub async fn failure_count(&self) -> u64 {
        *self.failures.lock().await
    }
}
