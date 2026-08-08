//! # Orchestrator Monitor Module
//!
//! `monitor` provides execution monitoring and metrics.
//!
//! Path: `xiaoyi::orchestrator::monitor`
//!
//! @module orchestrator::monitor
//! @brief Execution monitoring and metrics
//! @group Agent Runtime
//! @since 0.1.0
//! @author Miruamel
//! @see crate::orchestrator

use std::time::Instant;

/// Execution monitor.
///
/// @brief Monitor agent execution
/// @since 0.1.0
/// @group Agent Runtime
/// @since 0.1.0
#[derive(Debug, Clone)]
pub struct Monitor {
    start: Instant,
    steps: usize,
}

impl Monitor {
    /// Create new monitor.
    ///
    /// @return Monitor instance
    /// @since 0.1.0
    pub fn new() -> Self {
        Self {
            start: Instant::now(),
            steps: 0,
        }
    }

    /// Record a step.
    ///
    /// @since 0.1.0
    pub fn record_step(&mut self) {
        self.steps += 1;
    }

    /// Get elapsed time.
    ///
    /// @return Elapsed duration
    /// @since 0.1.0
    pub fn elapsed(&self) -> std::time::Duration {
        self.start.elapsed()
    }
}

impl Default for Monitor {
    fn default() -> Self {
        Self::new()
    }
}
