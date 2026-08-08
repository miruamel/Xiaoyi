//! @module knowledge::vector::embedding::local
//! @brief Local deterministic embedding provider using SHA256 hashing.
//! @group Knowledge
//! @since 0.1.0
//! @author Miruamel

use crate::xiaoyi::core::error::Result;
use crate::xiaoyi::knowledge::vector::embedding::EmbeddingProvider;
use async_trait::async_trait;
use sha2::{Digest, Sha256};
use std::collections::HashMap;

/// Local embedding provider with fixed dimensionality and deterministic hashing.
#[derive(Debug, Clone)]
pub struct LocalEmbeddingProvider {
    /// Vector dimensionality.
    dim: usize,
}

impl LocalEmbeddingProvider {
    /// Creates a new local embedding provider with the specified dimensionality.
    ///
    /// # Arguments
    ///
    /// * `dim` - Target dimensionality for generated embeddings.
    pub fn new(dim: usize) -> Self {
        Self { dim }
    }
}
impl Default for LocalEmbeddingProvider {
    fn default() -> Self {
        Self::new(256)
    }
}

#[async_trait]
impl EmbeddingProvider for LocalEmbeddingProvider {
    fn dim(&self) -> usize {
        self.dim
    }

    async fn embed(&self, text: &str) -> Result<Vec<f32>> {
        let mut weights: HashMap<usize, f32> = HashMap::new();

        for word in text
            .to_lowercase()
            .split(|c: char| !c.is_alphanumeric())
            .filter(|w| !w.is_empty())
        {
            let mut hasher = Sha256::new();
            hasher.update(word.as_bytes());
            let hash = hasher.finalize();
            let hash_val = u64::from_le_bytes(hash[..8].try_into().unwrap()) as usize;
            let bucket = hash_val % self.dim;
            *weights.entry(bucket).or_insert(0.0) += 1.0;
        }

        let mut vector = vec![0.0f32; self.dim];
        for (i, weight) in weights {
            vector[i] = weight;
        }

        let norm = vector.iter().map(|v| v * v).sum::<f32>().sqrt();
        if norm > 0.0 {
            for v in &mut vector {
                *v /= norm;
            }
        }

        Ok(vector)
    }
}
