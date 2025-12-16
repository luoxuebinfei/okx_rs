//! Raw（官方兼容）响应能力 #[pymethods] 块

use pyo3::prelude::*;

use crate::raw as raw_impl;

use super::PyOkxClient;

#[pymethods]
impl PyOkxClient {
    /// 公共接口：返回完整 JSON（code/msg/data）。
    #[pyo3(signature = (path, params_json=None))]
    fn get_public_raw(&self, path: &str, params_json: Option<&str>) -> PyResult<Py<PyAny>> {
        raw_impl::sync::get_public_raw(self, path, params_json)
    }

    /// 私有接口：返回完整 JSON（code/msg/data），会进行签名。
    #[pyo3(signature = (path, params_json=None))]
    fn get_private_raw(&self, path: &str, params_json: Option<&str>) -> PyResult<Py<PyAny>> {
        raw_impl::sync::get_private_raw(self, path, params_json)
    }

    /// 私有接口：POST 并返回完整 JSON（code/msg/data），会进行签名。
    fn post_private_raw(&self, path: &str, body_json: &str) -> PyResult<Py<PyAny>> {
        raw_impl::sync::post_private_raw(self, path, body_json)
    }

    /// 公共接口：返回数据和响应元数据。
    ///
    /// 返回 (data, ResponseMeta) 元组，可用于读取限速信息。
    #[pyo3(signature = (path, params_json=None))]
    fn get_public_with_meta(
        &self,
        path: &str,
        params_json: Option<&str>,
    ) -> PyResult<(Vec<Py<PyAny>>, crate::PyResponseMeta)> {
        raw_impl::sync::get_public_with_meta(self, path, params_json)
    }

    /// 私有接口：返回数据和响应元数据，会进行签名。
    #[pyo3(signature = (path, params_json=None))]
    fn get_private_with_meta(
        &self,
        path: &str,
        params_json: Option<&str>,
    ) -> PyResult<(Vec<Py<PyAny>>, crate::PyResponseMeta)> {
        raw_impl::sync::get_private_with_meta(self, path, params_json)
    }

    /// 私有接口：POST 并返回数据和响应元数据，会进行签名。
    fn post_private_with_meta(
        &self,
        path: &str,
        body_json: &str,
    ) -> PyResult<(Vec<Py<PyAny>>, crate::PyResponseMeta)> {
        raw_impl::sync::post_private_with_meta(self, path, body_json)
    }
}
