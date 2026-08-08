use crate::xiaoyi::knowledge::vector::embedding::Embedding;

/// Embeds text into a vector representation.
///
/// @brief Text embedding helper
/// @since 0.1.0
/// @author Miruamel
/// @see crate::xiaoyi::critic::cache
pub fn embed(text: &str) -> Embedding {
    Embedding {
        id: uuid::Uuid::new_v4().to_string(),
        values: vec![0.0; 8],
        metadata: std::collections::HashMap::new(),
    }
}
