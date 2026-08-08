//! # Gauge
//!
//! @module monitoring::metrics::gauge
//! @brief Up/down gauge metric
//! @group Monitoring
//! @since 0.1.0
//! @author Miruamel
//! @see crate::monitoring::metrics

use parking_lot::RwLock;
use std::sync::Arc;

use crate::xiaoyi::core::error::Result;

/// A gauge metric that may move up or down over time.
///
/// @brief Up/down gauge metric
/// @group Monitoring
/// @since 0.1.0
#[derive(Debug, Clone, Default)]
pub struct Gauge {
    value: Arc<RwLock<f64>>,
}

impl Gauge {
    /// Create a new gauge initialized to `0.0`.
    ///
    /// @return A new [`Gauge`] with value `0.0`.
    /// @since 0.1.0
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the gauge to `v`.
    ///
    /// @param v New gauge value.
    /// @since 0.1.0
    pub fn set(&self, v: f64) -> Result<()> {
        *self.value.write() = v;
        Ok(())
    }

    /// Increase the gauge by `delta`.
    ///
    /// @param delta Amount to add.
    /// @since 0.1.0
    pub fn inc(&self, delta: f64) -> Result<()> {
        *self.value.write() += delta;
        Ok(())
    }

    /// Decrease the gauge by `delta`.
    ///
    /// @param delta Amount to subtract.
    /// @since 0.1.0
    pub fn dec(&self, delta: f64) -> Result<()> {
        *self.value.write() -= delta;
        Ok(())
    }

    /// Current gauge value.
    ///
    /// @return The gauge's value.
    /// @since 0.1.0
    pub fn value(&self) -> f64 {
        *self.value.read()
    }
}
