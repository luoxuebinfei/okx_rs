//! # okx-core
//!
//! Core types, authentication, and utilities for OKX API.
//!
//! This crate provides:
//! - Configuration and credentials management
//! - Request signing (HMAC-SHA256)
//! - Error types
//! - Common data types (Balance, Order, Position, etc.)
//!
//! ## Example
//!
//! ```rust,no_run
//! use okx_core::{Config, Credentials};
//!
//! let credentials = Credentials::new(
//!     "api_key",
//!     "secret_key",
//!     "passphrase",
//! );
//!
//! let config = Config::new(credentials)
//!     .simulated(true);  // Use demo trading
//! ```

mod config;
mod credentials;
mod error;

pub mod signer;
pub mod types;

pub use config::Config;
pub use credentials::Credentials;
pub use error::{OkxError, Result};
pub use signer::Signer;

/// OKX API version
pub const API_VERSION: &str = "v5";

/// Default REST API base URL (production)
pub const REST_API_URL: &str = "https://www.okx.com";

/// Default REST API base URL (AWS)
pub const REST_API_URL_AWS: &str = "https://aws.okx.com";

/// WebSocket public URL (production)
pub const WS_PUBLIC_URL: &str = "wss://ws.okx.com:8443/ws/v5/public";

/// WebSocket private URL (production)
pub const WS_PRIVATE_URL: &str = "wss://ws.okx.com:8443/ws/v5/private";

/// WebSocket public URL (simulated/demo)
pub const WS_PUBLIC_URL_SIMULATED: &str = "wss://wspap.okx.com:8443/ws/v5/public?brokerId=9999";

/// WebSocket private URL (simulated/demo)
pub const WS_PRIVATE_URL_SIMULATED: &str = "wss://wspap.okx.com:8443/ws/v5/private?brokerId=9999";
