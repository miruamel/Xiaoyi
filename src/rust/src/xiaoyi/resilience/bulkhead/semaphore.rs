use crate::xiaoyi::core::error::{ErrorKind, XiaoyiError};

/// Concurrency semaphore for bulkhead isolation.
///
/// @brief Limit concurrent access
/// @since 0.1.0
/// @author Miruamel
#[derive(Debug, Clone)]
pub struct BulkheadSemaphore {
    pub max_concurrency: usize,
}

impl BulkheadSemaphore {
    /// Create a new bulkhead semaphore.
    ///
    /// @param max_concurrency Maximum concurrent permits
    /// @return BulkheadSemaphore instance
    /// @since 0.1.0
    pub fn new(max_concurrency: usize) -> Self {
        Self { max_concurrency }
    }

    /// Attempt to acquire a permit.
    ///
    /// @since 0.1.0
    pub fn acquire(&self) -> Result<(), XiaoyiError> {
        if self.max_concurrency == 0 {
            Err(XiaoyiError::new(
                ErrorKind::Policy,
                "bulkhead concurrency is zero",
            ))
        } else {
            Ok(())
        }
    }
}
