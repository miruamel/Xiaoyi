//! # Gateway Web Module
//!
//! `web` provides web UI server.
//!
//! Path: `xiaoyi::gateway::web`
//!
//! @module gateway::web
//! @brief Web UI server
//! @group User Interface
//! @since 0.1.0
//! @author Miruamel
//! @see crate::gateway

/// Web server configuration.
///
/// @brief Web server settings
/// @group User Interface
/// @since 0.1.0
#[derive(Debug, Clone)]
pub struct WebConfig {
    pub host: String,
    pub port: u16,
    pub static_dir: Option<String>,
}

impl Default for WebConfig {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".into(),
            port: 3000,
            static_dir: None,
        }
    }
}

/// Start web server.
///
/// @param config Web configuration
/// @return Server handle
/// @since 0.1.0
pub async fn start(config: WebConfig) -> crate::xiaoyi::core::error::Result<WebHandle> {
    Ok(WebHandle { config })
}

/// Running web server handle.
///
/// @brief Active web server
/// @group User Interface
/// @since 0.1.0
#[derive(Debug, Clone)]
pub struct WebHandle {
    config: WebConfig,
}