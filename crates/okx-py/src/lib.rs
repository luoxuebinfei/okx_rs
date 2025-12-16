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

#![allow(clippy::too_many_arguments)]
// PyO3 绑定需要直接暴露 OKX 的参数表，很多方法参数数量天然偏多；
// 该 lint 在此类“外部 API 适配层”上价值有限，统一放宽以避免噪音阻塞 clippy。

use pyo3::exceptions::{PyRuntimeError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::PyModule;
use pyo3::{create_exception, import_exception};
use serde_json::Value;

// 导入 Python 内置异常
import_exception!(builtins, ConnectionError);

// 自定义 OKX 异常层次结构
create_exception!(okx_py, OkxError, pyo3::exceptions::PyException);
create_exception!(okx_py, OkxHttpError, OkxError);
create_exception!(okx_py, OkxRateLimitError, OkxHttpError);
create_exception!(okx_py, OkxApiError, OkxError);
create_exception!(okx_py, OkxAuthError, OkxError);
create_exception!(okx_py, OkxWebSocketError, OkxError);
create_exception!(okx_py, OkxTimeoutError, OkxError);

mod account;
mod async_client;
mod block_rfq;
mod broker;
mod client;
mod convert;
mod copy_trading;
mod finance;
mod funding;
mod grid;
mod public;
mod raw;
mod response_meta;
mod spread;
mod subaccount;
mod time_sync;
mod trading_data;
mod types;
mod ws_client;

use async_client::*;
use client::*;
use response_meta::*;
use time_sync::*;
use types::*;
use ws_client::*;

/// Python module for OKX SDK.
#[pymodule]
fn okx_py(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;

    // 异常类型
    m.add("OkxError", m.py().get_type::<OkxError>())?;
    m.add("OkxHttpError", m.py().get_type::<OkxHttpError>())?;
    m.add("OkxRateLimitError", m.py().get_type::<OkxRateLimitError>())?;
    m.add("OkxApiError", m.py().get_type::<OkxApiError>())?;
    m.add("OkxAuthError", m.py().get_type::<OkxAuthError>())?;
    m.add("OkxWebSocketError", m.py().get_type::<OkxWebSocketError>())?;
    m.add("OkxTimeoutError", m.py().get_type::<OkxTimeoutError>())?;

    // Core types
    m.add_class::<PyCredentials>()?;
    m.add_class::<PyConfig>()?;

    // REST clients
    m.add_class::<PyOkxClient>()?;
    m.add_class::<PyAsyncOkxClient>()?;

    // WebSocket client
    m.add_class::<PyWsClient>()?;

    // Time synchronization
    m.add_class::<PyTimeSync>()?;

    // Response metadata
    m.add_class::<PyResponseMeta>()?;

    // Data types
    m.add_class::<PyBalance>()?;
    m.add_class::<PyBalanceDetail>()?;
    m.add_class::<PyPosition>()?;
    m.add_class::<PyOrder>()?;
    m.add_class::<PyTicker>()?;

    Ok(())
}

/// Convert OKX error to Python exception.
///
/// 根据错误类型返回对应的 Python 异常：
/// - `HttpStatus` 429 -> `OkxRateLimitError`
/// - `HttpStatus` 其他 -> `OkxHttpError`
/// - `Api` -> `OkxApiError`
/// - `Auth` -> `OkxAuthError`
/// - `WebSocket` / `ConnectionClosed` -> `OkxWebSocketError`
/// - `Timeout` -> `OkxTimeoutError`
/// - 其他 -> `OkxError`
fn to_py_err(e: okx_core::OkxError) -> PyErr {
    use okx_core::OkxError as E;
    match e {
        E::HttpStatus { status, ref body } => {
            let msg = format!("HTTP {status}: {body}");
            if status == 429 {
                OkxRateLimitError::new_err(msg)
            } else {
                OkxHttpError::new_err(msg)
            }
        }
        E::Api { ref code, ref msg } => OkxApiError::new_err(format!("API error [{code}]: {msg}")),
        E::Auth(ref msg) => OkxAuthError::new_err(msg.clone()),
        E::WebSocket(ref msg) => OkxWebSocketError::new_err(msg.clone()),
        E::ConnectionClosed => OkxWebSocketError::new_err("Connection closed"),
        E::Timeout => OkxTimeoutError::new_err("Request timeout"),
        _ => OkxError::new_err(e.to_string()),
    }
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

/// 将单个 `serde_json::Value` 转换为 Python 对象。
pub(crate) fn value_to_py_obj(value: Value) -> PyResult<Py<PyAny>> {
    Python::attach(|py| {
        let json = PyModule::import(py, "json")?;
        let s = serde_json::to_string(&value)
            .map_err(|e| PyRuntimeError::new_err(format!("序列化失败: {e}")))?;
        let obj = json.call_method1("loads", (s,))?;
        Ok(obj.into())
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

/// 解析必填 JSON 字符串为 `serde_json::Value`。
///
/// 与 `parse_json_value` 的差异：
/// - 该函数将 `""/空白` 视为错误，而非 `None`
pub(crate) fn parse_required_json_value(input: &str, field: &str) -> PyResult<Value> {
    let parsed = parse_json_value(Some(input), field)?;
    parsed.ok_or_else(|| PyValueError::new_err(format!("{field} 不能为空")))
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
mod tests {
    use super::*;
    use okx_core::OkxError;
    use pyo3::exceptions::{PyRuntimeError, PyValueError};
    use pyo3::types::{PyDict, PyList};
    use pyo3::Python;
    use serde_json::json;

    #[test]
    fn parse_json_value_handles_optional_inputs() {
        assert!(parse_json_value(None, "body").unwrap().is_none());
        assert!(parse_json_value(Some("   "), "body").unwrap().is_none());

        let parsed = parse_json_value(Some(r#"{ "a": 1 }"#), "body")
            .expect("应解析 JSON")
            .unwrap();
        assert_eq!(parsed["a"], 1);
    }

    #[test]
    fn parse_json_value_reports_error_for_invalid_json() {
        let err = parse_json_value(Some("{oops"), "payload").unwrap_err();
        Python::attach(|py| {
            assert!(err.is_instance_of::<PyValueError>(py));
            assert!(
                err.to_string().contains("payload JSON 解析失败"),
                "错误信息需包含字段名"
            );
        });
    }

    #[test]
    fn parse_json_array_covers_none_and_invalid_cases() {
        assert!(parse_json_array(None, "items").unwrap().is_none());
        assert!(parse_json_array(Some(""), "items").unwrap().is_none());

        let arr = parse_json_array(Some(r#"["a", 2]"#), "items")
            .expect("应成功解析数组")
            .unwrap();
        assert_eq!(arr.len(), 2);

        let err = parse_json_array(Some("not array"), "items").unwrap_err();
        Python::attach(|py| assert!(err.is_instance_of::<PyValueError>(py)));
    }

    #[test]
    fn values_to_py_list_roundtrips_json_values() {
        Python::attach(|py| {
            let values = vec![json!({"k": 1}), json!(["x", 2])];
            let py_values = values_to_py_list(values).expect("转换应成功");
            assert_eq!(py_values.len(), 2);

            let first = py_values[0].bind(py).cast::<PyDict>().expect("应为字典");
            assert_eq!(
                first
                    .get_item("k")
                    .expect("读取 k 失败")
                    .expect("需包含键")
                    .extract::<i64>()
                    .expect("应为整数"),
                1
            );

            let second = py_values[1].bind(py).cast::<PyList>().expect("应为列表");
            assert_eq!(
                second
                    .get_item(0)
                    .expect("读取索引 0 失败")
                    .extract::<String>()
                    .unwrap(),
                "x"
            );
            assert_eq!(
                second
                    .get_item(1)
                    .expect("读取索引 1 失败")
                    .extract::<i64>()
                    .unwrap(),
                2
            );
        });
    }

    #[test]
    fn map_values_converts_errors_to_python_runtime_error() {
        Python::attach(|py| {
            let err = map_values(Err(OkxError::Http("boom".into()))).unwrap_err();
            assert!(err.is_instance_of::<PyRuntimeError>(py));
            assert!(err.to_string().contains("HTTP error: boom"));
        });
    }
}
