//! Spread Trading 域绑定的同步/异步共享实现。

use pyo3::prelude::*;

use okx_rest::SpreadApi;

use crate::{map_values, parse_json_value, PyAsyncOkxClient, PyOkxClient, PyRuntimeError};

fn parse_params(params_json: Option<&str>) -> PyResult<Option<serde_json::Value>> {
    parse_json_value(params_json, "params")
}

fn parse_payload(payload_json: &str) -> PyResult<serde_json::Value> {
    parse_json_value(Some(payload_json), "payload")?
        .ok_or_else(|| PyRuntimeError::new_err("payload 不能为空"))
}

pub(crate) mod sync {
    use super::*;

    pub(crate) fn place_order(
        client: &PyOkxClient,
        payload_json: &str,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let payload = parse_payload(payload_json)?;
        let res = client.block_on_allow_threads(async {
            client.rest_client().spread_place_order(payload).await
        })?;
        map_values(Ok(res))
    }

    pub(crate) fn cancel_order(
        client: &PyOkxClient,
        payload_json: &str,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let payload = parse_payload(payload_json)?;
        let res = client.block_on_allow_threads(async {
            client.rest_client().spread_cancel_order(payload).await
        })?;
        map_values(Ok(res))
    }

    pub(crate) fn cancel_all_orders(
        client: &PyOkxClient,
        payload_json: &str,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let payload = parse_payload(payload_json)?;
        let res = client.block_on_allow_threads(async {
            client.rest_client().spread_cancel_all_orders(payload).await
        })?;
        map_values(Ok(res))
    }

    pub(crate) fn get_order_details(
        client: &PyOkxClient,
        params_json: &str,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let params = parse_json_value(Some(params_json), "params")?
            .ok_or_else(|| PyRuntimeError::new_err("params 不能为空"))?;
        let res = client.block_on_allow_threads(async {
            client.rest_client().spread_get_order_details(params).await
        })?;
        map_values(Ok(res))
    }

    pub(crate) fn get_active_orders(
        client: &PyOkxClient,
        params_json: Option<&str>,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let params = parse_params(params_json)?;
        let res = client.block_on_allow_threads(async {
            client.rest_client().spread_get_active_orders(params).await
        })?;
        map_values(Ok(res))
    }

    pub(crate) fn get_orders(
        client: &PyOkxClient,
        params_json: Option<&str>,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let params = parse_params(params_json)?;
        let res = client.block_on_allow_threads(async {
            client.rest_client().spread_get_orders(params).await
        })?;
        map_values(Ok(res))
    }

    pub(crate) fn get_trades(
        client: &PyOkxClient,
        params_json: Option<&str>,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let params = parse_params(params_json)?;
        let res = client.block_on_allow_threads(async {
            client.rest_client().spread_get_trades(params).await
        })?;
        map_values(Ok(res))
    }

    pub(crate) fn get_spreads(
        client: &PyOkxClient,
        params_json: Option<&str>,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let params = parse_params(params_json)?;
        let res = client.block_on_allow_threads(async {
            client.rest_client().spread_get_spreads(params).await
        })?;
        map_values(Ok(res))
    }

    pub(crate) fn get_order_book(
        client: &PyOkxClient,
        params_json: &str,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let params = parse_json_value(Some(params_json), "params")?
            .ok_or_else(|| PyRuntimeError::new_err("params 不能为空"))?;
        let res = client.block_on_allow_threads(async {
            client.rest_client().spread_get_order_book(params).await
        })?;
        map_values(Ok(res))
    }

    pub(crate) fn get_ticker(client: &PyOkxClient, params_json: &str) -> PyResult<Vec<Py<PyAny>>> {
        let params = parse_json_value(Some(params_json), "params")?
            .ok_or_else(|| PyRuntimeError::new_err("params 不能为空"))?;
        let res = client.block_on_allow_threads(async {
            client.rest_client().spread_get_ticker(params).await
        })?;
        map_values(Ok(res))
    }

    pub(crate) fn get_public_trades(
        client: &PyOkxClient,
        params_json: &str,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let params = parse_json_value(Some(params_json), "params")?
            .ok_or_else(|| PyRuntimeError::new_err("params 不能为空"))?;
        let res = client.block_on_allow_threads(async {
            client.rest_client().spread_get_public_trades(params).await
        })?;
        map_values(Ok(res))
    }
}

pub(crate) mod async_api {
    use super::*;

    pub(crate) fn place_order<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        payload_json: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        let payload = parse_payload(&payload_json)?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(rest.spread_place_order(payload).await)
        })
    }

    pub(crate) fn cancel_order<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        payload_json: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        let payload = parse_payload(&payload_json)?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(rest.spread_cancel_order(payload).await)
        })
    }

    pub(crate) fn cancel_all_orders<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        payload_json: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        let payload = parse_payload(&payload_json)?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(rest.spread_cancel_all_orders(payload).await)
        })
    }

    pub(crate) fn get_order_details<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        params_json: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        let params = parse_json_value(Some(&params_json), "params")?
            .ok_or_else(|| PyRuntimeError::new_err("params 不能为空"))?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(rest.spread_get_order_details(params).await)
        })
    }

    pub(crate) fn get_active_orders<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        params_json: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        let params = parse_params(params_json.as_deref())?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(rest.spread_get_active_orders(params).await)
        })
    }

    pub(crate) fn get_orders<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        params_json: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        let params = parse_params(params_json.as_deref())?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(rest.spread_get_orders(params).await)
        })
    }

    pub(crate) fn get_trades<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        params_json: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        let params = parse_params(params_json.as_deref())?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(rest.spread_get_trades(params).await)
        })
    }

    pub(crate) fn get_spreads<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        params_json: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        let params = parse_params(params_json.as_deref())?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(rest.spread_get_spreads(params).await)
        })
    }

    pub(crate) fn get_order_book<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        params_json: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        let params = parse_json_value(Some(&params_json), "params")?
            .ok_or_else(|| PyRuntimeError::new_err("params 不能为空"))?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(rest.spread_get_order_book(params).await)
        })
    }

    pub(crate) fn get_ticker<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        params_json: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        let params = parse_json_value(Some(&params_json), "params")?
            .ok_or_else(|| PyRuntimeError::new_err("params 不能为空"))?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(rest.spread_get_ticker(params).await)
        })
    }

    pub(crate) fn get_public_trades<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        params_json: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        let params = parse_json_value(Some(&params_json), "params")?
            .ok_or_else(|| PyRuntimeError::new_err("params 不能为空"))?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(rest.spread_get_public_trades(params).await)
        })
    }
}
