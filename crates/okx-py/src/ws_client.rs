//! WebSocket client Python bindings.

use std::sync::Arc;

use futures_util::StreamExt;
use pyo3::prelude::*;
use pyo3::types::PyDict;
use tokio::sync::Mutex;

use okx_ws::{Channel, ConnectionType, ReconnectConfig, ReconnectingWsClient, WsMessage};

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
        WsMessage::Unknown(text) => {
            dict.set_item("type", "unknown")?;
            dict.set_item("raw", text)?;
        }
    }

    Ok(dict.unbind().into())
}

#[cfg(test)]
mod tests {}
