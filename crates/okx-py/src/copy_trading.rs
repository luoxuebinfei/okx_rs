//! Copy Trading（跟单交易）域绑定的同步/异步共享实现。

use pyo3::prelude::*;
use serde_json::Value;

use okx_rest::CopyTradingApi;

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

    pub(crate) fn get_existing_lead_positions(
        client: &PyOkxClient,
        params_json: Option<&str>,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let params = parse_params(params_json)?;
        let res = client.block_on_allow_threads(async {
            client
                .rest_client()
                .get_existing_lead_positions(params)
                .await
        })?;
        map_values(Ok(res))
    }

    pub(crate) fn get_lead_position_history(
        client: &PyOkxClient,
        params_json: Option<&str>,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let params = parse_params(params_json)?;
        let res = client.block_on_allow_threads(async {
            client.rest_client().get_lead_position_history(params).await
        })?;
        map_values(Ok(res))
    }

    pub(crate) fn place_lead_stop_order(
        client: &PyOkxClient,
        request_json: &str,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let request = parse_request(request_json)?;
        let res = client.block_on_allow_threads(async {
            client.rest_client().place_lead_stop_order(request).await
        })?;
        map_values(Ok(res))
    }

    pub(crate) fn close_lead_position(
        client: &PyOkxClient,
        request_json: &str,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let request = parse_request(request_json)?;
        let res = client.block_on_allow_threads(async {
            client.rest_client().close_lead_position(request).await
        })?;
        map_values(Ok(res))
    }

    pub(crate) fn get_leading_instruments(
        client: &PyOkxClient,
        params_json: Option<&str>,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let params = parse_params(params_json)?;
        let res = client.block_on_allow_threads(async {
            client.rest_client().get_leading_instruments(params).await
        })?;
        map_values(Ok(res))
    }

    pub(crate) fn amend_leading_instruments(
        client: &PyOkxClient,
        request_json: &str,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let request = parse_request(request_json)?;
        let res = client.block_on_allow_threads(async {
            client
                .rest_client()
                .amend_leading_instruments(request)
                .await
        })?;
        map_values(Ok(res))
    }

    pub(crate) fn get_profit_sharing_details(
        client: &PyOkxClient,
        params_json: Option<&str>,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let params = parse_params(params_json)?;
        let res = client.block_on_allow_threads(async {
            client
                .rest_client()
                .get_profit_sharing_details(params)
                .await
        })?;
        map_values(Ok(res))
    }

    pub(crate) fn get_total_profit_sharing(client: &PyOkxClient) -> PyResult<Vec<Py<PyAny>>> {
        let res = client.block_on_allow_threads(async {
            client.rest_client().get_total_profit_sharing().await
        })?;
        map_values(Ok(res))
    }

    pub(crate) fn get_unrealized_profit_sharing_details(
        client: &PyOkxClient,
        params_json: Option<&str>,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let params = parse_params(params_json)?;
        let res = client.block_on_allow_threads(async {
            client
                .rest_client()
                .get_unrealized_profit_sharing_details(params)
                .await
        })?;
        map_values(Ok(res))
    }
}

pub(crate) mod async_api {
    use super::*;

    pub(crate) fn get_existing_lead_positions<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        params_json: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        let params = parse_params(params_json.as_deref())?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(rest.get_existing_lead_positions(params).await)
        })
    }

    pub(crate) fn get_lead_position_history<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        params_json: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        let params = parse_params(params_json.as_deref())?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(rest.get_lead_position_history(params).await)
        })
    }

    pub(crate) fn place_lead_stop_order<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        request_json: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        let request = parse_request(&request_json)?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(rest.place_lead_stop_order(request).await)
        })
    }

    pub(crate) fn close_lead_position<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        request_json: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        let request = parse_request(&request_json)?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(rest.close_lead_position(request).await)
        })
    }

    pub(crate) fn get_leading_instruments<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        params_json: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        let params = parse_params(params_json.as_deref())?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(rest.get_leading_instruments(params).await)
        })
    }

    pub(crate) fn amend_leading_instruments<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        request_json: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        let request = parse_request(&request_json)?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(rest.amend_leading_instruments(request).await)
        })
    }

    pub(crate) fn get_profit_sharing_details<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        params_json: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        let params = parse_params(params_json.as_deref())?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(rest.get_profit_sharing_details(params).await)
        })
    }

    pub(crate) fn get_total_profit_sharing<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(rest.get_total_profit_sharing().await)
        })
    }

    pub(crate) fn get_unrealized_profit_sharing_details<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        params_json: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        let params = parse_params(params_json.as_deref())?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(rest.get_unrealized_profit_sharing_details(params).await)
        })
    }
}
