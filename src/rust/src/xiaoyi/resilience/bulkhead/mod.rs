pub mod semaphore;

use crate::xiaoyi::core::error::XiaoyiError;
use crate::xiaoyi::resilience::bulkhead::semaphore::BulkheadSemaphore;

/// Bulkhead isolation for resource protection.
///
/// @brief Resource isolation via bulkhead pattern
/// @group AI Resilience
/// @since 0.1.0
/// @author Miruamel
#[derive(Debug, Clone)]
pub struct Bulkhead {
    pub name: String,
    pub semaphore: BulkheadSemaphore,
}

impl Bulkhead {
    /// Create a new bulkhead.
    ///
    /// @param name Bulkhead name
    /// @param max_concurrency Maximum concurrent operations
    /// @return Bulkhead instance
    /// @since 0.1.0
    pub fn new(name: impl Into<String>, max_concurrency: usize) -> Self {
        Self {
            name: name.into(),
            semaphore: BulkheadSemaphore::new(max_concurrency),
        }
    }

    /// Try to acquire a permit.
    ///
    /// @return Success or error
    /// @since 0.1.0
    pub fn acquire(&self) -> Result<(), XiaoyiError> {
        self.semaphore.acquire()
    }
}
