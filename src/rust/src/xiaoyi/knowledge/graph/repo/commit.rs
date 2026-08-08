use std::time::{SystemTime, UNIX_EPOCH};

/// Represents a repository commit snapshot.
///
/// @brief Commit snapshot for graph repo
/// @since 0.1.0
/// @author Miruamel
/// @see crate::xiaoyi::knowledge::graph::repo
#[derive(Debug, Clone)]
pub struct RepoCommit {
    pub sha: String,
    pub author: String,
    pub timestamp: u64,
}

impl RepoCommit {
    /// Create a new commit snapshot.
    ///
    /// @brief Initialize commit snapshot
    /// @param sha Commit SHA
    /// @param author Commit author
    /// @return RepoCommit instance
    /// @since 0.1.0
    pub fn new(sha: impl Into<String>, author: impl Into<String>) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        Self {
            sha: sha.into(),
            author: author.into(),
            timestamp,
        }
    }
}
