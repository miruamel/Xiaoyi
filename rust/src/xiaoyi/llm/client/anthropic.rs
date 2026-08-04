//! Anthropic LLM client.
//!
//! Path: `xiaoyi::llm::client::anthropic`

use super::{ChatRequest, ChatResponse, LlmClient, ErrorKind, Result, XiaoyiError};
use async_trait::async_trait;

#[derive(Debug, Clone)]
pub struct AnthropicClient {
    pub api_key: String,
    pub base_url: String,
    client: reqwest::Client,
}

impl AnthropicClient {
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            api_key: api_key.into(),
            base_url: "https://api.anthropic.com/v1".to_string(),
            client: reqwest::Client::new(),
        }
    }
}

#[async_trait]
impl LlmClient for AnthropicClient {
    async fn chat(&self, _request: ChatRequest) -> Result<ChatResponse> {
        unimplemented!("anthropic chat not yet implemented")
    }

    async fn chat_stream(&self, _request: ChatRequest) -> Result<Box<dyn futures::Stream<Item = Result<ChatResponse>> + Send + Unpin>> {
        unimplemented!("stream not yet implemented")
    }

    fn provider_name(&self) -> &'static str {
        "anthropic"
    }
}