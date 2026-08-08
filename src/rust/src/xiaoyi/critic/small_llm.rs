//! # Small LLM Critics
//!
//! `small_llm` provides lightweight LLM-based critics for style,
//! documentation, and accessibility checks.
//!
//! Path: `xiaoyi::critic::small_llm`
//!
//! @module critic::small_llm
//! @brief Small LLM critics (style, doc, accessibility)
//! @group AI Review
//! @since 0.1.0
//! @author Miruamel
//! @see crate::critic
//! @see crate::critic::large_llm

use crate::xiaoyi::core::error::Result;
use crate::xiaoyi::critic::SmallLlmFinding;
use crate::xiaoyi::llm::client::LlmClient;

/// Small LLM critic for style checks.
///
/// @brief Style and formatting critic
/// @group AI Review
/// @since 0.1.0
// #[derive(Debug, Clone)]
pub struct StyleCritic {
    client: Box<dyn LlmClient>,
}

impl StyleCritic {
    /// Create new style critic.
    ///
    /// @param client LLM client
    /// @return StyleCritic instance
    /// @since 0.1.0
    pub fn new(client: Box<dyn LlmClient>) -> Self {
        Self { client }
    }

    /// Review code for style issues.
    ///
    /// @param code Source code to review
    /// @return Vector of style findings
    /// @since 0.1.0
    pub async fn review(&self, code: &str) -> Result<Vec<SmallLlmFinding>> {
        let prompt = format!(
            "Review the following code for style issues (naming, formatting, conventions). \
             Return findings as JSON array with fields: critic, finding, confidence (0.0-1.0).\n\n{}",
            code
        );

        let request = crate::xiaoyi::llm::client::ChatRequest {
            model: "gpt-3.5-turbo".to_string(),
            messages: vec![crate::xiaoyi::llm::client::ChatMessage {
                role: crate::xiaoyi::llm::client::MessageRole::User,
                content: prompt,
                name: None,
            }],
            temperature: Some(0.1),
            max_tokens: Some(1000),
            stream: false,
        };

        let response = self.client.chat(request).await?;
        let content = response
            .choices
            .first()
            .map(|c| c.message.content.clone())
            .unwrap_or_default();

        // Parse JSON response (simplified)
        let findings: Vec<SmallLlmFinding> = serde_json::from_str(&content).unwrap_or_default();
        Ok(findings)
    }
}

/// Small LLM critic for documentation checks.
///
/// @brief Documentation completeness critic
/// @group AI Review
/// @since 0.1.0
// #[derive(Debug, Clone)]
pub struct DocCritic {
    client: Box<dyn LlmClient>,
}

impl DocCritic {
    /// Create new documentation critic.
    ///
    /// @param client LLM client
    /// @return DocCritic instance
    /// @since 0.1.0
    pub fn new(client: Box<dyn LlmClient>) -> Self {
        Self { client }
    }
    pub async fn review(&self, code: &str) -> Result<Vec<SmallLlmFinding>> {
        let prompt = format!(
            "Review the following code for documentation completeness (missing docs, unclear docs). \
             Return findings as JSON array with fields: critic, finding, confidence (0.0-1.0).\n\n{}",
            code
        );

        let request = crate::xiaoyi::llm::client::ChatRequest {
            model: "gpt-3.5-turbo".to_string(),
            messages: vec![crate::xiaoyi::llm::client::ChatMessage {
                role: crate::xiaoyi::llm::client::MessageRole::User,
                content: prompt,
                name: None,
            }],
            temperature: Some(0.1),
            max_tokens: Some(1000),
            stream: false,
        };

        let response = self.client.chat(request).await?;
        let content = response
            .choices
            .first()
            .map(|c| c.message.content.clone())
            .unwrap_or_default();

        let findings: Vec<SmallLlmFinding> = serde_json::from_str(&content).unwrap_or_default();
        Ok(findings)
    }
}

/// Small LLM critic for accessibility checks.
///
/// @brief Accessibility and usability critic
/// @group AI Review
/// @since 0.1.0
// #[derive(Debug, Clone)]
pub struct AccessibilityCritic {
    client: Box<dyn LlmClient>,
}

impl AccessibilityCritic {
    /// Create new accessibility critic.
    ///
    /// @param client LLM client
    /// @return AccessibilityCritic instance
    /// @since 0.1.0
    pub fn new(client: Box<dyn LlmClient>) -> Self {
        Self { client }
    }

    /// Review code for accessibility issues.
    ///
    /// @param code Source code to review
    /// @return Vector of accessibility findings
    /// @since 0.1.0
    pub async fn review(&self, code: &str) -> Result<Vec<SmallLlmFinding>> {
        let prompt = format!(
            "Review the following code for accessibility issues (color contrast, ARIA, keyboard nav). \
             Return findings as JSON array with fields: critic, finding, confidence (0.0-1.0).\n\n{}",
            code
        );

        let request = crate::xiaoyi::llm::client::ChatRequest {
            model: "gpt-3.5-turbo".to_string(),
            messages: vec![crate::xiaoyi::llm::client::ChatMessage {
                role: crate::xiaoyi::llm::client::MessageRole::User,
                content: prompt,
                name: None,
            }],
            temperature: Some(0.1),
            max_tokens: Some(1000),
            stream: false,
        };

        let response = self.client.chat(request).await?;
        let content = response
            .choices
            .first()
            .map(|c| c.message.content.clone())
            .unwrap_or_default();

        let findings: Vec<SmallLlmFinding> = serde_json::from_str(&content).unwrap_or_default();
        Ok(findings)
    }
}
