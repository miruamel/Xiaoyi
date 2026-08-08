//! # Counter
//!
//! @module monitoring::metrics::counter
//! @brief Monotonic counter metric
//! @group Monitoring
//! @since 0.1.0
//! @author Miruamel
//! @see crate::monitoring::metrics

use parking_lot::RwLock;
use std::sync::Arc;

use crate::xiaoyi::core::error::{ErrorKind, Result, XiaoyiError};

/// A monotonically increasing counter metric.
///
/// Counters only ever increase and are used to measure cumulative counts
/// such as total requests served or tokens consumed.
///
/// @brief Monotonic counter metric
/// @group Monitoring
/// @since 0.1.0
#[derive(Debug, Clone, Default)]
pub struct Counter {
    value: Arc<RwLock<u64>>,
}

impl Counter {
    /// Create a new counter initialized to zero.
    ///
    /// @return A new [`Counter`] with value `0`.
    /// @since 0.1.0
    pub fn new() -> Self {
        Self::default()
    }

    /// Increment the counter by `by`.
    ///
    /// @param by Amount to add (should be positive for a meaningful change).
    /// @throw XiaoyiError if the addition overflows the `u64` range.
    /// @since 0.1.0
    pub fn inc(&self, by: u64) -> Result<()> {
        let mut v = self.value.write();
        *v = v
            .checked_add(by)
            .ok_or_else(|| XiaoyiError::new(ErrorKind::Runtime, "counter overflow"))?;
        Ok(())
    }

    /// Current cumulative value.
    ///
    /// @return The counter's value.
    /// @since 0.1.0
    pub fn value(&self) -> u64 {
        *self.value.read()
    }
}
