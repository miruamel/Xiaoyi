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

// Public modules
pub mod xiaoyi;

// Python bindings (PyO3)
#[cfg(feature = "python")]
mod python_bindings;

// Node.js bindings (napi-rs)
#[cfg(feature = "nodejs")]
mod nodejs_bindings;

pub use xiaoyi::builder::AgentBuilder;
pub use xiaoyi::core::config::{Config, ConfigBuilder};
pub use xiaoyi::core::config::source::ConfigSource;
pub use xiaoyi::core::config::source::env::EnvSource;
pub use xiaoyi::core::config::source::file::FileSource;
pub use xiaoyi::core::config::source::vault::VaultSource;
pub use xiaoyi::core::error::{ErrorKind, XiaoyiError, Result};
pub use xiaoyi::core::result::Status;
pub use xiaoyi::critic::{CriticPlant, ReviewResult, Severity};
pub use xiaoyi::evaluator::{
    SandboxResult, BuildResult, TestResult, TestType, AnalysisFinding, Severity,
    BenchmarkResult, TokenUsage, GateResult, GateStatus, EvaluationResult, Evaluator,
};
pub use xiaoyi::lexer::Lexer;
pub use xiaoyi::llm::client::{ChatMessage, ChatRequest, ChatResponse, ChatChoice, Usage, LlmClient, MessageRole};
pub use xiaoyi::memory::stm::cache::LruCache;
pub use xiaoyi::orchestrator::Orchestrator;
pub use xiaoyi::workflow::dag::{Dag, DagEdge, DagGraph, DagNode, NodeKind, EdgeKind, NodeId};
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
