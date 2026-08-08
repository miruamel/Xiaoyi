/// Rate limiter middleware.
///
/// @brief Rate limit middleware configuration
/// @since 0.1.0
/// @author Miruamel
/// @see crate::xiaoyi::gateway::middleware
#[derive(Debug, Clone, Default)]
pub struct RateLimitMiddleware {
    pub max_requests: u32,
    pub window_secs: u64,
}
