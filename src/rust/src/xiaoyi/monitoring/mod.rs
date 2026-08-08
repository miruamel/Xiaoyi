//! # Monitoring Module
//!
//! `monitoring` is the real-time observation and response layer (Layer 9) of Xiaoyi.
//!
//! Path: `xiaoyi::monitoring`
//!
//! - `metrics` — counters, gauges, histograms, and a registry.
//! - `tracing` — span lifecycle and exporters.
//! - `finops` — cost and budget tracking.
//! - `alerts` — threshold-based alerting.
//!
//! @module monitoring
//! @brief Real-time observation and response substrate: metrics, tracing, finops, and alerts
//! @group Monitoring
//! @since 0.1.0
//! @author Miruamel
//! @see crate::xiaoyi

pub mod alerts;
pub mod finops;
pub mod metrics;
pub mod tracing;

pub use alerts::{Alert, AlertManager, AlertRule, AlertSeverity, Notifier};
pub use finops::{Budget, CostEstimate, CostTracker};
pub use metrics::{Counter, Gauge, Histogram, HistogramSnapshot, MetricRegistry};
pub use tracing::{Span, SpanContext, SpanKind, TraceExporter, Tracer};
