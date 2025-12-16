//! Raw（官方兼容）响应能力。
//!
//! 目标：在不破坏既有“类型化返回（只返回 data）”的前提下，提供返回完整 JSON 的接口，
//! 以对齐官方 python-okx 的使用习惯（code/msg/data）。

use pyo3::prelude::*;

use crate::{
    parse_json_value, parse_required_json_value, to_py_err, value_to_py_obj, values_to_py_list,
    PyAsyncOkxClient, PyOkxClient, PyResponseMeta,
};

pub(crate) mod sync {
    use super::*;

    pub(crate) fn get_public_raw(
        client: &PyOkxClient,
        path: &str,
        params_json: Option<&str>,
    ) -> PyResult<Py<PyAny>> {
        let params = parse_json_value(params_json, "params_json")?;
        let value = client.block_on_allow_threads(async {
            client
                .rest_client()
                .get_public_raw(path, params.as_ref())
                .await
        })?;
        value_to_py_obj(value)
    }

    pub(crate) fn get_private_raw(
        client: &PyOkxClient,
        path: &str,
        params_json: Option<&str>,
    ) -> PyResult<Py<PyAny>> {
        let params = parse_json_value(params_json, "params_json")?;
        let value = client.block_on_allow_threads(async {
            client.rest_client().get_raw(path, params.as_ref()).await
        })?;
        value_to_py_obj(value)
    }

    pub(crate) fn post_private_raw(
        client: &PyOkxClient,
        path: &str,
        body_json: &str,
    ) -> PyResult<Py<PyAny>> {
        let body = parse_required_json_value(body_json, "body_json")?;
        let value = client
            .block_on_allow_threads(async { client.rest_client().post_raw(path, &body).await })?;
        value_to_py_obj(value)
    }

    pub(crate) fn get_public_with_meta(
        client: &PyOkxClient,
        path: &str,
        params_json: Option<&str>,
    ) -> PyResult<(Vec<Py<PyAny>>, PyResponseMeta)> {
        let params = parse_json_value(params_json, "params_json")?;
        let (data, meta) = client.block_on_allow_threads(async {
            client
                .rest_client()
                .get_public_with_meta::<serde_json::Value, _>(path, params.as_ref())
                .await
        })?;
        let py_data = values_to_py_list(data)?;
        Ok((py_data, PyResponseMeta::from(meta)))
    }

    pub(crate) fn get_private_with_meta(
        client: &PyOkxClient,
        path: &str,
        params_json: Option<&str>,
    ) -> PyResult<(Vec<Py<PyAny>>, PyResponseMeta)> {
        let params = parse_json_value(params_json, "params_json")?;
        let (data, meta) = client.block_on_allow_threads(async {
            client
                .rest_client()
                .get_with_meta::<serde_json::Value, _>(path, params.as_ref())
                .await
        })?;
        let py_data = values_to_py_list(data)?;
        Ok((py_data, PyResponseMeta::from(meta)))
    }

    pub(crate) fn post_private_with_meta(
        client: &PyOkxClient,
        path: &str,
        body_json: &str,
    ) -> PyResult<(Vec<Py<PyAny>>, PyResponseMeta)> {
        let body = parse_required_json_value(body_json, "body_json")?;
        let (data, meta) = client.block_on_allow_threads(async {
            client
                .rest_client()
                .post_with_meta::<serde_json::Value, _>(path, &body)
                .await
        })?;
        let py_data = values_to_py_list(data)?;
        Ok((py_data, PyResponseMeta::from(meta)))
    }
}

pub(crate) mod async_api {
    use super::*;

    pub(crate) fn get_public_raw<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        path: String,
        params_json: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let params = parse_json_value(params_json.as_deref(), "params_json")?;
        let rest = client.rest_client();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let value = rest
                .get_public_raw(&path, params.as_ref())
                .await
                .map_err(to_py_err)?;
            value_to_py_obj(value)
        })
    }

    pub(crate) fn get_private_raw<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        path: String,
        params_json: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let params = parse_json_value(params_json.as_deref(), "params_json")?;
        let rest = client.rest_client();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let value = rest
                .get_raw(&path, params.as_ref())
                .await
                .map_err(to_py_err)?;
            value_to_py_obj(value)
        })
    }

    pub(crate) fn post_private_raw<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        path: String,
        body_json: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let body = parse_required_json_value(&body_json, "body_json")?;
        let rest = client.rest_client();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let value = rest.post_raw(&path, &body).await.map_err(to_py_err)?;
            value_to_py_obj(value)
        })
    }

    pub(crate) fn get_public_with_meta<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        path: String,
        params_json: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let params = parse_json_value(params_json.as_deref(), "params_json")?;
        let rest = client.rest_client();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let (data, meta) = rest
                .get_public_with_meta::<serde_json::Value, _>(&path, params.as_ref())
                .await
                .map_err(to_py_err)?;
            let py_data = values_to_py_list(data)?;
            let py_meta = PyResponseMeta::from(meta);
            Ok((py_data, py_meta))
        })
    }

    pub(crate) fn get_private_with_meta<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        path: String,
        params_json: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let params = parse_json_value(params_json.as_deref(), "params_json")?;
        let rest = client.rest_client();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let (data, meta) = rest
                .get_with_meta::<serde_json::Value, _>(&path, params.as_ref())
                .await
                .map_err(to_py_err)?;
            let py_data = values_to_py_list(data)?;
            let py_meta = PyResponseMeta::from(meta);
            Ok((py_data, py_meta))
        })
    }

    pub(crate) fn post_private_with_meta<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        path: String,
        body_json: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let body = parse_required_json_value(&body_json, "body_json")?;
        let rest = client.rest_client();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let (data, meta) = rest
                .post_with_meta::<serde_json::Value, _>(&path, &body)
                .await
                .map_err(to_py_err)?;
            let py_data = values_to_py_list(data)?;
            let py_meta = PyResponseMeta::from(meta);
            Ok((py_data, py_meta))
        })
    }
}
