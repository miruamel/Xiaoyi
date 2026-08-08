//! # Layer 9 — Monitoring / Alerts
//!
//! `monitoring` is the real-time observation and response layer of Xiaoyi.
//!
//! It unifies four deep-vertical concerns:
//!
//! - **Alerts** ([`crate::xiaoyi::monitoring::alerts`]) — conditional notifications based on metric thresholds and rules.
//! - **Tracing** ([`crate::xiaoyi::monitoring::tracing`]) — distributed span and event recording for debugging.
//! - **Metrics** ([`crate::xiaoyi::monitoring::metrics`]) — time-series metrics aggregation and queries.
//! - **FinOps** ([`crate::xiaoyi::monitoring::finops`]) — cost and resource usage analysis.
//!
//! The [`AlertManager`] facade wires every slice into a single addressable service so the
//! orchestrator (Layer 2) can emit events, trace spans, and profile workloads.
//!
//! Path: `xiaoyi::monitoring`
//!
//! - Layer 8: `monitoring` — real-time observation and response.
//! - Layer 9: `alerts` — conditional notifications based on metric thresholds and rules.
//!
//! @module monitoring::alerts
//! @brief Conditional notifications based on metric thresholds and rules
//! @group Monitoring
//! @since 0.1.0
//! @author Miruamel
//! @see crate::monitoring
//! @see crate::orchestrator
//!

pub mod alert;
pub mod notifier;
pub mod rule;

pub use alert::{Alert, AlertSeverity};
pub use rule::AlertRule;
pub use notifier::{Notifier, AlertManager};