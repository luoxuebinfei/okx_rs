//! Grid API #[pymethods] 块

use pyo3::prelude::*;

use crate::grid as grid_impl;

use super::PyOkxClient;

#[pymethods]
impl PyOkxClient {
    // ==================== Grid Algo API ====================

    /// 网格策略委托下单。
    fn grid_order_algo(&self, payload_json: &str) -> PyResult<Vec<Py<PyAny>>> {
        grid_impl::sync::grid_order_algo(self, payload_json)
    }

    /// 获取未完成网格策略委托单列表。
    #[pyo3(signature = (params_json=None))]
    fn grid_orders_algo_pending(&self, params_json: Option<&str>) -> PyResult<Vec<Py<PyAny>>> {
        grid_impl::sync::grid_orders_algo_pending(self, params_json)
    }

    /// 获取历史网格策略委托单列表。
    #[pyo3(signature = (params_json=None))]
    fn grid_orders_algo_history(&self, params_json: Option<&str>) -> PyResult<Vec<Py<PyAny>>> {
        grid_impl::sync::grid_orders_algo_history(self, params_json)
    }

    // ==================== Recurring Buy API ====================

    /// 定投策略委托下单。
    fn place_recurring_buy_order(&self, payload_json: &str) -> PyResult<Vec<Py<PyAny>>> {
        grid_impl::sync::place_recurring_buy_order(self, payload_json)
    }

    /// 获取定投策略委托单列表。
    #[pyo3(signature = (params_json=None))]
    fn get_recurring_buy_order_list(&self, params_json: Option<&str>) -> PyResult<Vec<Py<PyAny>>> {
        grid_impl::sync::get_recurring_buy_order_list(self, params_json)
    }

    /// 修改网格策略委托单。
    fn grid_amend_order_algo(&self, payload_json: &str) -> PyResult<Vec<Py<PyAny>>> {
        grid_impl::sync::grid_amend_order_algo(self, payload_json)
    }

    /// 停止网格策略委托单。
    fn grid_stop_order_algo(&self, payload_json: &str) -> PyResult<Vec<Py<PyAny>>> {
        grid_impl::sync::grid_stop_order_algo(self, payload_json)
    }

    /// 获取网格策略委托单详情。
    fn grid_orders_algo_details(&self, params_json: &str) -> PyResult<Vec<Py<PyAny>>> {
        grid_impl::sync::grid_orders_algo_details(self, params_json)
    }

    /// 获取网格策略委托子订单。
    fn grid_sub_orders(&self, params_json: &str) -> PyResult<Vec<Py<PyAny>>> {
        grid_impl::sync::grid_sub_orders(self, params_json)
    }

    /// 获取网格策略持仓。
    #[pyo3(signature = (params_json=None))]
    fn grid_positions(&self, params_json: Option<&str>) -> PyResult<Vec<Py<PyAny>>> {
        grid_impl::sync::grid_positions(self, params_json)
    }

    /// 网格策略提取利润。
    fn grid_withdraw_income(&self, payload_json: &str) -> PyResult<Vec<Py<PyAny>>> {
        grid_impl::sync::grid_withdraw_income(self, payload_json)
    }

    /// 计算网格保证金。
    fn grid_compute_margin_balance(&self, payload_json: &str) -> PyResult<Vec<Py<PyAny>>> {
        grid_impl::sync::grid_compute_margin_balance(self, payload_json)
    }

    /// 调整网格保证金。
    fn grid_margin_balance(&self, payload_json: &str) -> PyResult<Vec<Py<PyAny>>> {
        grid_impl::sync::grid_margin_balance(self, payload_json)
    }

    /// 获取网格 AI 参数。
    #[pyo3(signature = (params_json=None))]
    fn grid_ai_param(&self, params_json: Option<&str>) -> PyResult<Vec<Py<PyAny>>> {
        grid_impl::sync::grid_ai_param(self, params_json)
    }

    /// 修改定投策略委托单。
    fn amend_recurring_buy_order(&self, payload_json: &str) -> PyResult<Vec<Py<PyAny>>> {
        grid_impl::sync::amend_recurring_buy_order(self, payload_json)
    }

    /// 停止定投策略委托单。
    fn stop_recurring_buy_order(&self, payload_json: &str) -> PyResult<Vec<Py<PyAny>>> {
        grid_impl::sync::stop_recurring_buy_order(self, payload_json)
    }

    /// 获取定投策略委托单历史。
    #[pyo3(signature = (params_json=None))]
    fn get_recurring_buy_order_history(
        &self,
        params_json: Option<&str>,
    ) -> PyResult<Vec<Py<PyAny>>> {
        grid_impl::sync::get_recurring_buy_order_history(self, params_json)
    }

    /// 获取定投策略委托单详情。
    fn get_recurring_buy_order_details(&self, params_json: &str) -> PyResult<Vec<Py<PyAny>>> {
        grid_impl::sync::get_recurring_buy_order_details(self, params_json)
    }

    /// 获取定投策略委托子订单。
    fn get_recurring_buy_sub_orders(&self, params_json: &str) -> PyResult<Vec<Py<PyAny>>> {
        grid_impl::sync::get_recurring_buy_sub_orders(self, params_json)
    }
}
