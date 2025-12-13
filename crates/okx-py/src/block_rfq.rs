//! Block RFQ（大宗交易 / RFQ）域绑定的同步/异步共享实现。

use pyo3::prelude::*;
use serde_json::Value;

use okx_rest::BlockRfqApi;

use crate::{
    map_values, parse_json_value, parse_required_json_value, PyAsyncOkxClient, PyOkxClient,
};

fn parse_params(params_json: Option<&str>) -> PyResult<Option<Value>> {
    parse_json_value(params_json, "params_json")
}

fn parse_request(request_json: &str) -> PyResult<Value> {
    parse_required_json_value(request_json, "request_json")
}

pub(crate) mod sync {
    use super::*;

    pub(crate) fn get_counterparties(client: &PyOkxClient) -> PyResult<Vec<Py<PyAny>>> {
        let res = client
            .block_on_allow_threads(async { client.rest_client().get_counterparties().await })?;
        map_values(Ok(res))
    }

    pub(crate) fn create_rfq(client: &PyOkxClient, request_json: &str) -> PyResult<Vec<Py<PyAny>>> {
        let request = parse_request(request_json)?;
        let res = client
            .block_on_allow_threads(async { client.rest_client().create_rfq(request).await })?;
        map_values(Ok(res))
    }

    pub(crate) fn cancel_rfq(client: &PyOkxClient, request_json: &str) -> PyResult<Vec<Py<PyAny>>> {
        let request = parse_request(request_json)?;
        let res = client
            .block_on_allow_threads(async { client.rest_client().cancel_rfq(request).await })?;
        map_values(Ok(res))
    }

    pub(crate) fn cancel_batch_rfqs(
        client: &PyOkxClient,
        request_json: &str,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let request = parse_request(request_json)?;
        let res = client.block_on_allow_threads(async {
            client.rest_client().cancel_batch_rfqs(request).await
        })?;
        map_values(Ok(res))
    }

    pub(crate) fn cancel_all_rfqs(
        client: &PyOkxClient,
        request_json: &str,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let request = parse_request(request_json)?;
        let res = client.block_on_allow_threads(async {
            client.rest_client().cancel_all_rfqs(request).await
        })?;
        map_values(Ok(res))
    }

    pub(crate) fn execute_quote(
        client: &PyOkxClient,
        request_json: &str,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let request = parse_request(request_json)?;
        let res = client
            .block_on_allow_threads(async { client.rest_client().execute_quote(request).await })?;
        map_values(Ok(res))
    }

    pub(crate) fn create_quote(
        client: &PyOkxClient,
        request_json: &str,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let request = parse_request(request_json)?;
        let res = client
            .block_on_allow_threads(async { client.rest_client().create_quote(request).await })?;
        map_values(Ok(res))
    }

    pub(crate) fn cancel_quote(
        client: &PyOkxClient,
        request_json: &str,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let request = parse_request(request_json)?;
        let res = client
            .block_on_allow_threads(async { client.rest_client().cancel_quote(request).await })?;
        map_values(Ok(res))
    }

    pub(crate) fn cancel_batch_quotes(
        client: &PyOkxClient,
        request_json: &str,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let request = parse_request(request_json)?;
        let res = client.block_on_allow_threads(async {
            client.rest_client().cancel_batch_quotes(request).await
        })?;
        map_values(Ok(res))
    }

    pub(crate) fn cancel_all_quotes(
        client: &PyOkxClient,
        request_json: &str,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let request = parse_request(request_json)?;
        let res = client.block_on_allow_threads(async {
            client.rest_client().cancel_all_quotes(request).await
        })?;
        map_values(Ok(res))
    }

    pub(crate) fn get_rfqs(
        client: &PyOkxClient,
        params_json: Option<&str>,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let params = parse_params(params_json)?;
        let res =
            client.block_on_allow_threads(async { client.rest_client().get_rfqs(params).await })?;
        map_values(Ok(res))
    }

    pub(crate) fn get_quotes(
        client: &PyOkxClient,
        params_json: Option<&str>,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let params = parse_params(params_json)?;
        let res = client
            .block_on_allow_threads(async { client.rest_client().get_quotes(params).await })?;
        map_values(Ok(res))
    }

    pub(crate) fn get_trades(
        client: &PyOkxClient,
        params_json: Option<&str>,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let params = parse_params(params_json)?;
        let res = client
            .block_on_allow_threads(async { client.rest_client().get_trades(params).await })?;
        map_values(Ok(res))
    }

    pub(crate) fn get_public_trades(
        client: &PyOkxClient,
        params_json: Option<&str>,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let params = parse_params(params_json)?;
        let res = client.block_on_allow_threads(async {
            client.rest_client().get_public_trades(params).await
        })?;
        map_values(Ok(res))
    }

    pub(crate) fn reset_mmp(client: &PyOkxClient, request_json: &str) -> PyResult<Vec<Py<PyAny>>> {
        let request = parse_request(request_json)?;
        let res = client
            .block_on_allow_threads(async { client.rest_client().reset_mmp(request).await })?;
        map_values(Ok(res))
    }

    pub(crate) fn set_mmp_config(
        client: &PyOkxClient,
        request_json: &str,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let request = parse_request(request_json)?;
        let res = client
            .block_on_allow_threads(async { client.rest_client().set_mmp_config(request).await })?;
        map_values(Ok(res))
    }

    pub(crate) fn get_mmp_config(client: &PyOkxClient) -> PyResult<Vec<Py<PyAny>>> {
        let res =
            client.block_on_allow_threads(async { client.rest_client().get_mmp_config().await })?;
        map_values(Ok(res))
    }

    pub(crate) fn set_marker_instrument(
        client: &PyOkxClient,
        request_json: &str,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let request = parse_request(request_json)?;
        let res = client.block_on_allow_threads(async {
            client.rest_client().set_marker_instrument(request).await
        })?;
        map_values(Ok(res))
    }

    pub(crate) fn get_quote_products(
        client: &PyOkxClient,
        params_json: Option<&str>,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let params = parse_params(params_json)?;
        let res = client.block_on_allow_threads(async {
            client.rest_client().get_quote_products(params).await
        })?;
        map_values(Ok(res))
    }
}

pub(crate) mod async_api {
    use super::*;

    pub(crate) fn get_counterparties<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(rest.get_counterparties().await)
        })
    }

    pub(crate) fn create_rfq<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        request_json: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        let request = parse_request(&request_json)?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(rest.create_rfq(request).await)
        })
    }

    pub(crate) fn cancel_rfq<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        request_json: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        let request = parse_request(&request_json)?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(rest.cancel_rfq(request).await)
        })
    }

    pub(crate) fn cancel_batch_rfqs<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        request_json: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        let request = parse_request(&request_json)?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(rest.cancel_batch_rfqs(request).await)
        })
    }

    pub(crate) fn cancel_all_rfqs<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        request_json: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        let request = parse_request(&request_json)?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(rest.cancel_all_rfqs(request).await)
        })
    }

    pub(crate) fn execute_quote<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        request_json: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        let request = parse_request(&request_json)?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(rest.execute_quote(request).await)
        })
    }

    pub(crate) fn create_quote<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        request_json: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        let request = parse_request(&request_json)?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(rest.create_quote(request).await)
        })
    }

    pub(crate) fn cancel_quote<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        request_json: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        let request = parse_request(&request_json)?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(rest.cancel_quote(request).await)
        })
    }

    pub(crate) fn cancel_batch_quotes<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        request_json: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        let request = parse_request(&request_json)?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(rest.cancel_batch_quotes(request).await)
        })
    }

    pub(crate) fn cancel_all_quotes<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        request_json: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        let request = parse_request(&request_json)?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(rest.cancel_all_quotes(request).await)
        })
    }

    pub(crate) fn get_rfqs<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        params_json: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        let params = parse_params(params_json.as_deref())?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(rest.get_rfqs(params).await)
        })
    }

    pub(crate) fn get_quotes<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        params_json: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        let params = parse_params(params_json.as_deref())?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(rest.get_quotes(params).await)
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
            map_values(rest.get_trades(params).await)
        })
    }

    pub(crate) fn get_public_trades<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        params_json: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        let params = parse_params(params_json.as_deref())?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(rest.get_public_trades(params).await)
        })
    }

    pub(crate) fn reset_mmp<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        request_json: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        let request = parse_request(&request_json)?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(rest.reset_mmp(request).await)
        })
    }

    pub(crate) fn set_mmp_config<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        request_json: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        let request = parse_request(&request_json)?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(rest.set_mmp_config(request).await)
        })
    }

    pub(crate) fn get_mmp_config<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(rest.get_mmp_config().await)
        })
    }

    pub(crate) fn set_marker_instrument<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        request_json: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        let request = parse_request(&request_json)?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(rest.set_marker_instrument(request).await)
        })
    }

    pub(crate) fn get_quote_products<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        params_json: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        let params = parse_params(params_json.as_deref())?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(rest.get_quote_products(params).await)
        })
    }
}
