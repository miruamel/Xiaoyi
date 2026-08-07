//! # Xiaoyi Core Modules
//!
//! This module aggregates all core subsystems of the Xiaoyi runtime.
//!
//! # Module Structure
//!
//! - `core` — Foundational cross-cutting types (error, config, result)
//! - `domain` — Domain-specific primitives (tokens, syntax)
//! - `lexer` — Lexical analysis and tokenization
//! - `llm` — LLM client abstraction and providers
//! - `memory` — Memory systems (STM, LTM)
//! - `workflow` — Workflow DAG execution engine
//! - `builder` — Build agent with AST sanitization and code generation
//! - `orchestrator` — Autonomous agent loop orchestration
//!
//! # Architecture
//!
//! Each domain follows the deep vertical layering pattern:
//! - Layer 0: core (foundational cross-cutting types)
//! - Layer 1-6: domain-specific vertical layers
//!
//! @module xiaoyi
//! @brief Core module aggregation for Xiaoyi runtime
//! @group Core Runtime
//! @since 0.1.0
//! @see crate::core
//! @see crate::domain
//! @see crate::lexer
//! @see crate::llm
//! @see crate::memory
//! @see crate::workflow
//! @see crate::builder
//! @see crate::orchestrator
//! @see crate::gateway
//! @module xiaoyi
pub mod core;
pub mod domain;
pub mod lexer;
pub mod llm;
pub mod memory;
pub mod workflow;
pub mod builder;
pub mod orchestrator;
pub mod gateway;