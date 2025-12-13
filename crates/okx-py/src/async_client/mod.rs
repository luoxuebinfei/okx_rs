//! Async Python client wrapper for OKX REST API.
//!
//! 异步客户端按业务域拆分为多个 `#[pymethods]` 块：
//! - `account.rs` - Account API
//! - `trade.rs` - Trade API
//! - `market.rs` - Market API
//! - `funding.rs` - Funding API
//! - `public.rs` - Public API
//! - `subaccount.rs` - Subaccount API
//! - `convert.rs` - Convert API
//! - `grid.rs` - Grid API
//! - `spread.rs` - Spread API
//! - `trading_data.rs` - Trading Data API
//! - `broker.rs` - Broker API
//! - `copy_trading.rs` - Copy Trading API
//! - `finance.rs` - Finance API
//! - `block_rfq.rs` - Block RFQ API

use std::sync::Arc;

use pyo3::prelude::*;

use okx_rest::OkxRestClient;

use crate::types::PyConfig;

// 业务域 #[pymethods] 模块
mod account;
mod block_rfq;
mod broker;
mod convert;
mod copy_trading;
mod finance;
mod funding;
mod grid;
mod market;
mod public;
mod raw;
mod spread;
mod subaccount;
mod trade;
mod trading_data;

/// Async Python wrapper for OKX REST client.
///
/// Provides asynchronous access to OKX REST API using Python's asyncio.
///
/// Example:
///     ```python
///     import asyncio
///     from okx_py import AsyncOkxClient, Config, Credentials
///
///     async def main():
///         creds = Credentials("api_key", "secret_key", "passphrase")
///         config = Config(creds, simulated=True)
///         client = AsyncOkxClient(config)
///
///         # Async API calls
///         balance = await client.get_balance()
///         ticker = await client.get_ticker("BTC-USDT")
///
///     asyncio.run(main())
///     ```
#[pyclass(name = "AsyncOkxClient")]
pub struct PyAsyncOkxClient {
    client: Arc<OkxRestClient>,
}

impl PyAsyncOkxClient {
    pub(crate) fn rest_client(&self) -> Arc<OkxRestClient> {
        Arc::clone(&self.client)
    }
}

#[pymethods]
impl PyAsyncOkxClient {
    /// Create a new async OKX client.
    ///
    /// Args:
    ///     config: Client configuration
    #[new]
    fn new(config: PyConfig) -> Self {
        let client = Arc::new(OkxRestClient::new(config.inner));
        Self { client }
    }
}
