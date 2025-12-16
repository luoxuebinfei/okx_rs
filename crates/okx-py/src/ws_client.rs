//! WebSocket client Python bindings.

use std::sync::Arc;

use futures_util::StreamExt;
use pyo3::prelude::*;
use pyo3::types::PyDict;
use tokio::sync::Mutex;

use okx_core::TimestampProvider;
use okx_ws::{Channel, ConnectionType, ReconnectConfig, ReconnectingWsClient, WsMessage};

use crate::time_sync::PyTimeSync;
use crate::to_py_err;
use crate::types::PyConfig;

/// Python wrapper for WebSocket client.
///
/// Provides async access to OKX WebSocket API for real-time data streaming.
///
/// Example:
///     ```python
///     import asyncio
///     from okx_py import WsClient, Config, Credentials
///
///     async def main():
///         creds = Credentials("api_key", "secret_key", "passphrase")
///         config = Config(creds, simulated=True)
///
///         # Connect to public WebSocket
///         client = await WsClient.connect_public(config)
///
///         # Subscribe to ticker channel
///         await client.subscribe_tickers("BTC-USDT")
///
///         # Receive messages
///         async for msg in client:
///             print(msg)
///
///     asyncio.run(main())
///     ```
#[pyclass(name = "WsClient")]
pub struct PyWsClient {
    client: Arc<Mutex<ReconnectingWsClient>>,
    is_private: bool,
}

#[pymethods]
impl PyWsClient {
    /// Connect to the public WebSocket endpoint.
    ///
    /// Args:
    ///     config: Client configuration
    ///     max_reconnect_attempts: Maximum reconnection attempts (default: unlimited)
    ///
    /// Returns:
    ///     WsClient instance
    #[staticmethod]
    #[pyo3(signature = (config, max_reconnect_attempts=None))]
    fn connect_public<'py>(
        py: Python<'py>,
        config: PyConfig,
        max_reconnect_attempts: Option<u32>,
    ) -> PyResult<Bound<'py, PyAny>> {
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let mut reconnect_config = ReconnectConfig::default();
            if let Some(max) = max_reconnect_attempts {
                reconnect_config = reconnect_config.with_max_attempts(max);
            }

            let client = ReconnectingWsClient::connect(
                config.inner,
                ConnectionType::Public,
                reconnect_config,
            )
            .await
            .map_err(to_py_err)?;

            Ok(PyWsClient {
                client: Arc::new(Mutex::new(client)),
                is_private: false,
            })
        })
    }

    /// Connect to the private WebSocket endpoint.
    ///
    /// Args:
    ///     config: Client configuration
    ///     max_reconnect_attempts: Maximum reconnection attempts (default: unlimited)
    ///
    /// Returns:
    ///     WsClient instance
    #[staticmethod]
    #[pyo3(signature = (config, max_reconnect_attempts=None))]
    fn connect_private<'py>(
        py: Python<'py>,
        config: PyConfig,
        max_reconnect_attempts: Option<u32>,
    ) -> PyResult<Bound<'py, PyAny>> {
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let mut reconnect_config = ReconnectConfig::default();
            if let Some(max) = max_reconnect_attempts {
                reconnect_config = reconnect_config.with_max_attempts(max);
            }

            let client = ReconnectingWsClient::connect(
                config.inner,
                ConnectionType::Private,
                reconnect_config,
            )
            .await
            .map_err(to_py_err)?;

            Ok(PyWsClient {
                client: Arc::new(Mutex::new(client)),
                is_private: true,
            })
        })
    }

    /// Subscribe to ticker channel.
    ///
    /// Args:
    ///     inst_id: Instrument ID (e.g., "BTC-USDT")
    fn subscribe_tickers<'py>(
        &self,
        py: Python<'py>,
        inst_id: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let channel = Channel::Tickers { inst_id };
            client
                .lock()
                .await
                .subscribe(vec![channel])
                .await
                .map_err(to_py_err)
        })
    }

    /// 订阅策略订单频道（私有）。
    #[pyo3(signature = (inst_type, inst_family=None, inst_id=None))]
    fn subscribe_orders_algo<'py>(
        &self,
        py: Python<'py>,
        inst_type: String,
        inst_family: Option<String>,
        inst_id: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let channel = Channel::OrdersAlgo {
                inst_type,
                inst_family,
                inst_id,
            };
            client
                .lock()
                .await
                .subscribe(vec![channel])
                .await
                .map_err(to_py_err)
        })
    }

    /// 订阅余额与持仓合并频道（私有）。
    fn subscribe_balance_and_position<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let channel = Channel::BalanceAndPosition;
            client
                .lock()
                .await
                .subscribe(vec![channel])
                .await
                .map_err(to_py_err)
        })
    }

    /// Subscribe to order book channel.
    ///
    /// Args:
    ///     inst_id: Instrument ID (e.g., "BTC-USDT")
    fn subscribe_orderbook<'py>(
        &self,
        py: Python<'py>,
        inst_id: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let channel = Channel::Books { inst_id };
            client
                .lock()
                .await
                .subscribe(vec![channel])
                .await
                .map_err(to_py_err)
        })
    }

    /// 订阅 5 档订单簿。
    fn subscribe_orderbook5<'py>(
        &self,
        py: Python<'py>,
        inst_id: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let channel = Channel::Books5 { inst_id };
            client
                .lock()
                .await
                .subscribe(vec![channel])
                .await
                .map_err(to_py_err)
        })
    }

    /// 订阅 50 档 L2 TBT 订单簿。
    fn subscribe_orderbook50_l2_tbt<'py>(
        &self,
        py: Python<'py>,
        inst_id: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let channel = Channel::Books50L2Tbt { inst_id };
            client
                .lock()
                .await
                .subscribe(vec![channel])
                .await
                .map_err(to_py_err)
        })
    }

    /// 订阅全量 L2 TBT 订单簿。
    fn subscribe_orderbook_l2_tbt<'py>(
        &self,
        py: Python<'py>,
        inst_id: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let channel = Channel::BooksL2Tbt { inst_id };
            client
                .lock()
                .await
                .subscribe(vec![channel])
                .await
                .map_err(to_py_err)
        })
    }

    /// Subscribe to trades channel.
    ///
    /// Args:
    ///     inst_id: Instrument ID (e.g., "BTC-USDT")
    fn subscribe_trades<'py>(
        &self,
        py: Python<'py>,
        inst_id: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let channel = Channel::Trades { inst_id };
            client
                .lock()
                .await
                .subscribe(vec![channel])
                .await
                .map_err(to_py_err)
        })
    }

    /// 订阅标记价格频道。
    fn subscribe_mark_price<'py>(
        &self,
        py: Python<'py>,
        inst_id: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let channel = Channel::MarkPrice { inst_id };
            client
                .lock()
                .await
                .subscribe(vec![channel])
                .await
                .map_err(to_py_err)
        })
    }

    /// 订阅指数 Ticker 频道。
    fn subscribe_index_tickers<'py>(
        &self,
        py: Python<'py>,
        inst_id: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let channel = Channel::IndexTickers { inst_id };
            client
                .lock()
                .await
                .subscribe(vec![channel])
                .await
                .map_err(to_py_err)
        })
    }

    /// 订阅资金费率频道。
    fn subscribe_funding_rate<'py>(
        &self,
        py: Python<'py>,
        inst_id: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let channel = Channel::FundingRate { inst_id };
            client
                .lock()
                .await
                .subscribe(vec![channel])
                .await
                .map_err(to_py_err)
        })
    }

    /// Subscribe to candlestick channel.
    ///
    /// Args:
    ///     inst_id: Instrument ID (e.g., "BTC-USDT")
    ///     interval: Candle interval (1m, 5m, 15m, 1H, 4H, 1D)
    #[pyo3(signature = (inst_id, interval="1m"))]
    fn subscribe_candles<'py>(
        &self,
        py: Python<'py>,
        inst_id: String,
        interval: &str,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        let channel = match interval {
            "1m" => Channel::Candle1m { inst_id },
            "5m" => Channel::Candle5m { inst_id },
            "15m" => Channel::Candle15m { inst_id },
            "1H" => Channel::Candle1H { inst_id },
            "4H" => Channel::Candle4H { inst_id },
            "1D" => Channel::Candle1D { inst_id },
            _ => {
                return Err(pyo3::exceptions::PyValueError::new_err(format!(
                    "Invalid interval: {}. Valid values: 1m, 5m, 15m, 1H, 4H, 1D",
                    interval
                )))
            }
        };

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            client
                .lock()
                .await
                .subscribe(vec![channel])
                .await
                .map_err(to_py_err)
        })
    }

    /// Subscribe to account channel (private).
    ///
    /// Args:
    ///     ccy: Optional currency filter
    #[pyo3(signature = (ccy=None))]
    fn subscribe_account<'py>(
        &self,
        py: Python<'py>,
        ccy: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let channel = Channel::Account { ccy };
            client
                .lock()
                .await
                .subscribe(vec![channel])
                .await
                .map_err(to_py_err)
        })
    }

    /// Subscribe to positions channel (private).
    ///
    /// Args:
    ///     inst_type: Instrument type (MARGIN, SWAP, FUTURES, OPTION)
    ///     inst_id: Optional instrument ID filter
    #[pyo3(signature = (inst_type, inst_id=None))]
    fn subscribe_positions<'py>(
        &self,
        py: Python<'py>,
        inst_type: String,
        inst_id: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let channel = Channel::Positions {
                inst_type,
                inst_family: None,
                inst_id,
            };
            client
                .lock()
                .await
                .subscribe(vec![channel])
                .await
                .map_err(to_py_err)
        })
    }

    /// Subscribe to orders channel (private).
    ///
    /// Args:
    ///     inst_type: Instrument type (SPOT, MARGIN, SWAP, FUTURES, OPTION)
    ///     inst_id: Optional instrument ID filter
    #[pyo3(signature = (inst_type, inst_id=None))]
    fn subscribe_orders<'py>(
        &self,
        py: Python<'py>,
        inst_type: String,
        inst_id: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let channel = Channel::Orders {
                inst_type,
                inst_family: None,
                inst_id,
            };
            client
                .lock()
                .await
                .subscribe(vec![channel])
                .await
                .map_err(to_py_err)
        })
    }

    /// 订阅高级算法订单频道（私有）。
    #[pyo3(signature = (inst_type=None, inst_family=None, inst_id=None))]
    fn subscribe_algo_advance<'py>(
        &self,
        py: Python<'py>,
        inst_type: Option<String>,
        inst_family: Option<String>,
        inst_id: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let channel = Channel::AlgoAdvance {
                inst_type,
                inst_family,
                inst_id,
            };
            client
                .lock()
                .await
                .subscribe(vec![channel])
                .await
                .map_err(to_py_err)
        })
    }

    /// 订阅 business RFQ 频道（需私有/business 连接）。
    #[pyo3(signature = (inst_family=None))]
    fn subscribe_rfqs<'py>(
        &self,
        py: Python<'py>,
        inst_family: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let channel = Channel::Rfqs { inst_family };
            client
                .lock()
                .await
                .subscribe(vec![channel])
                .await
                .map_err(to_py_err)
        })
    }

    /// 订阅 business Quotes 频道（需私有/business 连接）。
    #[pyo3(signature = (inst_family=None))]
    fn subscribe_quotes<'py>(
        &self,
        py: Python<'py>,
        inst_family: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let channel = Channel::Quotes { inst_family };
            client
                .lock()
                .await
                .subscribe(vec![channel])
                .await
                .map_err(to_py_err)
        })
    }

    /// 订阅结构化大宗成交（私有）。
    #[pyo3(signature = (inst_family=None))]
    fn subscribe_struc_block_trades<'py>(
        &self,
        py: Python<'py>,
        inst_family: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let channel = Channel::StrucBlockTrades { inst_family };
            client
                .lock()
                .await
                .subscribe(vec![channel])
                .await
                .map_err(to_py_err)
        })
    }

    /// 订阅公开结构化大宗成交（公共）。
    #[pyo3(signature = (inst_family=None))]
    fn subscribe_public_struc_block_trades<'py>(
        &self,
        py: Python<'py>,
        inst_family: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let channel = Channel::PublicStrucBlockTrades { inst_family };
            client
                .lock()
                .await
                .subscribe(vec![channel])
                .await
                .map_err(to_py_err)
        })
    }

    /// 订阅公开大宗成交（公共）。
    #[pyo3(signature = (inst_family=None))]
    fn subscribe_public_block_trades<'py>(
        &self,
        py: Python<'py>,
        inst_family: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let channel = Channel::PublicBlockTrades { inst_family };
            client
                .lock()
                .await
                .subscribe(vec![channel])
                .await
                .map_err(to_py_err)
        })
    }

    /// 订阅 block tickers（公共）。
    #[pyo3(signature = (inst_family=None))]
    fn subscribe_block_tickers<'py>(
        &self,
        py: Python<'py>,
        inst_family: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let channel = Channel::BlockTickers { inst_family };
            client
                .lock()
                .await
                .subscribe(vec![channel])
                .await
                .map_err(to_py_err)
        })
    }

    /// 订阅现货网格订单（私有）。
    #[pyo3(signature = (algo_id=None, inst_id=None))]
    fn subscribe_grid_orders_spot<'py>(
        &self,
        py: Python<'py>,
        algo_id: Option<String>,
        inst_id: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let channel = Channel::GridOrdersSpot { algo_id, inst_id };
            client
                .lock()
                .await
                .subscribe(vec![channel])
                .await
                .map_err(to_py_err)
        })
    }

    /// 订阅合约网格订单（私有）。
    #[pyo3(signature = (algo_id=None, inst_id=None))]
    fn subscribe_grid_orders_contract<'py>(
        &self,
        py: Python<'py>,
        algo_id: Option<String>,
        inst_id: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let channel = Channel::GridOrdersContract { algo_id, inst_id };
            client
                .lock()
                .await
                .subscribe(vec![channel])
                .await
                .map_err(to_py_err)
        })
    }

    /// 订阅月亮网格订单（私有）。
    #[pyo3(signature = (algo_id=None, inst_id=None))]
    fn subscribe_grid_orders_moon<'py>(
        &self,
        py: Python<'py>,
        algo_id: Option<String>,
        inst_id: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let channel = Channel::GridOrdersMoon { algo_id, inst_id };
            client
                .lock()
                .await
                .subscribe(vec![channel])
                .await
                .map_err(to_py_err)
        })
    }

    /// 订阅网格仓位（私有）。
    #[pyo3(signature = (algo_id=None, inst_type=None, inst_id=None))]
    fn subscribe_grid_positions<'py>(
        &self,
        py: Python<'py>,
        algo_id: Option<String>,
        inst_type: Option<String>,
        inst_id: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let channel = Channel::GridPositions {
                algo_id,
                inst_type,
                inst_id,
            };
            client
                .lock()
                .await
                .subscribe(vec![channel])
                .await
                .map_err(to_py_err)
        })
    }

    /// 订阅网格子订单（私有）。
    #[pyo3(signature = (algo_id=None, inst_id=None))]
    fn subscribe_grid_sub_orders<'py>(
        &self,
        py: Python<'py>,
        algo_id: Option<String>,
        inst_id: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let channel = Channel::GridSubOrders { algo_id, inst_id };
            client
                .lock()
                .await
                .subscribe(vec![channel])
                .await
                .map_err(to_py_err)
        })
    }

    /// 订阅定投订单（私有）。
    #[pyo3(signature = (algo_id=None))]
    fn subscribe_algo_recurring_buy<'py>(
        &self,
        py: Python<'py>,
        algo_id: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let channel = Channel::AlgoRecurringBuy { algo_id };
            client
                .lock()
                .await
                .subscribe(vec![channel])
                .await
                .map_err(to_py_err)
        })
    }

    /// Receive the next message from the WebSocket.
    ///
    /// Returns:
    ///     dict with message data, or None if connection closed
    fn recv<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let mut guard = client.lock().await;
            match guard.next().await {
                Some(Ok(msg)) => Python::attach(|py| ws_message_to_py(py, msg)),
                Some(Err(e)) => Err(to_py_err(e)),
                None => Python::attach(|py| Ok(py.None())),
            }
        })
    }

    /// Check if the client is connected.
    fn is_connected<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            Ok(client.lock().await.is_connected())
        })
    }

    /// Manually trigger reconnection.
    fn reconnect<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            client.lock().await.reconnect().await.map_err(to_py_err)
        })
    }

    /// Login to the private WebSocket with an external Unix timestamp.
    ///
    /// This method allows using a server-synchronized timestamp instead of local time,
    /// which is useful when there's clock drift between client and server.
    ///
    /// Args:
    ///     timestamp_unix: Unix timestamp in seconds (as string, e.g., "1700000000")
    ///
    /// Raises:
    ///     OkxWebSocketError: If the connection is closed
    fn login_with_timestamp<'py>(
        &self,
        py: Python<'py>,
        timestamp_unix: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            client
                .lock()
                .await
                .login_with_timestamp(&timestamp_unix)
                .await
                .map_err(to_py_err)
        })
    }

    /// Set the timestamp provider for login during reconnection.
    ///
    /// When set, the provider will be used to get timestamps for login
    /// after automatic reconnection, ensuring consistent time synchronization.
    ///
    /// Args:
    ///     time_sync: A TimeSync instance to use as timestamp provider
    ///
    /// Example:
    ///     >>> time_sync = TimeSync(config)
    ///     >>> await time_sync.sync()
    ///     >>> client.set_timestamp_provider(time_sync)
    fn set_timestamp_provider<'py>(
        &self,
        py: Python<'py>,
        time_sync: &PyTimeSync,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        let provider: Arc<dyn TimestampProvider> = time_sync.inner.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            client.lock().await.set_timestamp_provider(provider);
            Ok(())
        })
    }

    /// Clear the timestamp provider (will use local time for login during reconnection).
    fn clear_timestamp_provider<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            client.lock().await.clear_timestamp_provider();
            Ok(())
        })
    }

    /// Close the WebSocket connection.
    fn close<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            client.lock().await.close().await.map_err(to_py_err)
        })
    }

    /// Get the number of active subscriptions.
    fn subscription_count<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            Ok(client.lock().await.subscription_count())
        })
    }

    fn __repr__(&self) -> String {
        format!(
            "WsClient(type='{}')",
            if self.is_private { "private" } else { "public" }
        )
    }

    /// Async iterator support.
    fn __aiter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    /// Async iterator next.
    fn __anext__<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        self.recv(py)
    }
}

/// Convert WsMessage to Python dict.
fn ws_message_to_py(py: Python<'_>, msg: WsMessage) -> PyResult<Py<PyAny>> {
    let dict = PyDict::new(py);

    match msg {
        WsMessage::Data { channel, arg, data } => {
            dict.set_item("type", "data")?;
            dict.set_item("channel", channel)?;
            dict.set_item("arg", arg.to_string())?;
            let data_list: Vec<String> = data.iter().map(|v| v.to_string()).collect();
            dict.set_item("data", data_list)?;
        }
        WsMessage::Event {
            event,
            arg,
            code,
            msg,
            conn_id,
        } => {
            dict.set_item("type", "event")?;
            dict.set_item("event", format!("{:?}", event))?;
            if let Some(a) = arg {
                dict.set_item("arg", a.to_string())?;
            }
            if let Some(c) = code {
                dict.set_item("code", c)?;
            }
            if let Some(m) = msg {
                dict.set_item("msg", m)?;
            }
            if let Some(id) = conn_id {
                dict.set_item("connId", id)?;
            }
        }
        WsMessage::Pong => {
            dict.set_item("type", "pong")?;
        }
        WsMessage::ChannelConnCount {
            channel,
            conn_count,
            conn_id,
        } => {
            dict.set_item("type", "channel_conn_count")?;
            dict.set_item("channel", channel)?;
            dict.set_item("connCount", conn_count)?;
            dict.set_item("connId", conn_id)?;
        }
        WsMessage::ChannelConnCountError { channel, code, msg } => {
            dict.set_item("type", "channel_conn_count_error")?;
            dict.set_item("channel", channel)?;
            dict.set_item("code", code)?;
            dict.set_item("msg", msg)?;
        }
        WsMessage::Unknown(text) => {
            dict.set_item("type", "unknown")?;
            dict.set_item("raw", text)?;
        }
    }

    Ok(dict.unbind().into())
}

#[cfg(test)]
mod tests {
    use super::*;
    use okx_ws::WsEvent;
    use pyo3::types::{PyDict, PyList};
    use pyo3::Python;
    use serde_json::{json, Value};

    #[test]
    fn ws_message_to_py_maps_data_branch() {
        Python::attach(|py| {
            let arg_value = json!({"channel":"tickers","instId":"BTC-USDT"});
            let msg = WsMessage::Data {
                channel: "tickers".to_string(),
                arg: arg_value.clone(),
                data: vec![json!({"px":"1.1"}), json!(3)],
            };

            let obj = ws_message_to_py(py, msg).expect("转换成功");
            let dict = obj.bind(py).cast::<PyDict>().expect("应为字典");

            let msg_type: String = dict
                .get_item("type")
                .expect("读取 type 失败")
                .expect("需包含 type")
                .extract()
                .unwrap();
            assert_eq!(msg_type, "data");
            let channel: String = dict
                .get_item("channel")
                .expect("读取 channel 失败")
                .expect("需包含 channel")
                .extract()
                .unwrap();
            assert_eq!(channel, "tickers");

            let arg: String = dict
                .get_item("arg")
                .expect("读取 arg 失败")
                .expect("需包含 arg")
                .extract()
                .unwrap();
            let parsed_arg: Value = serde_json::from_str(&arg).expect("arg 应为 JSON 字符串");
            assert_eq!(parsed_arg, arg_value);

            let data_obj = dict
                .get_item("data")
                .expect("读取 data 失败")
                .expect("需包含 data");
            let data = data_obj.cast::<PyList>().expect("data 应为列表");
            assert_eq!(data.len(), 2);
            let first: String = data.get_item(0).expect("需有首元素").extract().unwrap();
            let parsed_first: Value = serde_json::from_str(&first).unwrap();
            assert_eq!(parsed_first, json!({"px":"1.1"}));
        });
    }

    #[test]
    fn ws_message_to_py_maps_event_branch_with_optionals() {
        Python::attach(|py| {
            let msg = WsMessage::Event {
                event: WsEvent::Error,
                arg: Some(json!({"channel":"orders"})),
                code: Some("51000".into()),
                msg: Some("rejected".into()),
                conn_id: Some("cid-1".into()),
            };

            let obj = ws_message_to_py(py, msg).expect("转换成功");
            let dict = obj.bind(py).cast::<PyDict>().expect("应为字典");

            let msg_type: String = dict
                .get_item("type")
                .expect("读取 type 失败")
                .expect("需包含 type")
                .extract()
                .unwrap();
            assert_eq!(msg_type, "event");
            let event: String = dict
                .get_item("event")
                .expect("读取 event 失败")
                .expect("需包含 event")
                .extract()
                .unwrap();
            assert_eq!(event, "Error");

            let arg: String = dict
                .get_item("arg")
                .expect("读取 arg 失败")
                .expect("需包含 arg")
                .extract()
                .unwrap();
            let parsed_arg: Value = serde_json::from_str(&arg).unwrap();
            assert_eq!(parsed_arg, json!({"channel":"orders"}));

            assert_eq!(
                dict.get_item("code")
                    .expect("读取 code 失败")
                    .expect("需包含 code")
                    .extract::<String>()
                    .unwrap(),
                "51000"
            );
            assert_eq!(
                dict.get_item("msg")
                    .expect("读取 msg 失败")
                    .expect("需包含 msg")
                    .extract::<String>()
                    .unwrap(),
                "rejected"
            );
            assert_eq!(
                dict.get_item("connId")
                    .expect("读取 connId 失败")
                    .expect("需包含 connId")
                    .extract::<String>()
                    .unwrap(),
                "cid-1"
            );
        });
    }

    #[test]
    fn ws_message_to_py_maps_pong_and_unknown() {
        Python::attach(|py| {
            let pong = ws_message_to_py(py, WsMessage::Pong).expect("转换成功");
            let pong_dict = pong.bind(py).cast::<PyDict>().expect("应为字典");
            assert_eq!(
                pong_dict
                    .get_item("type")
                    .expect("读取 type 失败")
                    .expect("需包含 type")
                    .extract::<String>()
                    .unwrap(),
                "pong"
            );

            let raw = "weird payload";
            let unknown = ws_message_to_py(py, WsMessage::Unknown(raw.into())).expect("转换成功");
            let unknown_dict = unknown.bind(py).cast::<PyDict>().expect("应为字典");
            assert_eq!(
                unknown_dict
                    .get_item("type")
                    .expect("读取 type 失败")
                    .expect("需包含 type")
                    .extract::<String>()
                    .unwrap(),
                "unknown"
            );
            assert_eq!(
                unknown_dict
                    .get_item("raw")
                    .expect("读取 raw 失败")
                    .expect("需包含 raw")
                    .extract::<String>()
                    .unwrap(),
                raw
            );
        });
    }

    #[test]
    fn ws_message_to_py_maps_channel_conn_count() {
        Python::attach(|py| {
            let msg = WsMessage::ChannelConnCount {
                channel: "tickers".to_string(),
                conn_count: 5,
                conn_id: "conn-123".to_string(),
            };

            let obj = ws_message_to_py(py, msg).expect("转换成功");
            let dict = obj.bind(py).cast::<PyDict>().expect("应为字典");

            assert_eq!(
                dict.get_item("type")
                    .expect("读取 type 失败")
                    .expect("需包含 type")
                    .extract::<String>()
                    .unwrap(),
                "channel_conn_count"
            );
            assert_eq!(
                dict.get_item("channel")
                    .expect("读取 channel 失败")
                    .expect("需包含 channel")
                    .extract::<String>()
                    .unwrap(),
                "tickers"
            );
            assert_eq!(
                dict.get_item("connCount")
                    .expect("读取 connCount 失败")
                    .expect("需包含 connCount")
                    .extract::<u32>()
                    .unwrap(),
                5
            );
            assert_eq!(
                dict.get_item("connId")
                    .expect("读取 connId 失败")
                    .expect("需包含 connId")
                    .extract::<String>()
                    .unwrap(),
                "conn-123"
            );
        });
    }

    #[test]
    fn ws_message_to_py_maps_channel_conn_count_error() {
        Python::attach(|py| {
            let msg = WsMessage::ChannelConnCountError {
                channel: "orders".to_string(),
                code: "60001".to_string(),
                msg: "Invalid channel".to_string(),
            };

            let obj = ws_message_to_py(py, msg).expect("转换成功");
            let dict = obj.bind(py).cast::<PyDict>().expect("应为字典");

            assert_eq!(
                dict.get_item("type")
                    .expect("读取 type 失败")
                    .expect("需包含 type")
                    .extract::<String>()
                    .unwrap(),
                "channel_conn_count_error"
            );
            assert_eq!(
                dict.get_item("channel")
                    .expect("读取 channel 失败")
                    .expect("需包含 channel")
                    .extract::<String>()
                    .unwrap(),
                "orders"
            );
            assert_eq!(
                dict.get_item("code")
                    .expect("读取 code 失败")
                    .expect("需包含 code")
                    .extract::<String>()
                    .unwrap(),
                "60001"
            );
            assert_eq!(
                dict.get_item("msg")
                    .expect("读取 msg 失败")
                    .expect("需包含 msg")
                    .extract::<String>()
                    .unwrap(),
                "Invalid channel"
            );
        });
    }
}
