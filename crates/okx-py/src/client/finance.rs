//! Finance API #[pymethods] 块

use pyo3::prelude::*;

use crate::finance as finance_impl;

use super::PyOkxClient;

#[pymethods]
impl PyOkxClient {
    // ==================== Finance API ====================

    /// 获取 DeFi 产品列表。
    #[pyo3(signature = (params_json=None))]
    fn defi_get_offers(&self, params_json: Option<&str>) -> PyResult<Vec<Py<PyAny>>> {
        finance_impl::sync::defi_get_offers(self, params_json)
    }

    /// 申购 DeFi 产品。
    fn defi_purchase(&self, request_json: &str) -> PyResult<Vec<Py<PyAny>>> {
        finance_impl::sync::defi_purchase(self, request_json)
    }

    /// 赎回 DeFi 产品。
    fn defi_redeem(&self, request_json: &str) -> PyResult<Vec<Py<PyAny>>> {
        finance_impl::sync::defi_redeem(self, request_json)
    }

    /// 撤销 DeFi 产品订单。
    fn defi_cancel(&self, request_json: &str) -> PyResult<Vec<Py<PyAny>>> {
        finance_impl::sync::defi_cancel(self, request_json)
    }

    /// 获取 DeFi 活跃订单列表。
    #[pyo3(signature = (params_json=None))]
    fn defi_orders_active(&self, params_json: Option<&str>) -> PyResult<Vec<Py<PyAny>>> {
        finance_impl::sync::defi_orders_active(self, params_json)
    }

    /// 获取 DeFi 历史订单列表。
    #[pyo3(signature = (params_json=None))]
    fn defi_orders_history(&self, params_json: Option<&str>) -> PyResult<Vec<Py<PyAny>>> {
        finance_impl::sync::defi_orders_history(self, params_json)
    }

    /// 获取余币宝余额。
    #[pyo3(signature = (params_json=None))]
    fn saving_balance(&self, params_json: Option<&str>) -> PyResult<Vec<Py<PyAny>>> {
        finance_impl::sync::saving_balance(self, params_json)
    }

    /// 余币宝申购/赎回。
    fn saving_purchase_redemption(&self, request_json: &str) -> PyResult<Vec<Py<PyAny>>> {
        finance_impl::sync::saving_purchase_redemption(self, request_json)
    }

    /// 设置余币宝出借利率。
    fn saving_set_lending_rate(&self, request_json: &str) -> PyResult<Vec<Py<PyAny>>> {
        finance_impl::sync::saving_set_lending_rate(self, request_json)
    }

    /// 获取余币宝出借历史。
    #[pyo3(signature = (params_json=None))]
    fn saving_lending_history(&self, params_json: Option<&str>) -> PyResult<Vec<Py<PyAny>>> {
        finance_impl::sync::saving_lending_history(self, params_json)
    }

    /// 获取余币宝公共出借利率。
    #[pyo3(signature = (params_json=None))]
    fn saving_public_lending_rate(&self, params_json: Option<&str>) -> PyResult<Vec<Py<PyAny>>> {
        finance_impl::sync::saving_public_lending_rate(self, params_json)
    }

    /// 获取 Simple Earn 定期出借产品列表。
    #[pyo3(signature = (params_json=None))]
    fn simple_earn_get_lending_offers(
        &self,
        params_json: Option<&str>,
    ) -> PyResult<Vec<Py<PyAny>>> {
        finance_impl::sync::simple_earn_get_lending_offers(self, params_json)
    }

    /// 获取 Simple Earn 定期出借 APY 历史。
    #[pyo3(signature = (params_json=None))]
    fn simple_earn_get_lending_apy_history(
        &self,
        params_json: Option<&str>,
    ) -> PyResult<Vec<Py<PyAny>>> {
        finance_impl::sync::simple_earn_get_lending_apy_history(self, params_json)
    }

    /// 获取 Simple Earn 待出借额度。
    #[pyo3(signature = (params_json=None))]
    fn simple_earn_get_pending_lending_volume(
        &self,
        params_json: Option<&str>,
    ) -> PyResult<Vec<Py<PyAny>>> {
        finance_impl::sync::simple_earn_get_pending_lending_volume(self, params_json)
    }

    /// 下单 Simple Earn 定期出借。
    fn simple_earn_place_lending_order(&self, request_json: &str) -> PyResult<Vec<Py<PyAny>>> {
        finance_impl::sync::simple_earn_place_lending_order(self, request_json)
    }

    /// 修改 Simple Earn 定期出借订单。
    fn simple_earn_amend_lending_order(&self, request_json: &str) -> PyResult<Vec<Py<PyAny>>> {
        finance_impl::sync::simple_earn_amend_lending_order(self, request_json)
    }

    /// 获取 Simple Earn 定期出借订单列表。
    #[pyo3(signature = (params_json=None))]
    fn simple_earn_get_lending_orders_list(
        &self,
        params_json: Option<&str>,
    ) -> PyResult<Vec<Py<PyAny>>> {
        finance_impl::sync::simple_earn_get_lending_orders_list(self, params_json)
    }

    /// 获取 Simple Earn 定期出借子订单列表。
    #[pyo3(signature = (params_json=None))]
    fn simple_earn_get_lending_sub_orders(
        &self,
        params_json: Option<&str>,
    ) -> PyResult<Vec<Py<PyAny>>> {
        finance_impl::sync::simple_earn_get_lending_sub_orders(self, params_json)
    }
}
