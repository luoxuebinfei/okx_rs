//! Copy Trading API #[pymethods] 块

use pyo3::prelude::*;

use crate::copy_trading as copy_trading_impl;

use super::PyOkxClient;

#[pymethods]
impl PyOkxClient {
    // ==================== Copy Trading API ====================

    /// 获取现有带单仓位。
    #[pyo3(signature = (params_json=None))]
    fn get_existing_lead_positions(&self, params_json: Option<&str>) -> PyResult<Vec<Py<PyAny>>> {
        copy_trading_impl::sync::get_existing_lead_positions(self, params_json)
    }

    /// 获取带单仓位历史。
    #[pyo3(signature = (params_json=None))]
    fn get_lead_position_history(&self, params_json: Option<&str>) -> PyResult<Vec<Py<PyAny>>> {
        copy_trading_impl::sync::get_lead_position_history(self, params_json)
    }

    /// 下带单止损订单。
    fn place_lead_stop_order(&self, request_json: &str) -> PyResult<Vec<Py<PyAny>>> {
        copy_trading_impl::sync::place_lead_stop_order(self, request_json)
    }

    /// 平带单仓位。
    fn close_lead_position(&self, request_json: &str) -> PyResult<Vec<Py<PyAny>>> {
        copy_trading_impl::sync::close_lead_position(self, request_json)
    }

    /// 获取可带单交易产品。
    #[pyo3(signature = (params_json=None))]
    fn get_leading_instruments(&self, params_json: Option<&str>) -> PyResult<Vec<Py<PyAny>>> {
        copy_trading_impl::sync::get_leading_instruments(self, params_json)
    }

    /// 修改可带单交易产品。
    fn amend_leading_instruments(&self, request_json: &str) -> PyResult<Vec<Py<PyAny>>> {
        copy_trading_impl::sync::amend_leading_instruments(self, request_json)
    }

    /// 获取分润明细。
    #[pyo3(signature = (params_json=None))]
    fn get_profit_sharing_details(&self, params_json: Option<&str>) -> PyResult<Vec<Py<PyAny>>> {
        copy_trading_impl::sync::get_profit_sharing_details(self, params_json)
    }

    /// 获取总分润。
    fn get_total_profit_sharing(&self) -> PyResult<Vec<Py<PyAny>>> {
        copy_trading_impl::sync::get_total_profit_sharing(self)
    }

    /// 获取未实现分润明细。
    #[pyo3(signature = (params_json=None))]
    fn get_unrealized_profit_sharing_details(
        &self,
        params_json: Option<&str>,
    ) -> PyResult<Vec<Py<PyAny>>> {
        copy_trading_impl::sync::get_unrealized_profit_sharing_details(self, params_json)
    }
}
