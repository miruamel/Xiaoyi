//! # Semantic Cache
//!
//! `cache` provides vector DB-backed semantic caching for critic
//! findings to avoid re-analyzing similar code.
//!
//! Path: `xiaoyi::critic::cache`
//!
//! @module critic::cache
//! @brief Semantic cache with vector DB and embedding similarity
//! @group AI Review
//! @since 0.1.0
//! @author Miruamel
//! @see crate::critic
//! @see crate::memory

use crate::xiaoyi::critic::ReviewResult;

/// Cached review entry.
///
/// @brief Cached review with embedding
/// @group AI Review
/// @since 0.1.0
#[derive(Debug, Clone)]
pub struct CachedReview {
    /// Code hash for exact matching
    pub code_hash: String,
    /// Embedding vector
    pub embedding: Vec<f32>,
    /// Review result
    pub result: ReviewResult,
    /// Timestamp
    pub timestamp: std::time::SystemTime,
}

/// Semantic cache for critic results.
///
/// @brief Vector DB-backed semantic cache
/// @group AI Review
/// @since 0.1.0
#[derive(Debug, Clone)]
pub struct SemanticCache {
    /// Cache entries
    entries: Vec<CachedReview>,
    /// Similarity threshold (0.0 - 1.0)
    threshold: f32,
    /// Max entries
    max_entries: usize,
}

impl SemanticCache {
    /// Create new semantic cache.
    ///
    /// @param threshold Similarity threshold for cache hits
    /// @param max_entries Maximum cache entries
    /// @return SemanticCache instance
    /// @since 0.1.0
    pub fn new(threshold: f32, max_entries: usize) -> Self {
        Self {
            entries: Vec::new(),
            threshold: threshold.clamp(0.0, 1.0),
            max_entries,
        }
    }

    /// Get cached review if similar code exists.
    ///
    /// @param code Source code to check
    /// @param embedding Code embedding vector
    /// @return Cached result if similar code found
    /// @since 0.1.0
    pub fn get(&self, _code: &str, embedding: &[f32]) -> Option<&ReviewResult> {
        let mut best_match: Option<&CachedReview> = None;
        let mut best_similarity = self.threshold;

        for entry in &self.entries {
            let similarity = cosine_similarity(embedding, &entry.embedding);
            if similarity > best_similarity {
                best_similarity = similarity;
                best_match = Some(entry);
            }
        }

        best_match.map(|e| &e.result)
    }

    /// Store review result in cache.
    ///
    /// @param code Source code
    /// @param embedding Code embedding vector
    /// @param result Review result to cache
    /// @since 0.1.0
    pub fn put(&mut self, code: &str, embedding: Vec<f32>, result: ReviewResult) {
        // Remove oldest if at capacity
        if self.entries.len() >= self.max_entries {
            self.entries.remove(0);
        }

        // Compute code hash
        let code_hash = format!("{:x}", md5::compute(code));

        self.entries.push(CachedReview {
            code_hash,
            embedding,
            result,
            timestamp: std::time::SystemTime::now(),
        });
    }

    /// Clear cache.
    ///
    /// @since 0.1.0
    pub fn clear(&mut self) {
        self.entries.clear();
    }

    /// Get cache size.
    ///
    /// @return Number of entries
    /// @since 0.1.0
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Check if cache is empty.
    ///
    /// @return true if empty
    /// @since 0.1.0
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

impl Default for SemanticCache {
    fn default() -> Self {
        Self::new(0.85, 1000)
    }
}

/// Compute cosine similarity between two vectors.
///
/// @param a First vector
/// @param b Second vector
/// @return Cosine similarity (0.0 - 1.0)
/// @since 0.1.0
fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    if a.len() != b.len() || a.is_empty() {
        return 0.0;
    }

    let dot: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
    let norm_a: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
    let norm_b: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();

    if norm_a == 0.0 || norm_b == 0.0 {
        0.0
    } else {
        (dot / (norm_a * norm_b)).clamp(0.0, 1.0)
    }
}
