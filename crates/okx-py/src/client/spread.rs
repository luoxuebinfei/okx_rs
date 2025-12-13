//! Spread Trading API #[pymethods] 块

use pyo3::prelude::*;

use crate::spread as spread_impl;

use super::PyOkxClient;

#[pymethods]
impl PyOkxClient {
    // ==================== Spread Trading API ====================

    /// Spread 下单。
    fn spread_place_order(&self, payload_json: &str) -> PyResult<Vec<Py<PyAny>>> {
        spread_impl::sync::place_order(self, payload_json)
    }

    /// Spread 撤单。
    fn spread_cancel_order(&self, payload_json: &str) -> PyResult<Vec<Py<PyAny>>> {
        spread_impl::sync::cancel_order(self, payload_json)
    }

    /// Spread 撤销所有订单。
    fn spread_cancel_all_orders(&self, payload_json: &str) -> PyResult<Vec<Py<PyAny>>> {
        spread_impl::sync::cancel_all_orders(self, payload_json)
    }

    /// 获取 Spread 订单详情。
    fn spread_get_order_details(&self, params_json: &str) -> PyResult<Vec<Py<PyAny>>> {
        spread_impl::sync::get_order_details(self, params_json)
    }

    /// 获取 Spread 活跃订单。
    #[pyo3(signature = (params_json=None))]
    fn spread_get_active_orders(&self, params_json: Option<&str>) -> PyResult<Vec<Py<PyAny>>> {
        spread_impl::sync::get_active_orders(self, params_json)
    }

    /// 获取 Spread 订单历史。
    #[pyo3(signature = (params_json=None))]
    fn spread_get_orders(&self, params_json: Option<&str>) -> PyResult<Vec<Py<PyAny>>> {
        spread_impl::sync::get_orders(self, params_json)
    }

    /// 获取 Spread 成交记录。
    #[pyo3(signature = (params_json=None))]
    fn spread_get_trades(&self, params_json: Option<&str>) -> PyResult<Vec<Py<PyAny>>> {
        spread_impl::sync::get_trades(self, params_json)
    }

    /// 获取 Spread 列表。
    #[pyo3(signature = (params_json=None))]
    fn spread_get_spreads(&self, params_json: Option<&str>) -> PyResult<Vec<Py<PyAny>>> {
        spread_impl::sync::get_spreads(self, params_json)
    }

    /// 获取 Spread 深度。
    fn spread_get_order_book(&self, params_json: &str) -> PyResult<Vec<Py<PyAny>>> {
        spread_impl::sync::get_order_book(self, params_json)
    }

    /// 获取 Spread Ticker。
    fn spread_get_ticker(&self, params_json: &str) -> PyResult<Vec<Py<PyAny>>> {
        spread_impl::sync::get_ticker(self, params_json)
    }

    /// 获取 Spread 公共成交。
    fn spread_get_public_trades(&self, params_json: &str) -> PyResult<Vec<Py<PyAny>>> {
        spread_impl::sync::get_public_trades(self, params_json)
    }
}
