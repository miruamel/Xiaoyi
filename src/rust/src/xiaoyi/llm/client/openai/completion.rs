use crate::xiaoyi::llm::client::ChatRequest;

/// OpenAI completion helper.
///
/// @brief OpenAI completion helpers
/// @since 0.1.0
/// @author Miruamel
#[derive(Debug, Clone, Default)]
pub struct OpenAiCompletion;

impl OpenAiCompletion {
    /// Create a minimal chat request.
    ///
    /// @param model Model identifier
    /// @param prompt Prompt text
    /// @return Chat request
    /// @since 0.1.0
    pub fn simple(model: impl Into<String>, prompt: impl Into<String>) -> ChatRequest {
        ChatRequest {
            model: model.into(),
            messages: vec![crate::xiaoyi::llm::client::ChatMessage {
                role: crate::xiaoyi::llm::client::MessageRole::User,
                content: prompt.into(),
                name: None,
            }],
            temperature: None,
            max_tokens: None,
            stream: false,
        }
    }
}
