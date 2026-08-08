//! # LLM Client Tests
//!
//! Tests for `xiaoyi::llm::client` types and traits.
//!
//! @module tests::llm
//! @brief Unit tests for LLM client
//! @group AI Runtime
//! @since 0.1.0
//! @author Miruamel
//! @see crate::llm::client

use pretty_assertions::assert_eq;
use xiaoyi::xiaoyi::llm::client::{ChatChoice, LlmClient};
use xiaoyi::{ChatMessage, ChatRequest, ChatResponse, MessageRole, Usage};

#[test]
fn test_message_role_variants() {
    assert_eq!(MessageRole::System, MessageRole::System);
    assert_eq!(MessageRole::User, MessageRole::User);
    assert_eq!(MessageRole::Assistant, MessageRole::Assistant);
    assert_eq!(MessageRole::Tool, MessageRole::Tool);

    assert_ne!(MessageRole::User, MessageRole::Assistant);
}

#[test]
fn test_message_role_serialization() {
    let role = MessageRole::User;
    let json = serde_json::to_string(&role).unwrap();
    assert_eq!(json, "\"user\"");

    let role = MessageRole::Assistant;
    let json = serde_json::to_string(&role).unwrap();
    assert_eq!(json, "\"assistant\"");

    let role = MessageRole::System;
    let json = serde_json::to_string(&role).unwrap();
    assert_eq!(json, "\"system\"");

    let role = MessageRole::Tool;
    let json = serde_json::to_string(&role).unwrap();
    assert_eq!(json, "\"tool\"");
}

#[test]
fn test_message_role_deserialization() {
    let role: MessageRole = serde_json::from_str("\"user\"").unwrap();
    assert_eq!(role, MessageRole::User);

    let role: MessageRole = serde_json::from_str("\"assistant\"").unwrap();
    assert_eq!(role, MessageRole::Assistant);
}

#[test]
fn test_chat_message_creation() {
    let msg = ChatMessage {
        role: MessageRole::User,
        content: "Hello, world!".into(),
        name: None,
    };
    assert_eq!(msg.role, MessageRole::User);
    assert_eq!(msg.content, "Hello, world!");
    assert_eq!(msg.name, None);
}

#[test]
fn test_chat_message_with_name() {
    let msg = ChatMessage {
        role: MessageRole::Assistant,
        content: "Hi there!".into(),
        name: Some("assistant".into()),
    };
    assert_eq!(msg.name, Some("assistant".into()));
}

#[test]
fn test_chat_message_serialization() {
    let msg = ChatMessage {
        role: MessageRole::User,
        content: "Test".into(),
        name: None,
    };
    let json = serde_json::to_string(&msg).unwrap();
    assert!(json.contains("user"));
    assert!(json.contains("Test"));
}

#[test]
fn test_chat_request_creation() {
    let request = ChatRequest {
        model: "gpt-4o-mini".into(),
        messages: vec![ChatMessage {
            role: MessageRole::User,
            content: "Hello!".into(),
            name: None,
        }],
        temperature: Some(0.7),
        max_tokens: Some(100),
        stream: false,
    };
    assert_eq!(request.model, "gpt-4o-mini");
    assert_eq!(request.messages.len(), 1);
    assert_eq!(request.temperature, Some(0.7));
    assert_eq!(request.max_tokens, Some(100));
    assert!(!request.stream);
}

#[test]
fn test_chat_request_defaults() {
    let request = ChatRequest {
        model: "test-model".into(),
        messages: vec![],
        temperature: None,
        max_tokens: None,
        stream: false,
    };
    assert_eq!(request.temperature, None);
    assert_eq!(request.max_tokens, None);
}

#[test]
fn test_chat_choice_creation() {
    let choice = ChatChoice {
        index: 0,
        message: ChatMessage {
            role: MessageRole::Assistant,
            content: "Response".into(),
            name: None,
        },
        finish_reason: Some("stop".into()),
    };
    assert_eq!(choice.index, 0);
    assert_eq!(choice.message.content, "Response");
    assert_eq!(choice.finish_reason, Some("stop".into()));
}

#[test]
fn test_usage_creation() {
    let usage = Usage {
        prompt_tokens: 10,
        completion_tokens: 20,
        total_tokens: 30,
    };
    assert_eq!(usage.prompt_tokens, 10);
    assert_eq!(usage.completion_tokens, 20);
    assert_eq!(usage.total_tokens, 30);
}

#[test]
fn test_chat_response_creation() {
    let response = ChatResponse {
        id: "chatcmpl-123".into(),
        model: "gpt-4o-mini".into(),
        choices: vec![ChatChoice {
            index: 0,
            message: ChatMessage {
                role: MessageRole::Assistant,
                content: "Hello!".into(),
                name: None,
            },
            finish_reason: Some("stop".into()),
        }],
        usage: Some(Usage {
            prompt_tokens: 5,
            completion_tokens: 10,
            total_tokens: 15,
        }),
    };
    assert_eq!(response.id, "chatcmpl-123");
    assert_eq!(response.choices.len(), 1);
    assert_eq!(response.usage.as_ref().unwrap().total_tokens, 15);
}

#[test]
fn test_llm_client_trait_object_safety() {
    // Verify LlmClient is object-safe (can be used as dyn LlmClient)
    fn _accepts_client(_client: &dyn LlmClient) {}
    // This compiles if LlmClient is object-safe
}
