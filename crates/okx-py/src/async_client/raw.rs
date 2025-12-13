//! Raw（官方兼容）响应能力 #[pymethods] 块（异步）

use pyo3::prelude::*;

use crate::raw as raw_impl;

use super::PyAsyncOkxClient;

#[pymethods]
impl PyAsyncOkxClient {
    /// 公共接口：返回完整 JSON（code/msg/data）（异步）。
    #[pyo3(signature = (path, params_json=None))]
    fn get_public_raw<'py>(
        &self,
        py: Python<'py>,
        path: String,
        params_json: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        raw_impl::async_api::get_public_raw(self, py, path, params_json)
    }

    /// 私有接口：返回完整 JSON（code/msg/data），会进行签名（异步）。
    #[pyo3(signature = (path, params_json=None))]
    fn get_private_raw<'py>(
        &self,
        py: Python<'py>,
        path: String,
        params_json: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        raw_impl::async_api::get_private_raw(self, py, path, params_json)
    }

    /// 私有接口：POST 并返回完整 JSON（code/msg/data），会进行签名（异步）。
    fn post_private_raw<'py>(
        &self,
        py: Python<'py>,
        path: String,
        body_json: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        raw_impl::async_api::post_private_raw(self, py, path, body_json)
    }
}
