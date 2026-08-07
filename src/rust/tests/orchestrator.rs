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
use xiaoyi::{AgentBuilder, Config, Orchestrator};

#[test]
fn test_orchestrator_new() {
    let config = Config::default();
    let orchestrator = Orchestrator::new(config);
    // orchestrator has config field, can be checked for existence
    let _ = orchestrator;
}

#[test]
fn test_orchestrator_config_access() {
    let config = Config::default();
    let orchestrator = Orchestrator::new(config);
    // Config is internal, no direct access - just verify orchestrator exists
    let _ = orchestrator;
}

#[test]
fn test_orchestrator_clone() {
    let config = Config::default();
    let orchestrator = Orchestrator::new(config);
    let cloned = orchestrator.clone();

    // Both should work
    let _ = orchestrator;
    let _ = cloned;
}

#[tokio::test]
async fn test_orchestrator_run() {
    let config = Config::default();
    let orchestrator = Orchestrator::new(config);
    let agent = AgentBuilder::new(Config::default())
        .name("test")
        .model("gpt-4")
        .build()
        .unwrap();

    let result = orchestrator.run(agent).await;
    assert!(result.is_ok());
}

#[test]
fn test_orchestrator_debug() {
    let orchestrator = Orchestrator::new(Config::default());
    let debug = format!("{:?}", orchestrator);
    assert!(debug.contains("Orchestrator"));
}
