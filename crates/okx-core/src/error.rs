//! Error types for OKX SDK.

use thiserror::Error;

/// Result type alias for OKX operations.
pub type Result<T> = std::result::Result<T, OkxError>;

/// Errors that can occur when using the OKX SDK.
#[derive(Debug, Error)]
pub enum OkxError {
    /// HTTP request/response error
    #[error("HTTP error: {0}")]
    Http(String),

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
}
