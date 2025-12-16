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

    /// 公共接口：返回数据和响应元数据（异步）。
    ///
    /// 返回 (data, ResponseMeta) 元组，可用于读取限速信息。
    ///
    /// Args:
    ///     path: API 路径
    ///     params_json: 可选的 JSON 格式查询参数
    ///
    /// Returns:
    ///     (list, ResponseMeta): 数据列表和响应元数据
    #[pyo3(signature = (path, params_json=None))]
    fn get_public_with_meta<'py>(
        &self,
        py: Python<'py>,
        path: String,
        params_json: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        raw_impl::async_api::get_public_with_meta(self, py, path, params_json)
    }

    /// 私有接口：返回数据和响应元数据，会进行签名（异步）。
    #[pyo3(signature = (path, params_json=None))]
    fn get_private_with_meta<'py>(
        &self,
        py: Python<'py>,
        path: String,
        params_json: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        raw_impl::async_api::get_private_with_meta(self, py, path, params_json)
    }

    /// 私有接口：POST 并返回数据和响应元数据，会进行签名（异步）。
    fn post_private_with_meta<'py>(
        &self,
        py: Python<'py>,
        path: String,
        body_json: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        raw_impl::async_api::post_private_with_meta(self, py, path, body_json)
    }
}
