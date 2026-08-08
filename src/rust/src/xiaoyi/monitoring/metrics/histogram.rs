//! # Histogram
//!
//! @module monitoring::metrics::histogram
//! @brief Distribution histogram metric
//! @group Monitoring
//! @since 0.1.0
//! @author Miruamel
//! @see crate::monitoring::metrics

use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::xiaoyi::core::error::Result;

/// A single histogram bucket containing all samples `<= le`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistogramBucket {
    /// Upper bound (inclusive) of this bucket.
    pub le: f64,
    /// Cumulative sample count at or below `le`.
    pub count: u64,
}

/// Point-in-time snapshot of a [`Histogram`].
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HistogramSnapshot {
    /// Total number of recorded samples.
    pub count: u64,
    /// Sum of all recorded samples.
    pub sum: f64,
    /// 50th percentile estimate.
    pub p50: f64,
    /// 95th percentile estimate.
    pub p95: f64,
    /// 99th percentile estimate.
    pub p99: f64,
    /// Bucketized cumulative counts.
    pub buckets: Vec<HistogramBucket>,
}

/// A histogram recording `f64` samples into configurable buckets.
#[derive(Debug, Clone)]
pub struct Histogram {
    samples: Arc<RwLock<Vec<f64>>>,
    bounds: Vec<f64>,
}

impl Default for Histogram {
    fn default() -> Self {
        Self {
            samples: Arc::new(RwLock::new(Vec::new())),
            bounds: vec![0.1, 1.0, 10.0, 100.0, 1000.0],
        }
    }
}

impl Histogram {
    /// Create a histogram with the default bucket boundaries.
    ///
    /// @return A new [`Histogram`] with default bounds.
    /// @since 0.1.0
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a histogram with explicit ascending upper bounds.
    ///
    /// @param bounds Ascending upper-bound values for each bucket.
    /// @return A new [`Histogram`] using `bounds`.
    /// @since 0.1.0
    pub fn with_buckets(bounds: Vec<f64>) -> Self {
        Self {
            samples: Arc::new(RwLock::new(Vec::new())),
            bounds,
        }
    }

    /// Record a sample.
    ///
    /// @param v Observed value.
    /// @since 0.1.0
    pub fn record(&self, v: f64) -> Result<()> {
        self.samples.write().push(v);
        Ok(())
    }

    /// Produce a snapshot with percentile estimates.
    ///
    /// @return A [`HistogramSnapshot`] of the recorded samples.
    /// @since 0.1.0
    pub fn snapshot(&self) -> HistogramSnapshot {
        let mut samples = self.samples.read().clone();
        samples.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        let count = samples.len() as u64;
        let sum: f64 = samples.iter().sum();
        let pct = |p: f64| -> f64 {
            if samples.is_empty() {
                return 0.0;
            }
            let idx = ((p * count as f64) - 1.0).max(0.0).round() as usize;
            samples[idx.min(samples.len() - 1)]
        };
        let mut buckets = Vec::with_capacity(self.bounds.len());
        for b in &self.bounds {
            let c = samples.iter().filter(|&&s| s <= *b).count() as u64;
            buckets.push(HistogramBucket { le: *b, count: c });
        }
        HistogramSnapshot {
            count,
            sum,
            p50: pct(0.50),
            p95: pct(0.95),
            p99: pct(0.99),
            buckets,
        }
    }
}
