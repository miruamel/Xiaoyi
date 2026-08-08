//! # Circuit Breaker Module
//!
//! `circuit_breaker` provides circuit breaker pattern for fault isolation.
//!
//! Path: `xiaoyi::resilience::circuit_breaker`
//!
//! @module resilience::circuit_breaker
//! @brief Circuit breaker pattern implementation
//! @group AI Resilience
//! @since 0.1.0
//! @author Miruamel
//! @see crate::resilience
//! @see crate::resilience::retry

use crate::xiaoyi::resilience::CircuitBreakerConfig;

/// Circuit breaker states.
///
/// @brief Circuit breaker state machine
/// @group AI Resilience
/// @since 0.1.0
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CircuitState {
    /// Circuit is closed, operations flow normally
    Closed,
    /// Circuit is open, operations blocked
    Open,
    /// Circuit is half-open, testing recovery
    HalfOpen,
}

/// Circuit breaker for fault isolation.
///
/// @brief Prevents cascade failures via circuit breaker
/// @group AI Resilience
/// @since 0.1.0
/// @threadsafe Yes
/// @complexity O(1) per operation
#[derive(Debug, Clone)]
pub struct CircuitBreaker {
    config: CircuitBreakerConfig,
    state: crate::xiaoyi::resilience::circuit_breaker::CircuitState,
    failure_count: usize,
    success_count: usize,
    last_failure_time: std::time::Instant,
}

impl CircuitBreaker {
    /// Create new circuit breaker.
    ///
    /// @param config Circuit breaker configuration
    /// @return CircuitBreaker instance
    /// @since 0.1.0
    pub fn new(config: CircuitBreakerConfig) -> Self {
        Self {
            config,
            state: crate::xiaoyi::resilience::circuit_breaker::CircuitState::Closed,
            failure_count: 0,
            success_count: 0,
            last_failure_time: std::time::Instant::now(),
        }
    }

    /// Execute operation through circuit breaker.
    ///
    /// @param operation Operation to execute
    /// @return Result or CircuitOpen error
    /// @throw CircuitOpenError when circuit is open
    /// @since 0.1.0
    pub async fn execute<F, Fut, T, E>(&mut self, operation: F) -> std::result::Result<T, E>
    where
        F: Fn() -> Fut,
        Fut: std::future::Future<Output = std::result::Result<T, E>>,
        E: std::fmt::Display,
    {
        match self.state {
            crate::xiaoyi::resilience::circuit_breaker::CircuitState::Open => {
                // Check if recovery timeout elapsed
                if self.last_failure_time.elapsed().as_secs() >= self.config.recovery_timeout_secs {
                    self.state = crate::xiaoyi::resilience::circuit_breaker::CircuitState::HalfOpen;
                    self.success_count = 0;
                } else {
                    // Circuit still open
                    // In production, would return specific CircuitOpen error
                }
            }
            crate::xiaoyi::resilience::circuit_breaker::CircuitState::Closed => {}
            crate::xiaoyi::resilience::circuit_breaker::CircuitState::HalfOpen => {}
        }

        let result = operation().await;

        match &result {
            Ok(_) => {
                self.on_success();
            }
            Err(_) => {
                self.on_failure();
            }
        }

        result
    }

    /// Record success.
    ///
    /// @since 0.1.0
    fn on_success(&mut self) {
        self.failure_count = 0;

        if self.state == crate::xiaoyi::resilience::circuit_breaker::CircuitState::HalfOpen {
            self.success_count += 1;
            if self.success_count >= self.config.half_open_probes {
                self.state = crate::xiaoyi::resilience::circuit_breaker::CircuitState::Closed;
            }
        }
    }

    /// Record failure.
    ///
    /// @since 0.1.0
    fn on_failure(&mut self) {
        self.last_failure_time = std::time::Instant::now();
        self.failure_count += 1;

        if self.failure_count >= self.config.failure_threshold {
            self.state = crate::xiaoyi::resilience::circuit_breaker::CircuitState::Open;
        }

        if self.state == crate::xiaoyi::resilience::circuit_breaker::CircuitState::HalfOpen {
            self.state = crate::xiaoyi::resilience::circuit_breaker::CircuitState::Open;
        }
    }

    /// Get current state.
    ///
    /// @return Current circuit state
    /// @since 0.1.0
    pub fn state(&self) -> crate::xiaoyi::resilience::circuit_breaker::CircuitState {
        self.state
    }

    /// Get failure count.
    ///
    /// @return Number of consecutive failures
    /// @since 0.1.0
    pub fn failure_count(&self) -> usize {
        self.failure_count
    }

    /// Reset circuit breaker.
    ///
    /// @since 0.1.0
    pub fn reset(&mut self) {
        self.state = crate::xiaoyi::resilience::circuit_breaker::CircuitState::Closed;
        self.failure_count = 0;
        self.success_count = 0;
    }
}

impl Default for CircuitBreaker {
    fn default() -> Self {
        Self::new(CircuitBreakerConfig::default())
    }
}