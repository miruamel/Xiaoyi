use crate::xiaoyi::core::error::Result;
use crate::xiaoyi::llm::client::ChatRequest;

/// OpenAI streaming response helper.
///
/// @brief OpenAI streaming helpers
/// @since 0.1.0
/// @author Miruamel
#[derive(Debug, Clone, Default)]
pub struct OpenAiStream;

impl OpenAiStream {
    /// Request streaming chat completion.
    ///
    /// @param request Chat request
    /// @return Stream placeholder
    /// @since 0.1.0
    pub fn request_stream(
        &self,
        _request: ChatRequest,
    ) -> Result<crate::xiaoyi::llm::client::ChatResponse> {
        Ok(crate::xiaoyi::llm::client::ChatResponse {
            id: "stream-stub".into(),
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
