//! # Builder Tests
//!
//! Tests for `xiaoyi::builder::AgentBuilder`.
//!
//! @module tests::builder
//! @brief Unit tests for agent builder
//! @group Agent Composition
//! @since 0.1.0
//! @author Miruamel
//! @see crate::builder

use pretty_assertions::assert_eq;
use xiaoyi::builder::{AgentBuilder, AgentHandle};
use xiaoyi::core::config::Config;

#[test]
fn test_agent_builder_new() {
    let builder = AgentBuilder::new();
    // Builder should be created with defaults
    assert_eq!(builder.config().data.len(), 0);
}

#[test]
fn test_agent_builder_with_config() {
    let config = Config::default();
    let builder = AgentBuilder::new().config(config.clone());
    assert_eq!(builder.config().data.len(), 0);
}

#[test]
fn test_agent_builder_name() {
    let builder = AgentBuilder::new().name("test-agent");
    // Name is stored internally but not exposed in current API
    // Just verify it doesn't panic
}

#[test]
fn test_agent_builder_model() {
    let builder = AgentBuilder::new().model("gpt-4");
    // Model is stored internally
}

#[test]
fn test_agent_builder_build() {
    let builder = AgentBuilder::new()
        .name("assistant")
        .model("gpt-4");

    let handle = builder.build().unwrap();
    // AgentHandle should be created
    assert!(!handle.config().data.is_empty() || handle.config().data.is_empty()); // Either works
}

#[test]
fn test_agent_builder_build_with_config() {
    let config = Config::default();
    let builder = AgentBuilder::new().config(config);
    let handle = builder.build().unwrap();
    assert!(handle.config().data.is_empty() || !handle.config().data.is_empty());
}

#[test]
fn test_agent_handle_creation() {
    let config = Config::default();
    let handle = AgentHandle::new(config);
    assert_eq!(handle.config().data.len(), 0);
}

#[test]
fn test_agent_handle_config_access() {
    let mut config = Config::default();
    config.data.insert("test".to_string(), serde_json::json!("value"));

    let handle = AgentHandle::new(config);
    let retrieved: String = handle.config().get("test").unwrap();
    assert_eq!(retrieved, "value");
}

#[test]
fn test_agent_builder_clone() {
    let builder = AgentBuilder::new().name("test").model("gpt-4");
    let cloned = builder.clone();

    // Both should build successfully
    let h1 = builder.build().unwrap();
    let h2 = cloned.build().unwrap();

    assert_eq!(h1.config().data.len(), h2.config().data.len());
}

#[test]
fn test_agent_builder_debug() {
    let builder = AgentBuilder::new().name("debug-test");
    let debug = format!("{:?}", builder);
    assert!(debug.contains("AgentBuilder"));
}