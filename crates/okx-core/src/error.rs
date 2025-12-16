//! Error types for OKX SDK.

use thiserror::Error;

/// Result type alias for OKX operations.
pub type Result<T> = std::result::Result<T, OkxError>;

/// Errors that can occur when using the OKX SDK.
#[derive(Debug, Error)]
pub enum OkxError {
    /// HTTP request/response error (connection-level)
    #[error("HTTP error: {0}")]
    Http(String),

    /// HTTP response with non-2xx status code
    #[error("HTTP status {status}: {body}")]
    HttpStatus {
        /// HTTP status code
        status: u16,
        /// Response body
        body: String,
    },

    /// WebSocket error
    #[error("WebSocket error: {0}")]
    WebSocket(String),

    /// OKX API returned an error
    #[error("API error: code={code}, msg={msg}")]
    Api {
        /// Error code from OKX
        code: String,
        /// Error message from OKX
        msg: String,
    },

    /// Authentication error
    #[error("Authentication error: {0}")]
    Auth(String),

    /// Serialization/deserialization error
    #[error("Serialization error: {0}")]
    Serde(#[from] serde_json::Error),

    /// Request timeout
    #[error("Request timeout")]
    Timeout,

    /// Invalid parameter
    #[error("Invalid parameter: {0}")]
    InvalidParameter(String),

    /// Connection closed
    #[error("Connection closed")]
    ConnectionClosed,

    /// Other errors
    #[error("{0}")]
    Other(String),
}

impl OkxError {
    /// Create an API error from code and message.
    #[must_use]
    pub fn api(code: impl Into<String>, msg: impl Into<String>) -> Self {
        Self::Api {
            code: code.into(),
            msg: msg.into(),
        }
    }

    /// Check if this is an API error with a specific code.
    #[must_use]
    pub fn is_api_error(&self, code: &str) -> bool {
        matches!(self, Self::Api { code: c, .. } if c == code)
    }

    /// Create an HTTP status error.
    #[must_use]
    pub fn http_status(status: u16, body: impl Into<String>) -> Self {
        Self::HttpStatus {
            status,
            body: body.into(),
        }
    }

    /// Check if this is an HTTP status error with a specific status code.
    #[must_use]
    pub fn is_http_status(&self, status: u16) -> bool {
        matches!(self, Self::HttpStatus { status: s, .. } if *s == status)
    }

    /// Check if this is a rate limit error (HTTP 429).
    #[must_use]
    pub fn is_rate_limited(&self) -> bool {
        self.is_http_status(429)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn api_helper_builds_error_and_matches_code() {
        let err = OkxError::api("51000", "failure");
        assert!(matches!(err, OkxError::Api { .. }));
        assert!(err.is_api_error("51000"));
        assert!(!err.is_api_error("400"));
        assert_eq!(err.to_string(), "API error: code=51000, msg=failure");
    }

    #[test]
    fn http_status_helper_builds_error_and_matches_status() {
        let err = OkxError::http_status(429, "rate limited");
        assert!(matches!(err, OkxError::HttpStatus { .. }));
        assert!(err.is_http_status(429));
        assert!(!err.is_http_status(500));
        assert!(err.is_rate_limited());
        assert_eq!(err.to_string(), "HTTP status 429: rate limited");
    }

    #[test]
    fn http_status_500_is_not_rate_limited() {
        let err = OkxError::http_status(500, "internal error");
        assert!(!err.is_rate_limited());
        assert!(err.is_http_status(500));
    }
}
