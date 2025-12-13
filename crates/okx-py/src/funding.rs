//! Funding 域绑定的同步/异步共享实现。

use pyo3::prelude::*;

use okx_rest::api::funding::{
    CancelWithdrawalParams, ConvertDustAssetsRequest, GetAssetValuationParams,
    GetDepositHistoryParams, GetDepositLightningParams, GetDepositWithdrawStatusParams,
    GetFundingBillsParams, GetLendingHistoryParams, GetLendingRateHistoryParams,
    GetLendingRateSummaryParams, GetSavingBalanceParams, GetTransferStateParams,
    GetWithdrawalHistoryParams, PurchaseRedemptRequest, SetLendingRateRequest,
    WithdrawalLightningRequest,
};
use okx_rest::FundingApi;

use okx_core::types::{FundsTransferRequest, WithdrawalRequest};

use crate::types::{
    PyAssetBalance, PyCurrencyInfo, PyDepositAddress, PyDepositRecord, PyFundsTransferResult,
    PyWithdrawalRecord, PyWithdrawalResult,
};
use crate::{map_values, to_py_err, values_to_py_list, PyAsyncOkxClient, PyOkxClient};

pub(crate) mod sync {
    use super::*;

    pub(crate) fn get_asset_balances(
        client: &PyOkxClient,
        ccy: Option<&str>,
    ) -> PyResult<Vec<PyAssetBalance>> {
        client.block_on_allow_threads(async {
            client
                .rest_client()
                .get_asset_balances(ccy)
                .await
                .map(|v| v.into_iter().map(PyAssetBalance::from).collect())
        })
    }

    pub(crate) fn funds_transfer(
        client: &PyOkxClient,
        request: FundsTransferRequest,
    ) -> PyResult<Option<PyFundsTransferResult>> {
        client.block_on_allow_threads(async {
            client
                .rest_client()
                .funds_transfer(request)
                .await
                .map(|mut v| v.pop().map(PyFundsTransferResult::from))
        })
    }

    pub(crate) fn withdrawal(
        client: &PyOkxClient,
        request: WithdrawalRequest,
    ) -> PyResult<Option<PyWithdrawalResult>> {
        client.block_on_allow_threads(async {
            client
                .rest_client()
                .withdrawal(request)
                .await
                .map(|mut v| v.pop().map(PyWithdrawalResult::from))
        })
    }

    pub(crate) fn get_deposit_address(
        client: &PyOkxClient,
        ccy: &str,
    ) -> PyResult<Vec<PyDepositAddress>> {
        client.block_on_allow_threads(async {
            client
                .rest_client()
                .get_deposit_address(ccy)
                .await
                .map(|v| v.into_iter().map(PyDepositAddress::from).collect())
        })
    }

    pub(crate) fn get_deposit_history(
        client: &PyOkxClient,
        params: GetDepositHistoryParams,
    ) -> PyResult<Vec<PyDepositRecord>> {
        client.block_on_allow_threads(async {
            client
                .rest_client()
                .get_deposit_history(Some(params))
                .await
                .map(|v| v.into_iter().map(PyDepositRecord::from).collect())
        })
    }

    pub(crate) fn get_withdrawal_history(
        client: &PyOkxClient,
        params: GetWithdrawalHistoryParams,
    ) -> PyResult<Vec<PyWithdrawalRecord>> {
        client.block_on_allow_threads(async {
            client
                .rest_client()
                .get_withdrawal_history(Some(params))
                .await
                .map(|v| v.into_iter().map(PyWithdrawalRecord::from).collect())
        })
    }

    pub(crate) fn get_currencies(
        client: &PyOkxClient,
        ccy: Option<&str>,
    ) -> PyResult<Vec<PyCurrencyInfo>> {
        client.block_on_allow_threads(async {
            client
                .rest_client()
                .get_currencies(ccy)
                .await
                .map(|v| v.into_iter().map(PyCurrencyInfo::from).collect())
        })
    }

    pub(crate) fn get_non_tradable_assets(
        client: &PyOkxClient,
        ccy: Option<&str>,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let res = client.block_on_allow_threads(async {
            client.rest_client().get_non_tradable_assets(ccy).await
        })?;
        map_values(Ok(res))
    }

    pub(crate) fn get_asset_valuation(
        client: &PyOkxClient,
        ccy: Option<&str>,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let params = ccy.map(|v| GetAssetValuationParams {
            ccy: Some(v.to_string()),
        });
        let res = client.block_on_allow_threads(async {
            client.rest_client().get_asset_valuation(params).await
        })?;
        map_values(Ok(res))
    }

    pub(crate) fn get_transfer_state(
        client: &PyOkxClient,
        params: GetTransferStateParams,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let res = client.block_on_allow_threads(async {
            client.rest_client().get_transfer_state(params).await
        })?;
        map_values(Ok(res))
    }

    pub(crate) fn get_funding_bills(
        client: &PyOkxClient,
        params: Option<GetFundingBillsParams>,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let res = client.block_on_allow_threads(async {
            client.rest_client().get_funding_bills(params).await
        })?;
        map_values(Ok(res))
    }

    pub(crate) fn purchase_redempt(
        client: &PyOkxClient,
        request: PurchaseRedemptRequest,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let res = client.block_on_allow_threads(async {
            client.rest_client().purchase_redempt(request).await
        })?;
        map_values(Ok(res))
    }

    pub(crate) fn get_deposit_lightning(
        client: &PyOkxClient,
        params: GetDepositLightningParams,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let res = client.block_on_allow_threads(async {
            client.rest_client().get_deposit_lightning(params).await
        })?;
        map_values(Ok(res))
    }

    pub(crate) fn withdrawal_lightning(
        client: &PyOkxClient,
        request: WithdrawalLightningRequest,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let res = client.block_on_allow_threads(async {
            client.rest_client().withdrawal_lightning(request).await
        })?;
        map_values(Ok(res))
    }

    pub(crate) fn cancel_withdrawal(
        client: &PyOkxClient,
        params: CancelWithdrawalParams,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let res = client.block_on_allow_threads(async {
            client.rest_client().cancel_withdrawal(params).await
        })?;
        map_values(Ok(res))
    }

    pub(crate) fn get_deposit_withdraw_status(
        client: &PyOkxClient,
        params: GetDepositWithdrawStatusParams,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let res = client.block_on_allow_threads(async {
            client
                .rest_client()
                .get_deposit_withdraw_status(params)
                .await
        })?;
        values_to_py_list(res)
    }

    pub(crate) fn set_lending_rate(
        client: &PyOkxClient,
        request: SetLendingRateRequest,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let res = client.block_on_allow_threads(async {
            client.rest_client().set_lending_rate(request).await
        })?;
        map_values(Ok(res))
    }

    pub(crate) fn get_lending_history(
        client: &PyOkxClient,
        params: Option<GetLendingHistoryParams>,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let res = client.block_on_allow_threads(async {
            client.rest_client().get_lending_history(params).await
        })?;
        map_values(Ok(res))
    }

    pub(crate) fn get_lending_rate_history(
        client: &PyOkxClient,
        params: Option<GetLendingRateHistoryParams>,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let res = client.block_on_allow_threads(async {
            client.rest_client().get_lending_rate_history(params).await
        })?;
        map_values(Ok(res))
    }

    pub(crate) fn get_lending_rate_summary(
        client: &PyOkxClient,
        params: Option<GetLendingRateSummaryParams>,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let res = client.block_on_allow_threads(async {
            client.rest_client().get_lending_rate_summary(params).await
        })?;
        map_values(Ok(res))
    }

    pub(crate) fn convert_dust_assets(
        client: &PyOkxClient,
        request: ConvertDustAssetsRequest,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let res = client.block_on_allow_threads(async {
            client.rest_client().convert_dust_assets(request).await
        })?;
        map_values(Ok(res))
    }

    pub(crate) fn get_saving_balance(
        client: &PyOkxClient,
        params: Option<GetSavingBalanceParams>,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let res = client.block_on_allow_threads(async {
            client.rest_client().get_saving_balance(params).await
        })?;
        map_values(Ok(res))
    }
}

pub(crate) mod async_api {
    use super::*;

    pub(crate) fn get_asset_balances<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        ccy: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            rest.get_asset_balances(ccy.as_deref())
                .await
                .map(|v| v.into_iter().map(PyAssetBalance::from).collect::<Vec<_>>())
                .map_err(to_py_err)
        })
    }

    pub(crate) fn funds_transfer<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        request: FundsTransferRequest,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            rest.funds_transfer(request)
                .await
                .map(|mut v| v.pop().map(PyFundsTransferResult::from))
                .map_err(to_py_err)
        })
    }

    pub(crate) fn withdrawal<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        request: WithdrawalRequest,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            rest.withdrawal(request)
                .await
                .map(|mut v| v.pop().map(PyWithdrawalResult::from))
                .map_err(to_py_err)
        })
    }

    pub(crate) fn get_deposit_address<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        ccy: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            rest.get_deposit_address(&ccy)
                .await
                .map(|v| {
                    v.into_iter()
                        .map(PyDepositAddress::from)
                        .collect::<Vec<_>>()
                })
                .map_err(to_py_err)
        })
    }

    pub(crate) fn get_deposit_history<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        params: GetDepositHistoryParams,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            rest.get_deposit_history(Some(params))
                .await
                .map(|v| v.into_iter().map(PyDepositRecord::from).collect::<Vec<_>>())
                .map_err(to_py_err)
        })
    }

    pub(crate) fn get_withdrawal_history<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        params: GetWithdrawalHistoryParams,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            rest.get_withdrawal_history(Some(params))
                .await
                .map(|v| {
                    v.into_iter()
                        .map(PyWithdrawalRecord::from)
                        .collect::<Vec<_>>()
                })
                .map_err(to_py_err)
        })
    }

    pub(crate) fn get_currencies<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        ccy: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            rest.get_currencies(ccy.as_deref())
                .await
                .map(|v| v.into_iter().map(PyCurrencyInfo::from).collect::<Vec<_>>())
                .map_err(to_py_err)
        })
    }

    pub(crate) fn get_non_tradable_assets<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        ccy: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(rest.get_non_tradable_assets(ccy.as_deref()).await)
        })
    }

    pub(crate) fn get_asset_valuation<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        ccy: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        let params = ccy.map(|v| GetAssetValuationParams { ccy: Some(v) });
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(rest.get_asset_valuation(params).await)
        })
    }

    pub(crate) fn get_transfer_state<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        params: GetTransferStateParams,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(rest.get_transfer_state(params).await)
        })
    }

    pub(crate) fn get_funding_bills<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        params: Option<GetFundingBillsParams>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(rest.get_funding_bills(params).await)
        })
    }

    pub(crate) fn purchase_redempt<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        request: PurchaseRedemptRequest,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(rest.purchase_redempt(request).await)
        })
    }

    pub(crate) fn get_deposit_lightning<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        params: GetDepositLightningParams,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(rest.get_deposit_lightning(params).await)
        })
    }

    pub(crate) fn withdrawal_lightning<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        request: WithdrawalLightningRequest,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(rest.withdrawal_lightning(request).await)
        })
    }

    pub(crate) fn cancel_withdrawal<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        params: CancelWithdrawalParams,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(rest.cancel_withdrawal(params).await)
        })
    }

    pub(crate) fn get_deposit_withdraw_status<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        params: GetDepositWithdrawStatusParams,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            rest.get_deposit_withdraw_status(params)
                .await
                .map_err(to_py_err)
                .and_then(values_to_py_list)
        })
    }

    pub(crate) fn set_lending_rate<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        request: SetLendingRateRequest,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(rest.set_lending_rate(request).await)
        })
    }

    pub(crate) fn get_lending_history<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        params: Option<GetLendingHistoryParams>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(rest.get_lending_history(params).await)
        })
    }

    pub(crate) fn get_lending_rate_history<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        params: Option<GetLendingRateHistoryParams>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(rest.get_lending_rate_history(params).await)
        })
    }

    pub(crate) fn get_lending_rate_summary<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        params: Option<GetLendingRateSummaryParams>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(rest.get_lending_rate_summary(params).await)
        })
    }

    pub(crate) fn convert_dust_assets<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        request: ConvertDustAssetsRequest,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(rest.convert_dust_assets(request).await)
        })
    }

    pub(crate) fn get_saving_balance<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        params: Option<GetSavingBalanceParams>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(rest.get_saving_balance(params).await)
        })
    }
}
