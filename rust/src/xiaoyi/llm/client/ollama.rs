//! Ollama local LLM client.
//!
//! Path: `xiaoyi::llm::client::ollama`

use super::{ChatRequest, ChatResponse, LlmClient, ErrorKind, Result, XiaoyiError};
use async_trait::async_trait;

#[derive(Debug, Clone)]
pub struct OllamaClient {
    pub base_url: String,
    client: reqwest::Client,
}

impl OllamaClient {
    pub fn new(base_url: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into(),
            client: reqwest::Client::new(),
        }
    }
}

#[async_trait]
impl LlmClient for OllamaClient {
    async fn chat(&self, request: ChatRequest) -> Result<ChatResponse> {
        let url = format!("{}/api/chat", self.base_url);
        let resp = self.client
            .post(&url)
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
        "ollama"
    }
}