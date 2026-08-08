//! # Xiaoyi Core
//!
//! `xiaoyi` provides the core runtime for the Xiaoyi AI Agent Framework.
//!
//! Path: `xiaoyi`
//!
//! - Layer 0: `domain` — domain primitives.
//! - Layer 1: `core` — configuration, error handling, result types.
//! - Layer 2: `llm` — LLM client abstraction.
//! - Layer 3: `workflow` — DAG-based workflow execution.
//! - Layer 4: `memory` — Short-term memory (STM) with LRU cache.
//! - Layer 5: `builder`/`orchestrator`/`gateway`/`lexer` — composition layer.
//!
//! @module xiaoyi
//! @brief Xiaoyi AI Agent Framework - Core Runtime
//! @group Xiaoyi
//! @since 0.1.0
//! @author Miruamel
//! @see <https://github.com/miruamel/Xiaoyi>

// WIP vertical architecture: many layer config/state fields are stored for future
// wiring and are intentionally unused until higher layers (orchestrator loop,
// monitoring) are completed. Allow dead_code to keep the crate warning-free.
#![allow(dead_code)]
// Public modules
pub mod xiaoyi;

// Python bindings (PyO3)
#[cfg(feature = "python")]
mod python_bindings;

// Node.js bindings (napi-rs)
#[cfg(feature = "nodejs")]
mod nodejs_bindings;

pub use xiaoyi::builder::AgentBuilder;
pub use xiaoyi::core::config::source::ConfigSource;
pub use xiaoyi::core::config::source::env::EnvSource;
pub use xiaoyi::core::config::source::file::FileSource;
pub use xiaoyi::core::config::source::vault::VaultSource;
pub use xiaoyi::core::config::{Config, ConfigBuilder};
pub use xiaoyi::core::error::{ErrorKind, Result, XiaoyiError};
pub use xiaoyi::core::result::Status;
pub use xiaoyi::critic::{CriticPlant, ReviewResult, Severity};
pub use xiaoyi::domain::token::SyntaxKind;
pub use xiaoyi::evaluator::{
    AnalysisFinding, BenchmarkResult, BuildResult, EvaluationResult, Evaluator, GateResult,
    GateStatus, SandboxResult, TestResult, TestType, TokenUsage,
};
pub use xiaoyi::knowledge::{
    EntryKind, KnowledgeBase, KnowledgeConfig, KnowledgeEntry, KnowledgeStats,
};
pub use xiaoyi::lexer;
pub use xiaoyi::lexer::Lexer;
pub use xiaoyi::llm::client::{
    ChatChoice, ChatMessage, ChatRequest, ChatResponse, LlmClient, MessageRole, Usage,
};
pub use xiaoyi::memory::stm::cache::LruCache;
pub use xiaoyi::orchestrator::{OrchestrationReport, Orchestrator};

pub use xiaoyi::monitoring::{
    Alert, AlertManager, AlertRule, AlertSeverity, Budget, CostEstimate, CostTracker, Counter,
    Gauge, Histogram, HistogramSnapshot, MetricRegistry, Notifier, Span, SpanContext, SpanKind,
    TraceExporter, Tracer,
};
// Resilience re-exports
pub use xiaoyi::resilience::ResiliencePipeline;
pub use xiaoyi::resilience::circuit_breaker::CircuitBreaker;
pub use xiaoyi::resilience::retry::RetryPolicy;
pub use xiaoyi::resilience::{CircuitBreakerConfig, RetryConfig};
// Utils re-exports
pub use xiaoyi::utils::{format_duration, generate_id, slugify, truncate};
// Module trees used by integration tests
pub use xiaoyi::gateway;
pub use xiaoyi::llm;
pub use xiaoyi::memory;
pub use xiaoyi::workflow;
// Additional public types
pub use xiaoyi::core::result::ResultExt;
pub use xiaoyi::domain::token::primitive::PrimitiveKind;
pub use xiaoyi::domain::token::primitive::float::FloatKind;
pub use xiaoyi::domain::token::primitive::int::{IntKind, IntType, IntWidth};
pub use xiaoyi::gateway::Gateway;
pub use xiaoyi::memory::stm::cache::CacheStats;
pub use xiaoyi::workflow::dag::{Dag, DagEdge, DagGraph, DagNode, EdgeKind, NodeId, NodeKind};
/// Initialize the runtime.
///
/// @param config Optional configuration
/// @return Initialized runtime handle
/// @since 0.1.0
/// @threadsafe
pub async fn init(
    config: Option<xiaoyi::core::config::Config>,
) -> xiaoyi::core::error::Result<Runtime> {
    let config = config.unwrap_or_default();
    tracing_subscriber::fmt::init();
    Ok(Runtime { config })
}

/// Runtime handle.
///
/// @brief Runtime management
/// @group Xiaoyi
/// @since 0.1.0
/// @threadsafe
pub struct Runtime {
    config: xiaoyi::core::config::Config,
}
impl Runtime {
    /// Get configuration.
    ///
    /// @return Configuration reference
    /// @since 0.1.0
    pub fn config(&self) -> &xiaoyi::core::config::Config {
        &self.config
    }

    /// Create agent builder.
    ///
    /// @return AgentBuilder instance
    /// @since 0.1.0
    pub fn builder(&self) -> xiaoyi::builder::AgentBuilder {
        xiaoyi::builder::AgentBuilder::new(self.config.clone())
    }

    /// Shutdown runtime.
    ///
    /// @since 0.1.0
    pub async fn shutdown(self) -> xiaoyi::core::error::Result<()> {
        Ok(())
    }
}
