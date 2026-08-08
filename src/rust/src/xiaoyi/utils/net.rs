use std::time::Duration;

/// HTTP client configuration.
///
/// @brief Configuration for HTTP client
/// @group Core Runtime
/// @since 0.1.0
/// @author Miruamel
#[derive(Debug, Clone)]
pub struct HttpClientConfig {
    /// Request timeout.
    pub timeout: Duration,
    /// Follow redirects.
    pub follow_redirects: bool,
    /// Maximum retries.
    pub max_retries: u32,
}

impl Default for HttpClientConfig {
    fn default() -> Self {
        Self {
            timeout: Duration::from_secs(30),
            follow_redirects: true,
            max_retries: 3,
        }
    }
}
