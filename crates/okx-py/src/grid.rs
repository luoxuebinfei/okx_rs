use crate::{map_values, parse_json_value};
use okx_rest::GridApi;
use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;

/// Grid 模块 - 同步客户端方法实现
pub mod sync {
    use super::*;
    use crate::client::PyOkxClient;

    pub fn grid_order_algo(
        py_client: &PyOkxClient,
        payload_json: &str,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let payload = parse_json_value(Some(payload_json), "payload")?
            .ok_or_else(|| PyRuntimeError::new_err("payload 不能为空"))?;
        let client = py_client.rest_client();
        py_client
            .block_on_allow_threads(async move { client.grid_order_algo(payload).await })
            .and_then(crate::values_to_py_list)
    }

    pub fn grid_orders_algo_pending(
        py_client: &PyOkxClient,
        params_json: Option<&str>,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let params = parse_json_value(params_json, "params")?;
        let client = py_client.rest_client();
        py_client
            .block_on_allow_threads(async move { client.grid_orders_algo_pending(params).await })
            .and_then(crate::values_to_py_list)
    }

    pub fn grid_orders_algo_history(
        py_client: &PyOkxClient,
        params_json: Option<&str>,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let params = parse_json_value(params_json, "params")?;
        let client = py_client.rest_client();
        py_client
            .block_on_allow_threads(async move { client.grid_orders_algo_history(params).await })
            .and_then(crate::values_to_py_list)
    }

    pub fn place_recurring_buy_order(
        py_client: &PyOkxClient,
        payload_json: &str,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let payload = parse_json_value(Some(payload_json), "payload")?
            .ok_or_else(|| PyRuntimeError::new_err("payload 不能为空"))?;
        let client = py_client.rest_client();
        py_client
            .block_on_allow_threads(async move { client.place_recurring_buy_order(payload).await })
            .and_then(crate::values_to_py_list)
    }

    pub fn get_recurring_buy_order_list(
        py_client: &PyOkxClient,
        params_json: Option<&str>,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let params = parse_json_value(params_json, "params")?;
        let client = py_client.rest_client();
        py_client
            .block_on_allow_threads(
                async move { client.get_recurring_buy_order_list(params).await },
            )
            .and_then(crate::values_to_py_list)
    }

    pub fn grid_amend_order_algo(
        py_client: &PyOkxClient,
        payload_json: &str,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let payload = parse_json_value(Some(payload_json), "payload")?
            .ok_or_else(|| PyRuntimeError::new_err("payload 不能为空"))?;
        py_client
            .block_on_allow_threads(async move {
                py_client.rest_client().grid_amend_order_algo(payload).await
            })
            .and_then(crate::values_to_py_list)
    }

    pub fn grid_stop_order_algo(
        py_client: &PyOkxClient,
        payload_json: &str,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let payload = parse_json_value(Some(payload_json), "payload")?
            .ok_or_else(|| PyRuntimeError::new_err("payload 不能为空"))?;
        py_client
            .block_on_allow_threads(async move {
                py_client.rest_client().grid_stop_order_algo(payload).await
            })
            .and_then(crate::values_to_py_list)
    }

    pub fn grid_orders_algo_details(
        py_client: &PyOkxClient,
        params_json: &str,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let params = parse_json_value(Some(params_json), "params")?
            .ok_or_else(|| PyRuntimeError::new_err("params 不能为空"))?;
        py_client
            .block_on_allow_threads(async move {
                py_client
                    .rest_client()
                    .grid_orders_algo_details(params)
                    .await
            })
            .and_then(crate::values_to_py_list)
    }

    pub fn grid_sub_orders(py_client: &PyOkxClient, params_json: &str) -> PyResult<Vec<Py<PyAny>>> {
        let params = parse_json_value(Some(params_json), "params")?
            .ok_or_else(|| PyRuntimeError::new_err("params 不能为空"))?;
        py_client
            .block_on_allow_threads(
                async move { py_client.rest_client().grid_sub_orders(params).await },
            )
            .and_then(crate::values_to_py_list)
    }

    pub fn grid_positions(
        py_client: &PyOkxClient,
        params_json: Option<&str>,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let params = parse_json_value(params_json, "params")?;
        py_client
            .block_on_allow_threads(
                async move { py_client.rest_client().grid_positions(params).await },
            )
            .and_then(crate::values_to_py_list)
    }

    pub fn grid_withdraw_income(
        py_client: &PyOkxClient,
        payload_json: &str,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let payload = parse_json_value(Some(payload_json), "payload")?
            .ok_or_else(|| PyRuntimeError::new_err("payload 不能为空"))?;
        py_client
            .block_on_allow_threads(async move {
                py_client.rest_client().grid_withdraw_income(payload).await
            })
            .and_then(crate::values_to_py_list)
    }

    pub fn grid_compute_margin_balance(
        py_client: &PyOkxClient,
        payload_json: &str,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let payload = parse_json_value(Some(payload_json), "payload")?
            .ok_or_else(|| PyRuntimeError::new_err("payload 不能为空"))?;
        py_client
            .block_on_allow_threads(async move {
                py_client
                    .rest_client()
                    .grid_compute_margin_balance(payload)
                    .await
            })
            .and_then(crate::values_to_py_list)
    }

    pub fn grid_margin_balance(
        py_client: &PyOkxClient,
        payload_json: &str,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let payload = parse_json_value(Some(payload_json), "payload")?
            .ok_or_else(|| PyRuntimeError::new_err("payload 不能为空"))?;
        py_client
            .block_on_allow_threads(async move {
                py_client.rest_client().grid_margin_balance(payload).await
            })
            .and_then(crate::values_to_py_list)
    }

    pub fn grid_ai_param(
        py_client: &PyOkxClient,
        params_json: Option<&str>,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let params = parse_json_value(params_json, "params")?;
        py_client
            .block_on_allow_threads(
                async move { py_client.rest_client().grid_ai_param(params).await },
            )
            .and_then(crate::values_to_py_list)
    }

    pub fn amend_recurring_buy_order(
        py_client: &PyOkxClient,
        payload_json: &str,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let payload = parse_json_value(Some(payload_json), "payload")?
            .ok_or_else(|| PyRuntimeError::new_err("payload 不能为空"))?;
        py_client
            .block_on_allow_threads(async move {
                py_client
                    .rest_client()
                    .amend_recurring_buy_order(payload)
                    .await
            })
            .and_then(crate::values_to_py_list)
    }

    pub fn stop_recurring_buy_order(
        py_client: &PyOkxClient,
        payload_json: &str,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let payload = parse_json_value(Some(payload_json), "payload")?
            .ok_or_else(|| PyRuntimeError::new_err("payload 不能为空"))?;
        py_client
            .block_on_allow_threads(async move {
                py_client
                    .rest_client()
                    .stop_recurring_buy_order(payload)
                    .await
            })
            .and_then(crate::values_to_py_list)
    }

    pub fn get_recurring_buy_order_history(
        py_client: &PyOkxClient,
        params_json: Option<&str>,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let params = parse_json_value(params_json, "params")?;
        py_client
            .block_on_allow_threads(async move {
                py_client
                    .rest_client()
                    .get_recurring_buy_order_history(params)
                    .await
            })
            .and_then(crate::values_to_py_list)
    }

    pub fn get_recurring_buy_order_details(
        py_client: &PyOkxClient,
        params_json: &str,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let params = parse_json_value(Some(params_json), "params")?
            .ok_or_else(|| PyRuntimeError::new_err("params 不能为空"))?;
        py_client
            .block_on_allow_threads(async move {
                py_client
                    .rest_client()
                    .get_recurring_buy_order_details(params)
                    .await
            })
            .and_then(crate::values_to_py_list)
    }

    pub fn get_recurring_buy_sub_orders(
        py_client: &PyOkxClient,
        params_json: &str,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let params = parse_json_value(Some(params_json), "params")?
            .ok_or_else(|| PyRuntimeError::new_err("params 不能为空"))?;
        py_client
            .block_on_allow_threads(async move {
                py_client
                    .rest_client()
                    .get_recurring_buy_sub_orders(params)
                    .await
            })
            .and_then(crate::values_to_py_list)
    }
}

/// Grid 模块 - 异步客户端方法实现
pub mod async_api {
    use super::*;
    use crate::async_client::PyAsyncOkxClient;

    pub fn grid_order_algo<'py>(
        py_client: &PyAsyncOkxClient,
        py: Python<'py>,
        payload_json: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = py_client.rest_client();
        let payload = parse_json_value(Some(&payload_json), "payload")?
            .ok_or_else(|| PyRuntimeError::new_err("payload 不能为空"))?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(client.grid_order_algo(payload).await)
        })
    }

    pub fn grid_orders_algo_pending<'py>(
        py_client: &PyAsyncOkxClient,
        py: Python<'py>,
        params_json: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = py_client.rest_client();
        let params = parse_json_value(params_json.as_deref(), "params")?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(client.grid_orders_algo_pending(params).await)
        })
    }

    pub fn grid_orders_algo_history<'py>(
        py_client: &PyAsyncOkxClient,
        py: Python<'py>,
        params_json: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = py_client.rest_client();
        let params = parse_json_value(params_json.as_deref(), "params")?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(client.grid_orders_algo_history(params).await)
        })
    }

    pub fn place_recurring_buy_order<'py>(
        py_client: &PyAsyncOkxClient,
        py: Python<'py>,
        payload_json: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = py_client.rest_client();
        let payload = parse_json_value(Some(&payload_json), "payload")?
            .ok_or_else(|| PyRuntimeError::new_err("payload 不能为空"))?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(client.place_recurring_buy_order(payload).await)
        })
    }

    pub fn get_recurring_buy_order_list<'py>(
        py_client: &PyAsyncOkxClient,
        py: Python<'py>,
        params_json: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = py_client.rest_client();
        let params = parse_json_value(params_json.as_deref(), "params")?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(client.get_recurring_buy_order_list(params).await)
        })
    }

    pub fn grid_amend_order_algo<'py>(
        c: &PyAsyncOkxClient,
        py: Python<'py>,
        p: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = c.rest_client();
        let payload = parse_json_value(Some(&p), "payload")?
            .ok_or_else(|| PyRuntimeError::new_err("payload 不能为空"))?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(client.grid_amend_order_algo(payload).await)
        })
    }
    pub fn grid_stop_order_algo<'py>(
        c: &PyAsyncOkxClient,
        py: Python<'py>,
        p: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = c.rest_client();
        let payload = parse_json_value(Some(&p), "payload")?
            .ok_or_else(|| PyRuntimeError::new_err("payload 不能为空"))?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(client.grid_stop_order_algo(payload).await)
        })
    }
    pub fn grid_orders_algo_details<'py>(
        c: &PyAsyncOkxClient,
        py: Python<'py>,
        p: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = c.rest_client();
        let params = parse_json_value(Some(&p), "params")?
            .ok_or_else(|| PyRuntimeError::new_err("params 不能为空"))?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(client.grid_orders_algo_details(params).await)
        })
    }
    pub fn grid_sub_orders<'py>(
        c: &PyAsyncOkxClient,
        py: Python<'py>,
        p: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = c.rest_client();
        let params = parse_json_value(Some(&p), "params")?
            .ok_or_else(|| PyRuntimeError::new_err("params 不能为空"))?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(client.grid_sub_orders(params).await)
        })
    }
    pub fn grid_positions<'py>(
        c: &PyAsyncOkxClient,
        py: Python<'py>,
        p: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = c.rest_client();
        let params = parse_json_value(p.as_deref(), "params")?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(client.grid_positions(params).await)
        })
    }
    pub fn grid_withdraw_income<'py>(
        c: &PyAsyncOkxClient,
        py: Python<'py>,
        p: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = c.rest_client();
        let payload = parse_json_value(Some(&p), "payload")?
            .ok_or_else(|| PyRuntimeError::new_err("payload 不能为空"))?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(client.grid_withdraw_income(payload).await)
        })
    }
    pub fn grid_compute_margin_balance<'py>(
        c: &PyAsyncOkxClient,
        py: Python<'py>,
        p: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = c.rest_client();
        let payload = parse_json_value(Some(&p), "payload")?
            .ok_or_else(|| PyRuntimeError::new_err("payload 不能为空"))?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(client.grid_compute_margin_balance(payload).await)
        })
    }
    pub fn grid_margin_balance<'py>(
        c: &PyAsyncOkxClient,
        py: Python<'py>,
        p: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = c.rest_client();
        let payload = parse_json_value(Some(&p), "payload")?
            .ok_or_else(|| PyRuntimeError::new_err("payload 不能为空"))?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(client.grid_margin_balance(payload).await)
        })
    }
    pub fn grid_ai_param<'py>(
        c: &PyAsyncOkxClient,
        py: Python<'py>,
        p: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = c.rest_client();
        let params = parse_json_value(p.as_deref(), "params")?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(client.grid_ai_param(params).await)
        })
    }
    pub fn amend_recurring_buy_order<'py>(
        c: &PyAsyncOkxClient,
        py: Python<'py>,
        p: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = c.rest_client();
        let payload = parse_json_value(Some(&p), "payload")?
            .ok_or_else(|| PyRuntimeError::new_err("payload 不能为空"))?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(client.amend_recurring_buy_order(payload).await)
        })
    }
    pub fn stop_recurring_buy_order<'py>(
        c: &PyAsyncOkxClient,
        py: Python<'py>,
        p: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = c.rest_client();
        let payload = parse_json_value(Some(&p), "payload")?
            .ok_or_else(|| PyRuntimeError::new_err("payload 不能为空"))?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(client.stop_recurring_buy_order(payload).await)
        })
    }
    pub fn get_recurring_buy_order_history<'py>(
        c: &PyAsyncOkxClient,
        py: Python<'py>,
        p: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = c.rest_client();
        let params = parse_json_value(p.as_deref(), "params")?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(client.get_recurring_buy_order_history(params).await)
        })
    }
    pub fn get_recurring_buy_order_details<'py>(
        c: &PyAsyncOkxClient,
        py: Python<'py>,
        p: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = c.rest_client();
        let params = parse_json_value(Some(&p), "params")?
            .ok_or_else(|| PyRuntimeError::new_err("params 不能为空"))?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(client.get_recurring_buy_order_details(params).await)
        })
    }
    pub fn get_recurring_buy_sub_orders<'py>(
        c: &PyAsyncOkxClient,
        py: Python<'py>,
        p: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = c.rest_client();
        let params = parse_json_value(Some(&p), "params")?
            .ok_or_else(|| PyRuntimeError::new_err("params 不能为空"))?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(client.get_recurring_buy_sub_orders(params).await)
        })
    }
}
