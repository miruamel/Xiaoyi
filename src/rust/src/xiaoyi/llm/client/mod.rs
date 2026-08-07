//! # LLM Client
//!
//! `client` defines the unified LLM client trait and shared types.
//!
//! Path: `xiaoyi::llm::client`
//!
//! - Layer 0: `llm`
//! - Layer 1: `client` — client contract and types.
//! - Layer 2: `openai`/`anthropic`/`ollama` — implementations.
//!
//! @module llm::client
//! @brief Unified LLM client trait and types
//! @group AI Runtime
//! @since 0.1.0
//! @author Miruamel
//! @see crate::llm
//! @see crate::llm::client::openai
//! @see crate::llm::client::anthropic
//! @see crate::llm::client::ollama
//!
//! # Example
//!
//! ```no_run
//! use xiaoyi::llm::client::{OpenAiClient, LlmClient, ChatRequest, ChatMessage, MessageRole};
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     let client = OpenAiClient::new(std::env::var("OPENAI_API_KEY")?);
//!     let request = ChatRequest {
//!         model: "gpt-4o-mini".into(),
//!         messages: vec![ChatMessage { role: MessageRole::User, content: "Hello!".into(), name: None }],
//!         temperature: Some(0.7),
//!         max_tokens: Some(100),
//!         stream: false,
//!     };
//!     let resp = client.chat(request).await?;
//!     println!("{}", resp.choices[0].message.content);
//!     Ok(())
//! }
//! ```
//!
//! # Providers
//!
//! | Provider | Module | Env Var |
//! |----------|--------|---------|
//! | OpenAI | `openai` | `OPENAI_API_KEY` |
//! | Anthropic | `anthropic` | `ANTHROPIC_API_KEY` |
//! | Ollama | `ollama` | `OLLAMA_HOST` |

use crate::xiaoyi::core::error::Result;
use crate::xiaoyi::core::result::ResultExt;
use async_trait::async_trait;
use futures::Stream;
use serde::{Deserialize, Serialize};
use std::pin::Pin;

/// Chat message role.
///
/// @brief Message role in conversation
/// @group AI Runtime
/// @since 0.1.0
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum MessageRole {
    /// System message (instructions).
    System,
    /// User message.
    User,
    /// Assistant response.
    Assistant,
    /// Tool/function call result.
    Tool,
}

/// Chat message.
///
/// @brief Single message in chat conversation
/// @group AI Runtime
/// @since 0.1.0
/// @see MessageRole
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    /// Message role.
    pub role: MessageRole,
    /// Message content.
    pub content: String,
    /// Optional name for function/tool calls.
    pub name: Option<String>,
}

/// Chat completion request.
///
/// @brief Request parameters for chat completion
/// @group AI Runtime
/// @since 0.1.0
/// @see ChatMessage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatRequest {
    /// Model identifier (e.g., "gpt-4o-mini").
    pub model: String,
    /// Conversation messages.
    pub messages: Vec<ChatMessage>,
    /// Sampling temperature (0.0-2.0).
    pub temperature: Option<f32>,
    /// Maximum tokens to generate.
    pub max_tokens: Option<u32>,
    /// Enable streaming response.
    pub stream: bool,
}

/// Chat choice (single completion).
///
/// @brief Single completion choice
/// @group AI Runtime
/// @since 0.1.0
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatChoice {
    /// Choice index.
    pub index: u32,
    /// Generated message.
    pub message: ChatMessage,
    /// Finish reason (stop, length, tool_calls, etc.).
    pub finish_reason: Option<String>,
}

/// Token usage statistics.
///
/// @brief Token consumption metrics
/// @group AI Runtime
/// @since 0.1.0
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Usage {
    /// Prompt tokens.
    pub prompt_tokens: u32,
    /// Completion tokens.
    pub completion_tokens: u32,
    /// Total tokens.
    pub total_tokens: u32,
}

/// Chat completion response.
///
/// @brief Response from chat completion
/// @group AI Runtime
/// @since 0.1.0
/// @see ChatChoice
/// @see Usage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatResponse {
    /// Response ID.
    pub id: String,
    /// Model used.
    pub model: String,
    /// Generated choices.
    pub choices: Vec<ChatChoice>,
    /// Token usage.
    pub usage: Option<Usage>,
}

/// Unified LLM client trait.
///
/// @brief Provider-agnostic LLM client contract
/// @group AI Runtime
/// @since 0.1.0
#[async_trait]
pub trait LlmClient: Send + Sync {
    /// Send a chat completion request.
    ///
    /// @param request Chat completion request
    /// @return Chat response or error
    /// @throw Llm error on API failure
    /// @since 0.1.0
    async fn chat(&self, request: ChatRequest) -> Result<ChatResponse>;

    /// Send a streaming chat completion request.
    ///
    /// @param request Chat completion request (stream=true)
    /// @return Stream of partial responses
    /// @throw Llm error on stream failure
    /// @since 0.1.0
    async fn chat_stream(
        &self,
        request: ChatRequest,
    ) -> Result<Pin<Box<dyn Stream<Item = Result<ChatResponse>> + Send + Unpin>>>;

    /// Get provider name for logging/debugging.
    ///
    /// @return Static provider name
    /// @since 0.1.0
    fn provider_name(&self) -> &'static str;
}
