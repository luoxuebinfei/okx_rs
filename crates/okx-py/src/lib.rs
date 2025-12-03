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
