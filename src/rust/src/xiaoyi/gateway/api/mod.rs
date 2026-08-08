//! # Gateway API Module
//!
//! `api` provides REST and GraphQL API endpoints.
//!
//! Path: `xiaoyi::gateway::api`
//!
//! @module gateway::api
//! @brief REST/GraphQL API server
//! @group User Interface
//! @since 0.1.0
//! @author Miruamel
//! @see crate::gateway

pub mod graphql;
pub mod rest;

/// API server configuration.
///
/// @brief API server settings
/// @group User Interface
/// @since 0.1.0
#[derive(Debug, Clone)]
pub struct ApiConfig {
    pub host: String,
    pub port: u16,
}

impl Default for ApiConfig {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".into(),
            port: 8080,
        }
    }
}

/// Start API server.
///
/// @param config API configuration
/// @return Server handle
/// @since 0.1.0
pub async fn start(config: ApiConfig) -> crate::xiaoyi::core::error::Result<ApiHandle> {
    Ok(ApiHandle { config })
}

/// Running API server handle.
///
/// @brief Active API server
/// @group User Interface
/// @since 0.1.0
#[derive(Debug, Clone)]
pub struct ApiHandle {
    config: ApiConfig,
}
