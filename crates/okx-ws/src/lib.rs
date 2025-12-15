//! # okx-ws
//!
//! WebSocket API client for OKX exchange.
//!
//! This crate provides async WebSocket client for OKX streaming endpoints.
//!
//! ## Features
//!
//! - Public channels (tickers, orderbook, trades, candles)
//! - Private channels (account, positions, orders)
//! - Automatic reconnection with exponential backoff
//! - Subscription state recovery after reconnection
//! - Heartbeat (ping/pong) handling
//!
//! ## Example (Basic Client)
//!
//! ```rust,no_run
//! use okx_ws::{WsClient, WsMessage, Channel};
//! use okx_core::{Config, Credentials};
//! use futures_util::StreamExt;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let credentials = Credentials::new("api_key", "secret_key", "passphrase");
//!     let config = Config::new(credentials).simulated(true);
//!
//!     let mut client = WsClient::connect_public(&config).await?;
//!
//!     // Subscribe to ticker channel
//!     client.subscribe(vec![Channel::Tickers { inst_id: "BTC-USDT".to_string() }]).await?;
//!
//!     // Process messages
//!     while let Some(msg) = client.next().await {
//!         match msg {
//!             Ok(WsMessage::Data { channel, data, .. }) => {
//!                 println!("Channel: {}, Data: {:?}", channel, data);
//!             }
//!             Ok(WsMessage::Event { event, .. }) => {
//!                 println!("Event: {:?}", event);
//!             }
//!             Err(e) => eprintln!("Error: {}", e),
//!             _ => {}
//!         }
//!     }
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Example (Auto-Reconnecting Client)
//!
//! ```rust,no_run
//! use okx_ws::{ReconnectingWsClient, ReconnectConfig, ConnectionType, WsMessage, Channel};
//! use okx_core::{Config, Credentials};
//! use futures_util::StreamExt;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let credentials = Credentials::new("api_key", "secret_key", "passphrase");
//!     let config = Config::new(credentials).simulated(true);
//!
//!     // Configure reconnection behavior
//!     let reconnect_config = ReconnectConfig::default()
//!         .with_max_attempts(10);
//!
//!     let mut client = ReconnectingWsClient::connect(
//!         config,
//!         ConnectionType::Public,
//!         reconnect_config,
//!     ).await?;
//!
//!     // Subscribe - subscriptions are tracked and restored after reconnection
//!     client.subscribe(vec![Channel::Tickers { inst_id: "BTC-USDT".to_string() }]).await?;
//!
//!     // Process messages - reconnection is handled automatically
//!     while let Some(msg) = client.next().await {
//!         match msg {
//!             Ok(WsMessage::Data { channel, data, .. }) => {
//!                 println!("Channel: {}, Data: {:?}", channel, data);
//!             }
//!             Err(e) => {
//!                 eprintln!("Error: {}", e);
//!                 // Trigger manual reconnection if needed
//!                 if !client.is_connected() {
//!                     client.reconnect().await?;
//!                 }
//!             }
//!             _ => {}
//!         }
//!     }
//!
//!     Ok(())
//! }
//! ```

mod channel;
mod client;
mod message;
mod reconnect;

pub use channel::Channel;
pub use client::WsClient;
pub use message::{WsEvent, WsMessage};
pub use reconnect::{
    channel_from_key, channel_key_from, ConnectionState, ConnectionType, ReconnectConfig,
    ReconnectingWsClient,
};

// Re-export core types for standalone usage
// When using the unified `okx` crate, these are already available
pub use okx_core::{Config, Credentials, OkxError, Result};
