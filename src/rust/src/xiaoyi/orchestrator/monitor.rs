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

use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Instant;

/// Execution monitor.
///
/// Tracks elapsed wall-clock time and a step counter. Uses interior mutability
/// so it can be updated through a shared `&self` reference from the orchestrator loop.
///
/// @brief Monitor agent execution
/// @since 0.1.0
/// @group Agent Runtime
#[derive(Debug)]
pub struct Monitor {
    start: Instant,
    steps: AtomicUsize,
}

impl Monitor {
    /// Create new monitor.
    ///
    /// @return Monitor instance
    /// @since 0.1.0
    pub fn new() -> Self {
        Self {
            start: Instant::now(),
            steps: AtomicUsize::new(0),
        }
    }

    /// Record a step.
    ///
    /// @since 0.1.0
    pub fn record_step(&self) {
        self.steps.fetch_add(1, Ordering::Relaxed);
    }

    /// Get the number of recorded steps.
    ///
    /// @return Total steps recorded
    /// @since 0.1.0
    pub fn steps(&self) -> usize {
        self.steps.load(Ordering::Relaxed)
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
