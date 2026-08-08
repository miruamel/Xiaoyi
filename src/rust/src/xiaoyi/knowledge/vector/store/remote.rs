use crate::xiaoyi::knowledge::vector::store::in_memory::InMemoryVectorStore;

/// Remote vector store placeholder.
///
/// @brief Remote-backed vector store interface
/// @since 0.1.0
/// @author Miruamel
/// @see crate::xiaoyi::knowledge::vector::store
pub struct RemoteVectorStore {
    pub inner: InMemoryVectorStore,
    pub endpoint: String,
}
