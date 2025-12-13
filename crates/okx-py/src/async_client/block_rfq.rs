//! Block RFQ API #[pymethods] 块（异步）

use pyo3::prelude::*;

use crate::block_rfq as block_rfq_impl;

use super::PyAsyncOkxClient;

#[pymethods]
impl PyAsyncOkxClient {
    // ==================== Block RFQ API ====================

    /// 获取交易对手列表（大宗交易，异步）。
    fn get_counterparties<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        block_rfq_impl::async_api::get_counterparties(self, py)
    }

    /// 创建 RFQ（大宗交易，异步）。
    fn create_rfq<'py>(
        &self,
        py: Python<'py>,
        request_json: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        block_rfq_impl::async_api::create_rfq(self, py, request_json)
    }

    /// 取消 RFQ（大宗交易，异步）。
    fn cancel_rfq<'py>(
        &self,
        py: Python<'py>,
        request_json: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        block_rfq_impl::async_api::cancel_rfq(self, py, request_json)
    }

    /// 批量取消 RFQ（大宗交易，异步）。
    fn cancel_batch_rfqs<'py>(
        &self,
        py: Python<'py>,
        request_json: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        block_rfq_impl::async_api::cancel_batch_rfqs(self, py, request_json)
    }

    /// 全部取消 RFQ（大宗交易，异步）。
    fn cancel_all_rfqs<'py>(
        &self,
        py: Python<'py>,
        request_json: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        block_rfq_impl::async_api::cancel_all_rfqs(self, py, request_json)
    }

    /// 执行 Quote 成交（大宗交易，异步）。
    fn execute_quote<'py>(
        &self,
        py: Python<'py>,
        request_json: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        block_rfq_impl::async_api::execute_quote(self, py, request_json)
    }

    /// 创建 Quote（大宗交易，异步）。
    fn create_quote<'py>(
        &self,
        py: Python<'py>,
        request_json: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        block_rfq_impl::async_api::create_quote(self, py, request_json)
    }

    /// 取消 Quote（大宗交易，异步）。
    fn cancel_quote<'py>(
        &self,
        py: Python<'py>,
        request_json: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        block_rfq_impl::async_api::cancel_quote(self, py, request_json)
    }

    /// 批量取消 Quote（大宗交易，异步）。
    fn cancel_batch_quotes<'py>(
        &self,
        py: Python<'py>,
        request_json: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        block_rfq_impl::async_api::cancel_batch_quotes(self, py, request_json)
    }

    /// 全部取消 Quote（大宗交易，异步）。
    fn cancel_all_quotes<'py>(
        &self,
        py: Python<'py>,
        request_json: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        block_rfq_impl::async_api::cancel_all_quotes(self, py, request_json)
    }

    /// 查询 RFQ 列表（大宗交易，异步）。
    #[pyo3(signature = (params_json=None))]
    fn get_rfqs<'py>(
        &self,
        py: Python<'py>,
        params_json: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        block_rfq_impl::async_api::get_rfqs(self, py, params_json)
    }

    /// 查询 Quote 列表（大宗交易，异步）。
    #[pyo3(signature = (params_json=None))]
    fn get_quotes<'py>(
        &self,
        py: Python<'py>,
        params_json: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        block_rfq_impl::async_api::get_quotes(self, py, params_json)
    }

    /// 查询成交（大宗交易，异步）。
    ///
    /// 说明：为避免与 Market API 的 `get_trades` 冲突，该方法使用 `get_block_rfq_trades` 命名。
    #[pyo3(signature = (params_json=None))]
    fn get_block_rfq_trades<'py>(
        &self,
        py: Python<'py>,
        params_json: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        block_rfq_impl::async_api::get_trades(self, py, params_json)
    }

    /// 查询公共成交（大宗交易，异步）。
    #[pyo3(signature = (params_json=None))]
    fn get_public_trades<'py>(
        &self,
        py: Python<'py>,
        params_json: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        block_rfq_impl::async_api::get_public_trades(self, py, params_json)
    }

    /// 重置 MMP 配置（大宗交易，异步）。
    fn reset_mmp<'py>(&self, py: Python<'py>, request_json: String) -> PyResult<Bound<'py, PyAny>> {
        block_rfq_impl::async_api::reset_mmp(self, py, request_json)
    }

    /// 设置 MMP 配置（大宗交易，异步）。
    fn set_mmp_config<'py>(
        &self,
        py: Python<'py>,
        request_json: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        block_rfq_impl::async_api::set_mmp_config(self, py, request_json)
    }

    /// 获取 MMP 配置（大宗交易，异步）。
    fn get_mmp_config<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        block_rfq_impl::async_api::get_mmp_config(self, py)
    }

    /// 设置做市商合约配置（大宗交易，异步）。
    fn set_marker_instrument<'py>(
        &self,
        py: Python<'py>,
        request_json: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        block_rfq_impl::async_api::set_marker_instrument(self, py, request_json)
    }

    /// 获取 Quote 产品列表（大宗交易，异步）。
    #[pyo3(signature = (params_json=None))]
    fn get_quote_products<'py>(
        &self,
        py: Python<'py>,
        params_json: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        block_rfq_impl::async_api::get_quote_products(self, py, params_json)
    }
}
