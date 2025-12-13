//! SubAccount 域绑定的同步/异步共享实现。

use pyo3::prelude::*;

use okx_rest::api::subaccount::{
    ResetSubaccountApikeyRequest, SetTransferOutRequest, SetVipLoanRequest, SubaccountBillsParams,
    SubaccountInterestParams, SubaccountListParams, SubaccountTransferRequest,
};
use okx_rest::SubaccountApi;

use crate::{map_values, parse_json_value, PyAsyncOkxClient, PyOkxClient, PyRuntimeError};

pub(crate) mod sync {
    use super::*;

    pub(crate) fn get_balance(client: &PyOkxClient, sub_acct: &str) -> PyResult<Vec<Py<PyAny>>> {
        let res = client.block_on_allow_threads(async {
            client.rest_client().get_subaccount_balance(sub_acct).await
        })?;
        map_values(Ok(res))
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn get_bills(
        client: &PyOkxClient,
        ccy: Option<&str>,
        bill_type: Option<&str>,
        sub_acct: Option<&str>,
        after: Option<&str>,
        before: Option<&str>,
        limit: Option<u32>,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let params = if ccy.is_some()
            || bill_type.is_some()
            || sub_acct.is_some()
            || after.is_some()
            || before.is_some()
            || limit.is_some()
        {
            Some(SubaccountBillsParams {
                ccy: ccy.map(String::from),
                bill_type: bill_type.map(String::from),
                sub_acct: sub_acct.map(String::from),
                after: after.map(String::from),
                before: before.map(String::from),
                limit,
            })
        } else {
            None
        };
        let res = client.block_on_allow_threads(async {
            client.rest_client().get_subaccount_bills(params).await
        })?;
        map_values(Ok(res))
    }

    pub(crate) fn reset_apikey(
        client: &PyOkxClient,
        request: ResetSubaccountApikeyRequest,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let res = client.block_on_allow_threads(async {
            client.rest_client().reset_subaccount_apikey(request).await
        })?;
        map_values(Ok(res))
    }

    pub(crate) fn get_list(
        client: &PyOkxClient,
        params: Option<SubaccountListParams>,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let res = client.block_on_allow_threads(async {
            client.rest_client().get_subaccount_list(params).await
        })?;
        map_values(Ok(res))
    }

    pub(crate) fn transfer(
        client: &PyOkxClient,
        request: SubaccountTransferRequest,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let res = client.block_on_allow_threads(async {
            client.rest_client().subaccount_transfer(request).await
        })?;
        map_values(Ok(res))
    }

    pub(crate) fn get_entrust_list(
        client: &PyOkxClient,
        sub_acct: Option<&str>,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let res = client.block_on_allow_threads(async {
            client
                .rest_client()
                .get_entrust_subaccount_list(sub_acct)
                .await
        })?;
        map_values(Ok(res))
    }

    pub(crate) fn set_permission_transfer_out(
        client: &PyOkxClient,
        request: SetTransferOutRequest,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let res = client.block_on_allow_threads(async {
            client
                .rest_client()
                .set_permission_transfer_out(request)
                .await
        })?;
        map_values(Ok(res))
    }

    pub(crate) fn get_funding_balance(
        client: &PyOkxClient,
        sub_acct: &str,
        ccy: Option<&str>,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let res = client.block_on_allow_threads(async {
            client
                .rest_client()
                .get_funding_balance(sub_acct, ccy)
                .await
        })?;
        map_values(Ok(res))
    }

    pub(crate) fn get_affiliate_rebate_info(
        client: &PyOkxClient,
        api_key: &str,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let res = client.block_on_allow_threads(async {
            client
                .rest_client()
                .get_affiliate_rebate_info(api_key)
                .await
        })?;
        map_values(Ok(res))
    }

    pub(crate) fn set_vip_loan(
        client: &PyOkxClient,
        enable: bool,
        alloc_json: &str,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let alloc =
            parse_json_value(Some(alloc_json), "alloc")?.unwrap_or_else(|| serde_json::json!([]));
        let request = SetVipLoanRequest { enable, alloc };
        let res = client.block_on_allow_threads(async {
            client
                .rest_client()
                .set_sub_accounts_vip_loan(request)
                .await
        })?;
        map_values(Ok(res))
    }

    pub(crate) fn get_borrow_interest_and_limit(
        client: &PyOkxClient,
        sub_acct: Option<&str>,
        ccy: Option<&str>,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let params = if sub_acct.is_some() || ccy.is_some() {
            Some(SubaccountInterestParams {
                sub_acct: sub_acct.map(String::from),
                ccy: ccy.map(String::from),
            })
        } else {
            None
        };
        let res = client.block_on_allow_threads(async {
            client
                .rest_client()
                .get_sub_account_borrow_interest_and_limit(params)
                .await
        })?;
        map_values(Ok(res))
    }
}

pub(crate) mod async_api {
    use super::*;

    pub(crate) fn get_balance<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        sub_acct: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(rest.get_subaccount_balance(&sub_acct).await)
        })
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn get_bills<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        ccy: Option<String>,
        bill_type: Option<String>,
        sub_acct: Option<String>,
        after: Option<String>,
        before: Option<String>,
        limit: Option<u32>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        let params = if ccy.is_some()
            || bill_type.is_some()
            || sub_acct.is_some()
            || after.is_some()
            || before.is_some()
            || limit.is_some()
        {
            Some(SubaccountBillsParams {
                ccy,
                bill_type,
                sub_acct,
                after,
                before,
                limit,
            })
        } else {
            None
        };
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(rest.get_subaccount_bills(params).await)
        })
    }

    pub(crate) fn reset_apikey<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        request: ResetSubaccountApikeyRequest,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(rest.reset_subaccount_apikey(request).await)
        })
    }

    pub(crate) fn get_list<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        params: Option<SubaccountListParams>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(rest.get_subaccount_list(params).await)
        })
    }

    pub(crate) fn transfer<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        request: SubaccountTransferRequest,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(rest.subaccount_transfer(request).await)
        })
    }

    pub(crate) fn get_entrust_list<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        sub_acct: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(rest.get_entrust_subaccount_list(sub_acct.as_deref()).await)
        })
    }

    pub(crate) fn set_permission_transfer_out<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        request: SetTransferOutRequest,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(rest.set_permission_transfer_out(request).await)
        })
    }

    pub(crate) fn get_funding_balance<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        sub_acct: String,
        ccy: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(rest.get_funding_balance(&sub_acct, ccy.as_deref()).await)
        })
    }

    pub(crate) fn get_affiliate_rebate_info<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        api_key: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(rest.get_affiliate_rebate_info(&api_key).await)
        })
    }

    pub(crate) fn set_vip_loan<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        enable: bool,
        alloc_json: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        let alloc = parse_json_value(Some(&alloc_json), "alloc")?
            .ok_or_else(|| PyRuntimeError::new_err("alloc 不能为空"))?;
        let request = SetVipLoanRequest { enable, alloc };
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(rest.set_sub_accounts_vip_loan(request).await)
        })
    }

    pub(crate) fn get_borrow_interest_and_limit<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        sub_acct: Option<String>,
        ccy: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        let params = if sub_acct.is_some() || ccy.is_some() {
            Some(SubaccountInterestParams { sub_acct, ccy })
        } else {
            None
        };
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(rest.get_sub_account_borrow_interest_and_limit(params).await)
        })
    }
}
