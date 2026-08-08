//! @module knowledge::vector::store
//! @brief Vector storage implementations for knowledge indexing.
//! @group Knowledge
//! @since 0.1.0
//! @author Miruamel

use crate::xiaoyi::core::error::Result;
use async_trait::async_trait;

pub mod in_memory;
pub mod remote;

/// Vector storage trait for managing vector embeddings and associated payloads.
#[async_trait]
pub trait VectorStore: Send + Sync {
    /// Stores or replaces a vector embedding with associated metadata.
    async fn upsert(&self, id: String, vector: Vec<f32>, payload: Vec<u8>) -> Result<()>;

    /// Queries stored vectors by similarity to the query vector, returning top_k matches.
    async fn query(
        &self,
        vector: Vec<f32>,
        top_k: usize,
    ) -> Result<Vec<crate::xiaoyi::knowledge::vector::index::SearchResult>>;

    /// Deletes a vector embedding by its identifier.
    async fn delete(&self, id: &str) -> Result<()>;

    /// Returns the number of vector embeddings stored.
    async fn len(&self) -> usize;
}

pub use in_memory::InMemoryVectorStore;
