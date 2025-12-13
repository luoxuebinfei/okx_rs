//! Subaccount API #[pymethods] 块

use pyo3::prelude::*;

use okx_rest::api::subaccount::{
    ResetSubaccountApikeyRequest, SetTransferOutRequest, SubaccountListParams,
    SubaccountTransferRequest,
};

use crate::subaccount as subaccount_impl;

use super::PyOkxClient;

#[pymethods]
impl PyOkxClient {
    // ==================== Subaccount API ====================

    /// 获取子账户余额。
    fn get_subaccount_balance(&self, sub_acct: &str) -> PyResult<Vec<Py<PyAny>>> {
        subaccount_impl::sync::get_balance(self, sub_acct)
    }

    /// 获取子账户账单。
    #[pyo3(signature = (ccy=None, bill_type=None, sub_acct=None, after=None, before=None, limit=None))]
    fn get_subaccount_bills(
        &self,
        ccy: Option<&str>,
        bill_type: Option<&str>,
        sub_acct: Option<&str>,
        after: Option<&str>,
        before: Option<&str>,
        limit: Option<u32>,
    ) -> PyResult<Vec<Py<PyAny>>> {
        subaccount_impl::sync::get_bills(self, ccy, bill_type, sub_acct, after, before, limit)
    }

    /// 重置子账户 API Key。
    #[pyo3(signature = (sub_acct, api_key, label, perm=None, ip=None))]
    fn reset_subaccount_apikey(
        &self,
        sub_acct: &str,
        api_key: &str,
        label: &str,
        perm: Option<&str>,
        ip: Option<&str>,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let request = ResetSubaccountApikeyRequest {
            sub_acct: sub_acct.to_string(),
            api_key: api_key.to_string(),
            label: label.to_string(),
            perm: perm.map(String::from).unwrap_or_default(),
            ip: ip.map(String::from),
        };
        subaccount_impl::sync::reset_apikey(self, request)
    }

    /// 获取子账户列表。
    #[pyo3(signature = (enable=None, sub_acct=None, after=None, before=None, limit=None))]
    fn get_subaccount_list(
        &self,
        enable: Option<bool>,
        sub_acct: Option<&str>,
        after: Option<&str>,
        before: Option<&str>,
        limit: Option<u32>,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let params = if enable.is_some()
            || sub_acct.is_some()
            || after.is_some()
            || before.is_some()
            || limit.is_some()
        {
            Some(SubaccountListParams {
                enable,
                sub_acct: sub_acct.map(String::from),
                after: after.map(String::from),
                before: before.map(String::from),
                limit,
            })
        } else {
            None
        };
        subaccount_impl::sync::get_list(self, params)
    }

    /// 子账户划转。
    fn subaccount_transfer(
        &self,
        ccy: &str,
        amt: &str,
        froms: &str,
        to: &str,
        from_sub_account: &str,
        to_sub_account: &str,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let request = SubaccountTransferRequest {
            ccy: ccy.to_string(),
            amt: amt.to_string(),
            froms: froms.to_string(),
            to: to.to_string(),
            from_sub_account: from_sub_account.to_string(),
            to_sub_account: to_sub_account.to_string(),
            loan_trans: None,
            omit_pos_risk: None,
        };
        subaccount_impl::sync::transfer(self, request)
    }

    /// 获取托管子账户列表。
    #[pyo3(signature = (sub_acct=None))]
    fn get_entrust_subaccount_list(&self, sub_acct: Option<&str>) -> PyResult<Vec<Py<PyAny>>> {
        subaccount_impl::sync::get_entrust_list(self, sub_acct)
    }

    /// 设置子账户转出权限。
    fn set_permission_transfer_out(
        &self,
        sub_acct: &str,
        can_trans_out: bool,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let request = SetTransferOutRequest {
            sub_acct: sub_acct.to_string(),
            can_trans_out,
        };
        subaccount_impl::sync::set_permission_transfer_out(self, request)
    }

    /// 获取子账户资金账户余额。
    #[pyo3(signature = (sub_acct, ccy=None))]
    fn get_subaccount_funding_balance(
        &self,
        sub_acct: &str,
        ccy: Option<&str>,
    ) -> PyResult<Vec<Py<PyAny>>> {
        subaccount_impl::sync::get_funding_balance(self, sub_acct, ccy)
    }

    /// 获取代理商返佣信息。
    fn get_affiliate_rebate_info(&self, api_key: &str) -> PyResult<Vec<Py<PyAny>>> {
        subaccount_impl::sync::get_affiliate_rebate_info(self, api_key)
    }

    /// 设置子账户 VIP 借币。
    fn set_sub_accounts_vip_loan(
        &self,
        enable: bool,
        alloc_json: &str,
    ) -> PyResult<Vec<Py<PyAny>>> {
        subaccount_impl::sync::set_vip_loan(self, enable, alloc_json)
    }

    /// 获取子账户借币利息与限额。
    #[pyo3(signature = (sub_acct=None, ccy=None))]
    fn get_sub_account_borrow_interest_and_limit(
        &self,
        sub_acct: Option<&str>,
        ccy: Option<&str>,
    ) -> PyResult<Vec<Py<PyAny>>> {
        subaccount_impl::sync::get_borrow_interest_and_limit(self, sub_acct, ccy)
    }
}
