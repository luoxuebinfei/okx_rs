//! Configuration management.

use crate::{
    Credentials, REST_API_URL, WS_PRIVATE_URL, WS_PRIVATE_URL_SIMULATED, WS_PUBLIC_URL,
    WS_PUBLIC_URL_SIMULATED,
};

/// OKX client configuration.
#[derive(Debug, Clone)]
pub struct Config {
    /// API credentials
    credentials: Credentials,
    /// REST API base URL
    rest_url: String,
    /// WebSocket public URL
    ws_public_url: String,
    /// WebSocket private URL
    ws_private_url: String,
    /// Whether to use simulated (demo) trading
    simulated: bool,
    /// Request timeout in seconds
    timeout_secs: u64,
    /// HTTP/HTTPS proxy URL
    proxy_url: Option<String>,
}

impl Config {
    /// Create a new configuration with the given credentials.
    ///
    /// Defaults to production environment.
    #[must_use]
    pub fn new(credentials: Credentials) -> Self {
        Self {
            credentials,
            rest_url: REST_API_URL.to_string(),
            ws_public_url: WS_PUBLIC_URL.to_string(),
            ws_private_url: WS_PRIVATE_URL.to_string(),
            simulated: false,
            timeout_secs: 30,
            proxy_url: None,
        }
    }

    /// Enable or disable simulated (demo) trading.
    ///
    /// When enabled, uses the demo trading WebSocket URLs.
    #[must_use]
    pub fn simulated(mut self, simulated: bool) -> Self {
        self.simulated = simulated;
        if simulated {
            self.ws_public_url = WS_PUBLIC_URL_SIMULATED.to_string();
            self.ws_private_url = WS_PRIVATE_URL_SIMULATED.to_string();
        } else {
            self.ws_public_url = WS_PUBLIC_URL.to_string();
            self.ws_private_url = WS_PRIVATE_URL.to_string();
        }
        self
    }

    /// Set a custom REST API base URL.
    #[must_use]
    pub fn with_rest_url(mut self, url: impl Into<String>) -> Self {
        self.rest_url = url.into();
        self
    }

    /// Set a custom WebSocket public URL.
    #[must_use]
    pub fn with_ws_public_url(mut self, url: impl Into<String>) -> Self {
        self.ws_public_url = url.into();
        self
    }

    /// Set a custom WebSocket private URL.
    #[must_use]
    pub fn with_ws_private_url(mut self, url: impl Into<String>) -> Self {
        self.ws_private_url = url.into();
        self
    }

    /// Set the request timeout in seconds.
    #[must_use]
    pub fn with_timeout_secs(mut self, secs: u64) -> Self {
        self.timeout_secs = secs;
        self
    }

    /// Set the proxy URL.
    ///
    /// Supports HTTP, HTTPS, and SOCKS5 proxies.
    ///
    /// ## Examples
    ///
    /// - HTTP proxy: `http://127.0.0.1:7890`
    /// - HTTPS proxy: `https://127.0.0.1:7890`
    /// - SOCKS5 proxy: `socks5://127.0.0.1:1080`
    #[must_use]
    pub fn with_proxy(mut self, proxy_url: impl Into<String>) -> Self {
        self.proxy_url = Some(proxy_url.into());
        self
    }

    /// Get the credentials.
    #[must_use]
    pub fn credentials(&self) -> &Credentials {
        &self.credentials
    }

    /// Get the REST API base URL.
    #[must_use]
    pub fn rest_url(&self) -> &str {
        &self.rest_url
    }

    /// Get the WebSocket public URL.
    #[must_use]
    pub fn ws_public_url(&self) -> &str {
        &self.ws_public_url
    }

    /// Get the WebSocket private URL.
    #[must_use]
    pub fn ws_private_url(&self) -> &str {
        &self.ws_private_url
    }

    /// Check if simulated trading is enabled.
    #[must_use]
    pub fn is_simulated(&self) -> bool {
        self.simulated
    }

    /// Get the request timeout in seconds.
    #[must_use]
    pub fn timeout_secs(&self) -> u64 {
        self.timeout_secs
    }

    /// Get the proxy URL.
    #[must_use]
    pub fn proxy_url(&self) -> Option<&str> {
        self.proxy_url.as_deref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        REST_API_URL, WS_PRIVATE_URL, WS_PRIVATE_URL_SIMULATED, WS_PUBLIC_URL,
        WS_PUBLIC_URL_SIMULATED,
    };

    #[test]
    fn default_config_uses_prod_urls_and_defaults() {
        let cfg = Config::new(Credentials::new("k", "s", "p"));
        assert_eq!(cfg.rest_url(), REST_API_URL);
        assert_eq!(cfg.ws_public_url(), WS_PUBLIC_URL);
        assert_eq!(cfg.ws_private_url(), WS_PRIVATE_URL);
        assert!(!cfg.is_simulated());
        assert_eq!(cfg.timeout_secs(), 30);
        assert!(cfg.proxy_url().is_none());
    }

    #[test]
    fn simulated_switches_ws_urls() {
        let cfg = Config::new(Credentials::new("k", "s", "p")).simulated(true);
        assert!(cfg.is_simulated());
        assert_eq!(cfg.ws_public_url(), WS_PUBLIC_URL_SIMULATED);
        assert_eq!(cfg.ws_private_url(), WS_PRIVATE_URL_SIMULATED);
    }

    #[test]
    fn custom_overrides_are_applied() {
        let cfg = Config::new(Credentials::new("k", "s", "p"))
            .with_rest_url("https://alt.com")
            .with_ws_public_url("wss://pub")
            .with_ws_private_url("wss://pri")
            .with_timeout_secs(5)
            .with_proxy("http://127.0.0.1:7890");

        assert_eq!(cfg.rest_url(), "https://alt.com");
        assert_eq!(cfg.ws_public_url(), "wss://pub");
        assert_eq!(cfg.ws_private_url(), "wss://pri");
        assert_eq!(cfg.timeout_secs(), 5);
        assert_eq!(cfg.proxy_url(), Some("http://127.0.0.1:7890"));
    }
}
