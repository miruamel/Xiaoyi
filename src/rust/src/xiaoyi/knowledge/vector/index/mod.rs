//! @module knowledge::vector::index
//! @brief Index implementations for vector search.
//! @group Knowledge
//! @since 0.1.0
//! @author Miruamel

/// Search result containing vector ID, similarity score, and payload data.
#[derive(Debug, Clone, Default)]
pub struct SearchResult {
    pub id: String,
    pub score: f32,
    pub payload: Vec<u8>,
}

pub mod flat;

/// Flat brute-force vector index for similarity search.
pub use flat::FlatIndex;