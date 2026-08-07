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
use tokio_test::block_on;
use xiaoyi::core::config::{Config, ConfigBuilder, ConfigSource};
use xiaoyi::core::config::source::file::FileSource;
use xiaoyi::orchestrator::Orchestrator;
use xiaoyi::builder::AgentBuilder;
use xiaoyi::core::error::{XiaoyiError, ErrorKind, Result};
use xiaoyi::core::result::{Status, ResultExt};
use xiaoyi::memory::stm::cache::LruCache;

#[test]
fn test_config_to_orchestrator_to_builder() {
    // Build a config
    let config = Config::default();

    // Create orchestrator
    let orchestrator = Orchestrator::new(config);
    assert_eq!(orchestrator.config().data.len(), 0);

    // Build an agent
    let agent = AgentBuilder::new()
        .name("smoke-test")
        .model("gpt-4")
        .build()
        .unwrap();

    assert_eq!(agent.config().data.len(), 0);
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
    assert_eq!(err.kind(), ErrorKind::Config);
    assert_eq!(err.message(), "simulated failure");
    assert_eq!(err.meta().get("key"), Some(&"value".to_string()));

    // Test ResultExt recovery
    let recovered: Result<i32> = result.or_else(|_| Ok(42));
    assert_eq!(recovered.unwrap(), 42);
}

#[test]
fn test_status_error_conversion() {
    let err: XiaoyiError = Status::NotFound.into();
    assert_eq!(err.kind(), ErrorKind::Runtime);
    assert!(err.message().contains("NOT_FOUND"));

    let result: Result<()> = Err(err);
    assert!(result.is_err());
}

#[test]
fn test_lru_cache_integration() {
    let cache = LruCache::new(100);
    cache.insert("config:server:port".to_string(), "8080".to_string(), None);
    cache.insert("config:server:host".to_string(), "localhost".to_string(), None);

    let port: Option<String> = cache.get("config:server:port");
    assert_eq!(port, Some("8080".to_string()));

    let host: Option<String> = cache.get("config:server:host");
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

    let mut builder = ConfigBuilder::new();
    builder = builder.add_source(FileSource::new(file_path.to_str().unwrap()));
    let config = block_on(builder.build()).unwrap();

    let name: String = config.get("agent.name").unwrap();
    assert_eq!(name, "smoke");

    let model: String = config.get("agent.model").unwrap();
    assert_eq!(model, "gpt-4");

    let temp: f64 = config.get("agent.temperature").unwrap();
    assert!((temp - 0.5).abs() < f64::EPSILON);
}

#[test]
fn test_full_stack_types_compilation() {
    // This test ensures all public types can be used together
    use xiaoyi::core::config::Config;
    use xiaoyi::core::error::{XiaoyiError, ErrorKind, Result};
    use xiaoyi::core::result::{Status, ResultExt};
    use xiaoyi::memory::stm::cache::LruCache;
    use xiaoyi::workflow::dag::graph::{DagGraph, DagNode, DagEdge, NodeId, NodeKind, EdgeKind};
    use xiaoyi::domain::token::{PrimitiveKind, IntKind, IntWidth, SyntaxKind};
    use xiaoyi::builder::{AgentBuilder, AgentHandle};
    use xiaoyi::orchestrator::Orchestrator;
    use xiaoyi::llm::client::{MessageRole, ChatMessage, ChatRequest};

    let _config = Config::default();
    let _err = XiaoyiError::new(ErrorKind::Config, "test");
    let _result: Result<()> = Ok(());
    let _status = Status::Ok;
    let _cache = LruCache::new(10);
    let _graph = DagGraph::new();
    let _node = DagNode::new(NodeId("n".into()), "N", NodeKind::Task);
    let _edge = DagEdge::new(NodeId("a".into()), NodeId("b".into()), EdgeKind::Sequential);
    let _pk = PrimitiveKind::Int;
    let _ik = IntKind::Signed;
    let _iw = IntWidth::I32;
    let _sk = SyntaxKind::Keyword;
    let _builder = AgentBuilder::new();
    let _handle = AgentHandle::new(Config::default());
    let _orch = Orchestrator::new(Config::default());
    let _role = MessageRole::User;
    let _msg = ChatMessage { role: MessageRole::User, content: "".into(), name: None };
    let _req = ChatRequest { model: "".into(), messages: vec![], temperature: None, max_tokens: None, stream: false };

    // If this compiles, the types work together
    assert!(true);
}

#[test]
fn test_dag_with_config_integration() {
    let mut graph = DagGraph::new();
    graph.add_node(DagNode::new(NodeId("load-config".into()), "Load Config", NodeKind::Task));
    graph.add_node(DagNode::new(NodeId("validate".into()), "Validate", NodeKind::Task));
    graph.add_node(DagNode::new(NodeId("execute".into()), "Execute", NodeKind::Task));

    graph.add_edge(DagEdge::new(NodeId("load-config".into()), NodeId("validate".into()), EdgeKind::Sequential)).unwrap();
    graph.add_edge(DagEdge::new(NodeId("validate".into()), NodeId("execute".into()), EdgeKind::Sequential)).unwrap();

    let order = graph.topological_order().unwrap();
    assert_eq!(order.len(), 3);
}