//! # Config Module Tests
//!
//! Tests for `xiaoyi::core::config` configuration system.
//!
//! @module tests::config
//! @brief Unit tests for configuration system
//! @group Core Runtime
//! @since 0.1.0
//! @author Miruamel
//! @see crate::core::config

use pretty_assertions::assert_eq;
use std::collections::HashMap;
use std::path::Path;
use tokio_test::block_on;
use xiaoyi::core::config::{Config, ConfigBuilder, ConfigSource};
use xiaoyi::core::config::source::file::FileSource;
use xiaoyi::core::error::Result;

#[test]
fn test_config_builder_new() {
    let builder = ConfigBuilder::new();
    let config = block_on(builder.build()).unwrap();
    // Empty config should be valid
    assert!(!config.contains("any.key"));
}

#[test]
fn test_config_builder_add_source() {
    let mut builder = ConfigBuilder::new();
    // Add a file source (will fail if file doesn't exist, but we can test optional)
    builder = builder.add_source(FileSource::new("./nonexistent.toml").optional());
    let config = block_on(builder.build()).unwrap();
    assert!(!config.contains("any.key"));
}

#[test]
fn test_config_get_or_default() {
    let mut data = HashMap::new();
    data.insert("server.port".to_string(), serde_json::json!(8080));
    data.insert("server.host".to_string(), serde_json::json!("localhost"));

    let config = Config { data };

    let port: u16 = config.get_or_default("server.port", 3000);
    assert_eq!(port, 8080);

    let missing: u16 = config.get_or_default("server.missing", 9999);
    assert_eq!(missing, 9999);
}

#[test]
fn test_config_get_or() {
    let mut data = HashMap::new();
    data.insert("key".to_string(), serde_json::json!("value"));

    let config = Config { data };

    let result = config.get_or("key", || "default".to_string());
    assert_eq!(result, "value");

    let result = config.get_or("missing", || "default".to_string());
    assert_eq!(result, "default");
}

#[test]
fn test_config_contains() {
    let mut data = HashMap::new();
    data.insert("a.b".to_string(), serde_json::json!(1));

    let config = Config { data };

    assert!(config.contains("a.b"));
    assert!(!config.contains("a.c"));
    assert!(!config.contains("missing"));
}

#[test]
fn test_config_get_typed() {
    let mut data = HashMap::new();
    data.insert("num".to_string(), serde_json::json!(42));
    data.insert("str".to_string(), serde_json::json!("hello"));
    data.insert("bool".to_string(), serde_json::json!(true));
    data.insert("float".to_string(), serde_json::json!(3.14));
    data.insert("arr".to_string(), serde_json::json!([1, 2, 3]));
    data.insert("obj".to_string(), serde_json::json!({"nested": "value"}));

    let config = Config { data };

    let n: i64 = config.get("num").unwrap();
    assert_eq!(n, 42);

    let s: String = config.get("str").unwrap();
    assert_eq!(s, "hello");

    let b: bool = config.get("bool").unwrap();
    assert!(b);

    let f: f64 = config.get("float").unwrap();
    assert!((f - 3.14).abs() < f64::EPSILON);

    let arr: Vec<i64> = config.get("arr").unwrap();
    assert_eq!(arr, vec![1, 2, 3]);

    let obj: serde_json::Value = config.get("obj").unwrap();
    assert_eq!(obj["nested"], "value");
}

#[test]
fn test_config_get_missing_returns_error() {
    let config = Config::default();
    let result: Result<String> = config.get("missing");
    assert!(result.is_err());
}

#[tokio::test]
async fn test_file_source_toml() {
    // Create a temporary TOML file
    let temp_dir = tempfile::tempdir().unwrap();
    let file_path = temp_dir.path().join("config.toml");
    std::fs::write(&file_path, r#"
        [server]
        port = 8080
        host = "localhost"
    "#).unwrap();

    let source = FileSource::new(file_path.to_str().unwrap());
    let result = source.load().await;
    assert!(result.is_ok());

    let data = result.unwrap();
    assert_eq!(data.get("server.port"), Some(&serde_json::json!(8080)));
    assert_eq!(data.get("server.host"), Some(&serde_json::json!("localhost")));
}

#[tokio::test]
async fn test_file_source_json() {
    let temp_dir = tempfile::tempdir().unwrap();
    let file_path = temp_dir.path().join("config.json");
    std::fs::write(&file_path, r#"{"server": {"port": 3000, "host": "0.0.0.0"}}"#).unwrap();

    let source = FileSource::new(file_path.to_str().unwrap());
    let result = source.load().await;
    assert!(result.is_ok());

    let data = result.unwrap();
    assert_eq!(data.get("server.port"), Some(&serde_json::json!(3000)));
    assert_eq!(data.get("server.host"), Some(&serde_json::json!("0.0.0.0")));
}

#[tokio::test]
async fn test_file_source_yaml() {
    let temp_dir = tempfile::tempdir().unwrap();
    let file_path = temp_dir.path().join("config.yaml");
    std::fs::write(&file_path, r#"
        server:
          port: 9000
          host: "127.0.0.1"
    "#).unwrap();

    let source = FileSource::new(file_path.to_str().unwrap());
    let result = source.load().await;
    assert!(result.is_ok());

    let data = result.unwrap();
    assert_eq!(data.get("server.port"), Some(&serde_json::json!(9000)));
    assert_eq!(data.get("server.host"), Some(&serde_json::json!("127.0.0.1")));
}

#[tokio::test]
async fn test_file_source_missing_required() {
    let source = FileSource::new("/nonexistent/path/config.toml");
    let result = source.load().await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_file_source_missing_optional() {
    let source = FileSource::new("/nonexistent/path/config.toml").optional();
    let result = source.load().await;
    assert!(result.is_ok());
    assert!(result.unwrap().is_empty());
}

#[tokio::test]
async fn test_file_source_watch_noop() {
    let temp_dir = tempfile::tempdir().unwrap();
    let file_path = temp_dir.path().join("config.toml");
    std::fs::write(&file_path, "port = 8080").unwrap();

    let source = FileSource::new(file_path.to_str().unwrap());
    // Watch returns a channel that never fires (placeholder implementation)
    let _watcher = source.watch().await;
    // Just verify it doesn't panic
}

#[test]
fn test_config_builder_multiple_sources_override() {
    let temp_dir = tempfile::tempdir().unwrap();

    // First file with base config
    let file1 = temp_dir.path().join("base.toml");
    std::fs::write(&file1, "port = 8080\nkey = 'base'").unwrap();

    // Second file with override
    let file2 = temp_dir.path().join("override.toml");
    std::fs::write(&file2, "port = 9000").unwrap();

    let mut builder = ConfigBuilder::new();
    builder = builder.add_source(FileSource::new(file1.to_str().unwrap()));
    builder = builder.add_source(FileSource::new(file2.to_str().unwrap()));

    let config = block_on(builder.build()).unwrap();

    let port: i64 = config.get("port").unwrap();
    assert_eq!(port, 9000); // Second source overrides

    let key: String = config.get("key").unwrap();
    assert_eq!(key, "base"); // First source value preserved
}

#[test]
fn test_config_default() {
    let config = Config::default();
    assert!(!config.contains("anything"));
    assert_eq!(config.get_or_default("key", "default"), "default");
}