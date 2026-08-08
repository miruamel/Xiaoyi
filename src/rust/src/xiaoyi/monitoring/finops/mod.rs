//! # FinOps Module
//!
//! `finops` is the cost and budget management layer of Xiaoyi's monitoring stack.
//!
//! Path: `xiaoyi::monitoring::finops`
//!
//! - Layer 9: `finops` — cost and resource usage analysis.
//!
//! @module monitoring::finops
//! @brief Cost and budget management for resource usage analysis
//! @group Monitoring
//! @since 0.1.0
//! @author Miruamel
//! @see crate::monitoring
//! @see crate::xiaoyi::core::error

pub mod budget;
pub mod cost;
pub mod pricing;
pub mod tracker;

pub use budget::Budget;
pub use cost::CostEstimate;
pub use tracker::CostTracker;
