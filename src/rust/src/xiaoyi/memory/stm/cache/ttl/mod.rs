use std::time::Duration;

/// TTL cache entry metadata.
///
/// @brief Time-to-live metadata for cache entries
/// @since 0.1.0
/// @author Miruamel
#[derive(Debug, Clone)]
pub struct TtlEntry {
    pub ttl: Duration,
}

impl TtlEntry {
    /// Create TTL metadata.
    ///
    /// @param ttl Time-to-live duration
    /// @return TtlEntry instance
    /// @since 0.1.0
    pub fn new(ttl: Duration) -> Self {
        Self { ttl }
    }
}
