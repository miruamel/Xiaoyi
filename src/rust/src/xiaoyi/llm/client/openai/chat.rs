use crate::xiaoyi::llm::client::{ChatRequest, ChatResponse, LlmClient, MessageRole};

/// OpenAI chat completion helper.
///
/// @brief OpenAI chat-specific helpers
/// @since 0.1.0
/// @author Miruamel
#[derive(Debug, Clone, Default)]
pub struct OpenAiChat;

impl OpenAiChat {
    /// Create a user message.
    ///
    /// @param content Message text
    /// @return Chat message
    /// @since 0.1.0
    pub fn user(content: impl Into<String>) -> crate::xiaoyi::llm::client::ChatMessage {
        crate::xiaoyi::llm::client::ChatMessage {
            role: MessageRole::User,
            content: content.into(),
            name: None,
        }
    }
}
