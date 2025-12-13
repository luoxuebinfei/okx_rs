//! Copy Trading API #[pymethods] 块（异步）

use pyo3::prelude::*;

use crate::copy_trading as copy_trading_impl;

use super::PyAsyncOkxClient;

#[pymethods]
impl PyAsyncOkxClient {
    // ==================== Copy Trading API ====================

    /// 获取现有带单仓位（异步）。
    #[pyo3(signature = (params_json=None))]
    fn get_existing_lead_positions<'py>(
        &self,
        py: Python<'py>,
        params_json: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        copy_trading_impl::async_api::get_existing_lead_positions(self, py, params_json)
    }

    /// 获取带单仓位历史（异步）。
    #[pyo3(signature = (params_json=None))]
    fn get_lead_position_history<'py>(
        &self,
        py: Python<'py>,
        params_json: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        copy_trading_impl::async_api::get_lead_position_history(self, py, params_json)
    }

    /// 下带单止损订单（异步）。
    fn place_lead_stop_order<'py>(
        &self,
        py: Python<'py>,
        request_json: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        copy_trading_impl::async_api::place_lead_stop_order(self, py, request_json)
    }

    /// 平带单仓位（异步）。
    fn close_lead_position<'py>(
        &self,
        py: Python<'py>,
        request_json: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        copy_trading_impl::async_api::close_lead_position(self, py, request_json)
    }

    /// 获取可带单交易产品（异步）。
    #[pyo3(signature = (params_json=None))]
    fn get_leading_instruments<'py>(
        &self,
        py: Python<'py>,
        params_json: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        copy_trading_impl::async_api::get_leading_instruments(self, py, params_json)
    }

    /// 修改可带单交易产品（异步）。
    fn amend_leading_instruments<'py>(
        &self,
        py: Python<'py>,
        request_json: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        copy_trading_impl::async_api::amend_leading_instruments(self, py, request_json)
    }

    /// 获取分润明细（异步）。
    #[pyo3(signature = (params_json=None))]
    fn get_profit_sharing_details<'py>(
        &self,
        py: Python<'py>,
        params_json: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        copy_trading_impl::async_api::get_profit_sharing_details(self, py, params_json)
    }

    /// 获取总分润（异步）。
    fn get_total_profit_sharing<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        copy_trading_impl::async_api::get_total_profit_sharing(self, py)
    }

    /// 获取未实现分润明细（异步）。
    #[pyo3(signature = (params_json=None))]
    fn get_unrealized_profit_sharing_details<'py>(
        &self,
        py: Python<'py>,
        params_json: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        copy_trading_impl::async_api::get_unrealized_profit_sharing_details(self, py, params_json)
    }
}
