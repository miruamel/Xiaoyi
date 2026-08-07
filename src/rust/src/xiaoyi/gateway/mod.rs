//! # Gateway Module
//!
//! `gateway` provides user interface gateway (API, CLI, Web).
//!
//! Path: `xiaoyi::gateway`
//!
//! - Layer 0: `gateway` — Gateway layer.
//! - Layer 1: `api` — REST/GraphQL API.
//! - Layer 2: `cli` — Command-line interface.
//! - Layer 3: `web` — Web UI server.
//!
//! @module gateway
//! @brief User interface gateway
//! @group User Interface
//! @since 0.1.0
//! @author Miruamel
//! @see crate::builder
//! @see crate::orchestrator
//!
//! # Example
//!
//! ```rust
//! use xiaoyi::gateway::Gateway;
//!
//! let gateway = Gateway::new(config);
//! gateway.start().await?;
//! ```
pub mod api;
pub mod cli;
pub mod web;

use crate::xiaoyi::core::config::Config;

/// Gateway for exposing agents.
///
/// @brief API/CLI/Web gateway
/// @group User Interface
/// @since 0.1.0
#[derive(Debug, Clone)]
pub struct Gateway {
    config: Config,
}

impl Gateway {
    /// Create new gateway.
    ///
    /// @param config Runtime configuration
    /// @return Gateway instance
    /// @since 0.1.0
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// Start gateway services.
    ///
    /// @return Startup result
    /// @since 0.1.0
    pub async fn start(&self) -> crate::xiaoyi::core::error::Result<()> {
        Ok(())
    }
}