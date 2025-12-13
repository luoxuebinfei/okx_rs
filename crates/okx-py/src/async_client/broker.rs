//! Broker API #[pymethods] 块（异步）

use pyo3::prelude::*;

use crate::broker as broker_impl;

use super::PyAsyncOkxClient;

#[pymethods]
impl PyAsyncOkxClient {
    // ==================== Broker API ====================

    /// 生成返佣明细下载链接（FD Broker，异步）。
    fn fd_rebate_per_orders<'py>(
        &self,
        py: Python<'py>,
        params_json: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        broker_impl::async_api::fd_rebate_per_orders(self, py, params_json)
    }

    /// 获取返佣明细下载链接（FD Broker，异步）。
    fn fd_get_rebate_per_orders<'py>(
        &self,
        py: Python<'py>,
        params_json: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        broker_impl::async_api::fd_get_rebate_per_orders(self, py, params_json)
    }
}
