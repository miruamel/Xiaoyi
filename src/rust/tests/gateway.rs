//! # Gateway Tests
//!
//! Tests for `xiaoyi::gateway::Gateway`.
//!
//! @module tests::gateway
//! @brief Unit tests for gateway
//! @group User Interface
//! @since 0.1.0
//! @author Miruamel
//! @see crate::gateway

use pretty_assertions::assert_eq;
use tokio_test::block_on;
use xiaoyi::{Gateway, Config};

#[test]
fn test_gateway_new() {
    let config = Config::default();
    let gateway = Gateway::new(config);
    let _ = gateway; // Verify it compiles
}

#[tokio::test]
async fn test_gateway_start() {
    let config = Config::default();
    let gateway = Gateway::new(config);
    let result = gateway.start().await;
    // Should start without error (even if no servers actually bind)
    assert!(result.is_ok());
}

#[test]
fn test_gateway_clone() {
    let config = Config::default();
    let gateway = Gateway::new(config);
    let cloned = gateway.clone();
    let _ = cloned;
}

#[test]
fn test_gateway_debug() {
    let gateway = Gateway::new(Config::default());
    let debug = format!("{:?}", gateway);
    assert!(debug.contains("Gateway"));
}