use crate::xiaoyi::llm::client::{ChatRequest, ChatResponse, LlmClient, MessageRole};

/// OpenAI client stub.
///
/// @brief OpenAI provider implementation
/// @group AI Runtime
/// @since 0.1.0
/// @author Miruamel
/// @see crate::xiaoyi::llm::client
#[derive(Debug, Clone, Default)]
pub struct OpenAiClient {
    pub api_key: Option<String>,
}

impl OpenAiClient {
    /// Create a new OpenAI client.
    ///
    /// @param api_key API key
    /// @return OpenAiClient instance
    /// @since 0.1.0
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            api_key: Some(api_key.into()),
        }
    }
}

#[async_trait::async_trait]
impl LlmClient for OpenAiClient {
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
