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
//! @see https://github.com/miruamel/Xiaoyi

// Public modules
pub mod xiaoyi;

// Python bindings (PyO3)
#[cfg(feature = "python")]
mod python_bindings;

// Node.js bindings (napi-rs)
#[cfg(feature = "nodejs")]
mod nodejs_bindings;

// Re-exports
pub use xiaoyi::core::config::{Config, ConfigBuilder, ConfigSource};
pub use xiaoyi::core::config::source::file::FileSource;
pub use xiaoyi::core::config::source::ConfigSource as AsyncConfigSource;
pub use xiaoyi::core::error::{XiaoyiError, ErrorKind, Result};
pub use xiaoyi::core::result::{Status, ResultExt};
pub use xiaoyi::domain::token::{PrimitiveKind, IntKind, IntWidth, IntType, FloatKind, SyntaxKind};
pub use xiaoyi::llm;
pub use xiaoyi::llm::client::{LlmClient, ChatRequest, ChatResponse, ChatMessage, MessageRole, Usage};
pub use xiaoyi::workflow;
pub use xiaoyi::workflow::dag::{Dag, DagNode, DagEdge, DagGraph};
pub use xiaoyi::workflow::dag::graph::{NodeId, NodeKind, EdgeKind};
pub use xiaoyi::memory::stm::{StmCache, CacheEntry, CacheStats, LruCache};
pub use xiaoyi::builder::AgentBuilder;
pub use xiaoyi::orchestrator::Orchestrator;
pub use xiaoyi::gateway::Gateway;
pub use xiaoyi::lexer;
pub use xiaoyi::lexer::Lexer;
pub use xiaoyi::lexer::scanner::Scanner;
pub use xiaoyi::lexer::token::Token;

/// Initialize the runtime.
///
/// @param config Optional configuration
/// @return Initialized runtime handle
/// @since 0.1.0
/// @threadsafe
pub async fn init(config: Option<xiaoyi::core::config::Config>) -> xiaoyi::core::error::Result<Runtime> {
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