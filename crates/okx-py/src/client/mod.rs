//! Python client wrapper for OKX REST API.
//!
//! 同步客户端按业务域拆分为多个 `#[pymethods]` 块：
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

use std::future::Future;

use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;
use tokio::runtime::Runtime;

use okx_rest::OkxRestClient;

use crate::to_py_err;
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

/// Python wrapper for OKX REST client.
///
/// Provides synchronous access to OKX REST API.
#[pyclass(name = "OkxClient")]
pub struct PyOkxClient {
    pub(crate) client: OkxRestClient,
    pub(crate) runtime: Runtime,
}

impl PyOkxClient {
    /// 统一阻塞执行器，释放 GIL 避免死锁。
    #[allow(deprecated)]
    pub(crate) fn block_on_allow_threads<T, F>(&self, fut: F) -> PyResult<T>
    where
        F: Future<Output = okx_core::Result<T>> + Send,
        T: Send,
    {
        Python::with_gil(|py| py.allow_threads(|| self.runtime.block_on(fut).map_err(to_py_err)))
    }

    pub(crate) fn rest_client(&self) -> &OkxRestClient {
        &self.client
    }
}

#[pymethods]
impl PyOkxClient {
    /// Create a new OKX client.
    ///
    /// Args:
    ///     config: Client configuration
    #[new]
    fn new(config: PyConfig) -> PyResult<Self> {
        let runtime = Runtime::new()
            .map_err(|e| PyRuntimeError::new_err(format!("Failed to create runtime: {}", e)))?;
        let client = OkxRestClient::new(config.inner);
        Ok(Self { client, runtime })
    }
}
