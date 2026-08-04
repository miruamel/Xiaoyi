//! OpenAI-compatible LLM client.
//!
//! Path: `xiaoyi::llm::client::openai`

use super::{ChatRequest, ChatResponse, LlmClient, ErrorKind, Result, XiaoyiError};
use async_trait::async_trait;

#[derive(Debug, Clone)]
pub struct OpenAiClient {
    pub api_key: String,
    pub base_url: String,
    client: reqwest::Client,
}

impl OpenAiClient {
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            api_key: api_key.into(),
            base_url: "https://api.openai.com/v1".to_string(),
            client: reqwest::Client::new(),
        }
    }

    pub fn with_base_url(mut self, url: impl Into<String>) -> Self {
        self.base_url = url.into();
        self
    }
}

#[async_trait]
impl LlmClient for OpenAiClient {
    async fn chat(&self, request: ChatRequest) -> Result<ChatResponse> {
        let url = format!("{}/chat/completions", self.base_url);
        let resp = self.client
            .post(&url)
            .bearer_auth(&self.api_key)
            .json(&request)
            .send()
            .await
            .map_err(|e| XiaoyiError::new(ErrorKind::Llm, e.to_string()))?;

        if !resp.status().is_success() {
            let err = resp.text().await.unwrap_or_default();
            return Err(XiaoyiError::new(ErrorKind::Llm, err));
        }

        resp.json().await.map_err(|e| XiaoyiError::new(ErrorKind::Llm, e.to_string()))
    }

    async fn chat_stream(&self, _request: ChatRequest) -> Result<Box<dyn futures::Stream<Item = Result<ChatResponse>> + Send + Unpin>> {
        unimplemented!("stream not yet implemented")
    }

    fn provider_name(&self) -> &'static str {
        "openai"
    }
}