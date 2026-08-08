//! # Layer 9 — Monitoring & Observability
//!
//! `monitoring` provides runtime metrics and observability for the Xiaoyi autonomous agent.
//!
//! Path: `xiaoyi::monitoring`
//!
//! - Layer 9: `monitoring` — Metrics and observability.
//!
//! @module monitoring
//! @brief Metrics and observability substrate: counters, gauges, histograms, and registry
//! @group Monitoring
//! @since 0.1.0
//! @author Miruamel
//! @see crate::xiaoyi::core::error
//! @see crate::monitoring::metrics

pub mod counter;
pub mod gauge;
pub mod histogram;
pub mod registry;

pub use counter::Counter;
pub use gauge::Gauge;
pub use histogram::Histogram;
pub use registry::MetricRegistry;

pub use histogram::{HistogramSnapshot, HistogramBucket};