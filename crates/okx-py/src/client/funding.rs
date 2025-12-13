//! Funding API #[pymethods] 块

use pyo3::prelude::*;

use okx_core::types::{FundsTransferRequest, WithdrawalRequest};
use okx_rest::api::funding::{
    CancelWithdrawalParams, ConvertDustAssetsRequest, GetDepositHistoryParams,
    GetDepositLightningParams, GetDepositWithdrawStatusParams, GetFundingBillsParams,
    GetLendingHistoryParams, GetLendingRateHistoryParams, GetLendingRateSummaryParams,
    GetSavingBalanceParams, GetTransferStateParams, GetWithdrawalHistoryParams,
    PurchaseRedemptRequest, SetLendingRateRequest, WithdrawalLightningRequest,
};

use crate::funding as funding_impl;
use crate::types::*;

use super::PyOkxClient;

#[pymethods]
impl PyOkxClient {
    /// 获取资金账户余额。
    #[pyo3(signature = (ccy=None))]
    fn get_asset_balances(&self, ccy: Option<&str>) -> PyResult<Vec<PyAssetBalance>> {
        funding_impl::sync::get_asset_balances(self, ccy)
    }

    /// 资金划转。
    #[pyo3(signature = (ccy, amt, from_account, to_account, transfer_type=None, sub_acct=None, inst_id=None, to_inst_id=None, loan_trans=None))]
    fn funds_transfer(
        &self,
        ccy: &str,
        amt: &str,
        from_account: &str,
        to_account: &str,
        transfer_type: Option<&str>,
        sub_acct: Option<&str>,
        inst_id: Option<&str>,
        to_inst_id: Option<&str>,
        loan_trans: Option<bool>,
    ) -> PyResult<Option<PyFundsTransferResult>> {
        let request = FundsTransferRequest {
            ccy: ccy.to_string(),
            amt: amt.to_string(),
            from: from_account.to_string(),
            to: to_account.to_string(),
            r#type: transfer_type.map(String::from),
            sub_acct: sub_acct.map(String::from),
            inst_id: inst_id.map(String::from),
            to_inst_id: to_inst_id.map(String::from),
            loan_trans,
        };
        funding_impl::sync::funds_transfer(self, request)
    }

    /// 提币。
    #[pyo3(signature = (ccy, amt, dest, to_addr, chain=None, area_code=None, client_id=None, fee=None))]
    fn withdrawal(
        &self,
        ccy: &str,
        amt: &str,
        dest: &str,
        to_addr: &str,
        chain: Option<&str>,
        area_code: Option<&str>,
        client_id: Option<&str>,
        fee: Option<&str>,
    ) -> PyResult<Option<PyWithdrawalResult>> {
        let request = WithdrawalRequest {
            ccy: ccy.to_string(),
            amt: amt.to_string(),
            dest: dest.to_string(),
            to_addr: to_addr.to_string(),
            chain: chain.map(String::from),
            area_code: area_code.map(String::from),
            client_id: client_id.map(String::from),
            fee: fee.map(String::from),
        };
        funding_impl::sync::withdrawal(self, request)
    }

    /// 获取充值地址。
    fn get_deposit_address(&self, ccy: &str) -> PyResult<Vec<PyDepositAddress>> {
        funding_impl::sync::get_deposit_address(self, ccy)
    }

    /// 获取充值记录。
    #[pyo3(signature = (ccy=None, dep_id=None, tx_id=None, record_type=None, state=None, after=None, before=None, limit=None))]
    fn get_deposit_history(
        &self,
        ccy: Option<&str>,
        dep_id: Option<&str>,
        tx_id: Option<&str>,
        record_type: Option<&str>,
        state: Option<&str>,
        after: Option<&str>,
        before: Option<&str>,
        limit: Option<&str>,
    ) -> PyResult<Vec<PyDepositRecord>> {
        let params = GetDepositHistoryParams {
            ccy: ccy.map(String::from),
            dep_id: dep_id.map(String::from),
            tx_id: tx_id.map(String::from),
            r#type: record_type.map(String::from),
            state: state.map(String::from),
            after: after.map(String::from),
            before: before.map(String::from),
            limit: limit.map(String::from),
        };
        funding_impl::sync::get_deposit_history(self, params)
    }

    /// 获取提币记录。
    #[pyo3(signature = (ccy=None, wd_id=None, client_id=None, tx_id=None, record_type=None, state=None, after=None, before=None, limit=None))]
    fn get_withdrawal_history(
        &self,
        ccy: Option<&str>,
        wd_id: Option<&str>,
        client_id: Option<&str>,
        tx_id: Option<&str>,
        record_type: Option<&str>,
        state: Option<&str>,
        after: Option<&str>,
        before: Option<&str>,
        limit: Option<&str>,
    ) -> PyResult<Vec<PyWithdrawalRecord>> {
        let params = GetWithdrawalHistoryParams {
            ccy: ccy.map(String::from),
            wd_id: wd_id.map(String::from),
            client_id: client_id.map(String::from),
            tx_id: tx_id.map(String::from),
            r#type: record_type.map(String::from),
            state: state.map(String::from),
            after: after.map(String::from),
            before: before.map(String::from),
            limit: limit.map(String::from),
        };
        funding_impl::sync::get_withdrawal_history(self, params)
    }

    /// 获取币种列表。
    #[pyo3(signature = (ccy=None))]
    fn get_currencies(&self, ccy: Option<&str>) -> PyResult<Vec<PyCurrencyInfo>> {
        funding_impl::sync::get_currencies(self, ccy)
    }

    /// 获取不可交易资产。
    #[pyo3(signature = (ccy=None))]
    fn get_non_tradable_assets(&self, ccy: Option<&str>) -> PyResult<Vec<Py<PyAny>>> {
        funding_impl::sync::get_non_tradable_assets(self, ccy)
    }

    /// 获取资产估值。
    #[pyo3(signature = (ccy=None))]
    fn get_asset_valuation(&self, ccy: Option<&str>) -> PyResult<Vec<Py<PyAny>>> {
        funding_impl::sync::get_asset_valuation(self, ccy)
    }

    /// 查询划转状态。
    fn get_transfer_state(&self, trans_id: &str, type_: Option<&str>) -> PyResult<Vec<Py<PyAny>>> {
        let params = GetTransferStateParams {
            trans_id: trans_id.to_string(),
            r#type: type_.map(String::from),
        };
        funding_impl::sync::get_transfer_state(self, params)
    }

    /// 获取资金流水。
    #[pyo3(signature = (ccy=None, type_=None, after=None, before=None, limit=None))]
    fn get_funding_bills(
        &self,
        ccy: Option<&str>,
        type_: Option<&str>,
        after: Option<&str>,
        before: Option<&str>,
        limit: Option<&str>,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let params = if ccy.is_some()
            || type_.is_some()
            || after.is_some()
            || before.is_some()
            || limit.is_some()
        {
            Some(GetFundingBillsParams {
                ccy: ccy.map(String::from),
                r#type: type_.map(String::from),
                after: after.map(String::from),
                before: before.map(String::from),
                limit: limit.map(String::from),
            })
        } else {
            None
        };
        funding_impl::sync::get_funding_bills(self, params)
    }
}

// Funding API 续 - 闪电网络、借贷等
#[pymethods]
impl PyOkxClient {
    /// 闪电网络充值。
    fn get_deposit_lightning(
        &self,
        ccy: &str,
        amt: &str,
        to: Option<&str>,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let params = GetDepositLightningParams {
            ccy: ccy.to_string(),
            amt: amt.to_string(),
            to: to.map(String::from),
        };
        funding_impl::sync::get_deposit_lightning(self, params)
    }

    /// 闪电网络提币。
    fn withdrawal_lightning(
        &self,
        ccy: &str,
        invoice: &str,
        memo: Option<&str>,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let request = WithdrawalLightningRequest {
            ccy: ccy.to_string(),
            invoice: invoice.to_string(),
            memo: memo.map(String::from),
        };
        funding_impl::sync::withdrawal_lightning(self, request)
    }

    /// 撤销提币。
    #[pyo3(signature = (wd_id=None))]
    fn cancel_withdrawal(&self, wd_id: Option<&str>) -> PyResult<Vec<Py<PyAny>>> {
        let params = CancelWithdrawalParams {
            wd_id: wd_id.map(String::from),
        };
        funding_impl::sync::cancel_withdrawal(self, params)
    }

    /// 查询充值/提币状态。
    #[pyo3(signature = (wd_id=None, tx_id=None, ccy=None, to=None, chain=None))]
    fn get_deposit_withdraw_status(
        &self,
        wd_id: Option<&str>,
        tx_id: Option<&str>,
        ccy: Option<&str>,
        to: Option<&str>,
        chain: Option<&str>,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let params = GetDepositWithdrawStatusParams {
            wd_id: wd_id.map(String::from),
            tx_id: tx_id.map(String::from),
            ccy: ccy.map(String::from),
            to: to.map(String::from),
            chain: chain.map(String::from),
        };
        funding_impl::sync::get_deposit_withdraw_status(self, params)
    }

    /// 余币宝申购/赎回。
    fn purchase_redempt(
        &self,
        ccy: &str,
        amt: &str,
        side: &str,
        rate: Option<&str>,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let request = PurchaseRedemptRequest {
            ccy: ccy.to_string(),
            amt: amt.to_string(),
            side: side.to_string(),
            rate: rate.map(String::from),
        };
        funding_impl::sync::purchase_redempt(self, request)
    }

    /// 设置出借利率。
    fn set_lending_rate(&self, ccy: &str, rate: &str) -> PyResult<Vec<Py<PyAny>>> {
        let request = SetLendingRateRequest {
            ccy: ccy.to_string(),
            rate: rate.to_string(),
        };
        funding_impl::sync::set_lending_rate(self, request)
    }

    /// 获取出借历史。
    #[pyo3(signature = (ccy=None, after=None, before=None, limit=None))]
    fn get_lending_history(
        &self,
        ccy: Option<&str>,
        after: Option<&str>,
        before: Option<&str>,
        limit: Option<&str>,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let params = GetLendingHistoryParams {
            ccy: ccy.map(String::from),
            after: after.map(String::from),
            before: before.map(String::from),
            limit: limit.map(String::from),
        };
        funding_impl::sync::get_lending_history(self, Some(params))
    }

    /// 获取出借利率历史。
    #[pyo3(signature = (ccy=None, after=None, before=None, limit=None))]
    fn get_lending_rate_history(
        &self,
        ccy: Option<&str>,
        after: Option<&str>,
        before: Option<&str>,
        limit: Option<&str>,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let params = GetLendingRateHistoryParams {
            ccy: ccy.map(String::from),
            after: after.map(String::from),
            before: before.map(String::from),
            limit: limit.map(String::from),
        };
        funding_impl::sync::get_lending_rate_history(self, Some(params))
    }

    /// 获取出借利率汇总。
    #[pyo3(signature = (ccy=None))]
    fn get_lending_rate_summary(&self, ccy: Option<&str>) -> PyResult<Vec<Py<PyAny>>> {
        let params = GetLendingRateSummaryParams {
            ccy: ccy.map(String::from),
        };
        funding_impl::sync::get_lending_rate_summary(self, Some(params))
    }

    /// 小额资产兑换。
    fn convert_dust_assets(&self, ccy: Vec<String>) -> PyResult<Vec<Py<PyAny>>> {
        let request = ConvertDustAssetsRequest { ccy: Some(ccy) };
        funding_impl::sync::convert_dust_assets(self, request)
    }

    /// 获取余币宝余额。
    #[pyo3(signature = (ccy=None))]
    fn get_saving_balance(&self, ccy: Option<&str>) -> PyResult<Vec<Py<PyAny>>> {
        let params = GetSavingBalanceParams {
            ccy: ccy.map(String::from),
        };
        funding_impl::sync::get_saving_balance(self, Some(params))
    }
}
