//! @module knowledge::vector::store::in_memory
//! @brief In-memory vector store implementation.
//! @group Knowledge
//! @since 0.1.0
//! @author Miruamel

use crate::xiaoyi::knowledge::vector::index::SearchResult;
use crate::xiaoyi::knowledge::vector::store::VectorStore;
use crate::xiaoyi::core::error::Result;
use indexmap::IndexMap;
use parking_lot::RwLock;
use async_trait::async_trait;
use std::sync::Arc;

/// In-memory vector store providing fast local access to vector embeddings.
#[derive(Debug, Clone, Default)]
pub struct InMemoryVectorStore {
    data: Arc<RwLock<IndexMap<String, (Vec<f32>, Vec<u8>)>>>,
}

impl InMemoryVectorStore {
    /// Creates a new empty in-memory vector store.
    pub fn new() -> Self {
        Self {
            data: Arc::new(RwLock::new(IndexMap::new())),
        }
    }
}

#[async_trait]
impl VectorStore for InMemoryVectorStore {
    async fn upsert(&self, id: String, vector: Vec<f32>, payload: Vec<u8>) -> Result<()> {
        let mut data = self.data.write();
        data.insert(id, (vector, payload));
        Ok(())
    }

    async fn query(&self, query_vector: Vec<f32>, top_k: usize) -> Result<Vec<SearchResult>> {
        let data = self.data.read();
        let mut results = Vec::new();

        for (id, (stored_vector, payload)) in data.iter() {
            let score = cosine(&query_vector, stored_vector);
            results.push(SearchResult {
                id: id.clone(),
                score,
                payload: payload.clone(),
            });
        }

        results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));
        let results = if results.len() > top_k { results[..top_k].to_vec() } else { results };

        Ok(results)
    }

    async fn delete(&self, id: &str) -> Result<()> {
        let mut data = self.data.write();
        data.shift_remove(id);
        Ok(())
    }

    async fn len(&self) -> usize {
        let data = self.data.read();
        data.len()
    }
}

fn cosine(a: &[f32], b: &[f32]) -> f32 {
    if a.len() != b.len() {
        return 0.0;
    }

    let dot_product: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
    let norm_a: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
    let norm_b: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();

    if norm_a == 0.0 || norm_b == 0.0 {
        0.0
    } else {
        dot_product / (norm_a * norm_b)
    }
}