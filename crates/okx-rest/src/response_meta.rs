//! Response metadata for REST API calls.
//!
//! This module provides `ResponseMeta` for accessing HTTP response headers
//! and status information, useful for implementing rate limit handling.

use std::collections::HashMap;

/// HTTP response metadata.
///
/// Contains status code and headers from the HTTP response.
/// Useful for implementing rate limit handling and debugging.
///
/// ## Example
///
/// ```rust,no_run
/// use okx_rest::{OkxRestClient, ResponseMeta};
/// use okx_core::{Config, Credentials};
///
/// # async fn example() -> okx_core::Result<()> {
/// let config = Config::new(Credentials::new("key", "secret", "pass"));
/// let client = OkxRestClient::new(config);
///
/// // Get response with metadata
/// let (data, meta) = client.get_public_with_meta::<serde_json::Value, ()>(
///     "/api/v5/public/time",
///     None,
/// ).await?;
///
/// println!("Status: {}", meta.status());
/// if let Some(remaining) = meta.header("x-ratelimit-remaining") {
///     println!("Rate limit remaining: {}", remaining);
/// }
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct ResponseMeta {
    /// HTTP status code
    status: u16,
    /// Response headers (lowercase keys)
    headers: HashMap<String, String>,
}

impl ResponseMeta {
    /// Create a new `ResponseMeta`.
    #[must_use]
    pub fn new(status: u16, headers: HashMap<String, String>) -> Self {
        Self { status, headers }
    }

    /// Get the HTTP status code.
    #[must_use]
    pub const fn status(&self) -> u16 {
        self.status
    }

    /// Check if the response was successful (2xx status).
    #[must_use]
    pub const fn is_success(&self) -> bool {
        self.status >= 200 && self.status < 300
    }

    /// Get a header value by name (case-insensitive).
    #[must_use]
    pub fn header(&self, name: &str) -> Option<&str> {
        self.headers.get(&name.to_lowercase()).map(String::as_str)
    }

    /// Get all headers.
    #[must_use]
    pub fn headers(&self) -> &HashMap<String, String> {
        &self.headers
    }

    /// Get rate limit remaining (if present in headers).
    ///
    /// Looks for common rate limit header names.
    #[must_use]
    pub fn rate_limit_remaining(&self) -> Option<u32> {
        self.header("x-ratelimit-remaining")
            .or_else(|| self.header("ratelimit-remaining"))
            .and_then(|v| v.parse().ok())
    }

    /// Get rate limit total (if present in headers).
    #[must_use]
    pub fn rate_limit_limit(&self) -> Option<u32> {
        self.header("x-ratelimit-limit")
            .or_else(|| self.header("ratelimit-limit"))
            .and_then(|v| v.parse().ok())
    }

    /// Get rate limit reset time in seconds (if present in headers).
    #[must_use]
    pub fn rate_limit_reset(&self) -> Option<u64> {
        self.header("x-ratelimit-reset")
            .or_else(|| self.header("ratelimit-reset"))
            .and_then(|v| v.parse().ok())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn response_meta_basic_accessors() {
        let mut headers = HashMap::new();
        headers.insert("content-type".to_string(), "application/json".to_string());
        headers.insert("x-ratelimit-remaining".to_string(), "100".to_string());

        let meta = ResponseMeta::new(200, headers);

        assert_eq!(meta.status(), 200);
        assert!(meta.is_success());
        assert_eq!(meta.header("Content-Type"), Some("application/json"));
        assert_eq!(meta.rate_limit_remaining(), Some(100));
    }

    #[test]
    fn response_meta_case_insensitive_headers() {
        let mut headers = HashMap::new();
        headers.insert("x-custom-header".to_string(), "value".to_string());

        let meta = ResponseMeta::new(200, headers);

        assert_eq!(meta.header("X-Custom-Header"), Some("value"));
        assert_eq!(meta.header("x-custom-header"), Some("value"));
    }

    #[test]
    fn response_meta_non_success_status() {
        let meta = ResponseMeta::new(429, HashMap::new());

        assert_eq!(meta.status(), 429);
        assert!(!meta.is_success());
    }

    #[test]
    fn response_meta_missing_rate_limit_headers() {
        let meta = ResponseMeta::new(200, HashMap::new());

        assert!(meta.rate_limit_remaining().is_none());
        assert!(meta.rate_limit_limit().is_none());
        assert!(meta.rate_limit_reset().is_none());
    }
}
