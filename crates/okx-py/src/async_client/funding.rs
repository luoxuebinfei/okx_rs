//! Funding API #[pymethods] 块（异步）

use pyo3::prelude::*;

use okx_core::types::{FundsTransferRequest, WithdrawalRequest};
use okx_rest::api::funding::{GetDepositHistoryParams, GetWithdrawalHistoryParams};

use crate::funding as funding_impl;

use super::PyAsyncOkxClient;

#[pymethods]
impl PyAsyncOkxClient {
    /// 获取资金账户余额（异步）。
    #[pyo3(signature = (ccy=None))]
    fn get_funding_balance<'py>(
        &self,
        py: Python<'py>,
        ccy: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        funding_impl::async_api::get_asset_balances(self, py, ccy)
    }

    /// 获取资金账户余额（异步）。
    ///
    /// 说明：该方法名与 Rust `FundingApi.get_asset_balances` 保持一致；
    /// `get_funding_balance` 保留为兼容别名。
    #[pyo3(signature = (ccy=None))]
    fn get_asset_balances<'py>(
        &self,
        py: Python<'py>,
        ccy: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        funding_impl::async_api::get_asset_balances(self, py, ccy)
    }

    /// 资金划转（异步）。
    #[pyo3(signature = (ccy, amt, from_acct, to_acct, sub_acct=None, inst_id=None, to_inst_id=None))]
    fn funds_transfer<'py>(
        &self,
        py: Python<'py>,
        ccy: String,
        amt: String,
        from_acct: String,
        to_acct: String,
        sub_acct: Option<String>,
        inst_id: Option<String>,
        to_inst_id: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let request = FundsTransferRequest {
            ccy,
            amt,
            from: from_acct,
            to: to_acct,
            sub_acct,
            inst_id,
            to_inst_id,
            r#type: None,
            loan_trans: None,
        };
        funding_impl::async_api::funds_transfer(self, py, request)
    }

    /// 获取充值地址（异步）。
    fn get_deposit_address<'py>(
        &self,
        py: Python<'py>,
        ccy: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        funding_impl::async_api::get_deposit_address(self, py, ccy)
    }

    /// 获取充值记录（异步）。
    #[pyo3(signature = (ccy=None, dep_id=None, tx_id=None, state=None, after=None, before=None, limit=None))]
    fn get_deposit_history<'py>(
        &self,
        py: Python<'py>,
        ccy: Option<String>,
        dep_id: Option<String>,
        tx_id: Option<String>,
        state: Option<String>,
        after: Option<String>,
        before: Option<String>,
        limit: Option<u32>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let params = GetDepositHistoryParams {
            ccy,
            dep_id,
            tx_id,
            state,
            after,
            before,
            limit: limit.map(|v| v.to_string()),
            r#type: None,
        };
        funding_impl::async_api::get_deposit_history(self, py, params)
    }

    /// 提币（异步）。
    #[pyo3(signature = (ccy, amt, dest, to_addr, fee, chain=None, area_code=None, client_id=None))]
    fn withdrawal<'py>(
        &self,
        py: Python<'py>,
        ccy: String,
        amt: String,
        dest: String,
        to_addr: String,
        fee: String,
        chain: Option<String>,
        area_code: Option<String>,
        client_id: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let request = WithdrawalRequest {
            ccy,
            amt,
            dest,
            to_addr,
            fee: Some(fee),
            chain,
            area_code,
            client_id,
        };
        funding_impl::async_api::withdrawal(self, py, request)
    }

    /// 获取提币记录（异步）。
    #[pyo3(signature = (ccy=None, wd_id=None, tx_id=None, state=None, after=None, before=None, limit=None))]
    fn get_withdrawal_history<'py>(
        &self,
        py: Python<'py>,
        ccy: Option<String>,
        wd_id: Option<String>,
        tx_id: Option<String>,
        state: Option<String>,
        after: Option<String>,
        before: Option<String>,
        limit: Option<u32>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let params = GetWithdrawalHistoryParams {
            ccy,
            wd_id,
            tx_id,
            state,
            after,
            before,
            limit: limit.map(|v| v.to_string()),
            client_id: None,
            r#type: None,
        };
        funding_impl::async_api::get_withdrawal_history(self, py, params)
    }

    /// 获取币种列表（异步）。
    #[pyo3(signature = (ccy=None))]
    fn get_currencies<'py>(
        &self,
        py: Python<'py>,
        ccy: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        funding_impl::async_api::get_currencies(self, py, ccy)
    }

    /// 获取不可交易资产（异步）。
    #[pyo3(signature = (ccy=None))]
    fn get_non_tradable_assets<'py>(
        &self,
        py: Python<'py>,
        ccy: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        funding_impl::async_api::get_non_tradable_assets(self, py, ccy)
    }

    /// 获取资产估值（异步）。
    #[pyo3(signature = (ccy=None))]
    fn get_asset_valuation<'py>(
        &self,
        py: Python<'py>,
        ccy: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        funding_impl::async_api::get_asset_valuation(self, py, ccy)
    }

    /// 查询划转状态（异步）。
    fn get_transfer_state<'py>(
        &self,
        py: Python<'py>,
        trans_id: String,
        type_: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        use okx_rest::api::funding::GetTransferStateParams;
        let params = GetTransferStateParams {
            trans_id,
            r#type: type_,
        };
        funding_impl::async_api::get_transfer_state(self, py, params)
    }

    /// 获取资金流水（异步）。
    #[pyo3(signature = (ccy=None, type_=None, after=None, before=None, limit=None))]
    fn get_funding_bills<'py>(
        &self,
        py: Python<'py>,
        ccy: Option<String>,
        type_: Option<String>,
        after: Option<String>,
        before: Option<String>,
        limit: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        use okx_rest::api::funding::GetFundingBillsParams;
        let params = if ccy.is_some()
            || type_.is_some()
            || after.is_some()
            || before.is_some()
            || limit.is_some()
        {
            Some(GetFundingBillsParams {
                ccy,
                r#type: type_,
                after,
                before,
                limit,
            })
        } else {
            None
        };
        funding_impl::async_api::get_funding_bills(self, py, params)
    }

    /// 余币宝申购/赎回（异步）。
    fn purchase_redempt<'py>(
        &self,
        py: Python<'py>,
        ccy: String,
        amt: String,
        side: String,
        rate: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        use okx_rest::api::funding::PurchaseRedemptRequest;
        let request = PurchaseRedemptRequest {
            ccy,
            amt,
            side,
            rate,
        };
        funding_impl::async_api::purchase_redempt(self, py, request)
    }

    /// 闪电网络充值（异步）。
    fn get_deposit_lightning<'py>(
        &self,
        py: Python<'py>,
        ccy: String,
        amt: String,
        to: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        use okx_rest::api::funding::GetDepositLightningParams;
        let params = GetDepositLightningParams { ccy, amt, to };
        funding_impl::async_api::get_deposit_lightning(self, py, params)
    }

    /// 闪电网络提币（异步）。
    fn withdrawal_lightning<'py>(
        &self,
        py: Python<'py>,
        ccy: String,
        invoice: String,
        memo: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        use okx_rest::api::funding::WithdrawalLightningRequest;
        let request = WithdrawalLightningRequest { ccy, invoice, memo };
        funding_impl::async_api::withdrawal_lightning(self, py, request)
    }

    /// 撤销提币（异步）。
    #[pyo3(signature = (wd_id=None))]
    fn cancel_withdrawal<'py>(
        &self,
        py: Python<'py>,
        wd_id: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        use okx_rest::api::funding::CancelWithdrawalParams;
        let params = CancelWithdrawalParams { wd_id };
        funding_impl::async_api::cancel_withdrawal(self, py, params)
    }

    /// 查询充值/提币状态（异步）。
    #[pyo3(signature = (wd_id=None, tx_id=None, ccy=None, to=None, chain=None))]
    fn get_deposit_withdraw_status<'py>(
        &self,
        py: Python<'py>,
        wd_id: Option<String>,
        tx_id: Option<String>,
        ccy: Option<String>,
        to: Option<String>,
        chain: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        use okx_rest::api::funding::GetDepositWithdrawStatusParams;
        let params = GetDepositWithdrawStatusParams {
            wd_id,
            tx_id,
            ccy,
            to,
            chain,
        };
        funding_impl::async_api::get_deposit_withdraw_status(self, py, params)
    }

    /// 设置出借利率（异步）。
    fn set_lending_rate<'py>(
        &self,
        py: Python<'py>,
        ccy: String,
        rate: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        use okx_rest::api::funding::SetLendingRateRequest;
        let request = SetLendingRateRequest { ccy, rate };
        funding_impl::async_api::set_lending_rate(self, py, request)
    }

    /// 获取出借历史（异步）。
    #[pyo3(signature = (ccy=None, after=None, before=None, limit=None))]
    fn get_lending_history<'py>(
        &self,
        py: Python<'py>,
        ccy: Option<String>,
        after: Option<String>,
        before: Option<String>,
        limit: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        use okx_rest::api::funding::GetLendingHistoryParams;
        let params = GetLendingHistoryParams {
            ccy,
            after,
            before,
            limit,
        };
        funding_impl::async_api::get_lending_history(self, py, Some(params))
    }

    /// 获取出借利率历史（异步）。
    #[pyo3(signature = (ccy=None, after=None, before=None, limit=None))]
    fn get_lending_rate_history<'py>(
        &self,
        py: Python<'py>,
        ccy: Option<String>,
        after: Option<String>,
        before: Option<String>,
        limit: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        use okx_rest::api::funding::GetLendingRateHistoryParams;
        let params = GetLendingRateHistoryParams {
            ccy,
            after,
            before,
            limit,
        };
        funding_impl::async_api::get_lending_rate_history(self, py, Some(params))
    }

    /// 获取出借利率汇总（异步）。
    #[pyo3(signature = (ccy=None))]
    fn get_lending_rate_summary<'py>(
        &self,
        py: Python<'py>,
        ccy: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        use okx_rest::api::funding::GetLendingRateSummaryParams;
        let params = GetLendingRateSummaryParams { ccy };
        funding_impl::async_api::get_lending_rate_summary(self, py, Some(params))
    }

    /// 小额资产兑换（异步）。
    fn convert_dust_assets<'py>(
        &self,
        py: Python<'py>,
        ccy: Vec<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        use okx_rest::api::funding::ConvertDustAssetsRequest;
        let request = ConvertDustAssetsRequest { ccy: Some(ccy) };
        funding_impl::async_api::convert_dust_assets(self, py, request)
    }

    /// 获取余币宝余额（异步）。
    #[pyo3(signature = (ccy=None))]
    fn get_saving_balance<'py>(
        &self,
        py: Python<'py>,
        ccy: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        use okx_rest::api::funding::GetSavingBalanceParams;
        let params = GetSavingBalanceParams { ccy };
        funding_impl::async_api::get_saving_balance(self, py, Some(params))
    }
}
