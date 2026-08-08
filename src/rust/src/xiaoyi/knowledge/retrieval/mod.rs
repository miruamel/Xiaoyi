//! @module knowledge::retrieval
//! @brief Public retrieval facade for the Xiaoyi knowledge system.
//! @group Knowledge
//! @since 0.1.0
//! @author Miruamel
//! @see crate::knowledge
pub mod rag;

pub use rag::{RetrievalPipeline, RetrievalResult, RetrievedChunk};