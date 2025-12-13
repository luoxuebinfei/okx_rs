//! Broker API #[pymethods] 块

use pyo3::prelude::*;

use crate::broker as broker_impl;

use super::PyOkxClient;

#[pymethods]
impl PyOkxClient {
    // ==================== Broker API ====================

    /// 生成返佣明细下载链接（FD Broker）。
    fn fd_rebate_per_orders(&self, params_json: &str) -> PyResult<Vec<Py<PyAny>>> {
        broker_impl::sync::fd_rebate_per_orders(self, params_json)
    }

    /// 获取返佣明细下载链接（FD Broker）。
    fn fd_get_rebate_per_orders(&self, params_json: &str) -> PyResult<Vec<Py<PyAny>>> {
        broker_impl::sync::fd_get_rebate_per_orders(self, params_json)
    }
}
