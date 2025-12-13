//! Finance API #[pymethods] 块（异步）

use pyo3::prelude::*;

use crate::finance as finance_impl;

use super::PyAsyncOkxClient;

#[pymethods]
impl PyAsyncOkxClient {
    // ==================== Finance API ====================

    /// 获取 DeFi 产品列表（异步）。
    #[pyo3(signature = (params_json=None))]
    fn defi_get_offers<'py>(
        &self,
        py: Python<'py>,
        params_json: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        finance_impl::async_api::defi_get_offers(self, py, params_json)
    }

    /// 申购 DeFi 产品（异步）。
    fn defi_purchase<'py>(
        &self,
        py: Python<'py>,
        request_json: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        finance_impl::async_api::defi_purchase(self, py, request_json)
    }

    /// 赎回 DeFi 产品（异步）。
    fn defi_redeem<'py>(
        &self,
        py: Python<'py>,
        request_json: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        finance_impl::async_api::defi_redeem(self, py, request_json)
    }

    /// 撤销 DeFi 产品订单（异步）。
    fn defi_cancel<'py>(
        &self,
        py: Python<'py>,
        request_json: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        finance_impl::async_api::defi_cancel(self, py, request_json)
    }

    /// 获取 DeFi 活跃订单列表（异步）。
    #[pyo3(signature = (params_json=None))]
    fn defi_orders_active<'py>(
        &self,
        py: Python<'py>,
        params_json: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        finance_impl::async_api::defi_orders_active(self, py, params_json)
    }

    /// 获取 DeFi 历史订单列表（异步）。
    #[pyo3(signature = (params_json=None))]
    fn defi_orders_history<'py>(
        &self,
        py: Python<'py>,
        params_json: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        finance_impl::async_api::defi_orders_history(self, py, params_json)
    }

    /// 获取余币宝余额（异步）。
    #[pyo3(signature = (params_json=None))]
    fn saving_balance<'py>(
        &self,
        py: Python<'py>,
        params_json: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        finance_impl::async_api::saving_balance(self, py, params_json)
    }

    /// 余币宝申购/赎回（异步）。
    fn saving_purchase_redemption<'py>(
        &self,
        py: Python<'py>,
        request_json: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        finance_impl::async_api::saving_purchase_redemption(self, py, request_json)
    }

    /// 设置余币宝出借利率（异步）。
    fn saving_set_lending_rate<'py>(
        &self,
        py: Python<'py>,
        request_json: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        finance_impl::async_api::saving_set_lending_rate(self, py, request_json)
    }

    /// 获取余币宝出借历史（异步）。
    #[pyo3(signature = (params_json=None))]
    fn saving_lending_history<'py>(
        &self,
        py: Python<'py>,
        params_json: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        finance_impl::async_api::saving_lending_history(self, py, params_json)
    }

    /// 获取余币宝公共出借利率（异步）。
    #[pyo3(signature = (params_json=None))]
    fn saving_public_lending_rate<'py>(
        &self,
        py: Python<'py>,
        params_json: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        finance_impl::async_api::saving_public_lending_rate(self, py, params_json)
    }

    /// 获取余币宝出借利率历史（异步）。
    #[pyo3(signature = (params_json=None))]
    fn saving_lending_rate_history<'py>(
        &self,
        py: Python<'py>,
        params_json: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        finance_impl::async_api::saving_lending_rate_history(self, py, params_json)
    }

    /// Flexible Loan：获取可借币种列表（异步）。
    fn flexible_loan_borrow_currencies<'py>(
        &self,
        py: Python<'py>,
    ) -> PyResult<Bound<'py, PyAny>> {
        finance_impl::async_api::flexible_loan_borrow_currencies(self, py)
    }

    /// Flexible Loan：获取可作为抵押物的币种列表（异步）。
    #[pyo3(signature = (params_json=None))]
    fn flexible_loan_collateral_assets<'py>(
        &self,
        py: Python<'py>,
        params_json: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        finance_impl::async_api::flexible_loan_collateral_assets(self, py, params_json)
    }

    /// Flexible Loan：查询最大可借额度（异步）。
    fn flexible_loan_max_loan<'py>(
        &self,
        py: Python<'py>,
        request_json: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        finance_impl::async_api::flexible_loan_max_loan(self, py, request_json)
    }

    /// Flexible Loan：查询最大可赎回抵押物数量（异步）。
    #[pyo3(signature = (params_json=None))]
    fn flexible_loan_max_collateral_redeem_amount<'py>(
        &self,
        py: Python<'py>,
        params_json: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        finance_impl::async_api::flexible_loan_max_collateral_redeem_amount(self, py, params_json)
    }

    /// Flexible Loan：调整抵押物（异步）。
    fn flexible_loan_adjust_collateral<'py>(
        &self,
        py: Python<'py>,
        request_json: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        finance_impl::async_api::flexible_loan_adjust_collateral(self, py, request_json)
    }

    /// Flexible Loan：查询借款信息（异步）。
    fn flexible_loan_loan_info<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        finance_impl::async_api::flexible_loan_loan_info(self, py)
    }

    /// Flexible Loan：查询借款历史（异步）。
    #[pyo3(signature = (params_json=None))]
    fn flexible_loan_loan_history<'py>(
        &self,
        py: Python<'py>,
        params_json: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        finance_impl::async_api::flexible_loan_loan_history(self, py, params_json)
    }

    /// Flexible Loan：查询计息明细（异步）。
    #[pyo3(signature = (params_json=None))]
    fn flexible_loan_interest_accrued<'py>(
        &self,
        py: Python<'py>,
        params_json: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        finance_impl::async_api::flexible_loan_interest_accrued(self, py, params_json)
    }

    /// Staking-Defi（ETH）：获取产品信息（异步）。
    fn staking_defi_eth_product_info<'py>(
        &self,
        py: Python<'py>,
    ) -> PyResult<Bound<'py, PyAny>> {
        finance_impl::async_api::staking_defi_eth_product_info(self, py)
    }

    /// Staking-Defi（ETH）：获取余额（异步）。
    fn staking_defi_eth_balance<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        finance_impl::async_api::staking_defi_eth_balance(self, py)
    }

    /// Staking-Defi（ETH）：申购（异步）。
    fn staking_defi_eth_purchase<'py>(
        &self,
        py: Python<'py>,
        request_json: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        finance_impl::async_api::staking_defi_eth_purchase(self, py, request_json)
    }

    /// Staking-Defi（ETH）：赎回（异步）。
    fn staking_defi_eth_redeem<'py>(
        &self,
        py: Python<'py>,
        request_json: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        finance_impl::async_api::staking_defi_eth_redeem(self, py, request_json)
    }

    /// Staking-Defi（ETH）：申购/赎回历史（异步）。
    #[pyo3(signature = (params_json=None))]
    fn staking_defi_eth_purchase_redeem_history<'py>(
        &self,
        py: Python<'py>,
        params_json: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        finance_impl::async_api::staking_defi_eth_purchase_redeem_history(self, py, params_json)
    }

    /// Staking-Defi（ETH）：APY 历史（异步，必填 JSON 参数）。
    fn staking_defi_eth_apy_history<'py>(
        &self,
        py: Python<'py>,
        params_json: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        finance_impl::async_api::staking_defi_eth_apy_history(self, py, params_json)
    }

    /// Staking-Defi（SOL）：获取产品信息（异步）。
    fn staking_defi_sol_product_info<'py>(
        &self,
        py: Python<'py>,
    ) -> PyResult<Bound<'py, PyAny>> {
        finance_impl::async_api::staking_defi_sol_product_info(self, py)
    }

    /// Staking-Defi（SOL）：获取余额（异步）。
    fn staking_defi_sol_balance<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        finance_impl::async_api::staking_defi_sol_balance(self, py)
    }

    /// Staking-Defi（SOL）：申购（异步）。
    fn staking_defi_sol_purchase<'py>(
        &self,
        py: Python<'py>,
        request_json: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        finance_impl::async_api::staking_defi_sol_purchase(self, py, request_json)
    }

    /// Staking-Defi（SOL）：赎回（异步）。
    fn staking_defi_sol_redeem<'py>(
        &self,
        py: Python<'py>,
        request_json: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        finance_impl::async_api::staking_defi_sol_redeem(self, py, request_json)
    }

    /// Staking-Defi（SOL）：申购/赎回历史（异步）。
    #[pyo3(signature = (params_json=None))]
    fn staking_defi_sol_purchase_redeem_history<'py>(
        &self,
        py: Python<'py>,
        params_json: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        finance_impl::async_api::staking_defi_sol_purchase_redeem_history(self, py, params_json)
    }

    /// Staking-Defi（SOL）：APY 历史（异步，必填 JSON 参数）。
    fn staking_defi_sol_apy_history<'py>(
        &self,
        py: Python<'py>,
        params_json: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        finance_impl::async_api::staking_defi_sol_apy_history(self, py, params_json)
    }

    /// 获取 Simple Earn 定期出借产品列表（异步）。
    #[pyo3(signature = (params_json=None))]
    fn simple_earn_get_lending_offers<'py>(
        &self,
        py: Python<'py>,
        params_json: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        finance_impl::async_api::simple_earn_get_lending_offers(self, py, params_json)
    }

    /// 获取 Simple Earn 定期出借 APY 历史（异步）。
    #[pyo3(signature = (params_json=None))]
    fn simple_earn_get_lending_apy_history<'py>(
        &self,
        py: Python<'py>,
        params_json: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        finance_impl::async_api::simple_earn_get_lending_apy_history(self, py, params_json)
    }

    /// 获取 Simple Earn 待出借额度（异步）。
    #[pyo3(signature = (params_json=None))]
    fn simple_earn_get_pending_lending_volume<'py>(
        &self,
        py: Python<'py>,
        params_json: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        finance_impl::async_api::simple_earn_get_pending_lending_volume(self, py, params_json)
    }

    /// 下单 Simple Earn 定期出借（异步）。
    fn simple_earn_place_lending_order<'py>(
        &self,
        py: Python<'py>,
        request_json: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        finance_impl::async_api::simple_earn_place_lending_order(self, py, request_json)
    }

    /// 修改 Simple Earn 定期出借订单（异步）。
    fn simple_earn_amend_lending_order<'py>(
        &self,
        py: Python<'py>,
        request_json: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        finance_impl::async_api::simple_earn_amend_lending_order(self, py, request_json)
    }

    /// 获取 Simple Earn 定期出借订单列表（异步）。
    #[pyo3(signature = (params_json=None))]
    fn simple_earn_get_lending_orders_list<'py>(
        &self,
        py: Python<'py>,
        params_json: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        finance_impl::async_api::simple_earn_get_lending_orders_list(self, py, params_json)
    }

    /// 获取 Simple Earn 定期出借子订单列表（异步）。
    #[pyo3(signature = (params_json=None))]
    fn simple_earn_get_lending_sub_orders<'py>(
        &self,
        py: Python<'py>,
        params_json: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        finance_impl::async_api::simple_earn_get_lending_sub_orders(self, py, params_json)
    }
}
