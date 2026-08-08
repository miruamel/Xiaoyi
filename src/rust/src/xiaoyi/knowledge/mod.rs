//! # Layer 8 — Knowledge & Tools
//!
//! `knowledge` is the long-term, cross-agent memory and capability substrate of the
//! Xiaoyi autonomous agent. It unifies four deep-vertical concerns:
//!
//! - **Vector store** ([`vector`]) — local embeddings ([`vector::embedding`]) and an in-memory
//!   similarity index ([`vector::index`]) implementing the [`vector::store::VectorStore`] trait.
//! - **AST graph** ([`graph`]) — a git-native repository graph ([`graph::repo`]) backed by a
//!   typed [`graph::ast_graph::AstGraph`] for dependency / cycle analysis.
//! - **Tool registry** ([`tools`]) — a plugin registry ([`tools::registry`]) and OpenAPI/Schema
//!   store ([`tools::openapi`]) describing callable capabilities.
//! - **Retrieval** ([`retrieval`]) — a RAG pipeline ([`retrieval::rag`]) composing embeddings +
//!   vector store for semantic search.
//!
//! The [`KnowledgeBase`] facade wires every slice into a single addressable service so the
//! orchestrator (Layer 2) can index documents, register tools, and introspect repositories.
//!
//! Path: `xiaoyi::knowledge`
//!
//! - Layer 8: `knowledge` — Knowledge & Tools.
//!
//! @module knowledge
//! @brief Knowledge & Tools substrate: vector store, AST graph, tool registry, retrieval
//! @group Knowledge
//! @since 0.1.0
//! @author Miruamel
//! @see crate::orchestrator
//! @see crate::memory

pub mod graph;
pub mod retrieval;
pub mod tools;
pub mod vector;

use crate::xiaoyi::core::error::Result;
use crate::xiaoyi::knowledge::graph::ast_graph::AstGraph;
use crate::xiaoyi::knowledge::graph::repo::RepoScanner;
use crate::xiaoyi::knowledge::retrieval::rag::{RetrievalPipeline, RetrievalResult};
use crate::xiaoyi::knowledge::tools::openapi::OpenApiStore;
use crate::xiaoyi::knowledge::tools::registry::{ToolPlugin, ToolRegistry};
use crate::xiaoyi::knowledge::vector::store::VectorStore;
use crate::xiaoyi::knowledge::vector::{InMemoryVectorStore, LocalEmbeddingProvider};
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use std::path::Path;

/// Configuration for a [`KnowledgeBase`] instance.
///
/// @brief Tunable parameters for the knowledge substrate
/// @group Knowledge
/// @since 0.1.0
/// @author Miruamel
/// @see KnowledgeBase::new
#[derive(Debug, Clone)]
pub struct KnowledgeConfig {
    /// Dimensionality requested for the embedding space.
    ///
    /// @brief Requested embedding dimensionality
    /// @since 0.1.0
    pub embedding_dim: usize,
    /// Character window used when chunking documents for retrieval.
    ///
    /// @brief Retrieval chunk size in characters
    /// @since 0.1.0
    pub chunk_size: usize,
}

impl Default for KnowledgeConfig {
    fn default() -> Self {
        Self {
            embedding_dim: 256,
            chunk_size: 512,
        }
    }
}

/// Classification of a [`KnowledgeEntry`].
///
/// @brief Discriminator for knowledge registry entries
/// @group Knowledge
/// @since 0.1.0
/// @author Miruamel
/// @see KnowledgeEntry
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EntryKind {
    /// Vector-indexed document chunk.
    ///
    /// @brief Semantic vector entry
    /// @since 0.1.0
    Vector,
    /// Repository AST graph node/edge.
    ///
    /// @brief Structural graph entry
    /// @since 0.1.0
    Graph,
    /// Registered callable tool/plugin.
    ///
    /// @brief Tool/plugin entry
    /// @since 0.1.0
    Tool,
    /// OpenAPI/Schema descriptor.
    ///
    /// @brief API schema entry
    /// @since 0.1.0
    Api,
}

/// A single registry entry describing an indexed artefact.
///
/// @brief Metadata record for an indexed knowledge artefact
/// @group Knowledge
/// @since 0.1.0
/// @author Miruamel
/// @see KnowledgeBase
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeEntry {
    /// Stable identifier of the artefact.
    ///
    /// @brief Entry identifier
    /// @since 0.1.0
    pub id: String,
    /// Category of the artefact.
    ///
    /// @brief Entry category
    /// @since 0.1.0
    pub kind: EntryKind,
    /// Human-readable summary (e.g. first chunk or tool description).
    ///
    /// @brief Entry summary
    /// @since 0.1.0
    pub summary: String,
}

/// Aggregate statistics for a [`KnowledgeBase`].
///
/// @brief Snapshot counters for the knowledge substrate
/// @group Knowledge
/// @since 0.1.0
/// @author Miruamel
/// @see KnowledgeBase::stats
#[derive(Debug, Clone, Default)]
pub struct KnowledgeStats {
    /// Number of vectors stored.
    ///
    /// @brief Stored vector count
    /// @since 0.1.0
    pub vectors: usize,
    /// Number of registered tools.
    ///
    /// @brief Registered tool count
    /// @since 0.1.0
    pub tools: usize,
    /// Number of loaded API specs.
    ///
    /// @brief Loaded API spec count
    /// @since 0.1.0
    pub apis: usize,
    /// Number of nodes in the repository graph.
    ///
    /// @brief Graph node count
    /// @since 0.1.0
    pub graph_nodes: usize,
}

/// Unified knowledge & tools facade wiring every Layer 8 slice together.
///
/// @brief Single addressable service for indexing, retrieval, tools, and repo introspection
/// @group Knowledge
/// @since 0.1.0
/// @author Miruamel
/// @see KnowledgeConfig
/// @see crate::orchestrator::Orchestrator
pub struct KnowledgeBase {
    /// Active configuration.
    ///
    /// @brief Active configuration
    /// @since 0.1.0
    config: KnowledgeConfig,
    /// Backing vector store.
    ///
    /// @brief Vector store backend
    /// @since 0.1.0
    store: InMemoryVectorStore,
    /// Retrieval (RAG) pipeline.
    ///
    /// @brief Retrieval pipeline
    /// @since 0.1.0
    retrieval: RetrievalPipeline<LocalEmbeddingProvider, InMemoryVectorStore>,
    /// Tool/plugin registry.
    ///
    /// @brief Tool registry
    /// @since 0.1.0
    tools: ToolRegistry,
    /// OpenAPI/Schema store.
    ///
    /// @brief API schema store
    /// @since 0.1.0
    apis: OpenApiStore,
    /// Repository AST graph.
    ///
    /// @brief Repository graph
    /// @since 0.1.0
    graph: RwLock<AstGraph>,
}

impl KnowledgeBase {
    /// Construct a knowledge base from the supplied configuration.
    ///
    /// @param config Tunable parameters (embedding dim, chunk size).
    /// @return Fully wired [`KnowledgeBase`] ready for indexing and retrieval.
    /// @since 0.1.0
    /// @example
    /// ```rust
    /// use xiaoyi::knowledge::{KnowledgeBase, KnowledgeConfig};
    /// let kb = KnowledgeBase::new(KnowledgeConfig::default());
    /// ```
    /// @see KnowledgeConfig
    pub fn new(config: KnowledgeConfig) -> Self {
        let store = InMemoryVectorStore::new();
        let retrieval = RetrievalPipeline::new(LocalEmbeddingProvider::default(), store.clone())
            .with_chunk_size(config.chunk_size);
        Self {
            config,
            store,
            retrieval,
            tools: ToolRegistry::new(),
            apis: OpenApiStore::new(),
            graph: RwLock::new(AstGraph::new()),
        }
    }

    /// Construct a knowledge base with default configuration.
    ///
    /// @return Knowledge base using [`KnowledgeConfig::default`].
    /// @since 0.1.0
    pub fn new_default() -> Self {
        Self::new(KnowledgeConfig::default())
    }

    /// Embed and index a document for later semantic retrieval.
    ///
    /// @param doc_id Stable document identifier.
    /// @param text Document body to chunk and embed.
    /// @return [`Result::Ok`] on success.
    /// @throw XiaoyiError on embedding or storage failure.
    /// @since 0.1.0
    /// @see retrieve
    pub async fn index_document(&self, doc_id: &str, text: &str) -> Result<()> {
        self.retrieval.index(doc_id, text).await
    }

    /// Snapshot aggregate statistics for the knowledge substrate.
    ///
    /// @return [`KnowledgeStats`] with current counters.
    /// @since 0.1.0
    pub async fn stats(&self) -> KnowledgeStats {
        KnowledgeStats {
            vectors: self.store.len().await,
            tools: self.tools.list().len(),
            apis: self.apis.names().len(),
            graph_nodes: self.graph.read().nodes().len(),
        }
    }

    /// Retrieve the most semantically similar chunks for a query.
    ///
    /// @param query Natural-language query.
    /// @param top_k Maximum number of chunks to return.
    /// @return [`RetrievalResult`] with ranked chunks.
    /// @throw XiaoyiError on embedding or store failure.
    /// @since 0.1.0
    /// @see index_document
    pub async fn retrieve(&self, query: &str, top_k: usize) -> Result<RetrievalResult> {
        self.retrieval.retrieve(query, top_k).await
    }

    /// Register a callable tool/plugin.
    ///
    /// @param plugin Tool descriptor with handler.
    /// @return [`Result::Ok`] on success.
    /// @throw XiaoyiError if a tool with the same name is already registered.
    /// @since 0.1.0
    /// @see invoke_tool
    pub fn register_tool(&self, plugin: ToolPlugin) -> Result<()> {
        self.tools.register(plugin)
    }

    /// Invoke a previously registered tool by name.
    ///
    /// @param name Registered tool name.
    /// @param input JSON argument object.
    /// @return Tool output as JSON.
    /// @throw XiaoyiError if the tool is not registered or execution fails.
    /// @since 0.1.0
    /// @see register_tool
    pub fn invoke_tool(&self, name: &str, input: serde_json::Value) -> Result<serde_json::Value> {
        self.tools.invoke(name, input)
    }

    /// List the names of all registered tools.
    ///
    /// @return Vector of registered tool names.
    /// @since 0.1.0
    pub fn tool_names(&self) -> Vec<String> {
        self.tools.list()
    }

    /// Load an OpenAPI/Swagger specification document.
    ///
    /// @param name Logical spec name.
    /// @param json Raw OpenAPI JSON text.
    /// @return [`Result::Ok`] on success.
    /// @throw XiaoyiError on malformed JSON.
    /// @since 0.1.0
    /// @see endpoints
    pub fn load_api(&self, name: &str, json: &str) -> Result<()> {
        self.apis.load(name, json)
    }

    /// Enumerate (method, path) endpoints of a loaded API spec.
    ///
    /// @param name Logical spec name previously passed to [`KnowledgeBase::load_api`].
    /// @return Vector of (method, path) pairs.
    /// @since 0.1.0
    /// @see load_api
    pub fn endpoints(&self, name: &str) -> Vec<(String, String)> {
        self.apis.endpoints(name)
    }

    /// Scan a repository root and merge the resulting AST graph into the knowledge base.
    ///
    /// @param root Filesystem path to scan recursively.
    /// @return A clone of the merged [`AstGraph`].
    /// @throw XiaoyiError on I/O or parse failure.
    /// @since 0.1.0
    /// @see graph::repo::RepoScanner
    pub fn scan_repo(&self, root: &Path) -> Result<AstGraph> {
        let scanned = RepoScanner::new(root).scan()?;
        {
            let mut g = self.graph.write();
            *g = scanned.clone();
        }
        Ok(scanned)
    }

    /// Borrow the active configuration.
    ///
    /// @return Reference to the active [`KnowledgeConfig`].
    /// @since 0.1.0
    pub fn config(&self) -> &KnowledgeConfig {
        &self.config
    }
}
