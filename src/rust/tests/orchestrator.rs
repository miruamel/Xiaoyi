//! # Orchestrator Tests
//!
//! Tests for `xiaoyi::orchestrator::Orchestrator`.
//!
//! @module tests::orchestrator
//! @brief Unit tests for orchestrator
//! @group Agent Runtime
//! @since 0.1.0
//! @author Miruamel
//! @see crate::orchestrator

use pretty_assertions::assert_eq;
use tokio_test::block_on;
use xiaoyi::orchestrator::Orchestrator;
use xiaoyi::builder::AgentBuilder;
use xiaoyi::core::config::Config;

#[test]
fn test_orchestrator_new() {
    let config = Config::default();
    let orchestrator = Orchestrator::new(config);
    assert_eq!(orchestrator.config().data.len(), 0);
}

#[test]
fn test_orchestrator_config_access() {
    let mut config = Config::default();
    config.data.insert("key".to_string(), serde_json::json!("value"));

    let orchestrator = Orchestrator::new(config);
    let retrieved: String = orchestrator.config().get("key").unwrap();
    assert_eq!(retrieved, "value");
}

#[tokio::test]
async fn test_orchestrator_run() {
    let config = Config::default();
    let orchestrator = Orchestrator::new(config);
    let agent = AgentBuilder::new().name("test").model("gpt-4").build().unwrap();

    let result = orchestrator.run(agent).await;
    assert!(result.is_ok());
}

#[test]
fn test_orchestrator_clone() {
    let config = Config::default();
    let orchestrator = Orchestrator::new(config);
    let cloned = orchestrator.clone();

    assert_eq!(orchestrator.config().data.len(), cloned.config().data.len());
}

#[test]
fn test_orchestrator_debug() {
    let orchestrator = Orchestrator::new(Config::default());
    let debug = format!("{:?}", orchestrator);
    assert!(debug.contains("Orchestrator"));
}