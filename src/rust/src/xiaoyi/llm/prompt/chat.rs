use crate::xiaoyi::llm::client::{ChatMessage, ChatRequest, MessageRole};

/// Chat prompt builder.
///
/// @brief Build chat requests from prompts
/// @since 0.1.0
/// @author Miruamel
/// @see ChatRequest
#[derive(Debug, Clone, Default)]
pub struct ChatPromptBuilder {
    pub system: Option<String>,
}

impl ChatPromptBuilder {
    /// Create a new chat prompt builder.
    ///
    /// @param system Optional system prompt
    /// @return ChatPromptBuilder instance
    /// @since 0.1.0
    pub fn new(system: impl Into<Option<String>>) -> Self {
        Self {
            system: system.into(),
        }
    }

    /// Append a user message.
    ///
    /// @param content User message text
    /// @return Updated builder
    /// @since 0.1.0
    pub fn user(mut self, content: impl Into<String>) -> Self {
        self.system = self
            .system
            .map(|s| format!("{s}\nUser: {}", content.into()));
        self
    }

    /// Build a chat request.
    ///
    /// @param model Model identifier
    /// @return Chat request
    /// @since 0.1.0
    pub fn build(self, model: impl Into<String>) -> ChatRequest {
        let content = self.system.unwrap_or_default();
        ChatRequest {
            model: model.into(),
            messages: vec![ChatMessage {
                role: MessageRole::User,
                content: content.into(),
                name: None,
            }],
            temperature: None,
            max_tokens: None,
            stream: false,
        }
    }
}
