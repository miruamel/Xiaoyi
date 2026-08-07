//! # LLM Client Abstraction
//!
//! `llm` provides a unified interface for multiple LLM providers.
//!
//! Path: `xiaoyi::llm`
//!
//! - Layer 0: `llm` — LLM abstraction layer.
//! - Layer 1: `client` — client trait and types.
//! - Layer 2: `openai`/`anthropic`/`ollama` — provider implementations.
//! - Layer 3: `request`/`response`/`stream` — protocol details.
//!
//! @module llm
//! @brief Unified LLM client for multiple providers
//! @group AI Runtime
//! @since 0.1.0
//! @author Miruamel
//! @see crate::llm::client
//! @see crate::llm::client::openai
//! @see crate::llm::client::anthropic
//! @see crate::llm::client::ollama
pub mod client;
