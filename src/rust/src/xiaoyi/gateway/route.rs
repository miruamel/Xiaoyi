use crate::xiaoyi::gateway::middleware::auth::AuthMiddleware;
use crate::xiaoyi::gateway::middleware::ratelimit::RateLimitMiddleware;

/// HTTP route definition.
///
/// @brief Gateway route configuration
/// @since 0.1.0
/// @author Miruamel
/// @see crate::xiaoyi::gateway
#[derive(Debug, Clone)]
pub struct Route {
    pub path: String,
    pub method: String,
    pub middleware: Vec<RouteMiddleware>,
}

#[derive(Debug, Clone)]
pub enum RouteMiddleware {
    Auth(AuthMiddleware),
    RateLimit(RateLimitMiddleware),
}
