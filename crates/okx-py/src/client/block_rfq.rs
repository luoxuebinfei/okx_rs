//! Block RFQ API #[pymethods] 块

use pyo3::prelude::*;

use crate::block_rfq as block_rfq_impl;

use super::PyOkxClient;

#[pymethods]
impl PyOkxClient {
    // ==================== Block RFQ API ====================

    /// 获取交易对手列表（大宗交易）。
    fn get_counterparties(&self) -> PyResult<Vec<Py<PyAny>>> {
        block_rfq_impl::sync::get_counterparties(self)
    }

    /// 创建 RFQ（大宗交易）。
    ///
    /// 参数为 JSON 字符串，结构以 OKX 文档/官方 SDK 为准。
    fn create_rfq(&self, request_json: &str) -> PyResult<Vec<Py<PyAny>>> {
        block_rfq_impl::sync::create_rfq(self, request_json)
    }

    /// 取消 RFQ（大宗交易）。
    fn cancel_rfq(&self, request_json: &str) -> PyResult<Vec<Py<PyAny>>> {
        block_rfq_impl::sync::cancel_rfq(self, request_json)
    }

    /// 批量取消 RFQ（大宗交易）。
    fn cancel_batch_rfqs(&self, request_json: &str) -> PyResult<Vec<Py<PyAny>>> {
        block_rfq_impl::sync::cancel_batch_rfqs(self, request_json)
    }

    /// 全部取消 RFQ（大宗交易）。
    fn cancel_all_rfqs(&self, request_json: &str) -> PyResult<Vec<Py<PyAny>>> {
        block_rfq_impl::sync::cancel_all_rfqs(self, request_json)
    }

    /// 执行 Quote 成交（大宗交易）。
    fn execute_quote(&self, request_json: &str) -> PyResult<Vec<Py<PyAny>>> {
        block_rfq_impl::sync::execute_quote(self, request_json)
    }

    /// 创建 Quote（大宗交易）。
    fn create_quote(&self, request_json: &str) -> PyResult<Vec<Py<PyAny>>> {
        block_rfq_impl::sync::create_quote(self, request_json)
    }

    /// 取消 Quote（大宗交易）。
    fn cancel_quote(&self, request_json: &str) -> PyResult<Vec<Py<PyAny>>> {
        block_rfq_impl::sync::cancel_quote(self, request_json)
    }

    /// 批量取消 Quote（大宗交易）。
    fn cancel_batch_quotes(&self, request_json: &str) -> PyResult<Vec<Py<PyAny>>> {
        block_rfq_impl::sync::cancel_batch_quotes(self, request_json)
    }

    /// 全部取消 Quote（大宗交易）。
    fn cancel_all_quotes(&self, request_json: &str) -> PyResult<Vec<Py<PyAny>>> {
        block_rfq_impl::sync::cancel_all_quotes(self, request_json)
    }

    /// 查询 RFQ 列表（大宗交易）。
    #[pyo3(signature = (params_json=None))]
    fn get_rfqs(&self, params_json: Option<&str>) -> PyResult<Vec<Py<PyAny>>> {
        block_rfq_impl::sync::get_rfqs(self, params_json)
    }

    /// 查询 Quote 列表（大宗交易）。
    #[pyo3(signature = (params_json=None))]
    fn get_quotes(&self, params_json: Option<&str>) -> PyResult<Vec<Py<PyAny>>> {
        block_rfq_impl::sync::get_quotes(self, params_json)
    }

    /// 查询成交（大宗交易）。
    ///
    /// 说明：为避免与 Market API 的 `get_trades` 冲突，该方法使用 `get_block_rfq_trades` 命名。
    #[pyo3(signature = (params_json=None))]
    fn get_block_rfq_trades(&self, params_json: Option<&str>) -> PyResult<Vec<Py<PyAny>>> {
        block_rfq_impl::sync::get_trades(self, params_json)
    }

    /// 查询公共成交（大宗交易）。
    #[pyo3(signature = (params_json=None))]
    fn get_public_trades(&self, params_json: Option<&str>) -> PyResult<Vec<Py<PyAny>>> {
        block_rfq_impl::sync::get_public_trades(self, params_json)
    }

    /// 重置 MMP 配置（大宗交易）。
    fn reset_mmp(&self, request_json: &str) -> PyResult<Vec<Py<PyAny>>> {
        block_rfq_impl::sync::reset_mmp(self, request_json)
    }

    /// 设置 MMP 配置（大宗交易）。
    fn set_mmp_config(&self, request_json: &str) -> PyResult<Vec<Py<PyAny>>> {
        block_rfq_impl::sync::set_mmp_config(self, request_json)
    }

    /// 获取 MMP 配置（大宗交易）。
    fn get_mmp_config(&self) -> PyResult<Vec<Py<PyAny>>> {
        block_rfq_impl::sync::get_mmp_config(self)
    }

    /// 设置做市商合约配置（大宗交易）。
    fn set_marker_instrument(&self, request_json: &str) -> PyResult<Vec<Py<PyAny>>> {
        block_rfq_impl::sync::set_marker_instrument(self, request_json)
    }

    /// 获取 Quote 产品列表（大宗交易）。
    #[pyo3(signature = (params_json=None))]
    fn get_quote_products(&self, params_json: Option<&str>) -> PyResult<Vec<Py<PyAny>>> {
        block_rfq_impl::sync::get_quote_products(self, params_json)
    }
}
