//! Async Python client wrapper for OKX REST API.

use std::sync::Arc;

use pyo3::prelude::*;

use okx_core::types::{
    AmendOrderRequest, CancelAlgoOrderRequest, CancelOrderRequest, FundsTransferRequest,
    PlaceAlgoOrderRequest, PlaceOrderRequest, WithdrawalRequest,
};
use okx_core::types::{
    ConvertHistoryParams, ConvertTradeRequest, EasyConvertRequest, EstimateQuoteParams,
    OneClickRepayRequest,
};
use okx_rest::api::account::{
    AdjustmentMarginRequest, BorrowRepayHistoryParams, BorrowRepayRequest,
    GetAccountPositionTiersParams, GetBillsArchiveParams, GetBillsParams, GetFeeRatesParams,
    GetGreeksParams, GetInterestAccruedParams, GetLeverageInfoParams, GetMaxAvailSizeParams,
    GetMaxLoanParams, GetMaxSizeParams, GetMaxWithdrawalParams, GetPositionsHistoryParams,
    GetPositionsParams, GetSimulatedMarginParams, GetVipInterestParams, PositionBuilderRequest,
    SetAccountLevelRequest, SetAutoLoanRequest, SetGreeksRequest, SetIsolatedModeRequest,
    SetLeverageRequest, SetRiskOffsetTypeRequest, SpotBorrowRepayHistoryParams,
    SpotManualBorrowRepayRequest,
};
use okx_rest::api::funding::{
    CancelWithdrawalParams, ConvertDustAssetsRequest, GetAssetValuationParams,
    GetDepositHistoryParams, GetDepositLightningParams, GetDepositWithdrawStatusParams,
    GetFundingBillsParams, GetLendingHistoryParams, GetLendingRateHistoryParams,
    GetLendingRateSummaryParams, GetSavingBalanceParams, GetTransferStateParams,
    GetWithdrawalHistoryParams, PurchaseRedemptRequest, SetLendingRateRequest,
    WithdrawalLightningRequest,
};
use okx_rest::api::market::{
    GetBlockTickersParams, GetCandlesParams, GetHistoryTradesParams, GetIndexCandlesParams,
    GetIndexTickersParams, GetMarkPriceCandlesParams, GetTickersParams,
};
use okx_rest::api::public::{
    GetConvertContractCoinParams, GetDeliveryExerciseHistoryParams, GetDiscountQuotaParams,
    GetEstimatedPriceParams, GetFundingRateHistoryParams, GetInstrumentsParams,
    GetInsuranceFundParams, GetMarkPriceParams, GetOpenInterestParams, GetOptSummaryParams,
    GetPositionTiersParams, GetPriceLimitParams, GetUnderlyingParams,
};
use okx_rest::api::subaccount::{
    ResetSubaccountApikeyRequest, SetTransferOutRequest, SetVipLoanRequest, SubaccountBillsParams,
    SubaccountInterestParams, SubaccountListParams, SubaccountTransferRequest,
};
use okx_rest::api::trade::{
    AmendAlgoOrderRequest, ClosePositionRequest, GetAlgoOrderDetailsParams,
    GetAlgoOrdersHistoryParams, GetAlgoOrdersParams, GetFillsHistoryParams, GetFillsParams,
    GetOrderParams, GetOrdersHistoryArchiveParams, GetOrdersHistoryParams, GetOrdersPendingParams,
};
use okx_rest::{
    AccountApi, BlockRfqApi, BrokerApi, ConvertApi, CopyTradingApi, FinanceApi, FundingApi,
    GridApi, MarketApi, OkxRestClient, PublicApi, SpreadApi, StatusApi, SubaccountApi, TradeApi,
    TradingDataApi,
};

use crate::types::*;
use crate::{
    map_values, parse_json_array, parse_json_value, to_py_err, values_to_py_list, PyRuntimeError,
};

/// Async Python wrapper for OKX REST client.
///
/// Provides asynchronous access to OKX REST API using Python's asyncio.
///
/// Example:
///     ```python
///     import asyncio
///     from okx_py import AsyncOkxClient, Config, Credentials
///
///     async def main():
///         creds = Credentials("api_key", "secret_key", "passphrase")
///         config = Config(creds, simulated=True)
///         client = AsyncOkxClient(config)
///
///         # Async API calls
///         balance = await client.get_balance()
///         ticker = await client.get_ticker("BTC-USDT")
///
///     asyncio.run(main())
///     ```
#[pyclass(name = "AsyncOkxClient")]
pub struct PyAsyncOkxClient {
    client: Arc<OkxRestClient>,
}

#[pymethods]
impl PyAsyncOkxClient {
    /// Create a new async OKX client.
    ///
    /// Args:
    ///     config: Client configuration
    #[new]
    fn new(config: PyConfig) -> Self {
        let client = Arc::new(OkxRestClient::new(config.inner));
        Self { client }
    }

    // ==================== Account API ====================

    /// Get account balance (async).
    ///
    /// Args:
    ///     ccy: Optional currency filter (e.g., "BTC" or "BTC,ETH")
    ///
    /// Returns:
    ///     List of Balance objects
    #[pyo3(signature = (ccy=None))]
    fn get_balance<'py>(
        &self,
        py: Python<'py>,
        ccy: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            client
                .get_balance(ccy.as_deref())
                .await
                .map(|v| v.into_iter().map(PyBalance::from).collect::<Vec<_>>())
                .map_err(to_py_err)
        })
    }

    /// Get positions (async).
    ///
    /// Args:
    ///     inst_type: Optional instrument type filter
    ///     inst_id: Optional instrument ID filter
    ///
    /// Returns:
    ///     List of Position objects
    #[pyo3(signature = (inst_type=None, inst_id=None))]
    fn get_positions<'py>(
        &self,
        py: Python<'py>,
        inst_type: Option<String>,
        inst_id: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        let params = if inst_type.is_some() || inst_id.is_some() {
            Some(GetPositionsParams {
                inst_type,
                inst_id,
                pos_id: None,
            })
        } else {
            None
        };

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            client
                .get_positions(params)
                .await
                .map(|v| v.into_iter().map(PyPosition::from).collect::<Vec<_>>())
                .map_err(to_py_err)
        })
    }

    /// 获取账户配置（异步）。
    fn get_account_config<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            client
                .get_account_config()
                .await
                .map(|v| v.into_iter().map(PyAccountConfig::from).collect::<Vec<_>>())
                .map_err(to_py_err)
        })
    }

    /// 设置杠杆（异步）。
    #[pyo3(signature = (lever, mgn_mode, inst_id=None, ccy=None, pos_side=None))]
    fn set_leverage<'py>(
        &self,
        py: Python<'py>,
        lever: String,
        mgn_mode: String,
        inst_id: Option<String>,
        ccy: Option<String>,
        pos_side: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        let request = SetLeverageRequest {
            inst_id,
            ccy,
            lever,
            mgn_mode,
            pos_side,
        };

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            client
                .set_leverage(request)
                .await
                .map(|mut v| v.pop().map(PySetLeverageResult::from))
                .map_err(to_py_err)
        })
    }

    /// 查询杠杆信息（异步）。
    #[pyo3(signature = (mgn_mode, ccy=None, inst_id=None))]
    fn get_leverage_info<'py>(
        &self,
        py: Python<'py>,
        mgn_mode: String,
        ccy: Option<String>,
        inst_id: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        let params = GetLeverageInfoParams {
            mgn_mode,
            ccy,
            inst_id,
        };

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            client
                .get_leverage_info(params)
                .await
                .map(|v| v.into_iter().map(PyLeverageInfo::from).collect::<Vec<_>>())
                .map_err(to_py_err)
        })
    }

    /// 查询最大可下单张数（异步）。
    #[pyo3(signature = (inst_id, td_mode, ccy=None, px=None, leverage=None))]
    fn get_max_size<'py>(
        &self,
        py: Python<'py>,
        inst_id: String,
        td_mode: String,
        ccy: Option<String>,
        px: Option<String>,
        leverage: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        let params = GetMaxSizeParams {
            inst_id,
            td_mode,
            ccy,
            px,
            leverage,
        };

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            client
                .get_max_size(params)
                .await
                .map(|mut v| v.pop().map(PyMaxSize::from))
                .map_err(to_py_err)
        })
    }

    /// 查询最大可用张数（异步）。
    #[pyo3(signature = (inst_id, td_mode, ccy=None, reduce_only=None, quick_mgn_type=None))]
    fn get_max_avail_size<'py>(
        &self,
        py: Python<'py>,
        inst_id: String,
        td_mode: String,
        ccy: Option<String>,
        reduce_only: Option<bool>,
        quick_mgn_type: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        let params = GetMaxAvailSizeParams {
            inst_id,
            td_mode,
            ccy,
            reduce_only,
            quick_mgn_type,
        };

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            client
                .get_max_avail_size(params)
                .await
                .map(|mut v| v.pop().map(PyMaxAvailSize::from))
                .map_err(to_py_err)
        })
    }

    /// 查询最大可借（异步）。
    #[pyo3(signature = (inst_id, mgn_mode, mgn_ccy=None))]
    fn get_max_loan<'py>(
        &self,
        py: Python<'py>,
        inst_id: String,
        mgn_mode: String,
        mgn_ccy: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        let params = GetMaxLoanParams {
            inst_id,
            mgn_mode,
            mgn_ccy,
        };

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            client
                .get_max_loan(params)
                .await
                .map_err(to_py_err)
                .and_then(values_to_py_list)
        })
    }

    /// 查询借贷利率（异步）。
    #[pyo3(signature = (ccy=None))]
    fn get_interest_rate<'py>(
        &self,
        py: Python<'py>,
        ccy: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            client
                .get_interest_rate(ccy.as_deref())
                .await
                .map_err(to_py_err)
                .and_then(values_to_py_list)
        })
    }

    /// 查询手续费率（异步）。
    #[pyo3(signature = (inst_type, inst_id=None, uly=None, inst_family=None))]
    fn get_fee_rates<'py>(
        &self,
        py: Python<'py>,
        inst_type: String,
        inst_id: Option<String>,
        uly: Option<String>,
        inst_family: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        let params = GetFeeRatesParams {
            inst_type,
            inst_id,
            uly,
            inst_family,
        };

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            client
                .get_fee_rates(params)
                .await
                .map(|v| v.into_iter().map(PyFeeRates::from).collect::<Vec<_>>())
                .map_err(to_py_err)
        })
    }

    /// 设置持仓模式（异步）。
    fn set_position_mode<'py>(
        &self,
        py: Python<'py>,
        pos_mode: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        let mode = pos_mode;

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            client
                .set_position_mode(&mode)
                .await
                .map(|mut v| v.pop().map(PySetPositionModeResult::from))
                .map_err(to_py_err)
        })
    }

    /// 获取账户风险（异步）。
    fn get_account_position_risk<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            client
                .get_account_position_risk()
                .await
                .map(|v| {
                    v.into_iter()
                        .map(PyAccountPositionRisk::from)
                        .collect::<Vec<_>>()
                })
                .map_err(to_py_err)
        })
    }

    /// 获取历史持仓（异步）。
    #[pyo3(
        signature = (inst_type=None, inst_id=None, mgn_mode=None, type_=None, pos_id=None, after=None, before=None, limit=None)
    )]
    fn get_positions_history<'py>(
        &self,
        py: Python<'py>,
        inst_type: Option<String>,
        inst_id: Option<String>,
        mgn_mode: Option<String>,
        type_: Option<String>,
        pos_id: Option<String>,
        after: Option<String>,
        before: Option<String>,
        limit: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        let params = if inst_type.is_some()
            || inst_id.is_some()
            || mgn_mode.is_some()
            || type_.is_some()
            || pos_id.is_some()
            || after.is_some()
            || before.is_some()
            || limit.is_some()
        {
            Some(GetPositionsHistoryParams {
                inst_type,
                inst_id,
                mgn_mode,
                r#type: type_,
                pos_id,
                after,
                before,
                limit,
            })
        } else {
            None
        };

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            client
                .get_positions_history(params)
                .await
                .map_err(to_py_err)
                .and_then(values_to_py_list)
        })
    }

    /// 查询最大可提额度（异步）。
    #[pyo3(signature = (ccy=None))]
    fn get_max_withdrawal<'py>(
        &self,
        py: Python<'py>,
        ccy: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        let params = GetMaxWithdrawalParams { ccy };
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            client
                .get_max_withdrawal(params.ccy.as_deref())
                .await
                .map_err(to_py_err)
                .and_then(values_to_py_list)
        })
    }

    /// 账户账单（近 7 天，异步）。
    #[pyo3(
        signature = (inst_type=None, ccy=None, mgn_mode=None, ct_type=None, type_=None, sub_type=None, after=None, before=None, limit=None)
    )]
    fn get_account_bills<'py>(
        &self,
        py: Python<'py>,
        inst_type: Option<String>,
        ccy: Option<String>,
        mgn_mode: Option<String>,
        ct_type: Option<String>,
        type_: Option<String>,
        sub_type: Option<String>,
        after: Option<String>,
        before: Option<String>,
        limit: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        let params = if inst_type.is_some()
            || ccy.is_some()
            || mgn_mode.is_some()
            || ct_type.is_some()
            || type_.is_some()
            || sub_type.is_some()
            || after.is_some()
            || before.is_some()
            || limit.is_some()
        {
            Some(GetBillsParams {
                inst_type,
                ccy,
                mgn_mode,
                ct_type,
                r#type: type_,
                sub_type,
                after,
                before,
                limit,
            })
        } else {
            None
        };

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            client
                .get_account_bills(params)
                .await
                .map_err(to_py_err)
                .and_then(values_to_py_list)
        })
    }

    /// 账户账单归档（异步）。
    #[pyo3(
        signature = (inst_type=None, ccy=None, mgn_mode=None, ct_type=None, type_=None, sub_type=None, after=None, before=None, limit=None, begin=None, end=None)
    )]
    fn get_account_bills_archive<'py>(
        &self,
        py: Python<'py>,
        inst_type: Option<String>,
        ccy: Option<String>,
        mgn_mode: Option<String>,
        ct_type: Option<String>,
        type_: Option<String>,
        sub_type: Option<String>,
        after: Option<String>,
        before: Option<String>,
        limit: Option<String>,
        begin: Option<String>,
        end: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        let params = GetBillsArchiveParams {
            inst_type,
            ccy,
            mgn_mode,
            ct_type,
            r#type: type_,
            sub_type,
            after,
            before,
            limit,
            begin,
            end,
        };

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            client
                .get_account_bills_archive(params)
                .await
                .map_err(to_py_err)
                .and_then(values_to_py_list)
        })
    }

    /// 设置 Greeks（异步）。
    fn set_greeks<'py>(&self, py: Python<'py>, greeks_type: String) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        let request = SetGreeksRequest { greeks_type };

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            client
                .set_greeks(request)
                .await
                .map_err(to_py_err)
                .and_then(values_to_py_list)
        })
    }

    /// 设置逐仓模式（异步）。
    #[pyo3(signature = (iso_mode, type_))]
    fn set_isolated_mode<'py>(
        &self,
        py: Python<'py>,
        iso_mode: String,
        type_: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        let request = SetIsolatedModeRequest {
            iso_mode,
            r#type: type_,
        };

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            client
                .set_isolated_mode(request)
                .await
                .map_err(to_py_err)
                .and_then(values_to_py_list)
        })
    }

    /// 调整持仓保证金（异步）。
    #[pyo3(signature = (inst_id, pos_side, type_, amt, loan_trans=None))]
    fn adjustment_margin<'py>(
        &self,
        py: Python<'py>,
        inst_id: String,
        pos_side: String,
        type_: String,
        amt: String,
        loan_trans: Option<bool>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        let request = AdjustmentMarginRequest {
            inst_id,
            pos_side,
            r#type: type_,
            amt,
            loan_trans,
        };

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            client
                .adjustment_margin(request)
                .await
                .map_err(to_py_err)
                .and_then(values_to_py_list)
        })
    }

    /// 设置风险对冲类型（异步）。
    fn set_risk_offset_type<'py>(
        &self,
        py: Python<'py>,
        type_: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        let request = SetRiskOffsetTypeRequest { r#type: type_ };

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            client
                .set_risk_offset_type(request)
                .await
                .map_err(to_py_err)
                .and_then(values_to_py_list)
        })
    }

    /// 设置自动借币（异步）。
    #[pyo3(signature = (auto_loan=None))]
    fn set_auto_loan<'py>(
        &self,
        py: Python<'py>,
        auto_loan: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        let request = SetAutoLoanRequest { auto_loan };

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            client
                .set_auto_loan(request)
                .await
                .map_err(to_py_err)
                .and_then(values_to_py_list)
        })
    }

    /// 设置账户等级（异步）。
    fn set_account_level<'py>(
        &self,
        py: Python<'py>,
        acct_lv: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        let request = SetAccountLevelRequest { acct_lv };

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            client
                .set_account_level(request)
                .await
                .map_err(to_py_err)
                .and_then(values_to_py_list)
        })
    }

    /// 借币/还币（异步）。
    #[pyo3(signature = (ccy=None, side=None, amt=None, ord_id=None))]
    fn borrow_repay<'py>(
        &self,
        py: Python<'py>,
        ccy: Option<String>,
        side: Option<String>,
        amt: Option<String>,
        ord_id: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        let request = BorrowRepayRequest {
            ccy,
            side,
            amt,
            ord_id,
        };

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            client
                .borrow_repay(request)
                .await
                .map_err(to_py_err)
                .and_then(values_to_py_list)
        })
    }

    /// 借币/还币历史（异步）。
    #[pyo3(signature = (ccy=None, after=None, before=None, limit=None))]
    fn get_borrow_repay_history<'py>(
        &self,
        py: Python<'py>,
        ccy: Option<String>,
        after: Option<String>,
        before: Option<String>,
        limit: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        let params = if ccy.is_some() || after.is_some() || before.is_some() || limit.is_some() {
            Some(BorrowRepayHistoryParams {
                ccy,
                after,
                before,
                limit,
            })
        } else {
            None
        };

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            client
                .get_borrow_repay_history(params)
                .await
                .map_err(to_py_err)
                .and_then(values_to_py_list)
        })
    }

    /// 现货手动借币/还币（异步）。
    #[pyo3(signature = (ccy=None, side=None, amt=None))]
    fn spot_manual_borrow_repay<'py>(
        &self,
        py: Python<'py>,
        ccy: Option<String>,
        side: Option<String>,
        amt: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        let request = SpotManualBorrowRepayRequest { ccy, side, amt };

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            client
                .spot_manual_borrow_repay(request)
                .await
                .map_err(to_py_err)
                .and_then(values_to_py_list)
        })
    }

    /// 现货借币/还币历史（异步）。
    #[pyo3(signature = (ccy=None, type_=None, after=None, before=None, limit=None))]
    fn spot_borrow_repay_history<'py>(
        &self,
        py: Python<'py>,
        ccy: Option<String>,
        type_: Option<String>,
        after: Option<String>,
        before: Option<String>,
        limit: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        let params = if ccy.is_some()
            || type_.is_some()
            || after.is_some()
            || before.is_some()
            || limit.is_some()
        {
            Some(SpotBorrowRepayHistoryParams {
                ccy,
                r#type: type_,
                after,
                before,
                limit,
            })
        } else {
            None
        };

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            client
                .spot_borrow_repay_history(params)
                .await
                .map_err(to_py_err)
                .and_then(values_to_py_list)
        })
    }

    /// 已生息数据（异步）。
    #[pyo3(signature = (inst_id=None, ccy=None, mgn_mode=None, after=None, before=None, limit=None))]
    fn get_interest_accrued<'py>(
        &self,
        py: Python<'py>,
        inst_id: Option<String>,
        ccy: Option<String>,
        mgn_mode: Option<String>,
        after: Option<String>,
        before: Option<String>,
        limit: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        let params = GetInterestAccruedParams {
            inst_id,
            ccy,
            mgn_mode,
            after,
            before,
            limit,
        };

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            client
                .get_interest_accrued(params)
                .await
                .map_err(to_py_err)
                .and_then(values_to_py_list)
        })
    }

    /// VIP 已生息（异步）。
    #[pyo3(signature = (ccy=None, ord_id=None, after=None, before=None, limit=None))]
    fn get_vip_interest_accrued<'py>(
        &self,
        py: Python<'py>,
        ccy: Option<String>,
        ord_id: Option<String>,
        after: Option<String>,
        before: Option<String>,
        limit: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        let params = GetVipInterestParams {
            ccy,
            ord_id,
            after,
            before,
            limit,
        };

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            client
                .get_vip_interest_accrued(params)
                .await
                .map_err(to_py_err)
                .and_then(values_to_py_list)
        })
    }

    /// VIP 已扣息（异步）。
    #[pyo3(signature = (ccy=None, ord_id=None, after=None, before=None, limit=None))]
    fn get_vip_interest_deducted<'py>(
        &self,
        py: Python<'py>,
        ccy: Option<String>,
        ord_id: Option<String>,
        after: Option<String>,
        before: Option<String>,
        limit: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        let params = GetVipInterestParams {
            ccy,
            ord_id,
            after,
            before,
            limit,
        };

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            client
                .get_vip_interest_deducted(params)
                .await
                .map_err(to_py_err)
                .and_then(values_to_py_list)
        })
    }

    /// 模拟保证金（异步）。
    #[pyo3(signature = (inst_type, incl_real_pos=None, spot_offset_type=None, sim_pos_json=None))]
    fn get_simulated_margin<'py>(
        &self,
        py: Python<'py>,
        inst_type: String,
        incl_real_pos: Option<bool>,
        spot_offset_type: Option<String>,
        sim_pos_json: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        let sim_pos = parse_json_array(sim_pos_json.as_deref(), "sim_pos")?;
        let params = GetSimulatedMarginParams {
            inst_type,
            incl_real_pos,
            spot_offset_type,
            sim_pos,
        };

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            client
                .get_simulated_margin(params)
                .await
                .map_err(to_py_err)
                .and_then(values_to_py_list)
        })
    }

    /// 账户杠杆分层（异步）。
    fn get_account_position_tiers<'py>(
        &self,
        py: Python<'py>,
        inst_type: String,
        uly: Option<String>,
        inst_family: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        let params = GetAccountPositionTiersParams {
            inst_type,
            uly,
            inst_family,
        };

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            client
                .get_account_position_tiers(params)
                .await
                .map_err(to_py_err)
                .and_then(values_to_py_list)
        })
    }

    /// 查询 Greeks（异步）。
    #[pyo3(signature = (ccy=None))]
    fn get_greeks<'py>(&self, py: Python<'py>, ccy: Option<String>) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        let params = GetGreeksParams { ccy };

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            client
                .get_greeks(params)
                .await
                .map_err(to_py_err)
                .and_then(values_to_py_list)
        })
    }

    /// Position Builder（异步）。
    #[pyo3(
        signature = (acct_lv=None, incl_real_pos_and_eq=None, lever=None, greeks_type=None, sim_pos_json=None, sim_asset_json=None)
    )]
    fn position_builder<'py>(
        &self,
        py: Python<'py>,
        acct_lv: Option<String>,
        incl_real_pos_and_eq: Option<bool>,
        lever: Option<String>,
        greeks_type: Option<String>,
        sim_pos_json: Option<String>,
        sim_asset_json: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        let sim_pos = parse_json_value(sim_pos_json.as_deref(), "sim_pos")?;
        let sim_asset = parse_json_value(sim_asset_json.as_deref(), "sim_asset")?;
        let request = PositionBuilderRequest {
            acct_lv,
            incl_real_pos_and_eq,
            lever,
            greeks_type,
            sim_pos,
            sim_asset,
        };

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            client
                .position_builder(request)
                .await
                .map_err(to_py_err)
                .and_then(values_to_py_list)
        })
    }

    // ==================== Trade API ====================

    /// Place an order (async).
    ///
    /// Args:
    ///     inst_id: Instrument ID (e.g., "BTC-USDT")
    ///     td_mode: Trade mode (cash, cross, isolated)
    ///     side: Order side (buy, sell)
    ///     ord_type: Order type (market, limit, post_only, fok, ioc)
    ///     sz: Order size
    ///     px: Price (required for limit orders)
    ///     cl_ord_id: Client order ID (optional)
    ///
    /// Returns:
    ///     Order ID
    #[pyo3(signature = (inst_id, td_mode, side, ord_type, sz, px=None, cl_ord_id=None))]
    fn place_order<'py>(
        &self,
        py: Python<'py>,
        inst_id: String,
        td_mode: String,
        side: String,
        ord_type: String,
        sz: String,
        px: Option<String>,
        cl_ord_id: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        let request = PlaceOrderRequest {
            inst_id,
            td_mode,
            side,
            ord_type,
            sz,
            px,
            cl_ord_id,
            ccy: None,
            tag: None,
            pos_side: None,
            reduce_only: None,
            tgt_ccy: None,
            tp_trigger_px: None,
            tp_ord_px: None,
            sl_trigger_px: None,
            sl_ord_px: None,
            tp_trigger_px_type: None,
            sl_trigger_px_type: None,
            quick_mgn_type: None,
            stp_id: None,
            stp_mode: None,
            attach_algo_ords: None,
        };

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            client
                .place_order(request)
                .await
                .map(|v| v.first().map(|r| r.ord_id.clone()).unwrap_or_default())
                .map_err(to_py_err)
        })
    }

    /// Cancel an order (async).
    ///
    /// Args:
    ///     inst_id: Instrument ID
    ///     ord_id: Order ID (either ord_id or cl_ord_id required)
    ///     cl_ord_id: Client order ID
    ///
    /// Returns:
    ///     Cancelled order ID
    #[pyo3(signature = (inst_id, ord_id=None, cl_ord_id=None))]
    fn cancel_order<'py>(
        &self,
        py: Python<'py>,
        inst_id: String,
        ord_id: Option<String>,
        cl_ord_id: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        let request = CancelOrderRequest {
            inst_id,
            ord_id,
            cl_ord_id,
        };

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            client
                .cancel_order(request)
                .await
                .map(|v| v.first().map(|r| r.ord_id.clone()).unwrap_or_default())
                .map_err(to_py_err)
        })
    }

    /// Get order details (async).
    ///
    /// Args:
    ///     inst_id: Instrument ID
    ///     ord_id: Order ID (either ord_id or cl_ord_id required)
    ///     cl_ord_id: Client order ID
    ///
    /// Returns:
    ///     Order object or None
    #[pyo3(signature = (inst_id, ord_id=None, cl_ord_id=None))]
    fn get_order<'py>(
        &self,
        py: Python<'py>,
        inst_id: String,
        ord_id: Option<String>,
        cl_ord_id: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        let params = GetOrderParams {
            inst_id,
            ord_id,
            cl_ord_id,
        };

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            client
                .get_order(params)
                .await
                .map(|v| v.into_iter().next().map(PyOrder::from))
                .map_err(to_py_err)
        })
    }

    /// Get pending orders (async).
    ///
    /// Args:
    ///     inst_type: Optional instrument type filter
    ///     inst_id: Optional instrument ID filter
    ///
    /// Returns:
    ///     List of Order objects
    #[pyo3(signature = (inst_type=None, inst_id=None))]
    fn get_orders_pending<'py>(
        &self,
        py: Python<'py>,
        inst_type: Option<String>,
        inst_id: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        let params = if inst_type.is_some() || inst_id.is_some() {
            Some(GetOrdersPendingParams {
                inst_type,
                inst_id,
                ..Default::default()
            })
        } else {
            None
        };

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            client
                .get_orders_pending(params)
                .await
                .map(|v| v.into_iter().map(PyOrder::from).collect::<Vec<_>>())
                .map_err(to_py_err)
        })
    }

    /// 批量下单（异步）。
    #[pyo3(signature = (orders))]
    fn place_batch_orders<'py>(
        &self,
        py: Python<'py>,
        orders: Vec<(
            String,
            String,
            String,
            String,
            String,
            Option<String>,
            Option<String>,
        )>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        let requests: Vec<PlaceOrderRequest> = orders
            .into_iter()
            .map(
                |(inst_id, td_mode, side, ord_type, sz, px, cl_ord_id)| PlaceOrderRequest {
                    inst_id,
                    td_mode,
                    side,
                    ord_type,
                    sz,
                    px,
                    cl_ord_id,
                    ccy: None,
                    tag: None,
                    pos_side: None,
                    reduce_only: None,
                    tgt_ccy: None,
                    tp_trigger_px: None,
                    tp_ord_px: None,
                    sl_trigger_px: None,
                    sl_ord_px: None,
                    tp_trigger_px_type: None,
                    sl_trigger_px_type: None,
                    quick_mgn_type: None,
                    stp_id: None,
                    stp_mode: None,
                    attach_algo_ords: None,
                },
            )
            .collect();

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            client
                .place_batch_orders(requests)
                .await
                .map(|v| {
                    v.into_iter()
                        .map(PyPlaceOrderResult::from)
                        .collect::<Vec<_>>()
                })
                .map_err(to_py_err)
        })
    }

    /// 批量撤单（异步）。
    #[pyo3(signature = (orders))]
    fn cancel_batch_orders<'py>(
        &self,
        py: Python<'py>,
        orders: Vec<(String, Option<String>, Option<String>)>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        let requests: Vec<CancelOrderRequest> = orders
            .into_iter()
            .map(|(inst_id, ord_id, cl_ord_id)| CancelOrderRequest {
                inst_id,
                ord_id,
                cl_ord_id,
            })
            .collect();

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            client
                .cancel_batch_orders(requests)
                .await
                .map(|v| {
                    v.into_iter()
                        .map(PyCancelOrderResult::from)
                        .collect::<Vec<_>>()
                })
                .map_err(to_py_err)
        })
    }

    /// 改单（异步）。
    #[pyo3(
        signature = (inst_id, ord_id=None, cl_ord_id=None, req_id=None, new_sz=None, new_px=None, new_tp_trigger_px=None, new_tp_ord_px=None, new_sl_trigger_px=None, new_sl_ord_px=None, new_tp_trigger_px_type=None, new_sl_trigger_px_type=None)
    )]
    fn amend_order<'py>(
        &self,
        py: Python<'py>,
        inst_id: String,
        ord_id: Option<String>,
        cl_ord_id: Option<String>,
        req_id: Option<String>,
        new_sz: Option<String>,
        new_px: Option<String>,
        new_tp_trigger_px: Option<String>,
        new_tp_ord_px: Option<String>,
        new_sl_trigger_px: Option<String>,
        new_sl_ord_px: Option<String>,
        new_tp_trigger_px_type: Option<String>,
        new_sl_trigger_px_type: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        let request = AmendOrderRequest {
            inst_id,
            ord_id,
            cl_ord_id,
            req_id,
            new_sz,
            new_px,
            new_tp_trigger_px,
            new_tp_ord_px,
            new_sl_trigger_px,
            new_sl_ord_px,
            new_tp_trigger_px_type,
            new_sl_trigger_px_type,
        };

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            client
                .amend_order(request)
                .await
                .map(|mut v| v.pop().map(PyAmendOrderResult::from))
                .map_err(to_py_err)
        })
    }

    /// 批量改单（异步）。
    #[pyo3(signature = (orders))]
    fn amend_batch_orders<'py>(
        &self,
        py: Python<'py>,
        orders: Vec<(
            String,
            Option<String>,
            Option<String>,
            Option<String>,
            Option<String>,
            Option<String>,
            Option<String>,
            Option<String>,
            Option<String>,
            Option<String>,
            Option<String>,
            Option<String>,
        )>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let requests: Vec<AmendOrderRequest> = orders
                .into_iter()
                .map(
                    |(
                        inst_id,
                        ord_id,
                        cl_ord_id,
                        req_id,
                        new_sz,
                        new_px,
                        new_tp_trigger_px,
                        new_tp_ord_px,
                        new_sl_trigger_px,
                        new_sl_ord_px,
                        new_tp_trigger_px_type,
                        new_sl_trigger_px_type,
                    )| AmendOrderRequest {
                        inst_id,
                        ord_id,
                        cl_ord_id,
                        req_id,
                        new_sz,
                        new_px,
                        new_tp_trigger_px,
                        new_tp_ord_px,
                        new_sl_trigger_px,
                        new_sl_ord_px,
                        new_tp_trigger_px_type,
                        new_sl_trigger_px_type,
                    },
                )
                .collect();

            client
                .amend_batch_orders(requests)
                .await
                .map(|v| {
                    v.into_iter()
                        .map(PyAmendOrderResult::from)
                        .collect::<Vec<_>>()
                })
                .map_err(to_py_err)
        })
    }

    /// 查询历史订单（异步，近 7 天）。
    #[pyo3(
        signature = (inst_type, uly=None, inst_family=None, inst_id=None, ord_type=None, state=None, category=None, after=None, before=None, begin=None, end=None, limit=None)
    )]
    fn get_orders_history<'py>(
        &self,
        py: Python<'py>,
        inst_type: String,
        uly: Option<String>,
        inst_family: Option<String>,
        inst_id: Option<String>,
        ord_type: Option<String>,
        state: Option<String>,
        category: Option<String>,
        after: Option<String>,
        before: Option<String>,
        begin: Option<String>,
        end: Option<String>,
        limit: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        let params = GetOrdersHistoryParams {
            inst_type,
            uly,
            inst_family,
            inst_id,
            ord_type,
            state,
            category,
            after,
            before,
            begin,
            end,
            limit,
        };

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            client
                .get_orders_history(params)
                .await
                .map(|v| v.into_iter().map(PyOrder::from).collect::<Vec<_>>())
                .map_err(to_py_err)
        })
    }

    /// 查询历史订单归档（异步，近 3 个月）。
    #[pyo3(
        signature = (inst_type, uly=None, inst_family=None, inst_id=None, ord_type=None, state=None, category=None, after=None, before=None, begin=None, end=None, limit=None)
    )]
    fn get_orders_history_archive<'py>(
        &self,
        py: Python<'py>,
        inst_type: String,
        uly: Option<String>,
        inst_family: Option<String>,
        inst_id: Option<String>,
        ord_type: Option<String>,
        state: Option<String>,
        category: Option<String>,
        after: Option<String>,
        before: Option<String>,
        begin: Option<String>,
        end: Option<String>,
        limit: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        let params = GetOrdersHistoryArchiveParams {
            inst_type,
            uly,
            inst_family,
            inst_id,
            ord_type,
            state,
            category,
            after,
            before,
            begin,
            end,
            limit,
        };

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            client
                .get_orders_history_archive(params)
                .await
                .map(|v| v.into_iter().map(PyOrder::from).collect::<Vec<_>>())
                .map_err(to_py_err)
        })
    }

    /// 查询成交明细（异步）。
    #[pyo3(
        signature = (inst_type=None, uly=None, inst_family=None, inst_id=None, ord_id=None, after=None, before=None, begin=None, end=None, limit=None)
    )]
    fn get_fills<'py>(
        &self,
        py: Python<'py>,
        inst_type: Option<String>,
        uly: Option<String>,
        inst_family: Option<String>,
        inst_id: Option<String>,
        ord_id: Option<String>,
        after: Option<String>,
        before: Option<String>,
        begin: Option<String>,
        end: Option<String>,
        limit: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        let params = GetFillsParams {
            inst_type,
            uly,
            inst_family,
            inst_id,
            ord_id,
            after,
            before,
            begin,
            end,
            limit,
        };

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            client
                .get_fills(Some(params))
                .await
                .map(|v| v.into_iter().map(PyFill::from).collect::<Vec<_>>())
                .map_err(to_py_err)
        })
    }

    /// 查询历史成交（异步，近 3 个月）。
    #[pyo3(
        signature = (inst_type, uly=None, inst_family=None, inst_id=None, ord_id=None, after=None, before=None, limit=None)
    )]
    fn get_fills_history<'py>(
        &self,
        py: Python<'py>,
        inst_type: String,
        uly: Option<String>,
        inst_family: Option<String>,
        inst_id: Option<String>,
        ord_id: Option<String>,
        after: Option<String>,
        before: Option<String>,
        limit: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        let params = GetFillsHistoryParams {
            inst_type,
            uly,
            inst_family,
            inst_id,
            ord_id,
            after,
            before,
            limit,
        };

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            client
                .get_fills_history(params)
                .await
                .map(|v| v.into_iter().map(PyFill::from).collect::<Vec<_>>())
                .map_err(to_py_err)
        })
    }

    /// 下算法单（异步）。
    #[pyo3(
        signature = (inst_id, td_mode, side, ord_type, sz, ccy=None, pos_side=None, reduce_only=None, tgt_ccy=None, algo_cl_ord_id=None, trigger_px=None, order_px=None, trigger_px_type=None, tp_trigger_px=None, tp_ord_px=None, tp_trigger_px_type=None, sl_trigger_px=None, sl_ord_px=None, sl_trigger_px_type=None, callback_ratio=None, callback_spread=None, active_px=None)
    )]
    fn place_algo_order<'py>(
        &self,
        py: Python<'py>,
        inst_id: String,
        td_mode: String,
        side: String,
        ord_type: String,
        sz: String,
        ccy: Option<String>,
        pos_side: Option<String>,
        reduce_only: Option<bool>,
        tgt_ccy: Option<String>,
        algo_cl_ord_id: Option<String>,
        trigger_px: Option<String>,
        order_px: Option<String>,
        trigger_px_type: Option<String>,
        tp_trigger_px: Option<String>,
        tp_ord_px: Option<String>,
        tp_trigger_px_type: Option<String>,
        sl_trigger_px: Option<String>,
        sl_ord_px: Option<String>,
        sl_trigger_px_type: Option<String>,
        callback_ratio: Option<String>,
        callback_spread: Option<String>,
        active_px: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        let request = PlaceAlgoOrderRequest {
            inst_id,
            td_mode,
            side,
            ord_type,
            sz,
            ccy,
            pos_side,
            reduce_only,
            tgt_ccy,
            algo_cl_ord_id,
            trigger_px,
            order_px,
            trigger_px_type,
            tp_trigger_px,
            tp_ord_px,
            tp_trigger_px_type,
            sl_trigger_px,
            sl_ord_px,
            sl_trigger_px_type,
            callback_ratio,
            callback_spread,
            active_px,
        };

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            client
                .place_algo_order(request)
                .await
                .map(|v| {
                    v.into_iter()
                        .map(PyPlaceAlgoOrderResult::from)
                        .collect::<Vec<_>>()
                })
                .map_err(to_py_err)
        })
    }

    /// 批量撤算法单（异步）。
    #[pyo3(signature = (requests))]
    fn cancel_algo_orders<'py>(
        &self,
        py: Python<'py>,
        requests: Vec<(String, String)>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        let reqs: Vec<CancelAlgoOrderRequest> = requests
            .into_iter()
            .map(|(inst_id, algo_id)| CancelAlgoOrderRequest { inst_id, algo_id })
            .collect();

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            client
                .cancel_algo_orders(reqs)
                .await
                .map(|v| {
                    v.into_iter()
                        .map(PyCancelAlgoOrderResult::from)
                        .collect::<Vec<_>>()
                })
                .map_err(to_py_err)
        })
    }

    /// 修改算法单（异步）。
    #[pyo3(
        signature = (inst_id=None, algo_id=None, algo_cl_ord_id=None, cxl_on_fail=None, req_id=None, new_sz=None, new_tp_trigger_px=None, new_tp_ord_px=None, new_sl_trigger_px=None, new_sl_ord_px=None, new_tp_trigger_px_type=None, new_sl_trigger_px_type=None)
    )]
    fn amend_algo_order<'py>(
        &self,
        py: Python<'py>,
        inst_id: Option<String>,
        algo_id: Option<String>,
        algo_cl_ord_id: Option<String>,
        cxl_on_fail: Option<String>,
        req_id: Option<String>,
        new_sz: Option<String>,
        new_tp_trigger_px: Option<String>,
        new_tp_ord_px: Option<String>,
        new_sl_trigger_px: Option<String>,
        new_sl_ord_px: Option<String>,
        new_tp_trigger_px_type: Option<String>,
        new_sl_trigger_px_type: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        let request = AmendAlgoOrderRequest {
            inst_id,
            algo_id,
            algo_cl_ord_id,
            cxl_on_fail,
            req_id,
            new_sz,
            new_tp_trigger_px,
            new_tp_ord_px,
            new_sl_trigger_px,
            new_sl_ord_px,
            new_tp_trigger_px_type,
            new_sl_trigger_px_type,
        };

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            client
                .amend_algo_order(request)
                .await
                .map_err(to_py_err)
                .and_then(values_to_py_list)
        })
    }

    /// 查询在途算法单（异步）。
    #[pyo3(signature = (ord_type, algo_id=None, inst_type=None, inst_id=None, after=None, before=None, limit=None))]
    fn get_algo_orders_pending<'py>(
        &self,
        py: Python<'py>,
        ord_type: String,
        algo_id: Option<String>,
        inst_type: Option<String>,
        inst_id: Option<String>,
        after: Option<String>,
        before: Option<String>,
        limit: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        let params = GetAlgoOrdersParams {
            ord_type,
            algo_id,
            inst_type,
            inst_id,
            after,
            before,
            limit,
        };

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            client
                .get_algo_orders_pending(params)
                .await
                .map(|v| v.into_iter().map(PyAlgoOrder::from).collect::<Vec<_>>())
                .map_err(to_py_err)
        })
    }

    /// 查询历史算法单（异步）。
    #[pyo3(signature = (ord_type, state=None, algo_id=None, inst_type=None, inst_id=None, after=None, before=None, limit=None))]
    fn get_algo_orders_history<'py>(
        &self,
        py: Python<'py>,
        ord_type: String,
        state: Option<String>,
        algo_id: Option<String>,
        inst_type: Option<String>,
        inst_id: Option<String>,
        after: Option<String>,
        before: Option<String>,
        limit: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        let params = GetAlgoOrdersHistoryParams {
            ord_type,
            state,
            algo_id,
            inst_type,
            inst_id,
            after,
            before,
            limit,
        };

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            client
                .get_algo_orders_history(params)
                .await
                .map(|v| v.into_iter().map(PyAlgoOrder::from).collect::<Vec<_>>())
                .map_err(to_py_err)
        })
    }

    /// 获取算法单详情（异步）。
    #[pyo3(signature = (algo_id=None, algo_cl_ord_id=None))]
    fn get_algo_order_details<'py>(
        &self,
        py: Python<'py>,
        algo_id: Option<String>,
        algo_cl_ord_id: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        let params = GetAlgoOrderDetailsParams {
            algo_id,
            algo_cl_ord_id,
        };

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            client
                .get_algo_order_details(params)
                .await
                .map(|v| v.into_iter().map(PyAlgoOrder::from).collect::<Vec<_>>())
                .map_err(to_py_err)
        })
    }

    /// 一键平仓（异步）。
    #[pyo3(signature = (inst_id, mgn_mode, pos_side=None, ccy=None, auto_cancel=None, cl_ord_id=None, tag=None))]
    fn close_position<'py>(
        &self,
        py: Python<'py>,
        inst_id: String,
        mgn_mode: String,
        pos_side: Option<String>,
        ccy: Option<String>,
        auto_cancel: Option<bool>,
        cl_ord_id: Option<String>,
        tag: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        let request = ClosePositionRequest {
            inst_id,
            mgn_mode,
            pos_side,
            ccy,
            auto_cancel,
            cl_ord_id,
            tag,
        };

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            client
                .close_position(request)
                .await
                .map(|mut v| v.pop().map(PyClosePositionResult::from))
                .map_err(to_py_err)
        })
    }

    // ==================== Funding API ====================

    /// 查询资金账户余额（异步）。
    #[pyo3(signature = (ccy=None))]
    fn get_asset_balances<'py>(
        &self,
        py: Python<'py>,
        ccy: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        let currency = ccy;

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            client
                .get_asset_balances(currency.as_deref())
                .await
                .map(|v| v.into_iter().map(PyAssetBalance::from).collect::<Vec<_>>())
                .map_err(to_py_err)
        })
    }

    /// 资金划转（异步）。
    #[pyo3(
        signature = (ccy, amt, from_account, to_account, transfer_type=None, sub_acct=None, inst_id=None, to_inst_id=None, loan_trans=None)
    )]
    fn funds_transfer<'py>(
        &self,
        py: Python<'py>,
        ccy: String,
        amt: String,
        from_account: String,
        to_account: String,
        transfer_type: Option<String>,
        sub_acct: Option<String>,
        inst_id: Option<String>,
        to_inst_id: Option<String>,
        loan_trans: Option<bool>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        let request = FundsTransferRequest {
            ccy,
            amt,
            from: from_account,
            to: to_account,
            r#type: transfer_type,
            sub_acct,
            inst_id,
            to_inst_id,
            loan_trans,
        };

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            client
                .funds_transfer(request)
                .await
                .map(|mut v| v.pop().map(PyFundsTransferResult::from))
                .map_err(to_py_err)
        })
    }

    /// 提现（异步）。
    #[pyo3(signature = (ccy, amt, dest, to_addr, chain=None, area_code=None, client_id=None, fee=None))]
    fn withdrawal<'py>(
        &self,
        py: Python<'py>,
        ccy: String,
        amt: String,
        dest: String,
        to_addr: String,
        chain: Option<String>,
        area_code: Option<String>,
        client_id: Option<String>,
        fee: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        let request = WithdrawalRequest {
            ccy,
            amt,
            dest,
            to_addr,
            chain,
            area_code,
            client_id,
            fee,
        };

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            client
                .withdrawal(request)
                .await
                .map(|mut v| v.pop().map(PyWithdrawalResult::from))
                .map_err(to_py_err)
        })
    }

    /// 查询充值地址（异步）。
    fn get_deposit_address<'py>(
        &self,
        py: Python<'py>,
        ccy: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            client
                .get_deposit_address(&ccy)
                .await
                .map(|v| {
                    v.into_iter()
                        .map(PyDepositAddress::from)
                        .collect::<Vec<_>>()
                })
                .map_err(to_py_err)
        })
    }

    /// 查询充值记录（异步）。
    #[pyo3(signature = (ccy=None, dep_id=None, tx_id=None, record_type=None, state=None, after=None, before=None, limit=None))]
    fn get_deposit_history<'py>(
        &self,
        py: Python<'py>,
        ccy: Option<String>,
        dep_id: Option<String>,
        tx_id: Option<String>,
        record_type: Option<String>,
        state: Option<String>,
        after: Option<String>,
        before: Option<String>,
        limit: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        let params = GetDepositHistoryParams {
            ccy,
            dep_id,
            tx_id,
            r#type: record_type,
            state,
            after,
            before,
            limit,
        };

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            client
                .get_deposit_history(Some(params))
                .await
                .map(|v| v.into_iter().map(PyDepositRecord::from).collect::<Vec<_>>())
                .map_err(to_py_err)
        })
    }

    /// 查询提现记录（异步）。
    #[pyo3(signature = (ccy=None, wd_id=None, client_id=None, tx_id=None, record_type=None, state=None, after=None, before=None, limit=None))]
    fn get_withdrawal_history<'py>(
        &self,
        py: Python<'py>,
        ccy: Option<String>,
        wd_id: Option<String>,
        client_id: Option<String>,
        tx_id: Option<String>,
        record_type: Option<String>,
        state: Option<String>,
        after: Option<String>,
        before: Option<String>,
        limit: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        let params = GetWithdrawalHistoryParams {
            ccy,
            wd_id,
            client_id,
            tx_id,
            r#type: record_type,
            state,
            after,
            before,
            limit,
        };

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            client
                .get_withdrawal_history(Some(params))
                .await
                .map(|v| {
                    v.into_iter()
                        .map(PyWithdrawalRecord::from)
                        .collect::<Vec<_>>()
                })
                .map_err(to_py_err)
        })
    }

    /// 查询币种信息（异步）。
    #[pyo3(signature = (ccy=None))]
    fn get_currencies<'py>(
        &self,
        py: Python<'py>,
        ccy: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        let currency = ccy;

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            client
                .get_currencies(currency.as_deref())
                .await
                .map(|v| v.into_iter().map(PyCurrencyInfo::from).collect::<Vec<_>>())
                .map_err(to_py_err)
        })
    }

    /// 查询非交易资产（异步）。
    #[pyo3(signature = (ccy=None))]
    fn get_non_tradable_assets<'py>(
        &self,
        py: Python<'py>,
        ccy: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            client
                .get_non_tradable_assets(ccy.as_deref())
                .await
                .map_err(to_py_err)
                .and_then(values_to_py_list)
        })
    }

    /// 查询资产估值（异步）。
    #[pyo3(signature = (ccy=None))]
    fn get_asset_valuation<'py>(
        &self,
        py: Python<'py>,
        ccy: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        let params = if ccy.is_some() {
            Some(GetAssetValuationParams { ccy })
        } else {
            None
        };

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            client
                .get_asset_valuation(params)
                .await
                .map_err(to_py_err)
                .and_then(values_to_py_list)
        })
    }

    /// 查询划转状态（异步）。
    fn get_transfer_state<'py>(
        &self,
        py: Python<'py>,
        trans_id: String,
        type_: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        let params = GetTransferStateParams {
            trans_id,
            r#type: type_,
        };

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            client
                .get_transfer_state(params)
                .await
                .map_err(to_py_err)
                .and_then(values_to_py_list)
        })
    }

    /// 资金流水（资金账户，异步）。
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
        let client = Arc::clone(&self.client);
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

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            client
                .get_funding_bills(params)
                .await
                .map_err(to_py_err)
                .and_then(values_to_py_list)
        })
    }

    /// 申购/赎回余币宝（异步）。
    fn purchase_redempt<'py>(
        &self,
        py: Python<'py>,
        ccy: String,
        amt: String,
        side: String,
        rate: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        let request = PurchaseRedemptRequest {
            ccy,
            amt,
            side,
            rate,
        };

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            client
                .purchase_redempt(request)
                .await
                .map_err(to_py_err)
                .and_then(values_to_py_list)
        })
    }

    /// 闪电网络充值（异步）。
    fn get_deposit_lightning<'py>(
        &self,
        py: Python<'py>,
        ccy: String,
        amt: String,
        to: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        let params = GetDepositLightningParams { ccy, amt, to };

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            client
                .get_deposit_lightning(params)
                .await
                .map_err(to_py_err)
                .and_then(values_to_py_list)
        })
    }

    /// 闪电网络提现（异步）。
    fn withdrawal_lightning<'py>(
        &self,
        py: Python<'py>,
        ccy: String,
        invoice: String,
        memo: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        let request = WithdrawalLightningRequest { ccy, invoice, memo };

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            client
                .withdrawal_lightning(request)
                .await
                .map_err(to_py_err)
                .and_then(values_to_py_list)
        })
    }

    /// 取消提现（异步）。
    #[pyo3(signature = (wd_id=None))]
    fn cancel_withdrawal<'py>(
        &self,
        py: Python<'py>,
        wd_id: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        let params = CancelWithdrawalParams { wd_id };

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            client
                .cancel_withdrawal(params)
                .await
                .map_err(to_py_err)
                .and_then(values_to_py_list)
        })
    }

    /// 充值/提现状态（异步）。
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
        let client = Arc::clone(&self.client);
        let params = GetDepositWithdrawStatusParams {
            wd_id,
            tx_id,
            ccy,
            to,
            chain,
        };

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            client
                .get_deposit_withdraw_status(params)
                .await
                .map_err(to_py_err)
                .and_then(values_to_py_list)
        })
    }

    /// 设置借贷利率（异步）。
    fn set_lending_rate<'py>(
        &self,
        py: Python<'py>,
        ccy: String,
        rate: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        let request = SetLendingRateRequest { ccy, rate };

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            client
                .set_lending_rate(request)
                .await
                .map_err(to_py_err)
                .and_then(values_to_py_list)
        })
    }

    /// 借贷历史（异步）。
    #[pyo3(signature = (ccy=None, before=None, after=None, limit=None))]
    fn get_lending_history<'py>(
        &self,
        py: Python<'py>,
        ccy: Option<String>,
        before: Option<String>,
        after: Option<String>,
        limit: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        let params = if ccy.is_some() || before.is_some() || after.is_some() || limit.is_some() {
            Some(GetLendingHistoryParams {
                ccy,
                before,
                after,
                limit,
            })
        } else {
            None
        };

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            client
                .get_lending_history(params)
                .await
                .map_err(to_py_err)
                .and_then(values_to_py_list)
        })
    }

    /// 借贷利率历史（异步）。
    #[pyo3(signature = (ccy=None, after=None, before=None, limit=None))]
    fn get_lending_rate_history<'py>(
        &self,
        py: Python<'py>,
        ccy: Option<String>,
        after: Option<String>,
        before: Option<String>,
        limit: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        let params = if ccy.is_some() || after.is_some() || before.is_some() || limit.is_some() {
            Some(GetLendingRateHistoryParams {
                ccy,
                after,
                before,
                limit,
            })
        } else {
            None
        };

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            client
                .get_lending_rate_history(params)
                .await
                .map_err(to_py_err)
                .and_then(values_to_py_list)
        })
    }

    /// 借贷利率汇总（异步）。
    #[pyo3(signature = (ccy=None))]
    fn get_lending_rate_summary<'py>(
        &self,
        py: Python<'py>,
        ccy: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        let params = if ccy.is_some() {
            Some(GetLendingRateSummaryParams { ccy })
        } else {
            None
        };

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            client
                .get_lending_rate_summary(params)
                .await
                .map_err(to_py_err)
                .and_then(values_to_py_list)
        })
    }

    /// 小额资产兑换（异步）。
    #[pyo3(signature = (ccy=None))]
    fn convert_dust_assets<'py>(
        &self,
        py: Python<'py>,
        ccy: Option<Vec<String>>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        let request = ConvertDustAssetsRequest { ccy };

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            client
                .convert_dust_assets(request)
                .await
                .map_err(to_py_err)
                .and_then(values_to_py_list)
        })
    }

    /// 余币宝余额（异步）。
    #[pyo3(signature = (ccy=None))]
    fn get_saving_balance<'py>(
        &self,
        py: Python<'py>,
        ccy: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        let params = if ccy.is_some() {
            Some(GetSavingBalanceParams { ccy })
        } else {
            None
        };

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            client
                .get_saving_balance(params)
                .await
                .map_err(to_py_err)
                .and_then(values_to_py_list)
        })
    }

    // ==================== SubAccount API (async) ====================

    #[pyo3(signature = (sub_acct))]
    fn get_subaccount_balance<'py>(
        &self,
        py: Python<'py>,
        sub_acct: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(client.get_subaccount_balance(&sub_acct).await)
        })
    }

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
        let client = Arc::clone(&self.client);
        let params = SubaccountBillsParams {
            ccy,
            bill_type,
            sub_acct,
            after,
            before,
            limit,
        };
        let params = if params.ccy.is_some()
            || params.bill_type.is_some()
            || params.sub_acct.is_some()
            || params.after.is_some()
            || params.before.is_some()
            || params.limit.is_some()
        {
            Some(params)
        } else {
            None
        };
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(client.get_subaccount_bills(params).await)
        })
    }

    #[pyo3(signature = (sub_acct, api_key, label, perm, ip=None))]
    fn reset_subaccount_apikey<'py>(
        &self,
        py: Python<'py>,
        sub_acct: String,
        api_key: String,
        label: String,
        perm: String,
        ip: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        let request = ResetSubaccountApikeyRequest {
            sub_acct,
            api_key,
            label,
            perm,
            ip,
        };
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(client.reset_subaccount_apikey(request).await)
        })
    }

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
        let client = Arc::clone(&self.client);
        let params = SubaccountListParams {
            enable,
            sub_acct,
            after,
            before,
            limit,
        };
        let params = if params.enable.is_some()
            || params.sub_acct.is_some()
            || params.after.is_some()
            || params.before.is_some()
            || params.limit.is_some()
        {
            Some(params)
        } else {
            None
        };
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(client.get_subaccount_list(params).await)
        })
    }

    #[pyo3(signature = (ccy, amt, froms, to, from_sub_account, to_sub_account, loan_trans=None, omit_pos_risk=None))]
    fn subaccount_transfer<'py>(
        &self,
        py: Python<'py>,
        ccy: String,
        amt: String,
        froms: String,
        to: String,
        from_sub_account: String,
        to_sub_account: String,
        loan_trans: Option<bool>,
        omit_pos_risk: Option<bool>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        let request = SubaccountTransferRequest {
            ccy,
            amt,
            froms,
            to,
            from_sub_account,
            to_sub_account,
            loan_trans,
            omit_pos_risk,
        };
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(client.subaccount_transfer(request).await)
        })
    }

    #[pyo3(signature = (sub_acct=None))]
    fn get_entrust_subaccount_list<'py>(
        &self,
        py: Python<'py>,
        sub_acct: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(
                client
                    .get_entrust_subaccount_list(sub_acct.as_deref())
                    .await,
            )
        })
    }

    #[pyo3(signature = (sub_acct, can_trans_out))]
    fn set_permission_transfer_out<'py>(
        &self,
        py: Python<'py>,
        sub_acct: String,
        can_trans_out: bool,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        let request = SetTransferOutRequest {
            sub_acct,
            can_trans_out,
        };
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(client.set_permission_transfer_out(request).await)
        })
    }

    #[pyo3(signature = (sub_acct, ccy=None))]
    fn get_subaccount_funding_balance<'py>(
        &self,
        py: Python<'py>,
        sub_acct: String,
        ccy: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(client.get_funding_balance(&sub_acct, ccy.as_deref()).await)
        })
    }

    #[pyo3(signature = (api_key))]
    fn get_affiliate_rebate_info<'py>(
        &self,
        py: Python<'py>,
        api_key: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(client.get_affiliate_rebate_info(&api_key).await)
        })
    }

    #[pyo3(signature = (enable, alloc_json))]
    fn set_sub_accounts_vip_loan<'py>(
        &self,
        py: Python<'py>,
        enable: bool,
        alloc_json: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        let alloc =
            parse_json_value(Some(&alloc_json), "alloc")?.unwrap_or_else(|| serde_json::json!([]));
        let request = SetVipLoanRequest { enable, alloc };
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(client.set_sub_accounts_vip_loan(request).await)
        })
    }

    #[pyo3(signature = (sub_acct=None, ccy=None))]
    fn get_sub_account_borrow_interest_and_limit<'py>(
        &self,
        py: Python<'py>,
        sub_acct: Option<String>,
        ccy: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        let params = SubaccountInterestParams { sub_acct, ccy };
        let params = if params.sub_acct.is_some() || params.ccy.is_some() {
            Some(params)
        } else {
            None
        };
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(
                client
                    .get_sub_account_borrow_interest_and_limit(params)
                    .await,
            )
        })
    }

    // ==================== Convert / Easy Convert / One-Click Repay (async) ====================

    fn get_convert_currencies<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(client.get_convert_currencies().await)
        })
    }

    #[pyo3(signature = (from_ccy, to_ccy))]
    fn get_convert_currency_pair<'py>(
        &self,
        py: Python<'py>,
        from_ccy: String,
        to_ccy: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(client.get_convert_currency_pair(&from_ccy, &to_ccy).await)
        })
    }

    #[pyo3(signature = (base_ccy, quote_ccy, side, rfq_sz, rfq_sz_ccy, cl_q_req_id=None, tag=None))]
    fn estimate_convert_quote<'py>(
        &self,
        py: Python<'py>,
        base_ccy: String,
        quote_ccy: String,
        side: String,
        rfq_sz: String,
        rfq_sz_ccy: String,
        cl_q_req_id: Option<String>,
        tag: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        let params = EstimateQuoteParams {
            base_ccy,
            quote_ccy,
            side,
            rfq_sz,
            rfq_sz_ccy,
            cl_q_req_id,
            tag,
        };
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(client.estimate_convert_quote(params).await)
        })
    }

    #[pyo3(signature = (quote_id, base_ccy, quote_ccy, side, sz, sz_ccy, cl_t_req_id=None, tag=None))]
    fn convert_trade<'py>(
        &self,
        py: Python<'py>,
        quote_id: String,
        base_ccy: String,
        quote_ccy: String,
        side: String,
        sz: String,
        sz_ccy: String,
        cl_t_req_id: Option<String>,
        tag: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        let request = ConvertTradeRequest {
            quote_id,
            base_ccy,
            quote_ccy,
            side,
            sz,
            sz_ccy,
            cl_t_req_id,
            tag,
        };
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(client.convert_trade(request).await)
        })
    }

    #[pyo3(signature = (after=None, before=None, limit=None, tag=None))]
    fn get_convert_history<'py>(
        &self,
        py: Python<'py>,
        after: Option<String>,
        before: Option<String>,
        limit: Option<u32>,
        tag: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        let params = ConvertHistoryParams {
            after,
            before,
            limit,
            tag,
        };
        let params = if params.after.is_some()
            || params.before.is_some()
            || params.limit.is_some()
            || params.tag.is_some()
        {
            Some(params)
        } else {
            None
        };
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(client.get_convert_history(params).await)
        })
    }

    fn get_easy_convert_currency_list<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(client.get_easy_convert_currency_list().await)
        })
    }

    #[pyo3(signature = (from_ccy, to_ccy))]
    fn easy_convert<'py>(
        &self,
        py: Python<'py>,
        from_ccy: Vec<String>,
        to_ccy: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        let request = EasyConvertRequest { from_ccy, to_ccy };
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(client.easy_convert(request).await)
        })
    }

    #[pyo3(signature = (after=None, before=None, limit=None))]
    fn get_easy_convert_history<'py>(
        &self,
        py: Python<'py>,
        after: Option<String>,
        before: Option<String>,
        limit: Option<u32>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(
                client
                    .get_easy_convert_history(after.as_deref(), before.as_deref(), limit)
                    .await,
            )
        })
    }

    fn get_one_click_repay_currency_list<'py>(
        &self,
        py: Python<'py>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(client.get_one_click_repay_currency_list().await)
        })
    }

    #[pyo3(signature = (debt_ccy, repay_ccy))]
    fn one_click_repay<'py>(
        &self,
        py: Python<'py>,
        debt_ccy: Vec<String>,
        repay_ccy: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        let request = OneClickRepayRequest {
            debt_ccy,
            repay_ccy,
        };
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(client.one_click_repay(request).await)
        })
    }

    #[pyo3(signature = (after=None, before=None, limit=None))]
    fn get_one_click_repay_history<'py>(
        &self,
        py: Python<'py>,
        after: Option<String>,
        before: Option<String>,
        limit: Option<u32>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(
                client
                    .get_one_click_repay_history(after.as_deref(), before.as_deref(), limit)
                    .await,
            )
        })
    }

    // ==================== Trade 扩展（mass cancel / cancel-all-after / order-precheck） ====================

    #[pyo3(signature = (payload_json))]
    fn mass_cancel<'py>(
        &self,
        py: Python<'py>,
        payload_json: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        let payload = parse_json_value(Some(&payload_json), "payload")?
            .ok_or_else(|| PyRuntimeError::new_err("payload 不能为空"))?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(client.mass_cancel(payload).await)
        })
    }

    #[pyo3(signature = (payload_json))]
    fn cancel_all_after<'py>(
        &self,
        py: Python<'py>,
        payload_json: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        let payload = parse_json_value(Some(&payload_json), "payload")?
            .ok_or_else(|| PyRuntimeError::new_err("payload 不能为空"))?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(client.cancel_all_after(payload).await)
        })
    }

    #[pyo3(signature = (payload_json))]
    fn order_precheck<'py>(
        &self,
        py: Python<'py>,
        payload_json: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        let payload = parse_json_value(Some(&payload_json), "payload")?
            .ok_or_else(|| PyRuntimeError::new_err("payload 不能为空"))?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(client.order_precheck(payload).await)
        })
    }

    // ==================== Grid / Recurring Buy (async) ====================

    #[pyo3(signature = (payload_json))]
    fn grid_order_algo<'py>(
        &self,
        py: Python<'py>,
        payload_json: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        let payload = parse_json_value(Some(&payload_json), "payload")?
            .ok_or_else(|| PyRuntimeError::new_err("payload 不能为空"))?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(client.grid_order_algo(payload).await)
        })
    }

    #[pyo3(signature = (params_json=None))]
    fn grid_orders_algo_pending<'py>(
        &self,
        py: Python<'py>,
        params_json: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        let params = parse_json_value(params_json.as_deref(), "params")?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(client.grid_orders_algo_pending(params).await)
        })
    }

    #[pyo3(signature = (params_json=None))]
    fn grid_orders_algo_history<'py>(
        &self,
        py: Python<'py>,
        params_json: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        let params = parse_json_value(params_json.as_deref(), "params")?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(client.grid_orders_algo_history(params).await)
        })
    }

    #[pyo3(signature = (payload_json))]
    fn place_recurring_buy_order<'py>(
        &self,
        py: Python<'py>,
        payload_json: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        let payload = parse_json_value(Some(&payload_json), "payload")?
            .ok_or_else(|| PyRuntimeError::new_err("payload 不能为空"))?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(client.place_recurring_buy_order(payload).await)
        })
    }

    #[pyo3(signature = (params_json=None))]
    fn get_recurring_buy_order_list<'py>(
        &self,
        py: Python<'py>,
        params_json: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        let params = parse_json_value(params_json.as_deref(), "params")?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(client.get_recurring_buy_order_list(params).await)
        })
    }

    // ==================== Copy Trading (async) ====================

    #[pyo3(signature = (params_json=None))]
    fn get_existing_lead_positions<'py>(
        &self,
        py: Python<'py>,
        params_json: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        let params = parse_json_value(params_json.as_deref(), "params")?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(client.get_existing_lead_positions(params).await)
        })
    }

    #[pyo3(signature = (payload_json))]
    fn place_lead_stop_order<'py>(
        &self,
        py: Python<'py>,
        payload_json: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        let payload = parse_json_value(Some(&payload_json), "payload")?
            .ok_or_else(|| PyRuntimeError::new_err("payload 不能为空"))?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(client.place_lead_stop_order(payload).await)
        })
    }

    #[pyo3(signature = (payload_json))]
    fn close_lead_position<'py>(
        &self,
        py: Python<'py>,
        payload_json: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        let payload = parse_json_value(Some(&payload_json), "payload")?
            .ok_or_else(|| PyRuntimeError::new_err("payload 不能为空"))?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(client.close_lead_position(payload).await)
        })
    }

    fn get_total_profit_sharing<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(client.get_total_profit_sharing().await)
        })
    }

    // ==================== Broker (async) ====================

    #[pyo3(signature = (params_json))]
    fn fd_rebate_per_orders<'py>(
        &self,
        py: Python<'py>,
        params_json: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        let params = parse_json_value(Some(&params_json), "params")?
            .ok_or_else(|| PyRuntimeError::new_err("params 不能为空"))?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(client.fd_rebate_per_orders(params).await)
        })
    }

    #[pyo3(signature = (params_json))]
    fn fd_get_rebate_per_orders<'py>(
        &self,
        py: Python<'py>,
        params_json: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        let params = parse_json_value(Some(&params_json), "params")?
            .ok_or_else(|| PyRuntimeError::new_err("params 不能为空"))?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(client.fd_get_rebate_per_orders(params).await)
        })
    }

    // ==================== Finance / Savings / Simple Earn (async) ====================

    #[pyo3(signature = (params_json=None))]
    fn defi_get_offers<'py>(
        &self,
        py: Python<'py>,
        params_json: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        let params = parse_json_value(params_json.as_deref(), "params")?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(client.defi_get_offers(params).await)
        })
    }

    #[pyo3(signature = (payload_json))]
    fn defi_purchase<'py>(
        &self,
        py: Python<'py>,
        payload_json: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        let payload = parse_json_value(Some(&payload_json), "payload")?
            .ok_or_else(|| PyRuntimeError::new_err("payload 不能为空"))?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(client.defi_purchase(payload).await)
        })
    }

    #[pyo3(signature = (params_json=None))]
    fn saving_balance<'py>(
        &self,
        py: Python<'py>,
        params_json: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        let params = parse_json_value(params_json.as_deref(), "params")?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(client.saving_balance(params).await)
        })
    }

    #[pyo3(signature = (payload_json))]
    fn saving_purchase_redemption<'py>(
        &self,
        py: Python<'py>,
        payload_json: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        let payload = parse_json_value(Some(&payload_json), "payload")?
            .ok_or_else(|| PyRuntimeError::new_err("payload 不能为空"))?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(client.saving_purchase_redemption(payload).await)
        })
    }

    #[pyo3(signature = (params_json=None))]
    fn simple_earn_get_offers<'py>(
        &self,
        py: Python<'py>,
        params_json: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        let params = parse_json_value(params_json.as_deref(), "params")?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(client.simple_earn_get_offers(params).await)
        })
    }

    // ==================== Block / RFQ (async) ====================

    fn get_rfq_counterparties<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(client.get_counterparties().await)
        })
    }

    #[pyo3(signature = (payload_json))]
    fn create_rfq<'py>(
        &self,
        py: Python<'py>,
        payload_json: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        let payload = parse_json_value(Some(&payload_json), "payload")?
            .ok_or_else(|| PyRuntimeError::new_err("payload 不能为空"))?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(client.create_rfq(payload).await)
        })
    }

    #[pyo3(signature = (payload_json))]
    fn cancel_rfq<'py>(
        &self,
        py: Python<'py>,
        payload_json: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        let payload = parse_json_value(Some(&payload_json), "payload")?
            .ok_or_else(|| PyRuntimeError::new_err("payload 不能为空"))?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(client.cancel_rfq(payload).await)
        })
    }

    #[pyo3(signature = (payload_json))]
    fn cancel_batch_rfqs<'py>(
        &self,
        py: Python<'py>,
        payload_json: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        let payload = parse_json_value(Some(&payload_json), "payload")?
            .ok_or_else(|| PyRuntimeError::new_err("payload 不能为空"))?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(client.cancel_batch_rfqs(payload).await)
        })
    }

    #[pyo3(signature = (payload_json))]
    fn cancel_all_rfqs<'py>(
        &self,
        py: Python<'py>,
        payload_json: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        let payload = parse_json_value(Some(&payload_json), "payload")?
            .ok_or_else(|| PyRuntimeError::new_err("payload 不能为空"))?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(client.cancel_all_rfqs(payload).await)
        })
    }

    #[pyo3(signature = (payload_json))]
    fn create_quote<'py>(
        &self,
        py: Python<'py>,
        payload_json: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        let payload = parse_json_value(Some(&payload_json), "payload")?
            .ok_or_else(|| PyRuntimeError::new_err("payload 不能为空"))?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(client.create_quote(payload).await)
        })
    }

    #[pyo3(signature = (payload_json))]
    fn cancel_quote<'py>(
        &self,
        py: Python<'py>,
        payload_json: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        let payload = parse_json_value(Some(&payload_json), "payload")?
            .ok_or_else(|| PyRuntimeError::new_err("payload 不能为空"))?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(client.cancel_quote(payload).await)
        })
    }

    #[pyo3(signature = (payload_json))]
    fn cancel_batch_quotes<'py>(
        &self,
        py: Python<'py>,
        payload_json: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        let payload = parse_json_value(Some(&payload_json), "payload")?
            .ok_or_else(|| PyRuntimeError::new_err("payload 不能为空"))?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(client.cancel_batch_quotes(payload).await)
        })
    }

    #[pyo3(signature = (payload_json))]
    fn cancel_all_quotes<'py>(
        &self,
        py: Python<'py>,
        payload_json: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        let payload = parse_json_value(Some(&payload_json), "payload")?
            .ok_or_else(|| PyRuntimeError::new_err("payload 不能为空"))?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(client.cancel_all_quotes(payload).await)
        })
    }

    #[pyo3(signature = (payload_json))]
    fn execute_quote<'py>(
        &self,
        py: Python<'py>,
        payload_json: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        let payload = parse_json_value(Some(&payload_json), "payload")?
            .ok_or_else(|| PyRuntimeError::new_err("payload 不能为空"))?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(client.execute_quote(payload).await)
        })
    }

    #[pyo3(signature = (params_json=None))]
    fn get_rfqs<'py>(
        &self,
        py: Python<'py>,
        params_json: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        let params = parse_json_value(params_json.as_deref(), "params")?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(client.get_rfqs(params).await)
        })
    }

    #[pyo3(signature = (params_json=None))]
    fn get_rfq_quotes<'py>(
        &self,
        py: Python<'py>,
        params_json: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        let params = parse_json_value(params_json.as_deref(), "params")?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(client.get_quotes(params).await)
        })
    }

    #[pyo3(signature = (params_json=None))]
    fn get_rfq_trades<'py>(
        &self,
        py: Python<'py>,
        params_json: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        let params = parse_json_value(params_json.as_deref(), "params")?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(okx_rest::BlockRfqApi::get_trades(client.as_ref(), params).await)
        })
    }

    #[pyo3(signature = (params_json=None))]
    fn get_rfq_public_trades<'py>(
        &self,
        py: Python<'py>,
        params_json: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        let params = parse_json_value(params_json.as_deref(), "params")?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(okx_rest::BlockRfqApi::get_public_trades(client.as_ref(), params).await)
        })
    }

    #[pyo3(signature = (payload_json))]
    fn reset_rfq_mmp<'py>(
        &self,
        py: Python<'py>,
        payload_json: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        let payload = parse_json_value(Some(&payload_json), "payload")?
            .ok_or_else(|| PyRuntimeError::new_err("payload 不能为空"))?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(client.reset_mmp(payload).await)
        })
    }

    #[pyo3(signature = (payload_json))]
    fn set_rfq_mmp_config<'py>(
        &self,
        py: Python<'py>,
        payload_json: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        let payload = parse_json_value(Some(&payload_json), "payload")?
            .ok_or_else(|| PyRuntimeError::new_err("payload 不能为空"))?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(client.set_mmp_config(payload).await)
        })
    }

    fn get_rfq_mmp_config<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(client.get_mmp_config().await)
        })
    }

    #[pyo3(signature = (payload_json))]
    fn set_rfq_marker_instrument<'py>(
        &self,
        py: Python<'py>,
        payload_json: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        let payload = parse_json_value(Some(&payload_json), "payload")?
            .ok_or_else(|| PyRuntimeError::new_err("payload 不能为空"))?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(client.set_marker_instrument(payload).await)
        })
    }

    #[pyo3(signature = (params_json=None))]
    fn get_rfq_quote_products<'py>(
        &self,
        py: Python<'py>,
        params_json: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        let params = parse_json_value(params_json.as_deref(), "params")?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(client.get_quote_products(params).await)
        })
    }

    // ==================== Spread Trading (async) ====================

    #[pyo3(signature = (payload_json))]
    fn spread_place_order<'py>(
        &self,
        py: Python<'py>,
        payload_json: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        let payload = parse_json_value(Some(&payload_json), "payload")?
            .ok_or_else(|| PyRuntimeError::new_err("payload 不能为空"))?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(client.spread_place_order(payload).await)
        })
    }

    #[pyo3(signature = (payload_json))]
    fn spread_cancel_order<'py>(
        &self,
        py: Python<'py>,
        payload_json: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        let payload = parse_json_value(Some(&payload_json), "payload")?
            .ok_or_else(|| PyRuntimeError::new_err("payload 不能为空"))?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(client.spread_cancel_order(payload).await)
        })
    }

    #[pyo3(signature = (payload_json))]
    fn spread_cancel_all_orders<'py>(
        &self,
        py: Python<'py>,
        payload_json: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        let payload = parse_json_value(Some(&payload_json), "payload")?
            .ok_or_else(|| PyRuntimeError::new_err("payload 不能为空"))?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(client.spread_cancel_all_orders(payload).await)
        })
    }

    #[pyo3(signature = (params_json))]
    fn spread_get_order_details<'py>(
        &self,
        py: Python<'py>,
        params_json: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        let params = parse_json_value(Some(&params_json), "params")?
            .ok_or_else(|| PyRuntimeError::new_err("params 不能为空"))?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(client.spread_get_order_details(params).await)
        })
    }

    #[pyo3(signature = (params_json=None))]
    fn spread_get_active_orders<'py>(
        &self,
        py: Python<'py>,
        params_json: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        let params = parse_json_value(params_json.as_deref(), "params")?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(client.spread_get_active_orders(params).await)
        })
    }

    #[pyo3(signature = (params_json=None))]
    fn spread_get_orders<'py>(
        &self,
        py: Python<'py>,
        params_json: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        let params = parse_json_value(params_json.as_deref(), "params")?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(client.spread_get_orders(params).await)
        })
    }

    #[pyo3(signature = (params_json=None))]
    fn spread_get_trades<'py>(
        &self,
        py: Python<'py>,
        params_json: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        let params = parse_json_value(params_json.as_deref(), "params")?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(client.spread_get_trades(params).await)
        })
    }

    #[pyo3(signature = (params_json=None))]
    fn spread_get_spreads<'py>(
        &self,
        py: Python<'py>,
        params_json: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        let params = parse_json_value(params_json.as_deref(), "params")?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(client.spread_get_spreads(params).await)
        })
    }

    #[pyo3(signature = (params_json))]
    fn spread_get_order_book<'py>(
        &self,
        py: Python<'py>,
        params_json: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        let params = parse_json_value(Some(&params_json), "params")?
            .ok_or_else(|| PyRuntimeError::new_err("params 不能为空"))?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(client.spread_get_order_book(params).await)
        })
    }

    #[pyo3(signature = (params_json))]
    fn spread_get_ticker<'py>(
        &self,
        py: Python<'py>,
        params_json: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        let params = parse_json_value(Some(&params_json), "params")?
            .ok_or_else(|| PyRuntimeError::new_err("params 不能为空"))?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(client.spread_get_ticker(params).await)
        })
    }

    #[pyo3(signature = (params_json))]
    fn spread_get_public_trades<'py>(
        &self,
        py: Python<'py>,
        params_json: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        let params = parse_json_value(Some(&params_json), "params")?
            .ok_or_else(|| PyRuntimeError::new_err("params 不能为空"))?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(client.spread_get_public_trades(params).await)
        })
    }

    // ==================== Trading Data (async) ====================

    fn get_support_coin<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(client.get_support_coin().await)
        })
    }

    #[pyo3(signature = (params_json=None))]
    fn get_taker_volume<'py>(
        &self,
        py: Python<'py>,
        params_json: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        let params = parse_json_value(params_json.as_deref(), "params")?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(client.get_taker_volume(params).await)
        })
    }

    #[pyo3(signature = (params_json=None))]
    fn get_margin_lending_ratio<'py>(
        &self,
        py: Python<'py>,
        params_json: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        let params = parse_json_value(params_json.as_deref(), "params")?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(client.get_margin_lending_ratio(params).await)
        })
    }

    #[pyo3(signature = (params_json=None))]
    fn get_long_short_ratio<'py>(
        &self,
        py: Python<'py>,
        params_json: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        let params = parse_json_value(params_json.as_deref(), "params")?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(client.get_long_short_ratio(params).await)
        })
    }

    #[pyo3(signature = (params_json=None))]
    fn get_contracts_open_interest_volume<'py>(
        &self,
        py: Python<'py>,
        params_json: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        let params = parse_json_value(params_json.as_deref(), "params")?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(client.get_contracts_open_interest_volume(params).await)
        })
    }

    #[pyo3(signature = (params_json=None))]
    fn get_options_open_interest_volume<'py>(
        &self,
        py: Python<'py>,
        params_json: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        let params = parse_json_value(params_json.as_deref(), "params")?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(client.get_options_open_interest_volume(params).await)
        })
    }

    #[pyo3(signature = (params_json=None))]
    fn get_put_call_ratio<'py>(
        &self,
        py: Python<'py>,
        params_json: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        let params = parse_json_value(params_json.as_deref(), "params")?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(client.get_put_call_ratio(params).await)
        })
    }

    #[pyo3(signature = (params_json=None))]
    fn get_open_interest_volume_expiry<'py>(
        &self,
        py: Python<'py>,
        params_json: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        let params = parse_json_value(params_json.as_deref(), "params")?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(client.get_open_interest_volume_expiry(params).await)
        })
    }

    #[pyo3(signature = (params_json=None))]
    fn get_interest_volume_strike<'py>(
        &self,
        py: Python<'py>,
        params_json: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        let params = parse_json_value(params_json.as_deref(), "params")?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(client.get_interest_volume_strike(params).await)
        })
    }

    #[pyo3(signature = (params_json=None))]
    fn get_taker_flow<'py>(
        &self,
        py: Python<'py>,
        params_json: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        let params = parse_json_value(params_json.as_deref(), "params")?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(client.get_taker_flow(params).await)
        })
    }

    // ==================== Market API ====================

    /// 查询单个交易对行情（异步）。
    fn get_ticker<'py>(&self, py: Python<'py>, inst_id: String) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            client
                .get_ticker(&inst_id)
                .await
                .map(|v| v.into_iter().next().map(PyTicker::from))
                .map_err(to_py_err)
        })
    }

    /// 查询指定品种的全部行情（异步）。
    fn get_tickers<'py>(&self, py: Python<'py>, inst_type: String) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        let params = GetTickersParams {
            inst_type,
            uly: None,
            inst_family: None,
        };

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            client
                .get_tickers(params)
                .await
                .map(|v| v.into_iter().map(PyTicker::from).collect::<Vec<_>>())
                .map_err(to_py_err)
        })
    }

    /// 查询订单簿（异步）。
    #[pyo3(signature = (inst_id, sz=None))]
    fn get_orderbook<'py>(
        &self,
        py: Python<'py>,
        inst_id: String,
        sz: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        let depth = sz.and_then(|v| v.parse::<u32>().ok());
        let instrument = inst_id;

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            client
                .get_orderbook(&instrument, depth)
                .await
                .map(|mut v| v.pop().map(PyOrderBook::from))
                .map_err(to_py_err)
        })
    }

    /// 查询 K 线（异步）。
    #[pyo3(signature = (inst_id, bar=None, after=None, before=None, limit=None))]
    fn get_candles<'py>(
        &self,
        py: Python<'py>,
        inst_id: String,
        bar: Option<String>,
        after: Option<String>,
        before: Option<String>,
        limit: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        let params = GetCandlesParams {
            inst_id,
            bar,
            after,
            before,
            limit,
        };

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            client
                .get_candles(params)
                .await
                .map(|v| {
                    v.into_iter()
                        .filter_map(|arr| okx_core::types::Candle::from_array(&arr))
                        .map(PyCandle::from)
                        .collect::<Vec<_>>()
                })
                .map_err(to_py_err)
        })
    }

    /// 查询历史 K 线（异步）。
    #[pyo3(signature = (inst_id, bar=None, after=None, before=None, limit=None))]
    fn get_history_candles<'py>(
        &self,
        py: Python<'py>,
        inst_id: String,
        bar: Option<String>,
        after: Option<String>,
        before: Option<String>,
        limit: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        let params = GetCandlesParams {
            inst_id,
            bar,
            after,
            before,
            limit,
        };

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            client
                .get_history_candles(params)
                .await
                .map(|v| {
                    v.into_iter()
                        .filter_map(|arr| okx_core::types::Candle::from_array(&arr))
                        .map(PyCandle::from)
                        .collect::<Vec<_>>()
                })
                .map_err(to_py_err)
        })
    }

    /// 查询指数 K 线（异步）。
    #[pyo3(signature = (inst_id, bar=None, after=None, before=None, limit=None))]
    fn get_index_candles<'py>(
        &self,
        py: Python<'py>,
        inst_id: String,
        bar: Option<String>,
        after: Option<String>,
        before: Option<String>,
        limit: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        let params = GetIndexCandlesParams {
            inst_id,
            bar,
            after,
            before,
            limit,
        };

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            client
                .get_index_candles(params)
                .await
                .map(|v| {
                    v.into_iter()
                        .filter_map(|arr| okx_core::types::Candle::from_array(&arr))
                        .map(PyCandle::from)
                        .collect::<Vec<_>>()
                })
                .map_err(to_py_err)
        })
    }

    /// 查询标记价格 K 线（异步）。
    #[pyo3(signature = (inst_id, bar=None, after=None, before=None, limit=None))]
    fn get_mark_price_candles<'py>(
        &self,
        py: Python<'py>,
        inst_id: String,
        bar: Option<String>,
        after: Option<String>,
        before: Option<String>,
        limit: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        let params = GetMarkPriceCandlesParams {
            inst_id,
            bar,
            after,
            before,
            limit,
        };

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            client
                .get_mark_price_candles(params)
                .await
                .map(|v| {
                    v.into_iter()
                        .filter_map(|arr| okx_core::types::Candle::from_array(&arr))
                        .map(PyCandle::from)
                        .collect::<Vec<_>>()
                })
                .map_err(to_py_err)
        })
    }

    /// 查询最新成交（异步）。
    #[pyo3(signature = (inst_id, limit=None))]
    fn get_trades<'py>(
        &self,
        py: Python<'py>,
        inst_id: String,
        limit: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        let parsed_limit = limit.and_then(|v| v.parse::<u32>().ok());
        let instrument = inst_id;

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            okx_rest::MarketApi::get_trades(client.as_ref(), &instrument, parsed_limit)
                .await
                .map(|v| v.into_iter().map(PyPublicTrade::from).collect::<Vec<_>>())
                .map_err(to_py_err)
        })
    }

    /// 查询历史成交（异步）。
    #[pyo3(signature = (inst_id, after=None, before=None, limit=None, type_=None))]
    fn get_history_trades<'py>(
        &self,
        py: Python<'py>,
        inst_id: String,
        after: Option<String>,
        before: Option<String>,
        limit: Option<String>,
        type_: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        let params = GetHistoryTradesParams {
            inst_id,
            after,
            before,
            limit,
            r#type: type_,
        };

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            client
                .get_history_trades(params)
                .await
                .map(|v| v.into_iter().map(PyPublicTrade::from).collect::<Vec<_>>())
                .map_err(to_py_err)
        })
    }

    /// 轻量深度（异步）。
    fn get_orderbook_lite<'py>(
        &self,
        py: Python<'py>,
        inst_id: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            client
                .get_orderbook_lite(&inst_id)
                .await
                .map(|mut v| v.pop().map(PyOrderBook::from))
                .map_err(to_py_err)
        })
    }

    /// 块交易行情（单个，异步）。
    fn get_block_ticker<'py>(
        &self,
        py: Python<'py>,
        inst_id: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            client
                .get_block_ticker(&inst_id)
                .await
                .map_err(to_py_err)
                .and_then(values_to_py_list)
        })
    }

    /// 块交易行情（列表，异步）。
    fn get_block_tickers<'py>(
        &self,
        py: Python<'py>,
        inst_type: String,
        uly: Option<String>,
        inst_family: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        let params = GetBlockTickersParams {
            inst_type,
            uly,
            inst_family,
        };

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            client
                .get_block_tickers(params)
                .await
                .map_err(to_py_err)
                .and_then(values_to_py_list)
        })
    }

    /// 块交易成交（异步）。
    fn get_block_trades<'py>(
        &self,
        py: Python<'py>,
        inst_id: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            client
                .get_block_trades(&inst_id)
                .await
                .map_err(to_py_err)
                .and_then(values_to_py_list)
        })
    }

    /// 期权系列成交（异步）。
    fn get_option_family_trades<'py>(
        &self,
        py: Python<'py>,
        inst_family: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            client
                .get_option_family_trades(&inst_family)
                .await
                .map_err(to_py_err)
                .and_then(values_to_py_list)
        })
    }

    /// 查询指数行情（异步）。
    #[pyo3(signature = (quote_ccy=None, inst_id=None))]
    fn get_index_tickers<'py>(
        &self,
        py: Python<'py>,
        quote_ccy: Option<String>,
        inst_id: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        let params = GetIndexTickersParams { quote_ccy, inst_id };

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            client
                .get_index_tickers(params)
                .await
                .map(|v| v.into_iter().map(PyIndexTicker::from).collect::<Vec<_>>())
                .map_err(to_py_err)
        })
    }

    // ==================== Public API ====================

    /// 查询合约/币对列表（异步）。
    #[pyo3(signature = (inst_type, inst_id=None))]
    fn get_instruments<'py>(
        &self,
        py: Python<'py>,
        inst_type: String,
        inst_id: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        let params = GetInstrumentsParams {
            inst_type,
            uly: None,
            inst_family: None,
            inst_id,
        };

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let instruments = client.get_instruments(params).await.map_err(to_py_err)?;

            Python::attach(|py| {
                instruments
                    .into_iter()
                    .map(|inst| {
                        let dict = pyo3::types::PyDict::new(py);
                        dict.set_item("instId", &inst.inst_id).ok();
                        dict.set_item("instType", &inst.inst_type).ok();
                        dict.set_item("baseCcy", &inst.base_ccy).ok();
                        dict.set_item("quoteCcy", &inst.quote_ccy).ok();
                        dict.set_item("tickSz", &inst.tick_sz).ok();
                        dict.set_item("lotSz", &inst.lot_sz).ok();
                        dict.set_item("minSz", &inst.min_sz).ok();
                        dict.set_item("state", &inst.state).ok();
                        Ok(dict.unbind().into())
                    })
                    .collect::<PyResult<Vec<Py<PyAny>>>>()
            })
        })
    }

    /// 查询交割/行权历史（异步）。
    #[pyo3(signature = (inst_type, uly=None, inst_family=None, after=None, before=None, limit=None))]
    fn get_delivery_exercise_history<'py>(
        &self,
        py: Python<'py>,
        inst_type: String,
        uly: Option<String>,
        inst_family: Option<String>,
        after: Option<String>,
        before: Option<String>,
        limit: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        let params = GetDeliveryExerciseHistoryParams {
            inst_type,
            uly,
            inst_family,
            after,
            before,
            limit,
        };

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            client
                .get_delivery_exercise_history(params)
                .await
                .map_err(to_py_err)
                .and_then(values_to_py_list)
        })
    }

    /// 查询持仓总量（异步）。
    #[pyo3(signature = (inst_type, uly=None, inst_id=None, inst_family=None))]
    fn get_open_interest<'py>(
        &self,
        py: Python<'py>,
        inst_type: String,
        uly: Option<String>,
        inst_id: Option<String>,
        inst_family: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        let params = GetOpenInterestParams {
            inst_type,
            uly,
            inst_id,
            inst_family,
        };

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            client
                .get_open_interest(params)
                .await
                .map_err(to_py_err)
                .and_then(values_to_py_list)
        })
    }

    /// 查询杠杆分层（异步）。
    #[pyo3(signature = (inst_type, td_mode, uly=None, inst_id=None, ccy=None, tier=None, inst_family=None))]
    fn get_position_tiers<'py>(
        &self,
        py: Python<'py>,
        inst_type: String,
        td_mode: String,
        uly: Option<String>,
        inst_id: Option<String>,
        ccy: Option<String>,
        tier: Option<String>,
        inst_family: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        let params = GetPositionTiersParams {
            inst_type,
            td_mode,
            uly,
            inst_id,
            ccy,
            tier,
            inst_family,
        };

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            client
                .get_position_tiers(params)
                .await
                .map_err(to_py_err)
                .and_then(values_to_py_list)
        })
    }

    /// 查询资金费率（异步）。
    fn get_funding_rate<'py>(
        &self,
        py: Python<'py>,
        inst_id: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        let instrument = inst_id;

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            client
                .get_funding_rate(&instrument)
                .await
                .map(|v| v.into_iter().next().map(PyFundingRate::from))
                .map_err(to_py_err)
        })
    }

    /// 查询资金费率历史（异步）。
    #[pyo3(signature = (inst_id, after=None, before=None, limit=None))]
    fn get_funding_rate_history<'py>(
        &self,
        py: Python<'py>,
        inst_id: String,
        after: Option<String>,
        before: Option<String>,
        limit: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        let params = GetFundingRateHistoryParams {
            inst_id,
            after,
            before,
            limit,
        };

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            client
                .get_funding_rate_history(params)
                .await
                .map(|v| v.into_iter().map(PyFundingRate::from).collect::<Vec<_>>())
                .map_err(to_py_err)
        })
    }

    /// 查询标记价格（异步）。
    #[pyo3(signature = (inst_type, uly=None, inst_family=None, inst_id=None))]
    fn get_mark_price<'py>(
        &self,
        py: Python<'py>,
        inst_type: String,
        uly: Option<String>,
        inst_family: Option<String>,
        inst_id: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        let params = GetMarkPriceParams {
            inst_type,
            uly,
            inst_family,
            inst_id,
        };

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            client
                .get_mark_price(params)
                .await
                .map(|v| v.into_iter().map(PyMarkPrice::from).collect::<Vec<_>>())
                .map_err(to_py_err)
        })
    }

    /// 查询价格限制 / Get price limit (async).
    #[pyo3(signature = (inst_id))]
    fn get_price_limit<'py>(
        &self,
        py: Python<'py>,
        inst_id: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        let params = GetPriceLimitParams { inst_id };
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            client
                .get_price_limit(params)
                .await
                .map_err(to_py_err)
                .and_then(values_to_py_list)
        })
    }

    /// 查询期权一览 / Get option summary (async).
    #[pyo3(signature = (uly=None, exp_time=None, inst_family=None))]
    fn get_opt_summary<'py>(
        &self,
        py: Python<'py>,
        uly: Option<String>,
        exp_time: Option<String>,
        inst_family: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        let params = GetOptSummaryParams {
            uly,
            exp_time,
            inst_family,
        };

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            client
                .get_opt_summary(params)
                .await
                .map_err(to_py_err)
                .and_then(values_to_py_list)
        })
    }

    /// 查询预估交割/行权价 / Get estimated delivery/exercise price (async).
    #[pyo3(signature = (inst_id))]
    fn get_estimated_price<'py>(
        &self,
        py: Python<'py>,
        inst_id: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        let params = GetEstimatedPriceParams { inst_id };
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            client
                .get_estimated_price(params)
                .await
                .map_err(to_py_err)
                .and_then(values_to_py_list)
        })
    }

    /// 查询折算汇率与免息额度 / Get discount rate and interest-free quota (async).
    #[pyo3(signature = (ccy=None))]
    fn get_discount_interest_free_quota<'py>(
        &self,
        py: Python<'py>,
        ccy: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        let params = GetDiscountQuotaParams { ccy };
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            client
                .get_discount_interest_free_quota(params)
                .await
                .map_err(to_py_err)
                .and_then(values_to_py_list)
        })
    }

    /// 查询公共利率与借币限额 / Get interest rate and loan quota (async).
    fn get_interest_rate_loan_quota<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            client
                .get_interest_rate_loan_quota()
                .await
                .map_err(to_py_err)
                .and_then(values_to_py_list)
        })
    }

    /// 查询 VIP 利率与借币限额 / Get VIP interest rate and loan quota (async).
    fn get_vip_interest_rate_loan_quota<'py>(
        &self,
        py: Python<'py>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            client
                .get_vip_interest_rate_loan_quota()
                .await
                .map_err(to_py_err)
                .and_then(values_to_py_list)
        })
    }

    /// 查询标的列表 / Get underlying list (async).
    #[pyo3(signature = (inst_type=None))]
    fn get_underlying<'py>(
        &self,
        py: Python<'py>,
        inst_type: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        let params = GetUnderlyingParams { inst_type };
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            client
                .get_underlying(params)
                .await
                .map_err(to_py_err)
                .and_then(values_to_py_list)
        })
    }

    /// 查询保险基金 / Get insurance fund (async).
    #[pyo3(signature = (inst_type=None, type_=None, uly=None, ccy=None, before=None, after=None, limit=None, inst_family=None))]
    fn get_insurance_fund<'py>(
        &self,
        py: Python<'py>,
        inst_type: Option<String>,
        type_: Option<String>,
        uly: Option<String>,
        ccy: Option<String>,
        before: Option<String>,
        after: Option<String>,
        limit: Option<String>,
        inst_family: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        let params = GetInsuranceFundParams {
            inst_type,
            r#type: type_,
            uly,
            ccy,
            before,
            after,
            limit,
            inst_family,
        };

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            client
                .get_insurance_fund(params)
                .await
                .map_err(to_py_err)
                .and_then(values_to_py_list)
        })
    }

    /// 合约币种单位换算 / Convert contract coin units (async).
    #[pyo3(signature = (type_=None, inst_id=None, sz=None, px=None, unit=None))]
    fn get_convert_contract_coin<'py>(
        &self,
        py: Python<'py>,
        type_: Option<String>,
        inst_id: Option<String>,
        sz: Option<String>,
        px: Option<String>,
        unit: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        let params = GetConvertContractCoinParams {
            r#type: type_,
            inst_id,
            sz,
            px,
            unit,
        };

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            client
                .get_convert_contract_coin(params)
                .await
                .map_err(to_py_err)
                .and_then(values_to_py_list)
        })
    }

    /// 获取服务器时间戳（异步，毫秒）。
    fn get_system_time<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            client
                .get_system_time()
                .await
                .map(|v| v.first().map(|t| t.ts.clone()).unwrap_or_default())
                .map_err(to_py_err)
        })
    }

    /// 获取系统状态（异步）。
    #[pyo3(signature = (state=None))]
    fn get_system_status<'py>(
        &self,
        py: Python<'py>,
        state: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.client);
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            client
                .get_system_status(state.as_deref())
                .await
                .map_err(to_py_err)
                .and_then(values_to_py_list)
        })
    }

    fn __repr__(&self) -> String {
        format!(
            "AsyncOkxClient(simulated={})",
            self.client.config().is_simulated()
        )
    }
}
