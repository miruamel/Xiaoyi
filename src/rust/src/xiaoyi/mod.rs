/// # Xiaoyi Core Modules
///
/// This module aggregates all core subsystems of the Xiaoyi runtime.
///
/// # Module Structure
///
/// - `core` — Foundational cross-cutting types (error, config, result)
/// - `domain` — Domain-specific primitives (tokens, syntax)
/// - `lexer` — Lexical analysis and tokenization
/// - `llm` — LLM client abstraction and providers
/// - `memory` — Memory systems (STM, LTM)
/// - `workflow` — Workflow DAG execution engine
/// - `builder` — Build agent with AST sanitization and code generation
/// - `orchestrator` — Autonomous agent loop orchestration
/// - `evaluator` — Evaluation and feedback toolchain
/// - `gateway` — API gateway layer
///
/// # Architecture
///
/// Each domain follows the deep vertical layering pattern:
/// - Layer 0: core (foundational cross-cutting types)
/// - Layer 1-6: domain-specific vertical layers
///
/// @module xiaoyi
/// @brief Core module aggregation for Xiaoyi runtime
/// @group Core Runtime
/// @since 0.1.0
/// @see crate::core
/// @see crate::domain
/// @see crate::lexer
/// @see crate::llm
/// @see crate::memory
/// @see crate::workflow
/// @see crate::builder
/// @see crate::orchestrator
/// @see crate::gateway
/// @see crate::evaluator
pub mod builder;
pub mod core;
pub mod critic;
pub mod domain;
pub mod evaluator;
pub mod gateway;
pub mod knowledge;
pub mod monitoring;
pub mod lexer;
pub mod llm;
pub mod memory;
pub mod orchestrator;
pub mod resilience;
pub mod workflow;

// Knowledge re-exports
pub use knowledge::{
    EntryKind, KnowledgeBase, KnowledgeConfig, KnowledgeEntry, KnowledgeStats,
};

// Monitoring re-exports
pub use monitoring::{
    Alert, AlertManager, AlertRule, AlertSeverity, Notifier, Budget, CostEstimate, CostTracker,
    Counter, Gauge, Histogram, HistogramSnapshot, MetricRegistry, Span, SpanContext, SpanKind,
    TraceExporter, Tracer,
};

// Re-exports for public API
pub use core::config::{Config, ConfigBuilder};
pub use core::error::{ErrorKind, XiaoyiError, Result};
pub use core::result::Status;
pub use evaluator::{Evaluator, EvaluationResult, SandboxResult, BuildResult, TestResult, TestType, AnalysisFinding, Severity, BenchmarkResult, TokenUsage, GateStatus, GateResult};