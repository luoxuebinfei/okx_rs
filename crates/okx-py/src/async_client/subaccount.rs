//! Subaccount API #[pymethods] 块（异步）

use pyo3::prelude::*;

use okx_rest::api::subaccount::{
    ResetSubaccountApikeyRequest, SetTransferOutRequest, SubaccountListParams,
    SubaccountTransferRequest,
};

use crate::subaccount as subaccount_impl;

use super::PyAsyncOkxClient;

#[pymethods]
impl PyAsyncOkxClient {
    /// 获取子账户余额（异步）。
    fn get_subaccount_balance<'py>(
        &self,
        py: Python<'py>,
        sub_acct: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        subaccount_impl::async_api::get_balance(self, py, sub_acct)
    }

    /// 获取子账户账单（异步）。
    #[pyo3(signature = (ccy=None, bill_type=None, sub_acct=None, after=None, before=None, limit=None))]
    fn get_subaccount_bills<'py>(
        &self,
        py: Python<'py>,
        ccy: Option<String>,
        bill_type: Option<String>,
        sub_acct: Option<String>,
        after: Option<String>,
        before: Option<String>,
        limit: Option<u32>,
    ) -> PyResult<Bound<'py, PyAny>> {
        subaccount_impl::async_api::get_bills(
            self, py, ccy, bill_type, sub_acct, after, before, limit,
        )
    }

    /// 重置子账户 API Key（异步）。
    #[pyo3(signature = (sub_acct, api_key, label, perm=None, ip=None))]
    fn reset_subaccount_apikey<'py>(
        &self,
        py: Python<'py>,
        sub_acct: String,
        api_key: String,
        label: String,
        perm: Option<String>,
        ip: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let request = ResetSubaccountApikeyRequest {
            sub_acct,
            api_key,
            label,
            perm: perm.unwrap_or_default(),
            ip,
        };
        subaccount_impl::async_api::reset_apikey(self, py, request)
    }

    /// 获取子账户列表（异步）。
    #[pyo3(signature = (enable=None, sub_acct=None, after=None, before=None, limit=None))]
    fn get_subaccount_list<'py>(
        &self,
        py: Python<'py>,
        enable: Option<bool>,
        sub_acct: Option<String>,
        after: Option<String>,
        before: Option<String>,
        limit: Option<u32>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let params = if enable.is_some()
            || sub_acct.is_some()
            || after.is_some()
            || before.is_some()
            || limit.is_some()
        {
            Some(SubaccountListParams {
                enable,
                sub_acct,
                after,
                before,
                limit,
            })
        } else {
            None
        };
        subaccount_impl::async_api::get_list(self, py, params)
    }

    /// 子账户划转（异步）。
    fn subaccount_transfer<'py>(
        &self,
        py: Python<'py>,
        ccy: String,
        amt: String,
        froms: String,
        to: String,
        from_sub_account: String,
        to_sub_account: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let request = SubaccountTransferRequest {
            ccy,
            amt,
            froms,
            to,
            from_sub_account,
            to_sub_account,
            loan_trans: None,
            omit_pos_risk: None,
        };
        subaccount_impl::async_api::transfer(self, py, request)
    }

    /// 获取托管子账户列表（异步）。
    #[pyo3(signature = (sub_acct=None))]
    fn get_entrust_subaccount_list<'py>(
        &self,
        py: Python<'py>,
        sub_acct: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        subaccount_impl::async_api::get_entrust_list(self, py, sub_acct)
    }

    /// 设置子账户转出权限（异步）。
    fn set_permission_transfer_out<'py>(
        &self,
        py: Python<'py>,
        sub_acct: String,
        can_trans_out: bool,
    ) -> PyResult<Bound<'py, PyAny>> {
        let request = SetTransferOutRequest {
            sub_acct,
            can_trans_out,
        };
        subaccount_impl::async_api::set_permission_transfer_out(self, py, request)
    }

    /// 获取子账户资金账户余额（异步）。
    #[pyo3(signature = (sub_acct, ccy=None))]
    fn get_subaccount_funding_balance<'py>(
        &self,
        py: Python<'py>,
        sub_acct: String,
        ccy: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        subaccount_impl::async_api::get_funding_balance(self, py, sub_acct, ccy)
    }

    /// 获取代理商返佣信息（异步）。
    fn get_affiliate_rebate_info<'py>(
        &self,
        py: Python<'py>,
        api_key: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        subaccount_impl::async_api::get_affiliate_rebate_info(self, py, api_key)
    }

    /// 设置子账户 VIP 借币（异步）。
    fn set_sub_accounts_vip_loan<'py>(
        &self,
        py: Python<'py>,
        enable: bool,
        alloc_json: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        subaccount_impl::async_api::set_vip_loan(self, py, enable, alloc_json)
    }

    /// 获取子账户借币利息与限额（异步）。
    #[pyo3(signature = (sub_acct=None, ccy=None))]
    fn get_sub_account_borrow_interest_and_limit<'py>(
        &self,
        py: Python<'py>,
        sub_acct: Option<String>,
        ccy: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        subaccount_impl::async_api::get_borrow_interest_and_limit(self, py, sub_acct, ccy)
    }
}
