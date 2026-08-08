//! # Large LLM Critics
//!
//! `large_llm` provides heavyweight LLM critics for security,
//! architecture, and complex logic analysis.
//!
//! Path: `xiaoyi::critic::large_llm`
//!
//! @module critic::large_llm
//! @brief Large LLM critics (security, architecture, complex logic)
//! @group AI Review
//! @since 0.1.0
//! @author Miruamel
//! @see crate::critic
//! @see crate::critic::small_llm

use crate::xiaoyi::critic::LargeLlmFinding;
use crate::xiaoyi::core::error::Result;
use crate::xiaoyi::llm::client::LlmClient;

/// Large LLM critic for security analysis.
///
/// @brief Security vulnerability critic
/// @group AI Review
/// @since 0.1.0
// #[derive(Debug, Clone)]
pub struct SecurityCritic {
    client: Box<dyn LlmClient>,
}

impl SecurityCritic {
    /// Create new security critic.
    ///
    /// @param client LLM client
    /// @return SecurityCritic instance
    /// @since 0.1.0
    pub fn new(client: Box<dyn LlmClient>) -> Self {
        Self { client }
    }

    /// Review code for security vulnerabilities.
    ///
    /// @param code Source code to review
    /// @return Vector of security findings
    /// @since 0.1.0
    pub async fn review(&self, code: &str) -> Result<Vec<LargeLlmFinding>> {
        let prompt = format!(
            "Review the following code for security vulnerabilities (injection, auth bypass, \
             data exposure, crypto issues, OWASP Top 10). \
             Return findings as JSON array with fields: critic, finding, confidence (0.0-1.0), suggested_fix.\n\n{}",
            code
        );

        let request = crate::xiaoyi::llm::client::ChatRequest {
            model: "gpt-4".to_string(),
            messages: vec![crate::xiaoyi::llm::client::ChatMessage {
                role: crate::xiaoyi::llm::client::MessageRole::User,
                content: prompt,
                name: None,
            }],
            temperature: Some(0.1),
            max_tokens: Some(2000),
            stream: false,
        };

        let response = self.client.chat(request).await?;
        let content = response.choices.first().map(|c| c.message.content.clone()).unwrap_or_default();

        let findings: Vec<LargeLlmFinding> = serde_json::from_str(&content).unwrap_or_default();
        Ok(findings)
    }
}

/// Large LLM critic for architecture analysis.
///
/// @brief Architecture and design pattern critic
/// @group AI Review
/// @since 0.1.0
// #[derive(Debug, Clone)]
pub struct ArchitectureCritic {
    client: Box<dyn LlmClient>,
}

impl ArchitectureCritic {
    /// Create new architecture critic.
    ///
    /// @param client LLM client
    /// @return ArchitectureCritic instance
    /// @since 0.1.0
    pub fn new(client: Box<dyn LlmClient>) -> Self {
        Self { client }
    }
    pub async fn review(&self, code: &str) -> Result<Vec<LargeLlmFinding>> {
        let prompt = format!(
            "Review the following code for architecture issues (SOLID violations, coupling, \
             design patterns, scalability, maintainability). \
             Return findings as JSON array with fields: critic, finding, confidence (0.0-1.0), suggested_fix.\n\n{}",
            code
        );

        let request = crate::xiaoyi::llm::client::ChatRequest {
            model: "gpt-4".to_string(),
            messages: vec![crate::xiaoyi::llm::client::ChatMessage {
                role: crate::xiaoyi::llm::client::MessageRole::User,
                content: prompt,
                name: None,
            }],
            temperature: Some(0.1),
            max_tokens: Some(2000),
            stream: false,
        };

        let response = self.client.chat(request).await?;
        let content = response.choices.first().map(|c| c.message.content.clone()).unwrap_or_default();

        let findings: Vec<LargeLlmFinding> = serde_json::from_str(&content).unwrap_or_default();
        Ok(findings)
    }
}

/// Large LLM critic for complex logic analysis.
///
/// @brief Complex logic and algorithm critic
/// @group AI Review
/// @since 0.1.0
// #[derive(Debug, Clone)]
pub struct LogicCritic {
    client: Box<dyn LlmClient>,
}

impl LogicCritic {
    /// Create new logic critic.
    ///
    /// @param client LLM client
    /// @return LogicCritic instance
    /// @since 0.1.0
    pub fn new(client: Box<dyn LlmClient>) -> Self {
        Self { client }
    }

    /// Review code for logic issues.
    ///
    /// @param code Source code to review
    /// @return Vector of logic findings
    /// @since 0.1.0
    pub async fn review(&self, code: &str) -> Result<Vec<LargeLlmFinding>> {
        let prompt = format!(
            "Review the following code for logic issues (off-by-one, edge cases, race conditions, \
             infinite loops, incorrect algorithms, boundary conditions). \
             Return findings as JSON array with fields: critic, finding, confidence (0.0-1.0), suggested_fix.\n\n{}",
            code
        );

        let request = crate::xiaoyi::llm::client::ChatRequest {
            model: "gpt-4".to_string(),
            messages: vec![crate::xiaoyi::llm::client::ChatMessage {
                role: crate::xiaoyi::llm::client::MessageRole::User,
                content: prompt,
                name: None,
            }],
            temperature: Some(0.1),
            max_tokens: Some(2000),
            stream: false,
        };

        let response = self.client.chat(request).await?;
        let content = response.choices.first().map(|c| c.message.content.clone()).unwrap_or_default();

        let findings: Vec<LargeLlmFinding> = serde_json::from_str(&content).unwrap_or_default();
        Ok(findings)
    }
}