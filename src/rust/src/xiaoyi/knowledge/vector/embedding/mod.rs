//! @module knowledge::vector::embedding
//! @brief Embedding providers for vector generation.
//! @group Knowledge
//! @since 0.1.0
//! @author Miruamel

use async_trait::async_trait;

pub mod local;

/// Embedding provider trait for generating vector embeddings from text.
#[async_trait]
pub trait EmbeddingProvider: Send + Sync {
    /// Returns the dimensionality of embeddings produced by this provider.
    fn dim(&self) -> usize;

    /// Generates a normalized vector embedding for the given input text.
    async fn embed(&self, text: &str) -> Result<Vec<f32>>;
}
use crate::xiaoyi::core::error::Result;

pub use local::LocalEmbeddingProvider;
