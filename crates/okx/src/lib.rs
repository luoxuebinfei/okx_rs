//! OKX Exchange API client for Rust.
//!
//! # Features
//!
//! - `rest` - REST API client (enabled by default)
//! - `ws` - WebSocket API client (enabled by default)
//! - `full` - All features
//!
//! # Usage
//!
//! ```toml
//! # 默认：REST + WebSocket
//! okx = { git = "https://github.com/user/okx_rs" }
//!
//! # 仅 REST API
//! okx = { git = "https://github.com/user/okx_rs", default-features = false, features = ["rest"] }
//!
//! # 仅 WebSocket API
//! okx = { git = "https://github.com/user/okx_rs", default-features = false, features = ["ws"] }
//! ```
//!
//! # Example
//!
//! ```rust,ignore
//! use okx::prelude::*;
//!
//! #[tokio::main]
//! async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
//!     let config = Config::new(Credentials::new(
//!         "api-key",
//!         "secret-key",
//!         "passphrase",
//!     ));
//!
//!     // REST API
//!     let client = RestClient::new(config.clone());
//!     let balance = client.account().get_balance(None).await?;
//!
//!     // WebSocket API
//!     let mut ws = WsClient::connect_public(&config).await?;
//!     ws.subscribe(vec![Channel::Tickers { inst_id: "BTC-USDT".into() }]).await?;
//!
//!     Ok(())
//! }
//! ```

/// Prelude module - import common types with `use okx::prelude::*`
pub mod prelude {
    pub use okx_core::{Config, Credentials, OkxError, Result};

    #[cfg(feature = "rest")]
    pub use okx_rest::OkxRestClient as RestClient;

    #[cfg(feature = "ws")]
    pub use okx_ws::{Channel, ReconnectConfig, ReconnectingWsClient, WsClient, WsMessage};
}

// ============================================================================
// Core types (always available)
// ============================================================================

pub use okx_core::*;

// ============================================================================
// REST API (feature = "rest")
// ============================================================================

#[cfg(feature = "rest")]
pub mod rest {
    //! REST API client and endpoint modules.
    pub use okx_rest::*;
}

/// REST client type alias for convenience.
#[cfg(feature = "rest")]
pub use okx_rest::OkxRestClient as RestClient;

// ============================================================================
// WebSocket API (feature = "ws")
// ============================================================================

#[cfg(feature = "ws")]
pub mod ws {
    //! WebSocket API client and types.
    pub use okx_ws::*;
}

#[cfg(feature = "ws")]
pub use okx_ws::{
    channel_from_key, channel_key_from, Channel, ConnectionState, ConnectionType, ReconnectConfig,
    ReconnectingWsClient, WsClient, WsEvent, WsMessage,
};
