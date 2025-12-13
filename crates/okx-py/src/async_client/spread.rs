//! Spread Trading API #[pymethods] 块（异步）

use pyo3::prelude::*;

use crate::spread as spread_impl;

use super::PyAsyncOkxClient;

#[pymethods]
impl PyAsyncOkxClient {
    /// Spread 下单（异步）。
    fn spread_place_order<'py>(
        &self,
        py: Python<'py>,
        payload_json: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        spread_impl::async_api::place_order(self, py, payload_json)
    }

    /// Spread 撤单（异步）。
    fn spread_cancel_order<'py>(
        &self,
        py: Python<'py>,
        payload_json: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        spread_impl::async_api::cancel_order(self, py, payload_json)
    }

    /// Spread 撤销所有订单（异步）。
    fn spread_cancel_all_orders<'py>(
        &self,
        py: Python<'py>,
        payload_json: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        spread_impl::async_api::cancel_all_orders(self, py, payload_json)
    }

    /// 获取 Spread 订单详情（异步）。
    fn spread_get_order_details<'py>(
        &self,
        py: Python<'py>,
        params_json: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        spread_impl::async_api::get_order_details(self, py, params_json)
    }

    /// 获取 Spread 活跃订单（异步）。
    #[pyo3(signature = (params_json=None))]
    fn spread_get_active_orders<'py>(
        &self,
        py: Python<'py>,
        params_json: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        spread_impl::async_api::get_active_orders(self, py, params_json)
    }

    /// 获取 Spread 订单历史（异步）。
    #[pyo3(signature = (params_json=None))]
    fn spread_get_orders<'py>(
        &self,
        py: Python<'py>,
        params_json: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        spread_impl::async_api::get_orders(self, py, params_json)
    }

    /// 获取 Spread 成交记录（异步）。
    #[pyo3(signature = (params_json=None))]
    fn spread_get_trades<'py>(
        &self,
        py: Python<'py>,
        params_json: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        spread_impl::async_api::get_trades(self, py, params_json)
    }

    /// 获取 Spread 列表（异步）。
    #[pyo3(signature = (params_json=None))]
    fn spread_get_spreads<'py>(
        &self,
        py: Python<'py>,
        params_json: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        spread_impl::async_api::get_spreads(self, py, params_json)
    }

    /// 获取 Spread 深度（异步）。
    fn spread_get_order_book<'py>(
        &self,
        py: Python<'py>,
        params_json: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        spread_impl::async_api::get_order_book(self, py, params_json)
    }

    /// 获取 Spread Ticker（异步）。
    fn spread_get_ticker<'py>(
        &self,
        py: Python<'py>,
        params_json: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        spread_impl::async_api::get_ticker(self, py, params_json)
    }

    /// 获取 Spread 公共成交（异步）。
    fn spread_get_public_trades<'py>(
        &self,
        py: Python<'py>,
        params_json: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        spread_impl::async_api::get_public_trades(self, py, params_json)
    }
}
