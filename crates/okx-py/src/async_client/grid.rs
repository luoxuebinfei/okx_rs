//! Grid API #[pymethods] 块（异步）

use pyo3::prelude::*;

use crate::grid as grid_impl;

use super::PyAsyncOkxClient;

#[pymethods]
impl PyAsyncOkxClient {
    /// 网格策略委托下单（异步）。
    fn grid_order_algo<'py>(
        &self,
        py: Python<'py>,
        payload_json: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        grid_impl::async_api::grid_order_algo(self, py, payload_json)
    }

    /// 获取未完成网格策略委托单列表（异步）。
    #[pyo3(signature = (params_json=None))]
    fn grid_orders_algo_pending<'py>(
        &self,
        py: Python<'py>,
        params_json: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        grid_impl::async_api::grid_orders_algo_pending(self, py, params_json)
    }

    /// 获取历史网格策略委托单列表（异步）。
    #[pyo3(signature = (params_json=None))]
    fn grid_orders_algo_history<'py>(
        &self,
        py: Python<'py>,
        params_json: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        grid_impl::async_api::grid_orders_algo_history(self, py, params_json)
    }

    /// 定投策略委托下单（异步）。
    fn place_recurring_buy_order<'py>(
        &self,
        py: Python<'py>,
        payload_json: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        grid_impl::async_api::place_recurring_buy_order(self, py, payload_json)
    }

    /// 获取定投策略委托单列表（异步）。
    #[pyo3(signature = (params_json=None))]
    fn get_recurring_buy_order_list<'py>(
        &self,
        py: Python<'py>,
        params_json: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        grid_impl::async_api::get_recurring_buy_order_list(self, py, params_json)
    }

    fn grid_amend_order_algo<'py>(
        &self,
        py: Python<'py>,
        payload_json: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        grid_impl::async_api::grid_amend_order_algo(self, py, payload_json)
    }
    fn grid_stop_order_algo<'py>(
        &self,
        py: Python<'py>,
        payload_json: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        grid_impl::async_api::grid_stop_order_algo(self, py, payload_json)
    }
    fn grid_orders_algo_details<'py>(
        &self,
        py: Python<'py>,
        params_json: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        grid_impl::async_api::grid_orders_algo_details(self, py, params_json)
    }
    fn grid_sub_orders<'py>(
        &self,
        py: Python<'py>,
        params_json: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        grid_impl::async_api::grid_sub_orders(self, py, params_json)
    }
    #[pyo3(signature = (params_json=None))]
    fn grid_positions<'py>(
        &self,
        py: Python<'py>,
        params_json: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        grid_impl::async_api::grid_positions(self, py, params_json)
    }
    fn grid_withdraw_income<'py>(
        &self,
        py: Python<'py>,
        payload_json: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        grid_impl::async_api::grid_withdraw_income(self, py, payload_json)
    }
    fn grid_compute_margin_balance<'py>(
        &self,
        py: Python<'py>,
        payload_json: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        grid_impl::async_api::grid_compute_margin_balance(self, py, payload_json)
    }
    fn grid_margin_balance<'py>(
        &self,
        py: Python<'py>,
        payload_json: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        grid_impl::async_api::grid_margin_balance(self, py, payload_json)
    }
    #[pyo3(signature = (params_json=None))]
    fn grid_ai_param<'py>(
        &self,
        py: Python<'py>,
        params_json: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        grid_impl::async_api::grid_ai_param(self, py, params_json)
    }
    fn amend_recurring_buy_order<'py>(
        &self,
        py: Python<'py>,
        payload_json: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        grid_impl::async_api::amend_recurring_buy_order(self, py, payload_json)
    }
    fn stop_recurring_buy_order<'py>(
        &self,
        py: Python<'py>,
        payload_json: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        grid_impl::async_api::stop_recurring_buy_order(self, py, payload_json)
    }
    #[pyo3(signature = (params_json=None))]
    fn get_recurring_buy_order_history<'py>(
        &self,
        py: Python<'py>,
        params_json: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        grid_impl::async_api::get_recurring_buy_order_history(self, py, params_json)
    }
    fn get_recurring_buy_order_details<'py>(
        &self,
        py: Python<'py>,
        params_json: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        grid_impl::async_api::get_recurring_buy_order_details(self, py, params_json)
    }
    fn get_recurring_buy_sub_orders<'py>(
        &self,
        py: Python<'py>,
        params_json: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        grid_impl::async_api::get_recurring_buy_sub_orders(self, py, params_json)
    }
}
