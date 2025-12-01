//! # okx-py
//!
//! Python bindings for OKX SDK using PyO3.
//!
//! This crate exposes Rust functionality to Python.
//!
//! ## Features
//!
//! - Synchronous client (`OkxClient`) for simple use cases
//! - Asynchronous client (`AsyncOkxClient`) for high-performance async code
//! - WebSocket client (`WsClient`) for real-time data streaming
//!
//! ## Example (Sync)
//!
//! ```python
//! from okx_py import OkxClient, Config, Credentials
//!
//! creds = Credentials("api_key", "secret_key", "passphrase")
//! config = Config(creds, simulated=True)
//! client = OkxClient(config)
//!
//! balance = client.get_balance()
//! ```
//!
//! ## Example (Async)
//!
//! ```python
//! import asyncio
//! from okx_py import AsyncOkxClient, Config, Credentials
//!
//! async def main():
//!     creds = Credentials("api_key", "secret_key", "passphrase")
//!     config = Config(creds, simulated=True)
//!     client = AsyncOkxClient(config)
//!
//!     balance = await client.get_balance()
//!
//! asyncio.run(main())
//! ```

use pyo3::exceptions::{PyRuntimeError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::PyModule;
use serde_json::Value;

mod async_client;
mod client;
mod types;
mod ws_client;

use async_client::*;
use client::*;
use types::*;
use ws_client::*;

/// Python module for OKX SDK.
#[pymodule]
fn okx_py(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;

    // Core types
    m.add_class::<PyCredentials>()?;
    m.add_class::<PyConfig>()?;

    // REST clients
    m.add_class::<PyOkxClient>()?;
    m.add_class::<PyAsyncOkxClient>()?;

    // WebSocket client
    m.add_class::<PyWsClient>()?;

    // Data types
    m.add_class::<PyBalance>()?;
    m.add_class::<PyBalanceDetail>()?;
    m.add_class::<PyPosition>()?;
    m.add_class::<PyOrder>()?;
    m.add_class::<PyTicker>()?;

    Ok(())
}

/// Convert OKX error to Python exception.
fn to_py_err(e: okx_core::OkxError) -> PyErr {
    PyRuntimeError::new_err(e.to_string())
}

/// 将 `serde_json::Value` 列表转换为 Python 对象列表。
pub(crate) fn values_to_py_list(values: Vec<Value>) -> PyResult<Vec<Py<PyAny>>> {
    Python::attach(|py| {
        let json = PyModule::import(py, "json")?;
        values
            .into_iter()
            .map(|v| {
                let s = serde_json::to_string(&v)
                    .map_err(|e| PyRuntimeError::new_err(format!("序列化失败: {e}")))?;
                let obj = json.call_method1("loads", (s,))?;
                Ok(obj.into())
            })
            .collect()
    })
}

/// 解析 JSON 字符串为 `serde_json::Value`。
pub(crate) fn parse_json_value(input: Option<&str>, field: &str) -> PyResult<Option<Value>> {
    match input {
        None => Ok(None),
        Some(s) if s.trim().is_empty() => Ok(None),
        Some(s) => serde_json::from_str(s)
            .map(Some)
            .map_err(|e| PyValueError::new_err(format!("{field} JSON 解析失败: {e}"))),
    }
}

/// 解析 JSON 字符串为 `Vec<Value>`。
pub(crate) fn parse_json_array(input: Option<&str>, field: &str) -> PyResult<Option<Vec<Value>>> {
    match input {
        None => Ok(None),
        Some(s) if s.trim().is_empty() => Ok(None),
        Some(s) => serde_json::from_str(s)
            .map(Some)
            .map_err(|e| PyValueError::new_err(format!("{field} JSON 数组解析失败: {e}"))),
    }
}

/// 将 OKX Result<Vec<Value>> 映射为 Python 对象列表。
pub(crate) fn map_values(res: okx_core::Result<Vec<Value>>) -> PyResult<Vec<Py<PyAny>>> {
    res.map_err(to_py_err).and_then(values_to_py_list)
}

#[cfg(test)]
mod tests {}
