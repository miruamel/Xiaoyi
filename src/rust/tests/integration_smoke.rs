//! # Integration Smoke Tests
//!
//! Cross-module smoke tests that exercise the full stack.
//!
//! @module tests::integration_smoke
//! @brief Cross-module integration smoke tests
//! @group Integration
//! @since 0.1.0
//! @author Miruamel
//! @see crate::core::config
//! @see crate::orchestrator
//! @see crate::builder

use pretty_assertions::assert_eq;
use xiaoyi::{Config, ConfigBuilder, Result, XiaoyiError, ErrorKind, Status, ResultExt};
use xiaoyi::{FileSource, AsyncConfigSource, LruCache, DagGraph, DagNode, DagEdge};
use xiaoyi::workflow::dag::graph::{NodeId, NodeKind, EdgeKind};
use xiaoyi::{AgentBuilder, Orchestrator, Gateway, Lexer};
use xiaoyi::llm::client::{MessageRole, ChatMessage, ChatRequest};
use tempfile;

#[test]
fn test_config_to_orchestrator_to_builder() {
    // Build a config
    let config = Config::default();

    // Create orchestrator
    let orchestrator = Orchestrator::new(config);
    // Config is internal, just verify orchestrator exists
    let _ = orchestrator;

    // Build an agent
    let agent = AgentBuilder::new(Config::default())
        .name("smoke-test")
        .model("gpt-4")
        .build()
        .unwrap();

    assert_eq!(agent.name, "smoke-test");
    assert_eq!(agent.model, "gpt-4");
}

#[test]
fn test_error_result_workflow() {
    fn fallible_operation() -> Result<i32> {
        Err(XiaoyiError::new(ErrorKind::Config, "simulated failure")
            .with_meta("key", "value"))
    }

    let result = fallible_operation();
    assert!(result.is_err());

    let err = result.unwrap_err();
    assert_eq!(err.kind, ErrorKind::Config);
    assert_eq!(err.message, "simulated failure");
    assert_eq!(err.meta.iter().find(|(k, _)| k == "key").map(|(_, v)| v), Some(&"value".to_string()));

    // Test ResultExt recovery with a fresh result
    let recovered: Result<i32> = fallible_operation().or_else(|_| Ok(42));
    assert_eq!(recovered.unwrap(), 42);
}

#[test]
fn test_status_error_conversion() {
    let err: XiaoyiError = Status::NotFound.into();
    assert_eq!(err.kind, ErrorKind::Runtime);
    assert!(err.message.contains("NotFound"));

    let result: Result<()> = Err(err);
    assert!(result.is_err());
}

#[test]
fn test_lru_cache_integration() {
    let cache: LruCache<String, String> = LruCache::new(100);
    cache.insert("config:server:port".to_string(), "8080".to_string(), None);
    cache.insert("config:server:host".to_string(), "localhost".to_string(), None);

    let port: Option<String> = cache.get(&"config:server:port".to_string());
    assert_eq!(port, Some("8080".to_string()));

    let host: Option<String> = cache.get(&"config:server:host".to_string());
    assert_eq!(host, Some("localhost".to_string()));
}

#[test]
fn test_config_with_file_source() {
    let temp_dir = tempfile::tempdir().unwrap();
    let file_path = temp_dir.path().join("smoke.toml");
    std::fs::write(&file_path, r#"
        [agent]
        name = "smoke"
        model = "gpt-4"
        temperature = 0.5
    "#).unwrap();

    let source = FileSource::new(file_path.to_str().unwrap());
    let data = tokio_test::block_on(source.load()).unwrap();

    let name: String = data.get("agent.name").and_then(|v| v.as_str()).unwrap().to_string();
    assert_eq!(name, "smoke");

    let model: String = data.get("agent.model").and_then(|v| v.as_str()).unwrap().to_string();
    assert_eq!(model, "gpt-4");

    let temp: f64 = data.get("agent.temperature").and_then(|v| v.as_f64()).unwrap();
    assert!((temp - 0.5).abs() < f64::EPSILON);
}

#[test]
fn test_full_stack_types_compilation() {
    // This test ensures all public types can be used together
    let _config = Config::default();
    let _err = XiaoyiError::new(ErrorKind::Config, "test");
    let _result: Result<()> = Ok(());
    let _status = Status::Ok;
    let _cache: LruCache<String, String> = LruCache::new(10);
    let _graph = DagGraph::new();
    let _node = DagNode::new(NodeId::new("n"), "N", NodeKind::Task);
    let _edge = DagEdge::new(NodeId::new("a"), NodeId::new("b"), EdgeKind::Sequential);
    let _builder = AgentBuilder::new(Config::default());
    let _orch = Orchestrator::new(Config::default());
    let _gateway = Gateway::new(Config::default());
    let _lexer = Lexer::new("test");
    let _role = MessageRole::User;
    let _msg = ChatMessage { role: MessageRole::User, content: "".into(), name: None };
    let _req = ChatRequest { model: "".into(), messages: vec![], temperature: None, max_tokens: None, stream: false };

    // If this compiles, the types work together
    assert!(true);
}

#[test]
fn test_dag_with_config_integration() {
    let mut graph = DagGraph::new();
    graph.add_node(DagNode::new(NodeId::new("load-config"), "Load Config", NodeKind::Task));
    graph.add_node(DagNode::new(NodeId::new("validate"), "Validate", NodeKind::Task));
    graph.add_node(DagNode::new(NodeId::new("execute"), "Execute", NodeKind::Task));

    graph.add_edge(DagEdge::new(NodeId::new("load-config"), NodeId::new("validate"), EdgeKind::Sequential)).unwrap();
    graph.add_edge(DagEdge::new(NodeId::new("validate"), NodeId::new("execute"), EdgeKind::Sequential)).unwrap();

    let order = graph.topological_order().unwrap();
    assert_eq!(order.len(), 3);
}