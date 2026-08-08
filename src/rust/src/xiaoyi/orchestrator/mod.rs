//! # Orchestrator Module
//!
//! `orchestrator` provides autonomous agent loop orchestration. It ties together the
//! observation, review, evaluation, and resilience layers into a single execution loop.
//!
//! Path: `xiaoyi::orchestrator`
//!
//! - Layer 0: `orchestrator` — Orchestration layer.
//! - Layer 1: `loop` — Agent execution loop.
//! - Layer 2: `policy` — Decision policies.
//! - Layer 3: `monitor` — Execution monitoring.
//!
//! The [`Orchestrator`] owns a [`Monitor`][monitor::Monitor], a [`Policy`][policy::Policy],
//! a [`MetricRegistry`] for live telemetry, a
//! [`CriticPlant`], an
//! [`Evaluator`], and a
//! [`ResiliencePipeline`] that wraps each
//! review/evaluate call in circuit breaking and retry.
//!
//! @module orchestrator
//! @brief Autonomous agent loop orchestration
//! @group Agent Runtime
//! @since 0.1.0
//! @author Miruamel
//! @see crate::builder
//! @see crate::gateway
//! @see crate::monitoring
//! @see crate::critic
//! @see crate::evaluator
//! @see crate::resilience
//!
//! # Example
//!
//! ```rust
//! use xiaoyi::Orchestrator;
//! use xiaoyi::AgentBuilder;
//! use xiaoyi::Config;
//!
//! # async fn run() -> xiaoyi::Result<()> {
//! let config = Config::default();
//! let agent = AgentBuilder::new(config.clone()).name("test").model("test").build()?;
//! let orchestrator = Orchestrator::new(config);
//! orchestrator.run(agent).await?;
//! # Ok(())
//! # }
//! ```
pub mod loop_;
pub mod monitor;
pub mod policy;

use crate::xiaoyi::builder::AgentHandle;
use crate::xiaoyi::core::config::Config;
use crate::xiaoyi::core::result::Result;

use crate::xiaoyi::monitoring::MetricRegistry;

use crate::xiaoyi::critic::CriticPlant;
use crate::xiaoyi::evaluator::Evaluator;
use crate::xiaoyi::resilience::ResiliencePipeline;
use crate::xiaoyi::resilience::circuit_breaker::CircuitBreaker;
use crate::xiaoyi::resilience::retry::RetryPolicy;
use crate::xiaoyi::resilience::{CircuitBreakerConfig, RetryConfig};

/// Summary produced by [`Orchestrator::orchestrate`].
///
/// @brief Aggregate outcome of a critic/evaluator orchestration run
/// @group Agent Runtime
/// @since 0.1.0
#[derive(Debug, Clone, Default)]
pub struct OrchestrationReport {
    /// Number of loop iterations executed.
    pub iterations: usize,
    /// Successful critic reviews completed.
    pub reviews_completed: u64,
    /// Successful evaluations completed.
    pub evals_completed: u64,
    /// Total resilience-wrapped operations that returned an error.
    pub errors: u64,
    /// Wall-clock duration of the run in milliseconds.
    pub elapsed_ms: u128,
}

/// Orchestrator for running agent loops.
///
/// Owns the monitoring, policy, critic, evaluator, and resilience subsystems and
/// drives them through the agent execution loop.
///
/// @brief Autonomous agent execution with integrated review/eval/resilience
/// @group Agent Runtime
/// @since 0.1.0
#[derive(Debug)]
pub struct Orchestrator {
    config: Config,
    monitor: monitor::Monitor,
    policy: policy::Policy,
    metrics: MetricRegistry,
    critic: CriticPlant,
    evaluator: Evaluator,
    resilience: ResiliencePipeline,
}

impl Orchestrator {
    /// Create a new orchestrator wired with monitoring, critic, evaluator, and resilience.
    ///
    /// @param config Runtime configuration
    /// @return Orchestrator instance
    /// @since 0.1.0
    pub fn new(config: Config) -> Self {
        let metrics = MetricRegistry::new();
        let _ = metrics.register_counter("orchestrator.iterations");
        let _ = metrics.register_counter("orchestrator.reviews");
        let _ = metrics.register_counter("orchestrator.evals");
        let _ = metrics.register_counter("orchestrator.errors");

        let resilience = ResiliencePipeline::new(
            CircuitBreaker::new(CircuitBreakerConfig::default()),
            RetryPolicy::new(RetryConfig::default()),
            None,
            None,
            None,
        );

        Self {
            config,
            monitor: monitor::Monitor::new(),
            policy: policy::Policy::default(),
            metrics,
            critic: CriticPlant::new(),
            evaluator: Evaluator::new(),
            resilience,
        }
    }

    /// Run the agent execution loop (monitoring + policy driven).
    ///
    /// Iterates until the [`Policy`][policy::Policy] budget is exhausted or the monitor
    /// timeout elapses, recording each step in the live [`MetricRegistry`].
    ///
    /// @param agent Agent to run (held for future task extraction)
    /// @return Execution result
    /// @since 0.1.0
    pub async fn run(&self, _agent: AgentHandle) -> Result<()> {
        let mut state = loop_::LoopState::default();
        while policy::should_continue(&state, &self.policy) {
            loop_::step(&mut state)?;
            self.monitor.record_step();
            if let Some(counter) = self.metrics.counter("orchestrator.iterations") {
                let _ = counter.inc(1);
            }
            if self.monitor.elapsed().as_millis() > self.policy.timeout_ms as u128 {
                break;
            }
        }
        Ok(())
    }

    /// Orchestrate a full review/evaluate cycle over `code` for `language`.
    ///
    /// Each iteration wraps the critic review and the evaluator pass in the
    /// [`ResiliencePipeline`], records live counters, and accumulates an
    /// [`OrchestrationReport`]. The loop honours the policy iteration budget and
    /// monitor timeout.
    ///
    /// @param goal High-level objective (reserved for future DAG planning)
    /// @param code Source code to review and evaluate
    /// @param language Programming language of `code`
    /// @return Aggregated [`OrchestrationReport`]
    /// @since 0.1.0
    pub async fn orchestrate(
        &self,
        _goal: &str,
        code: &str,
        language: &str,
    ) -> Result<OrchestrationReport> {
        let started = std::time::Instant::now();
        let mut report = OrchestrationReport::default();
        let mut state = loop_::LoopState::default();

        while policy::should_continue(&state, &self.policy) {
            loop_::step(&mut state)?;
            self.monitor.record_step();
            if let Some(counter) = self.metrics.counter("orchestrator.iterations") {
                let _ = counter.inc(1);
            }

            match self
                .resilience
                .execute(|| async { self.critic.review(code).await })
                .await
            {
                Ok(_) => {
                    report.reviews_completed += 1;
                    if let Some(counter) = self.metrics.counter("orchestrator.reviews") {
                        let _ = counter.inc(1);
                    }
                }
                Err(_) => {
                    report.errors += 1;
                    if let Some(counter) = self.metrics.counter("orchestrator.errors") {
                        let _ = counter.inc(1);
                    }
                }
            }

            match self
                .resilience
                .execute(|| async { self.evaluator.evaluate(code, language).await })
                .await
            {
                Ok(_) => {
                    report.evals_completed += 1;
                    if let Some(counter) = self.metrics.counter("orchestrator.evals") {
                        let _ = counter.inc(1);
                    }
                }
                Err(_) => {
                    report.errors += 1;
                    if let Some(counter) = self.metrics.counter("orchestrator.errors") {
                        let _ = counter.inc(1);
                    }
                }
            }

            if self.monitor.elapsed().as_millis() > self.policy.timeout_ms as u128 {
                break;
            }
        }

        report.iterations = state.iterations;
        report.elapsed_ms = started.elapsed().as_millis();
        Ok(report)
    }
}
