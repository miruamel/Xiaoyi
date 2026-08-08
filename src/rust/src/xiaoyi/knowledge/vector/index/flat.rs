//! @module knowledge::vector::index::flat
//! @brief Flat brute-force vector index for similarity search.
//! @group Knowledge
//! @since 0.1.0
//! @author Miruamel

use crate::xiaoyi::knowledge::vector::index::SearchResult;
use parking_lot::RwLock;

/// Flat vector index storing vectors and payloads in memory for fast similarity search.
#[derive(Debug, Default)]
pub struct FlatIndex {
    data: RwLock<Vec<(String, Vec<f32>, Vec<u8>)>>,
}

impl FlatIndex {
    /// Creates a new empty flat index.
    pub fn new() -> Self {
        Self { data: RwLock::new(Vec::new()) }
    }

    /// Inserts a vector embedding with associated payload into the index.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier for the vector.
    /// * `vector` - Vector embedding to store.
    /// * `payload` - Associated metadata payload.
    pub fn insert(&self, id: String, vector: Vec<f32>, payload: Vec<u8>) {
        let mut data = self.data.write();
        data.push((id, vector, payload));
    }

    /// Searches for vectors similar to the query vector.
    ///
    /// # Arguments
    ///
    /// * `query` - Query vector embedding.
    /// * `top_k` - Number of top matches to return.
    ///
    /// Returns a vector of search results sorted by descending similarity score.
    pub fn search(&self, query: &[f32], top_k: usize) -> Vec<SearchResult> {
        let data = self.data.read();
        let mut results = Vec::new();

        for (id, vector, payload) in data.iter() {
            let score = cosine(query, vector);
            results.push(SearchResult {
                id: id.clone(),
                score,
                payload: payload.clone(),
            });
        }

        results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));
        let results = if results.len() > top_k { results[..top_k].to_vec() } else { results };

        results
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