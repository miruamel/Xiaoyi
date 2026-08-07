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
use xiaoyi::{AgentBuilder, Config};

#[test]
fn test_agent_builder_new() {
    let builder = AgentBuilder::new(Config::default());
}

#[test]
fn test_agent_builder_name() {
    let builder = AgentBuilder::new(Config::default()).name("test");
}

#[test]
fn test_agent_builder_model() {
    let builder = AgentBuilder::new(Config::default()).model("gpt-4");
}

#[test]
fn test_agent_builder_build() {
    let builder = AgentBuilder::new(Config::default())
        .name("assistant")
        .model("gpt-4");
    let handle = builder.build().unwrap();
    // AgentHandle has name and model fields per public API
    assert_eq!(handle.name, "assistant");
    assert_eq!(handle.model, "gpt-4");
}
