//! # okx-rest
//!
//! REST API client for OKX exchange.
//!
//! This crate provides async HTTP client for OKX REST API endpoints.
//!
//! ## Example
//!
//! ```rust,no_run
//! use okx_rest::{OkxRestClient, AccountApi, MarketApi};
//! use okx_core::{Config, Credentials};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let credentials = Credentials::new("api_key", "secret_key", "passphrase");
//!     let config = Config::new(credentials).simulated(true);
//!     let client = OkxRestClient::new(config);
//!
//!     // Get account balance
//!     let balance = client.get_balance(None).await?;
//!     println!("Balance: {:?}", balance);
//!
//!     // Get ticker (public endpoint, no auth required)
//!     let ticker = client.get_ticker("BTC-USDT").await?;
//!     println!("Ticker: {:?}", ticker);
//!
//!     Ok(())
//! }
//! ```

pub mod api;
mod client;

pub use api::account::AccountApi;
pub use api::block_rfq::BlockRfqApi;
pub use api::broker::BrokerApi;
pub use api::convert::ConvertApi;
pub use api::copy_trading::CopyTradingApi;
pub use api::finance::FinanceApi;
pub use api::funding::FundingApi;
pub use api::grid::GridApi;
pub use api::market::MarketApi;
pub use api::public::PublicApi;
pub use api::spread::SpreadApi;
pub use api::status::StatusApi;
pub use api::subaccount::SubaccountApi;
pub use api::trade::TradeApi;
pub use api::trading_data::TradingDataApi;
pub use client::OkxRestClient;

// Re-export core types for convenience
pub use okx_core::*;
