//! @module knowledge::retrieval::rag::pipeline
//! @brief Retrieval-augmented generation pipeline over embedded vector storage.
//! @group Knowledge
//! @since 0.1.0
//! @author Miruamel
//! @see crate::knowledge::retrieval
//! @see crate::knowledge::vector

use crate::xiaoyi::core::error::Result;
use crate::xiaoyi::knowledge::vector::embedding::EmbeddingProvider;
use crate::xiaoyi::knowledge::vector::index::SearchResult;
use crate::xiaoyi::knowledge::vector::store::VectorStore;

/// A single retrieved text chunk with its similarity score.
///
/// @brief One chunk returned by a retrieval query
/// @group Knowledge
/// @since 0.1.0
/// @author Miruamel
#[derive(Debug, Clone, Default)]
pub struct RetrievedChunk {
    /// Identifier of the source document (without the `#<index>` chunk suffix).
    pub doc_id: String,
    /// Similarity score produced by the vector store; higher is closer.
    pub score: f32,
    /// Decoded chunk text recovered from the stored payload.
    pub text: String,
}

/// The full result of a retrieval query.
///
/// @brief Aggregated retrieval output for a single query string
/// @group Knowledge
/// @since 0.1.0
/// @author Miruamel
#[derive(Debug, Clone, Default)]
pub struct RetrievalResult {
    /// The original query string that produced this result.
    pub query: String,
    /// Ranked chunks retrieved from the vector store.
    pub chunks: Vec<RetrievedChunk>,
}

/// A retrieval-augmented generation (RAG) pipeline that embeds documents,
/// indexes them into a vector store, and retrieves ranked chunks for queries.
///
/// @brief Configurable RAG pipeline composing an embedder and a vector store
/// @group Knowledge
/// @since 0.1.0
/// @author Miruamel
/// @tparam E Embedding provider implementation (`EmbeddingProvider`).
/// @tparam S Vector store implementation (`VectorStore`).
pub struct RetrievalPipeline<E, S>
where
    E: EmbeddingProvider,
    S: VectorStore,
{
    /// Embedding provider used to vectorize document chunks and queries.
    embedding: E,
    /// Vector store that persists and queries chunk embeddings.
    store: S,
    /// Character window size used when chunking documents.
    chunk_size: usize,
}

impl<E, S> std::fmt::Debug for RetrievalPipeline<E, S>
where
    E: EmbeddingProvider,
    S: VectorStore,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RetrievalPipeline {{ chunk_size: {} }}", self.chunk_size)
    }
}

impl<E, S> RetrievalPipeline<E, S>
where
    E: EmbeddingProvider,
    S: VectorStore,
{
    /// Create a new retrieval pipeline with the default chunk size of 512
    /// characters.
    ///
    /// @brief Construct a pipeline with default chunking
    /// @param embedding Embedding provider implementation.
    /// @param store Vector store implementation.
    /// @return A ready-to-use `RetrievalPipeline` with `chunk_size` set to 512.
    pub fn new(embedding: E, store: S) -> Self {
        Self {
            embedding,
            store,
            chunk_size: 512,
        }
    }

    /// Override the character window size used when chunking documents.
    ///
    /// @brief Set a custom chunk size
    /// @param chunk_size Number of characters per chunk window.
    /// @return The same pipeline with the updated `chunk_size`.
    pub fn with_chunk_size(mut self, chunk_size: usize) -> Self {
        self.chunk_size = chunk_size;
        self
    }

    /// Split `text` into overlapping character windows and embed each chunk into
    /// the vector store under the key `<doc_id>#<index>`.
    ///
    /// Chunks advance by `chunk_size - chunk_size / 4` characters, yielding an
    /// overlap of `chunk_size / 4` characters between consecutive windows.
    ///
    /// @brief Index a document by chunking, embedding, and upserting
    /// @param doc_id Stable identifier for the source document.
    /// @param text Document content to chunk, embed, and store.
    /// @return `Ok(())` once every chunk has been upserted.
    pub async fn index(&self, doc_id: &str, text: &str) -> Result<()> {
        let overlap = self.chunk_size / 4;
        let step = self.chunk_size.saturating_sub(overlap).max(1);

        // Byte offsets of every char boundary, plus the final end offset.
        let boundaries: Vec<usize> = text.char_indices().map(|(i, _)| i).collect();
        let n = boundaries.len();

        let mut start = 0usize;
        let mut index = 0usize;
        while start < n {
            let end = (start + self.chunk_size).min(n);
            let chunk = &text[boundaries[start]..boundaries[end]];
            let vector = self.embedding.embed(chunk).await?;
            self.store
                .upsert(
                    format!("{doc_id}#{index}"),
                    vector,
                    chunk.as_bytes().to_vec(),
                )
                .await?;
            index += 1;
            start += step;
        }
        Ok(())
    }

    /// Embed `query`, search the vector store for the `top_k` nearest chunks,
    /// and return them as a ranked `RetrievalResult`.
    ///
    /// @brief Retrieve the most similar chunks for a query
    /// @param query Natural-language query to embed and search with.
    /// @param top_k Maximum number of chunks to return, ranked by score.
    /// @return A `RetrievalResult` containing the query and ranked chunks.
    pub async fn retrieve(&self, query: &str, top_k: usize) -> Result<RetrievalResult> {
        let q = self.embedding.embed(query).await?;
        let hits: Vec<SearchResult> = self.store.query(q, top_k).await?;
        let chunks = hits
            .into_iter()
            .map(|hit| RetrievedChunk {
                doc_id: hit.id,
                score: hit.score,
                text: String::from_utf8_lossy(&hit.payload).into_owned(),
            })
            .collect();
        Ok(RetrievalResult {
            query: query.to_string(),
            chunks,
        })
    }
}
