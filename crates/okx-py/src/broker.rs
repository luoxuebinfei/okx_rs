//! Broker（经纪商）域绑定的同步/异步共享实现。

use pyo3::prelude::*;
use serde_json::Value;

use okx_rest::BrokerApi;

use crate::{map_values, parse_required_json_value, PyAsyncOkxClient, PyOkxClient};

fn parse_params(params_json: &str) -> PyResult<Value> {
    parse_required_json_value(params_json, "params_json")
}

pub(crate) mod sync {
    use super::*;

    pub(crate) fn fd_rebate_per_orders(
        client: &PyOkxClient,
        params_json: &str,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let params = parse_params(params_json)?;
        let res = client.block_on_allow_threads(async {
            client.rest_client().fd_rebate_per_orders(params).await
        })?;
        map_values(Ok(res))
    }

    pub(crate) fn fd_get_rebate_per_orders(
        client: &PyOkxClient,
        params_json: &str,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let params = parse_params(params_json)?;
        let res = client.block_on_allow_threads(async {
            client.rest_client().fd_get_rebate_per_orders(params).await
        })?;
        map_values(Ok(res))
    }
}

pub(crate) mod async_api {
    use super::*;

    pub(crate) fn fd_rebate_per_orders<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        params_json: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        let params = parse_params(&params_json)?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(rest.fd_rebate_per_orders(params).await)
        })
    }

    pub(crate) fn fd_get_rebate_per_orders<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        params_json: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        let params = parse_params(&params_json)?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(rest.fd_get_rebate_per_orders(params).await)
        })
    }
}
