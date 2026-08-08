use crate::xiaoyi::llm::client::{ChatRequest, ChatResponse, LlmClient};

/// Ollama client stub.
///
/// @brief Ollama provider implementation
/// @group AI Runtime
/// @since 0.1.0
/// @author Miruamel
/// @see crate::xiaoyi::llm::client
#[derive(Debug, Clone, Default)]
pub struct OllamaClient {
    pub host: Option<String>,
}

impl OllamaClient {
    /// Create a new Ollama client.
    ///
    /// @param host Ollama server host
    /// @return OllamaClient instance
    /// @since 0.1.0
    pub fn new(host: impl Into<String>) -> Self {
        Self {
            host: Some(host.into()),
        }
    }
}

#[async_trait::async_trait]
impl LlmClient for OllamaClient {
    async fn chat(&self, _request: ChatRequest) -> crate::xiaoyi::core::error::Result<ChatResponse> {
        Ok(ChatResponse {
            id: "stub".into(),
            model: _request.model,
            choices: vec![],
            usage: crate::xiaoyi::llm::client::Usage {
                prompt_tokens: 0,
                completion_tokens: 0,
                total_tokens: 0,
            },
        })
    }
}
