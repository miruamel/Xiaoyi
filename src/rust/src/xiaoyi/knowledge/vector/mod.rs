//! @module knowledge::vector
//! @brief Vector storage and embedding module for knowledge indexing.
//! @group Knowledge
//! @since 0.1.0
//! @author Miruamel
//! @see crate::knowledge
pub mod embedding;
pub mod index;
pub mod store;

pub use store::{VectorStore, InMemoryVectorStore};
pub use embedding::{EmbeddingProvider, LocalEmbeddingProvider};
pub use index::{SearchResult, FlatIndex};