//! @module knowledge::retrieval::rag
//! @brief Retrieval-augmented generation (RAG) building blocks.
//! @group Knowledge
//! @since 0.1.0
//! @author Miruamel
//! @see crate::knowledge::retrieval
//! @see crate::knowledge::vector

pub mod pipeline;

pub use pipeline::{RetrievalPipeline, RetrievalResult, RetrievedChunk};
