/// Authentication middleware.
///
/// @brief Auth middleware configuration
/// @since 0.1.0
/// @author Miruamel
/// @see crate::xiaoyi::gateway::middleware
#[derive(Debug, Clone, Default)]
pub struct AuthMiddleware {
    pub token_header: String,
}
