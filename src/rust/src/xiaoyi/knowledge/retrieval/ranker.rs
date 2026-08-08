use crate::xiaoyi::knowledge::vector::embedding::Embedding;

/// Scores retrieval results.
///
/// @brief Result ranking helper
/// @since 0.1.0
/// @author Miruamel
/// @see crate::xiaoyi::knowledge::retrieval
pub fn rank(_query: &Embedding, _candidates: &[Embedding]) -> Vec<usize> {
    Vec::new()
}
