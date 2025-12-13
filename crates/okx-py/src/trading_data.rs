//! Trading Data（Rubik）域绑定的同步/异步共享实现。

use pyo3::prelude::*;
use serde_json::Value;

use okx_rest::TradingDataApi;

use crate::{map_values, parse_json_value, PyAsyncOkxClient, PyOkxClient};

fn parse_params(params_json: Option<&str>) -> PyResult<Option<Value>> {
    parse_json_value(params_json, "params")
}

pub(crate) mod sync {
    use super::*;

    pub(crate) fn get_support_coin(client: &PyOkxClient) -> PyResult<Vec<Py<PyAny>>> {
        let res = client
            .block_on_allow_threads(async { client.rest_client().get_support_coin().await })?;
        map_values(Ok(res))
    }

    pub(crate) fn get_taker_volume(
        client: &PyOkxClient,
        params_json: Option<&str>,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let params = parse_params(params_json)?;
        let res = client.block_on_allow_threads(async {
            client.rest_client().get_taker_volume(params).await
        })?;
        map_values(Ok(res))
    }

    pub(crate) fn get_margin_lending_ratio(
        client: &PyOkxClient,
        params_json: Option<&str>,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let params = parse_params(params_json)?;
        let res = client.block_on_allow_threads(async {
            client.rest_client().get_margin_lending_ratio(params).await
        })?;
        map_values(Ok(res))
    }

    pub(crate) fn get_long_short_ratio(
        client: &PyOkxClient,
        params_json: Option<&str>,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let params = parse_params(params_json)?;
        let res = client.block_on_allow_threads(async {
            client.rest_client().get_long_short_ratio(params).await
        })?;
        map_values(Ok(res))
    }

    pub(crate) fn get_contracts_open_interest_volume(
        client: &PyOkxClient,
        params_json: Option<&str>,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let params = parse_params(params_json)?;
        let res = client.block_on_allow_threads(async {
            client
                .rest_client()
                .get_contracts_open_interest_volume(params)
                .await
        })?;
        map_values(Ok(res))
    }

    pub(crate) fn get_options_open_interest_volume(
        client: &PyOkxClient,
        params_json: Option<&str>,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let params = parse_params(params_json)?;
        let res = client.block_on_allow_threads(async {
            client
                .rest_client()
                .get_options_open_interest_volume(params)
                .await
        })?;
        map_values(Ok(res))
    }

    pub(crate) fn get_put_call_ratio(
        client: &PyOkxClient,
        params_json: Option<&str>,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let params = parse_params(params_json)?;
        let res = client.block_on_allow_threads(async {
            client.rest_client().get_put_call_ratio(params).await
        })?;
        map_values(Ok(res))
    }

    pub(crate) fn get_open_interest_volume_expiry(
        client: &PyOkxClient,
        params_json: Option<&str>,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let params = parse_params(params_json)?;
        let res = client.block_on_allow_threads(async {
            client
                .rest_client()
                .get_open_interest_volume_expiry(params)
                .await
        })?;
        map_values(Ok(res))
    }

    pub(crate) fn get_interest_volume_strike(
        client: &PyOkxClient,
        params_json: Option<&str>,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let params = parse_params(params_json)?;
        let res = client.block_on_allow_threads(async {
            client
                .rest_client()
                .get_interest_volume_strike(params)
                .await
        })?;
        map_values(Ok(res))
    }

    pub(crate) fn get_taker_flow(
        client: &PyOkxClient,
        params_json: Option<&str>,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let params = parse_params(params_json)?;
        let res = client
            .block_on_allow_threads(async { client.rest_client().get_taker_flow(params).await })?;
        map_values(Ok(res))
    }
}

pub(crate) mod async_api {
    use super::*;

    pub(crate) fn get_support_coin<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(rest.get_support_coin().await)
        })
    }

    pub(crate) fn get_taker_volume<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        params_json: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        let params = parse_params(params_json.as_deref())?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(rest.get_taker_volume(params).await)
        })
    }

    pub(crate) fn get_margin_lending_ratio<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        params_json: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        let params = parse_params(params_json.as_deref())?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(rest.get_margin_lending_ratio(params).await)
        })
    }

    pub(crate) fn get_long_short_ratio<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        params_json: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        let params = parse_params(params_json.as_deref())?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(rest.get_long_short_ratio(params).await)
        })
    }

    pub(crate) fn get_contracts_open_interest_volume<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        params_json: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        let params = parse_params(params_json.as_deref())?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(rest.get_contracts_open_interest_volume(params).await)
        })
    }

    pub(crate) fn get_options_open_interest_volume<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        params_json: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        let params = parse_params(params_json.as_deref())?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(rest.get_options_open_interest_volume(params).await)
        })
    }

    pub(crate) fn get_put_call_ratio<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        params_json: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        let params = parse_params(params_json.as_deref())?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(rest.get_put_call_ratio(params).await)
        })
    }

    pub(crate) fn get_open_interest_volume_expiry<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        params_json: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        let params = parse_params(params_json.as_deref())?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(rest.get_open_interest_volume_expiry(params).await)
        })
    }

    pub(crate) fn get_interest_volume_strike<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        params_json: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        let params = parse_params(params_json.as_deref())?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(rest.get_interest_volume_strike(params).await)
        })
    }

    pub(crate) fn get_taker_flow<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        params_json: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        let params = parse_params(params_json.as_deref())?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(rest.get_taker_flow(params).await)
        })
    }
}
