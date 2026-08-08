//! # Metric Registry
//!
//! @module monitoring::metrics::registry
//! @brief Central registry of counters, gauges, and histograms
//! @group Monitoring
//! @since 0.1.0
//! @author Miruamel
//! @see crate::monitoring::metrics

use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::xiaoyi::core::error::{ErrorKind, Result, XiaoyiError};

use super::{Counter, Gauge, Histogram, HistogramSnapshot};

/// Serializable point-in-time snapshot of all registered metrics.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MetricSnapshot {
    /// Counter values keyed by name.
    pub counters: HashMap<String, u64>,
    /// Gauge values keyed by name.
    pub gauges: HashMap<String, f64>,
    /// Histogram snapshots keyed by name.
    pub histograms: HashMap<String, HistogramSnapshot>,
}

/// Central registry aggregating the three metric kinds.
#[derive(Debug, Default)]
pub struct MetricRegistry {
    counters: RwLock<HashMap<String, Counter>>,
    gauges: RwLock<HashMap<String, Gauge>>,
    histograms: RwLock<HashMap<String, Histogram>>,
}

impl MetricRegistry {
    /// Create an empty registry.
    ///
    /// @return A new, empty [`MetricRegistry`].
    /// @since 0.1.0
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a new counter, failing if the name already exists.
    ///
    /// @param name Unique metric name.
    /// @throw XiaoyiError with [`ErrorKind::State`] if `name` is taken.
    /// @since 0.1.0
    pub fn register_counter(&self, name: &str) -> Result<()> {
        let mut m = self.counters.write();
        if m.contains_key(name) {
            return Err(XiaoyiError::new(
                ErrorKind::State,
                format!("counter '{}' already registered", name),
            ));
        }
        m.insert(name.to_string(), Counter::new());
        Ok(())
    }

    /// Register a new gauge, failing if the name already exists.
    ///
    /// @param name Unique metric name.
    /// @throw XiaoyiError with [`ErrorKind::State`] if `name` is taken.
    /// @since 0.1.0
    pub fn register_gauge(&self, name: &str) -> Result<()> {
        let mut m = self.gauges.write();
        if m.contains_key(name) {
            return Err(XiaoyiError::new(
                ErrorKind::State,
                format!("gauge '{}' already registered", name),
            ));
        }
        m.insert(name.to_string(), Gauge::new());
        Ok(())
    }

    /// Register a new histogram, failing if the name already exists.
    ///
    /// @param name Unique metric name.
    /// @throw XiaoyiError with [`ErrorKind::State`] if `name` is taken.
    /// @since 0.1.0
    pub fn register_histogram(&self, name: &str) -> Result<()> {
        let mut m = self.histograms.write();
        if m.contains_key(name) {
            return Err(XiaoyiError::new(
                ErrorKind::State,
                format!("histogram '{}' already registered", name),
            ));
        }
        m.insert(name.to_string(), Histogram::new());
        Ok(())
    }

    /// Borrow a registered counter by name.
    ///
    /// @param name Metric name previously registered.
    /// @return A clone of the [`Counter`], or `None` if absent.
    /// @since 0.1.0
    pub fn counter(&self, name: &str) -> Option<Counter> {
        self.counters.read().get(name).cloned()
    }

    /// Borrow a registered gauge by name.
    ///
    /// @param name Metric name previously registered.
    /// @return A clone of the [`Gauge`], or `None` if absent.
    /// @since 0.1.0
    pub fn gauge(&self, name: &str) -> Option<Gauge> {
        self.gauges.read().get(name).cloned()
    }

    /// Borrow a registered histogram by name.
    ///
    /// @param name Metric name previously registered.
    /// @return A clone of the [`Histogram`], or `None` if absent.
    /// @since 0.1.0
    pub fn histogram(&self, name: &str) -> Option<Histogram> {
        self.histograms.read().get(name).cloned()
    }

    /// Capture a full snapshot of every registered metric.
    ///
    /// @return A [`MetricSnapshot`] of all current values.
    /// @since 0.1.0
    pub fn snapshot(&self) -> MetricSnapshot {
        let mut snap = MetricSnapshot::default();
        for (k, c) in self.counters.read().iter() {
            snap.counters.insert(k.clone(), c.value());
        }
        for (k, g) in self.gauges.read().iter() {
            snap.gauges.insert(k.clone(), g.value());
        }
        for (k, h) in self.histograms.read().iter() {
            snap.histograms.insert(k.clone(), h.snapshot());
        }
        snap
    }
}
