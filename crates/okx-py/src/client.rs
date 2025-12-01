//! Python client wrapper for OKX REST API.

use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;
use tokio::runtime::Runtime;

use okx_core::types::{
    AmendOrderRequest, CancelAlgoOrderRequest, CancelOrderRequest, FundsTransferRequest,
    PlaceAlgoOrderRequest, PlaceOrderRequest, WithdrawalRequest,
};
use okx_rest::api::account::{
    BorrowRepayHistoryParams, BorrowRepayRequest, GetAccountPositionTiersParams,
    GetBillsArchiveParams, GetBillsParams, GetFeeRatesParams, GetGreeksParams,
    GetInterestAccruedParams, GetLeverageInfoParams, GetMaxAvailSizeParams, GetMaxSizeParams,
    GetMaxWithdrawalParams, GetPositionsHistoryParams, GetSimulatedMarginParams,
    GetVipInterestParams, PositionBuilderRequest, SetAccountLevelRequest, SetGreeksRequest,
    SetIsolatedModeRequest, SetLeverageRequest, SpotBorrowRepayHistoryParams,
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
    GetDeliveryExerciseHistoryParams, GetFundingRateHistoryParams, GetInstrumentsParams,
    GetMarkPriceParams, GetOpenInterestParams, GetPositionTiersParams,
};
use okx_rest::api::trade::{
    AmendAlgoOrderRequest, ClosePositionRequest, GetAlgoOrderDetailsParams,
    GetAlgoOrdersHistoryParams, GetAlgoOrdersParams, GetFillsHistoryParams, GetFillsParams,
    GetOrderParams, GetOrdersHistoryArchiveParams, GetOrdersHistoryParams, GetOrdersPendingParams,
};
use okx_rest::{AccountApi, FundingApi, MarketApi, OkxRestClient, PublicApi, TradeApi};

use crate::types::*;
use crate::{map_values, parse_json_array, parse_json_value, to_py_err, values_to_py_list};

/// Python wrapper for OKX REST client.
///
/// Provides synchronous access to OKX REST API.
#[pyclass(name = "OkxClient")]
pub struct PyOkxClient {
    client: OkxRestClient,
    runtime: Runtime,
}

#[pymethods]
impl PyOkxClient {
    /// Create a new OKX client.
    ///
    /// Args:
    ///     config: Client configuration
    #[new]
    fn new(config: PyConfig) -> PyResult<Self> {
        let runtime = Runtime::new()
            .map_err(|e| PyRuntimeError::new_err(format!("Failed to create runtime: {}", e)))?;
        let client = OkxRestClient::new(config.inner);
        Ok(Self { client, runtime })
    }

    // ==================== Account API ====================

    /// Get account balance.
    ///
    /// Args:
    ///     ccy: Optional currency filter (e.g., "BTC" or "BTC,ETH")
    ///
    /// Returns:
    ///     List of Balance objects
    #[pyo3(signature = (ccy=None))]
    fn get_balance(&self, ccy: Option<&str>) -> PyResult<Vec<PyBalance>> {
        self.runtime.block_on(async {
            self.client
                .get_balance(ccy)
                .await
                .map(|v| v.into_iter().map(PyBalance::from).collect())
                .map_err(to_py_err)
        })
    }

    /// Get positions.
    ///
    /// Args:
    ///     inst_type: Optional instrument type filter
    ///     inst_id: Optional instrument ID filter
    ///
    /// Returns:
    ///     List of Position objects
    #[pyo3(signature = (inst_type=None, inst_id=None))]
    fn get_positions(
        &self,
        inst_type: Option<&str>,
        inst_id: Option<&str>,
    ) -> PyResult<Vec<PyPosition>> {
        use okx_rest::api::account::GetPositionsParams;

        let params = if inst_type.is_some() || inst_id.is_some() {
            Some(GetPositionsParams {
                inst_type: inst_type.map(String::from),
                inst_id: inst_id.map(String::from),
                pos_id: None,
            })
        } else {
            None
        };

        self.runtime.block_on(async {
            self.client
                .get_positions(params)
                .await
                .map(|v| v.into_iter().map(PyPosition::from).collect())
                .map_err(to_py_err)
        })
    }

    /// 获取账户配置。
    fn get_account_config(&self) -> PyResult<Vec<PyAccountConfig>> {
        self.runtime.block_on(async {
            self.client
                .get_account_config()
                .await
                .map(|v| v.into_iter().map(PyAccountConfig::from).collect())
                .map_err(to_py_err)
        })
    }

    /// 设置杠杆。
    #[pyo3(signature = (lever, mgn_mode, inst_id=None, ccy=None, pos_side=None))]
    fn set_leverage(
        &self,
        lever: &str,
        mgn_mode: &str,
        inst_id: Option<&str>,
        ccy: Option<&str>,
        pos_side: Option<&str>,
    ) -> PyResult<Option<PySetLeverageResult>> {
        let request = SetLeverageRequest {
            inst_id: inst_id.map(String::from),
            ccy: ccy.map(String::from),
            lever: lever.to_string(),
            mgn_mode: mgn_mode.to_string(),
            pos_side: pos_side.map(String::from),
        };

        self.runtime.block_on(async {
            self.client
                .set_leverage(request)
                .await
                .map(|mut v| v.pop().map(PySetLeverageResult::from))
                .map_err(to_py_err)
        })
    }

    /// 查询杠杆信息。
    #[pyo3(signature = (mgn_mode, ccy=None, inst_id=None))]
    fn get_leverage_info(
        &self,
        mgn_mode: &str,
        ccy: Option<&str>,
        inst_id: Option<&str>,
    ) -> PyResult<Vec<PyLeverageInfo>> {
        let params = GetLeverageInfoParams {
            mgn_mode: mgn_mode.to_string(),
            ccy: ccy.map(String::from),
            inst_id: inst_id.map(String::from),
        };

        self.runtime.block_on(async {
            self.client
                .get_leverage_info(params)
                .await
                .map(|v| v.into_iter().map(PyLeverageInfo::from).collect())
                .map_err(to_py_err)
        })
    }

    /// 查询最大可下单张数。
    #[pyo3(signature = (inst_id, td_mode, ccy=None, px=None, leverage=None))]
    fn get_max_size(
        &self,
        inst_id: &str,
        td_mode: &str,
        ccy: Option<&str>,
        px: Option<&str>,
        leverage: Option<&str>,
    ) -> PyResult<Option<PyMaxSize>> {
        let params = GetMaxSizeParams {
            inst_id: inst_id.to_string(),
            td_mode: td_mode.to_string(),
            ccy: ccy.map(String::from),
            px: px.map(String::from),
            leverage: leverage.map(String::from),
        };

        self.runtime.block_on(async {
            self.client
                .get_max_size(params)
                .await
                .map(|mut v| v.pop().map(PyMaxSize::from))
                .map_err(to_py_err)
        })
    }

    /// 查询最大可用张数。
    #[pyo3(signature = (inst_id, td_mode, ccy=None, reduce_only=None, quick_mgn_type=None))]
    fn get_max_avail_size(
        &self,
        inst_id: &str,
        td_mode: &str,
        ccy: Option<&str>,
        reduce_only: Option<bool>,
        quick_mgn_type: Option<&str>,
    ) -> PyResult<Option<PyMaxAvailSize>> {
        let params = GetMaxAvailSizeParams {
            inst_id: inst_id.to_string(),
            td_mode: td_mode.to_string(),
            ccy: ccy.map(String::from),
            reduce_only,
            quick_mgn_type: quick_mgn_type.map(String::from),
        };

        self.runtime.block_on(async {
            self.client
                .get_max_avail_size(params)
                .await
                .map(|mut v| v.pop().map(PyMaxAvailSize::from))
                .map_err(to_py_err)
        })
    }

    /// 查询手续费率。
    #[pyo3(signature = (inst_type, inst_id=None, uly=None, inst_family=None))]
    fn get_fee_rates(
        &self,
        inst_type: &str,
        inst_id: Option<&str>,
        uly: Option<&str>,
        inst_family: Option<&str>,
    ) -> PyResult<Vec<PyFeeRates>> {
        let params = GetFeeRatesParams {
            inst_type: inst_type.to_string(),
            inst_id: inst_id.map(String::from),
            uly: uly.map(String::from),
            inst_family: inst_family.map(String::from),
        };

        self.runtime.block_on(async {
            self.client
                .get_fee_rates(params)
                .await
                .map(|v| v.into_iter().map(PyFeeRates::from).collect())
                .map_err(to_py_err)
        })
    }

    /// 设置持仓模式。
    fn set_position_mode(&self, pos_mode: &str) -> PyResult<Option<PySetPositionModeResult>> {
        self.runtime.block_on(async {
            self.client
                .set_position_mode(pos_mode)
                .await
                .map(|mut v| v.pop().map(PySetPositionModeResult::from))
                .map_err(to_py_err)
        })
    }

    /// 获取账户风险。
    fn get_account_position_risk(&self) -> PyResult<Vec<PyAccountPositionRisk>> {
        self.runtime.block_on(async {
            self.client
                .get_account_position_risk()
                .await
                .map(|v| v.into_iter().map(PyAccountPositionRisk::from).collect())
                .map_err(to_py_err)
        })
    }

    /// 获取历史持仓（最多 3 个月）。
    #[pyo3(
        signature = (inst_type=None, inst_id=None, mgn_mode=None, type_=None, pos_id=None, after=None, before=None, limit=None)
    )]
    fn get_positions_history(
        &self,
        inst_type: Option<&str>,
        inst_id: Option<&str>,
        mgn_mode: Option<&str>,
        type_: Option<&str>,
        pos_id: Option<&str>,
        after: Option<&str>,
        before: Option<&str>,
        limit: Option<&str>,
    ) -> PyResult<Vec<Py<PyAny>>> {
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
                inst_type: inst_type.map(String::from),
                inst_id: inst_id.map(String::from),
                mgn_mode: mgn_mode.map(String::from),
                r#type: type_.map(String::from),
                pos_id: pos_id.map(String::from),
                after: after.map(String::from),
                before: before.map(String::from),
                limit: limit.map(String::from),
            })
        } else {
            None
        };

        self.runtime
            .block_on(async { map_values(self.client.get_positions_history(params).await) })
    }

    /// 查询最大可提额度。
    #[pyo3(signature = (ccy=None))]
    fn get_max_withdrawal(&self, ccy: Option<&str>) -> PyResult<Vec<Py<PyAny>>> {
        let params = GetMaxWithdrawalParams {
            ccy: ccy.map(String::from),
        };
        self.runtime.block_on(async {
            map_values(self.client.get_max_withdrawal(params.ccy.as_deref()).await)
        })
    }

    /// 查询账户账单（近 7 天）。
    #[pyo3(
        signature = (inst_type=None, ccy=None, mgn_mode=None, ct_type=None, type_=None, sub_type=None, after=None, before=None, limit=None)
    )]
    fn get_account_bills(
        &self,
        inst_type: Option<&str>,
        ccy: Option<&str>,
        mgn_mode: Option<&str>,
        ct_type: Option<&str>,
        type_: Option<&str>,
        sub_type: Option<&str>,
        after: Option<&str>,
        before: Option<&str>,
        limit: Option<&str>,
    ) -> PyResult<Vec<Py<PyAny>>> {
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
                inst_type: inst_type.map(String::from),
                ccy: ccy.map(String::from),
                mgn_mode: mgn_mode.map(String::from),
                ct_type: ct_type.map(String::from),
                r#type: type_.map(String::from),
                sub_type: sub_type.map(String::from),
                after: after.map(String::from),
                before: before.map(String::from),
                limit: limit.map(String::from),
            })
        } else {
            None
        };

        self.runtime.block_on(async {
            self.client
                .get_account_bills(params)
                .await
                .map_err(to_py_err)
                .and_then(values_to_py_list)
        })
    }

    /// 查询账户账单归档（近 3 个月）。
    #[pyo3(
        signature = (inst_type=None, ccy=None, mgn_mode=None, ct_type=None, type_=None, sub_type=None, after=None, before=None, limit=None, begin=None, end=None)
    )]
    fn get_account_bills_archive(
        &self,
        inst_type: Option<&str>,
        ccy: Option<&str>,
        mgn_mode: Option<&str>,
        ct_type: Option<&str>,
        type_: Option<&str>,
        sub_type: Option<&str>,
        after: Option<&str>,
        before: Option<&str>,
        limit: Option<&str>,
        begin: Option<&str>,
        end: Option<&str>,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let params = GetBillsArchiveParams {
            inst_type: inst_type.map(String::from),
            ccy: ccy.map(String::from),
            mgn_mode: mgn_mode.map(String::from),
            ct_type: ct_type.map(String::from),
            r#type: type_.map(String::from),
            sub_type: sub_type.map(String::from),
            after: after.map(String::from),
            before: before.map(String::from),
            limit: limit.map(String::from),
            begin: begin.map(String::from),
            end: end.map(String::from),
        };

        self.runtime.block_on(async {
            self.client
                .get_account_bills_archive(params)
                .await
                .map_err(to_py_err)
                .and_then(values_to_py_list)
        })
    }

    /// 设置 Greeks 显示方式。
    fn set_greeks(&self, greeks_type: &str) -> PyResult<Vec<Py<PyAny>>> {
        let request = SetGreeksRequest {
            greeks_type: greeks_type.to_string(),
        };
        self.runtime.block_on(async {
            self.client
                .set_greeks(request)
                .await
                .map_err(to_py_err)
                .and_then(values_to_py_list)
        })
    }

    /// 设置逐仓模式。
    #[pyo3(signature = (iso_mode, type_))]
    fn set_isolated_mode(&self, iso_mode: &str, type_: &str) -> PyResult<Vec<Py<PyAny>>> {
        let request = SetIsolatedModeRequest {
            iso_mode: iso_mode.to_string(),
            r#type: type_.to_string(),
        };
        self.runtime.block_on(async {
            self.client
                .set_isolated_mode(request)
                .await
                .map_err(to_py_err)
                .and_then(values_to_py_list)
        })
    }

    /// 设置账户等级。
    fn set_account_level(&self, acct_lv: &str) -> PyResult<Vec<Py<PyAny>>> {
        let request = SetAccountLevelRequest {
            acct_lv: acct_lv.to_string(),
        };
        self.runtime.block_on(async {
            self.client
                .set_account_level(request)
                .await
                .map_err(to_py_err)
                .and_then(values_to_py_list)
        })
    }

    /// 借币/还币。
    #[pyo3(signature = (ccy=None, side=None, amt=None, ord_id=None))]
    fn borrow_repay(
        &self,
        ccy: Option<&str>,
        side: Option<&str>,
        amt: Option<&str>,
        ord_id: Option<&str>,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let request = BorrowRepayRequest {
            ccy: ccy.map(String::from),
            side: side.map(String::from),
            amt: amt.map(String::from),
            ord_id: ord_id.map(String::from),
        };

        self.runtime.block_on(async {
            self.client
                .borrow_repay(request)
                .await
                .map_err(to_py_err)
                .and_then(values_to_py_list)
        })
    }

    /// 借币/还币历史。
    #[pyo3(signature = (ccy=None, after=None, before=None, limit=None))]
    fn get_borrow_repay_history(
        &self,
        ccy: Option<&str>,
        after: Option<&str>,
        before: Option<&str>,
        limit: Option<&str>,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let params = if ccy.is_some() || after.is_some() || before.is_some() || limit.is_some() {
            Some(BorrowRepayHistoryParams {
                ccy: ccy.map(String::from),
                after: after.map(String::from),
                before: before.map(String::from),
                limit: limit.map(String::from),
            })
        } else {
            None
        };

        self.runtime.block_on(async {
            self.client
                .get_borrow_repay_history(params)
                .await
                .map_err(to_py_err)
                .and_then(values_to_py_list)
        })
    }

    /// 现货手动借币/还币。
    #[pyo3(signature = (ccy=None, side=None, amt=None))]
    fn spot_manual_borrow_repay(
        &self,
        ccy: Option<&str>,
        side: Option<&str>,
        amt: Option<&str>,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let request = SpotManualBorrowRepayRequest {
            ccy: ccy.map(String::from),
            side: side.map(String::from),
            amt: amt.map(String::from),
        };

        self.runtime.block_on(async {
            self.client
                .spot_manual_borrow_repay(request)
                .await
                .map_err(to_py_err)
                .and_then(values_to_py_list)
        })
    }

    /// 现货借币/还币历史。
    #[pyo3(signature = (ccy=None, type_=None, after=None, before=None, limit=None))]
    fn spot_borrow_repay_history(
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
            Some(SpotBorrowRepayHistoryParams {
                ccy: ccy.map(String::from),
                r#type: type_.map(String::from),
                after: after.map(String::from),
                before: before.map(String::from),
                limit: limit.map(String::from),
            })
        } else {
            None
        };

        self.runtime.block_on(async {
            self.client
                .spot_borrow_repay_history(params)
                .await
                .map_err(to_py_err)
                .and_then(values_to_py_list)
        })
    }

    /// 已生息数据。
    #[pyo3(signature = (inst_id=None, ccy=None, mgn_mode=None, after=None, before=None, limit=None))]
    fn get_interest_accrued(
        &self,
        inst_id: Option<&str>,
        ccy: Option<&str>,
        mgn_mode: Option<&str>,
        after: Option<&str>,
        before: Option<&str>,
        limit: Option<&str>,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let params = GetInterestAccruedParams {
            inst_id: inst_id.map(String::from),
            ccy: ccy.map(String::from),
            mgn_mode: mgn_mode.map(String::from),
            after: after.map(String::from),
            before: before.map(String::from),
            limit: limit.map(String::from),
        };

        self.runtime.block_on(async {
            self.client
                .get_interest_accrued(params)
                .await
                .map_err(to_py_err)
                .and_then(values_to_py_list)
        })
    }

    /// VIP 已生息数据。
    #[pyo3(signature = (ccy=None, ord_id=None, after=None, before=None, limit=None))]
    fn get_vip_interest_accrued(
        &self,
        ccy: Option<&str>,
        ord_id: Option<&str>,
        after: Option<&str>,
        before: Option<&str>,
        limit: Option<&str>,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let params = GetVipInterestParams {
            ccy: ccy.map(String::from),
            ord_id: ord_id.map(String::from),
            after: after.map(String::from),
            before: before.map(String::from),
            limit: limit.map(String::from),
        };

        self.runtime.block_on(async {
            self.client
                .get_vip_interest_accrued(params)
                .await
                .map_err(to_py_err)
                .and_then(values_to_py_list)
        })
    }

    /// VIP 已扣息数据。
    #[pyo3(signature = (ccy=None, ord_id=None, after=None, before=None, limit=None))]
    fn get_vip_interest_deducted(
        &self,
        ccy: Option<&str>,
        ord_id: Option<&str>,
        after: Option<&str>,
        before: Option<&str>,
        limit: Option<&str>,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let params = GetVipInterestParams {
            ccy: ccy.map(String::from),
            ord_id: ord_id.map(String::from),
            after: after.map(String::from),
            before: before.map(String::from),
            limit: limit.map(String::from),
        };

        self.runtime.block_on(async {
            self.client
                .get_vip_interest_deducted(params)
                .await
                .map_err(to_py_err)
                .and_then(values_to_py_list)
        })
    }

    /// 模拟保证金计算。
    #[pyo3(signature = (inst_type, incl_real_pos=None, spot_offset_type=None, sim_pos_json=None))]
    fn get_simulated_margin(
        &self,
        inst_type: &str,
        incl_real_pos: Option<bool>,
        spot_offset_type: Option<&str>,
        sim_pos_json: Option<&str>,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let sim_pos = parse_json_array(sim_pos_json, "sim_pos")?;
        let params = GetSimulatedMarginParams {
            inst_type: inst_type.to_string(),
            incl_real_pos,
            spot_offset_type: spot_offset_type.map(String::from),
            sim_pos,
        };

        self.runtime.block_on(async {
            self.client
                .get_simulated_margin(params)
                .await
                .map_err(to_py_err)
                .and_then(values_to_py_list)
        })
    }

    /// 杠杆分层。
    fn get_account_position_tiers(
        &self,
        inst_type: &str,
        uly: Option<&str>,
        inst_family: Option<&str>,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let params = GetAccountPositionTiersParams {
            inst_type: inst_type.to_string(),
            uly: uly.map(String::from),
            inst_family: inst_family.map(String::from),
        };

        self.runtime.block_on(async {
            self.client
                .get_account_position_tiers(params)
                .await
                .map_err(to_py_err)
                .and_then(values_to_py_list)
        })
    }

    /// 查询 Greeks。
    #[pyo3(signature = (ccy=None))]
    fn get_greeks(&self, ccy: Option<&str>) -> PyResult<Vec<Py<PyAny>>> {
        let params = GetGreeksParams {
            ccy: ccy.map(String::from),
        };
        self.runtime.block_on(async {
            self.client
                .get_greeks(params)
                .await
                .map_err(to_py_err)
                .and_then(values_to_py_list)
        })
    }

    /// Position Builder 构建模拟仓位。
    #[pyo3(
        signature = (acct_lv=None, incl_real_pos_and_eq=None, lever=None, greeks_type=None, sim_pos_json=None, sim_asset_json=None)
    )]
    fn position_builder(
        &self,
        acct_lv: Option<&str>,
        incl_real_pos_and_eq: Option<bool>,
        lever: Option<&str>,
        greeks_type: Option<&str>,
        sim_pos_json: Option<&str>,
        sim_asset_json: Option<&str>,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let sim_pos = parse_json_value(sim_pos_json, "sim_pos")?;
        let sim_asset = parse_json_value(sim_asset_json, "sim_asset")?;
        let request = PositionBuilderRequest {
            acct_lv: acct_lv.map(String::from),
            incl_real_pos_and_eq,
            lever: lever.map(String::from),
            greeks_type: greeks_type.map(String::from),
            sim_pos,
            sim_asset,
        };

        self.runtime.block_on(async {
            self.client
                .position_builder(request)
                .await
                .map_err(to_py_err)
                .and_then(values_to_py_list)
        })
    }

    // ==================== Trade API ====================

    /// 下单（单笔）。
    #[pyo3(signature = (inst_id, td_mode, side, ord_type, sz, px=None, cl_ord_id=None))]
    fn place_order(
        &self,
        inst_id: &str,
        td_mode: &str,
        side: &str,
        ord_type: &str,
        sz: &str,
        px: Option<&str>,
        cl_ord_id: Option<&str>,
    ) -> PyResult<String> {
        let request = PlaceOrderRequest {
            inst_id: inst_id.to_string(),
            td_mode: td_mode.to_string(),
            side: side.to_string(),
            ord_type: ord_type.to_string(),
            sz: sz.to_string(),
            px: px.map(String::from),
            cl_ord_id: cl_ord_id.map(String::from),
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

        self.runtime.block_on(async {
            self.client
                .place_order(request)
                .await
                .map(|v| v.first().map(|r| r.ord_id.clone()).unwrap_or_default())
                .map_err(to_py_err)
        })
    }

    /// 批量下单。
    #[pyo3(signature = (orders))]
    fn place_batch_orders(
        &self,
        orders: Vec<(
            String,
            String,
            String,
            String,
            String,
            Option<String>,
            Option<String>,
        )>,
    ) -> PyResult<Vec<PyPlaceOrderResult>> {
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

        self.runtime.block_on(async {
            self.client
                .place_batch_orders(requests)
                .await
                .map(|v| v.into_iter().map(PyPlaceOrderResult::from).collect())
                .map_err(to_py_err)
        })
    }

    /// 撤单。
    #[pyo3(signature = (inst_id, ord_id=None, cl_ord_id=None))]
    fn cancel_order(
        &self,
        inst_id: &str,
        ord_id: Option<&str>,
        cl_ord_id: Option<&str>,
    ) -> PyResult<String> {
        let request = CancelOrderRequest {
            inst_id: inst_id.to_string(),
            ord_id: ord_id.map(String::from),
            cl_ord_id: cl_ord_id.map(String::from),
        };

        self.runtime.block_on(async {
            self.client
                .cancel_order(request)
                .await
                .map(|v| v.first().map(|r| r.ord_id.clone()).unwrap_or_default())
                .map_err(to_py_err)
        })
    }

    /// 批量撤单。
    #[pyo3(signature = (orders))]
    fn cancel_batch_orders(
        &self,
        orders: Vec<(String, Option<String>, Option<String>)>,
    ) -> PyResult<Vec<PyCancelOrderResult>> {
        let requests: Vec<CancelOrderRequest> = orders
            .into_iter()
            .map(|(inst_id, ord_id, cl_ord_id)| CancelOrderRequest {
                inst_id,
                ord_id,
                cl_ord_id,
            })
            .collect();

        self.runtime.block_on(async {
            self.client
                .cancel_batch_orders(requests)
                .await
                .map(|v| v.into_iter().map(PyCancelOrderResult::from).collect())
                .map_err(to_py_err)
        })
    }

    /// 改单。
    #[pyo3(
        signature = (inst_id, ord_id=None, cl_ord_id=None, req_id=None, new_sz=None, new_px=None, new_tp_trigger_px=None, new_tp_ord_px=None, new_sl_trigger_px=None, new_sl_ord_px=None, new_tp_trigger_px_type=None, new_sl_trigger_px_type=None)
    )]
    fn amend_order(
        &self,
        inst_id: &str,
        ord_id: Option<&str>,
        cl_ord_id: Option<&str>,
        req_id: Option<&str>,
        new_sz: Option<&str>,
        new_px: Option<&str>,
        new_tp_trigger_px: Option<&str>,
        new_tp_ord_px: Option<&str>,
        new_sl_trigger_px: Option<&str>,
        new_sl_ord_px: Option<&str>,
        new_tp_trigger_px_type: Option<&str>,
        new_sl_trigger_px_type: Option<&str>,
    ) -> PyResult<Option<PyAmendOrderResult>> {
        let request = AmendOrderRequest {
            inst_id: inst_id.to_string(),
            ord_id: ord_id.map(String::from),
            cl_ord_id: cl_ord_id.map(String::from),
            req_id: req_id.map(String::from),
            new_sz: new_sz.map(String::from),
            new_px: new_px.map(String::from),
            new_tp_trigger_px: new_tp_trigger_px.map(String::from),
            new_tp_ord_px: new_tp_ord_px.map(String::from),
            new_sl_trigger_px: new_sl_trigger_px.map(String::from),
            new_sl_ord_px: new_sl_ord_px.map(String::from),
            new_tp_trigger_px_type: new_tp_trigger_px_type.map(String::from),
            new_sl_trigger_px_type: new_sl_trigger_px_type.map(String::from),
        };

        self.runtime.block_on(async {
            self.client
                .amend_order(request)
                .await
                .map(|mut v| v.pop().map(PyAmendOrderResult::from))
                .map_err(to_py_err)
        })
    }

    /// 查询订单详情。
    #[pyo3(signature = (inst_id, ord_id=None, cl_ord_id=None))]
    fn get_order(
        &self,
        inst_id: &str,
        ord_id: Option<&str>,
        cl_ord_id: Option<&str>,
    ) -> PyResult<Option<PyOrder>> {
        let params = GetOrderParams {
            inst_id: inst_id.to_string(),
            ord_id: ord_id.map(String::from),
            cl_ord_id: cl_ord_id.map(String::from),
        };

        self.runtime.block_on(async {
            self.client
                .get_order(params)
                .await
                .map(|v| v.into_iter().next().map(PyOrder::from))
                .map_err(to_py_err)
        })
    }

    /// 查询挂单。
    #[pyo3(signature = (inst_type=None, inst_id=None))]
    fn get_orders_pending(
        &self,
        inst_type: Option<&str>,
        inst_id: Option<&str>,
    ) -> PyResult<Vec<PyOrder>> {
        let params = if inst_type.is_some() || inst_id.is_some() {
            Some(GetOrdersPendingParams {
                inst_type: inst_type.map(String::from),
                inst_id: inst_id.map(String::from),
                ..Default::default()
            })
        } else {
            None
        };

        self.runtime.block_on(async {
            self.client
                .get_orders_pending(params)
                .await
                .map(|v| v.into_iter().map(PyOrder::from).collect())
                .map_err(to_py_err)
        })
    }

    /// 查询历史订单（近 7 天）。
    #[pyo3(
        signature = (inst_type, uly=None, inst_family=None, inst_id=None, ord_type=None, state=None, category=None, after=None, before=None, begin=None, end=None, limit=None)
    )]
    fn get_orders_history(
        &self,
        inst_type: &str,
        uly: Option<&str>,
        inst_family: Option<&str>,
        inst_id: Option<&str>,
        ord_type: Option<&str>,
        state: Option<&str>,
        category: Option<&str>,
        after: Option<&str>,
        before: Option<&str>,
        begin: Option<&str>,
        end: Option<&str>,
        limit: Option<&str>,
    ) -> PyResult<Vec<PyOrder>> {
        let params = GetOrdersHistoryParams {
            inst_type: inst_type.to_string(),
            uly: uly.map(String::from),
            inst_family: inst_family.map(String::from),
            inst_id: inst_id.map(String::from),
            ord_type: ord_type.map(String::from),
            state: state.map(String::from),
            category: category.map(String::from),
            after: after.map(String::from),
            before: before.map(String::from),
            begin: begin.map(String::from),
            end: end.map(String::from),
            limit: limit.map(String::from),
        };

        self.runtime.block_on(async {
            self.client
                .get_orders_history(params)
                .await
                .map(|v| v.into_iter().map(PyOrder::from).collect())
                .map_err(to_py_err)
        })
    }

    /// 查询历史订单归档（近 3 个月）。
    #[pyo3(
        signature = (inst_type, uly=None, inst_family=None, inst_id=None, ord_type=None, state=None, category=None, after=None, before=None, begin=None, end=None, limit=None)
    )]
    fn get_orders_history_archive(
        &self,
        inst_type: &str,
        uly: Option<&str>,
        inst_family: Option<&str>,
        inst_id: Option<&str>,
        ord_type: Option<&str>,
        state: Option<&str>,
        category: Option<&str>,
        after: Option<&str>,
        before: Option<&str>,
        begin: Option<&str>,
        end: Option<&str>,
        limit: Option<&str>,
    ) -> PyResult<Vec<PyOrder>> {
        let params = GetOrdersHistoryArchiveParams {
            inst_type: inst_type.to_string(),
            uly: uly.map(String::from),
            inst_family: inst_family.map(String::from),
            inst_id: inst_id.map(String::from),
            ord_type: ord_type.map(String::from),
            state: state.map(String::from),
            category: category.map(String::from),
            after: after.map(String::from),
            before: before.map(String::from),
            begin: begin.map(String::from),
            end: end.map(String::from),
            limit: limit.map(String::from),
        };

        self.runtime.block_on(async {
            self.client
                .get_orders_history_archive(params)
                .await
                .map(|v| v.into_iter().map(PyOrder::from).collect())
                .map_err(to_py_err)
        })
    }

    /// 查询成交明细。
    #[pyo3(
        signature = (inst_type=None, uly=None, inst_family=None, inst_id=None, ord_id=None, after=None, before=None, begin=None, end=None, limit=None)
    )]
    fn get_fills(
        &self,
        inst_type: Option<&str>,
        uly: Option<&str>,
        inst_family: Option<&str>,
        inst_id: Option<&str>,
        ord_id: Option<&str>,
        after: Option<&str>,
        before: Option<&str>,
        begin: Option<&str>,
        end: Option<&str>,
        limit: Option<&str>,
    ) -> PyResult<Vec<PyFill>> {
        let params = GetFillsParams {
            inst_type: inst_type.map(String::from),
            uly: uly.map(String::from),
            inst_family: inst_family.map(String::from),
            inst_id: inst_id.map(String::from),
            ord_id: ord_id.map(String::from),
            after: after.map(String::from),
            before: before.map(String::from),
            begin: begin.map(String::from),
            end: end.map(String::from),
            limit: limit.map(String::from),
        };

        self.runtime.block_on(async {
            self.client
                .get_fills(Some(params))
                .await
                .map(|v| v.into_iter().map(PyFill::from).collect())
                .map_err(to_py_err)
        })
    }

    /// 查询历史成交（近 3 个月）。
    #[pyo3(
        signature = (inst_type, uly=None, inst_family=None, inst_id=None, ord_id=None, after=None, before=None, limit=None)
    )]
    fn get_fills_history(
        &self,
        inst_type: &str,
        uly: Option<&str>,
        inst_family: Option<&str>,
        inst_id: Option<&str>,
        ord_id: Option<&str>,
        after: Option<&str>,
        before: Option<&str>,
        limit: Option<&str>,
    ) -> PyResult<Vec<PyFill>> {
        let params = GetFillsHistoryParams {
            inst_type: inst_type.to_string(),
            uly: uly.map(String::from),
            inst_family: inst_family.map(String::from),
            inst_id: inst_id.map(String::from),
            ord_id: ord_id.map(String::from),
            after: after.map(String::from),
            before: before.map(String::from),
            limit: limit.map(String::from),
        };

        self.runtime.block_on(async {
            self.client
                .get_fills_history(params)
                .await
                .map(|v| v.into_iter().map(PyFill::from).collect())
                .map_err(to_py_err)
        })
    }

    /// 下算法单。
    #[pyo3(
        signature = (inst_id, td_mode, side, ord_type, sz, ccy=None, pos_side=None, reduce_only=None, tgt_ccy=None, algo_cl_ord_id=None, trigger_px=None, order_px=None, trigger_px_type=None, tp_trigger_px=None, tp_ord_px=None, tp_trigger_px_type=None, sl_trigger_px=None, sl_ord_px=None, sl_trigger_px_type=None, callback_ratio=None, callback_spread=None, active_px=None)
    )]
    fn place_algo_order(
        &self,
        inst_id: &str,
        td_mode: &str,
        side: &str,
        ord_type: &str,
        sz: &str,
        ccy: Option<&str>,
        pos_side: Option<&str>,
        reduce_only: Option<bool>,
        tgt_ccy: Option<&str>,
        algo_cl_ord_id: Option<&str>,
        trigger_px: Option<&str>,
        order_px: Option<&str>,
        trigger_px_type: Option<&str>,
        tp_trigger_px: Option<&str>,
        tp_ord_px: Option<&str>,
        tp_trigger_px_type: Option<&str>,
        sl_trigger_px: Option<&str>,
        sl_ord_px: Option<&str>,
        sl_trigger_px_type: Option<&str>,
        callback_ratio: Option<&str>,
        callback_spread: Option<&str>,
        active_px: Option<&str>,
    ) -> PyResult<Vec<PyPlaceAlgoOrderResult>> {
        let request = PlaceAlgoOrderRequest {
            inst_id: inst_id.to_string(),
            td_mode: td_mode.to_string(),
            side: side.to_string(),
            ord_type: ord_type.to_string(),
            sz: sz.to_string(),
            ccy: ccy.map(String::from),
            pos_side: pos_side.map(String::from),
            reduce_only,
            tgt_ccy: tgt_ccy.map(String::from),
            algo_cl_ord_id: algo_cl_ord_id.map(String::from),
            trigger_px: trigger_px.map(String::from),
            order_px: order_px.map(String::from),
            trigger_px_type: trigger_px_type.map(String::from),
            tp_trigger_px: tp_trigger_px.map(String::from),
            tp_ord_px: tp_ord_px.map(String::from),
            tp_trigger_px_type: tp_trigger_px_type.map(String::from),
            sl_trigger_px: sl_trigger_px.map(String::from),
            sl_ord_px: sl_ord_px.map(String::from),
            sl_trigger_px_type: sl_trigger_px_type.map(String::from),
            callback_ratio: callback_ratio.map(String::from),
            callback_spread: callback_spread.map(String::from),
            active_px: active_px.map(String::from),
        };

        self.runtime.block_on(async {
            self.client
                .place_algo_order(request)
                .await
                .map(|v| v.into_iter().map(PyPlaceAlgoOrderResult::from).collect())
                .map_err(to_py_err)
        })
    }

    /// 批量撤算法单。
    #[pyo3(signature = (requests))]
    fn cancel_algo_orders(
        &self,
        requests: Vec<(String, String)>,
    ) -> PyResult<Vec<PyCancelAlgoOrderResult>> {
        let reqs: Vec<CancelAlgoOrderRequest> = requests
            .into_iter()
            .map(|(inst_id, algo_id)| CancelAlgoOrderRequest { inst_id, algo_id })
            .collect();

        self.runtime.block_on(async {
            self.client
                .cancel_algo_orders(reqs)
                .await
                .map(|v| v.into_iter().map(PyCancelAlgoOrderResult::from).collect())
                .map_err(to_py_err)
        })
    }

    /// 修改算法单。
    #[pyo3(
        signature = (inst_id=None, algo_id=None, algo_cl_ord_id=None, cxl_on_fail=None, req_id=None, new_sz=None, new_tp_trigger_px=None, new_tp_ord_px=None, new_sl_trigger_px=None, new_sl_ord_px=None, new_tp_trigger_px_type=None, new_sl_trigger_px_type=None)
    )]
    fn amend_algo_order(
        &self,
        inst_id: Option<&str>,
        algo_id: Option<&str>,
        algo_cl_ord_id: Option<&str>,
        cxl_on_fail: Option<&str>,
        req_id: Option<&str>,
        new_sz: Option<&str>,
        new_tp_trigger_px: Option<&str>,
        new_tp_ord_px: Option<&str>,
        new_sl_trigger_px: Option<&str>,
        new_sl_ord_px: Option<&str>,
        new_tp_trigger_px_type: Option<&str>,
        new_sl_trigger_px_type: Option<&str>,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let request = AmendAlgoOrderRequest {
            inst_id: inst_id.map(String::from),
            algo_id: algo_id.map(String::from),
            algo_cl_ord_id: algo_cl_ord_id.map(String::from),
            cxl_on_fail: cxl_on_fail.map(String::from),
            req_id: req_id.map(String::from),
            new_sz: new_sz.map(String::from),
            new_tp_trigger_px: new_tp_trigger_px.map(String::from),
            new_tp_ord_px: new_tp_ord_px.map(String::from),
            new_sl_trigger_px: new_sl_trigger_px.map(String::from),
            new_sl_ord_px: new_sl_ord_px.map(String::from),
            new_tp_trigger_px_type: new_tp_trigger_px_type.map(String::from),
            new_sl_trigger_px_type: new_sl_trigger_px_type.map(String::from),
        };

        self.runtime.block_on(async {
            self.client
                .amend_algo_order(request)
                .await
                .map_err(to_py_err)
                .and_then(values_to_py_list)
        })
    }

    /// 查询在途算法单。
    #[pyo3(signature = (ord_type, algo_id=None, inst_type=None, inst_id=None, after=None, before=None, limit=None))]
    fn get_algo_orders_pending(
        &self,
        ord_type: &str,
        algo_id: Option<&str>,
        inst_type: Option<&str>,
        inst_id: Option<&str>,
        after: Option<&str>,
        before: Option<&str>,
        limit: Option<&str>,
    ) -> PyResult<Vec<PyAlgoOrder>> {
        let params = GetAlgoOrdersParams {
            ord_type: ord_type.to_string(),
            algo_id: algo_id.map(String::from),
            inst_type: inst_type.map(String::from),
            inst_id: inst_id.map(String::from),
            after: after.map(String::from),
            before: before.map(String::from),
            limit: limit.map(String::from),
        };

        self.runtime.block_on(async {
            self.client
                .get_algo_orders_pending(params)
                .await
                .map(|v| v.into_iter().map(PyAlgoOrder::from).collect())
                .map_err(to_py_err)
        })
    }

    /// 查询历史算法单。
    #[pyo3(signature = (ord_type, state=None, algo_id=None, inst_type=None, inst_id=None, after=None, before=None, limit=None))]
    fn get_algo_orders_history(
        &self,
        ord_type: &str,
        state: Option<&str>,
        algo_id: Option<&str>,
        inst_type: Option<&str>,
        inst_id: Option<&str>,
        after: Option<&str>,
        before: Option<&str>,
        limit: Option<&str>,
    ) -> PyResult<Vec<PyAlgoOrder>> {
        let params = GetAlgoOrdersHistoryParams {
            ord_type: ord_type.to_string(),
            state: state.map(String::from),
            algo_id: algo_id.map(String::from),
            inst_type: inst_type.map(String::from),
            inst_id: inst_id.map(String::from),
            after: after.map(String::from),
            before: before.map(String::from),
            limit: limit.map(String::from),
        };

        self.runtime.block_on(async {
            self.client
                .get_algo_orders_history(params)
                .await
                .map(|v| v.into_iter().map(PyAlgoOrder::from).collect())
                .map_err(to_py_err)
        })
    }

    /// 获取算法单详情。
    #[pyo3(signature = (algo_id=None, algo_cl_ord_id=None))]
    fn get_algo_order_details(
        &self,
        algo_id: Option<&str>,
        algo_cl_ord_id: Option<&str>,
    ) -> PyResult<Vec<PyAlgoOrder>> {
        let params = GetAlgoOrderDetailsParams {
            algo_id: algo_id.map(String::from),
            algo_cl_ord_id: algo_cl_ord_id.map(String::from),
        };

        self.runtime.block_on(async {
            self.client
                .get_algo_order_details(params)
                .await
                .map(|v| v.into_iter().map(PyAlgoOrder::from).collect())
                .map_err(to_py_err)
        })
    }

    /// 一键平仓。
    #[pyo3(signature = (inst_id, mgn_mode, pos_side=None, ccy=None, auto_cancel=None, cl_ord_id=None, tag=None))]
    fn close_position(
        &self,
        inst_id: &str,
        mgn_mode: &str,
        pos_side: Option<&str>,
        ccy: Option<&str>,
        auto_cancel: Option<bool>,
        cl_ord_id: Option<&str>,
        tag: Option<&str>,
    ) -> PyResult<Option<PyClosePositionResult>> {
        let request = ClosePositionRequest {
            inst_id: inst_id.to_string(),
            mgn_mode: mgn_mode.to_string(),
            pos_side: pos_side.map(String::from),
            ccy: ccy.map(String::from),
            auto_cancel,
            cl_ord_id: cl_ord_id.map(String::from),
            tag: tag.map(String::from),
        };

        self.runtime.block_on(async {
            self.client
                .close_position(request)
                .await
                .map(|mut v| v.pop().map(PyClosePositionResult::from))
                .map_err(to_py_err)
        })
    }

    // ==================== Funding API ====================

    /// 查询资金账户余额。
    #[pyo3(signature = (ccy=None))]
    fn get_asset_balances(&self, ccy: Option<&str>) -> PyResult<Vec<PyAssetBalance>> {
        self.runtime.block_on(async {
            self.client
                .get_asset_balances(ccy)
                .await
                .map(|v| v.into_iter().map(PyAssetBalance::from).collect())
                .map_err(to_py_err)
        })
    }

    /// 资金划转。
    #[pyo3(
        signature = (ccy, amt, from_account, to_account, transfer_type=None, sub_acct=None, inst_id=None, to_inst_id=None, loan_trans=None)
    )]
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

        self.runtime.block_on(async {
            self.client
                .funds_transfer(request)
                .await
                .map(|mut v| v.pop().map(PyFundsTransferResult::from))
                .map_err(to_py_err)
        })
    }

    /// 提现。
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

        self.runtime.block_on(async {
            self.client
                .withdrawal(request)
                .await
                .map(|mut v| v.pop().map(PyWithdrawalResult::from))
                .map_err(to_py_err)
        })
    }

    /// 查询充值地址。
    fn get_deposit_address(&self, ccy: &str) -> PyResult<Vec<PyDepositAddress>> {
        self.runtime.block_on(async {
            self.client
                .get_deposit_address(ccy)
                .await
                .map(|v| v.into_iter().map(PyDepositAddress::from).collect())
                .map_err(to_py_err)
        })
    }

    /// 查询充值记录。
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

        self.runtime.block_on(async {
            self.client
                .get_deposit_history(Some(params))
                .await
                .map(|v| v.into_iter().map(PyDepositRecord::from).collect())
                .map_err(to_py_err)
        })
    }

    /// 查询提现记录。
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

        self.runtime.block_on(async {
            self.client
                .get_withdrawal_history(Some(params))
                .await
                .map(|v| v.into_iter().map(PyWithdrawalRecord::from).collect())
                .map_err(to_py_err)
        })
    }

    /// 查询币种信息。
    #[pyo3(signature = (ccy=None))]
    fn get_currencies(&self, ccy: Option<&str>) -> PyResult<Vec<PyCurrencyInfo>> {
        self.runtime.block_on(async {
            self.client
                .get_currencies(ccy)
                .await
                .map(|v| v.into_iter().map(PyCurrencyInfo::from).collect())
                .map_err(to_py_err)
        })
    }

    /// 查询非交易资产。
    #[pyo3(signature = (ccy=None))]
    fn get_non_tradable_assets(&self, ccy: Option<&str>) -> PyResult<Vec<Py<PyAny>>> {
        self.runtime.block_on(async {
            self.client
                .get_non_tradable_assets(ccy)
                .await
                .map_err(to_py_err)
                .and_then(values_to_py_list)
        })
    }

    /// 查询资产估值。
    #[pyo3(signature = (ccy=None))]
    fn get_asset_valuation(&self, ccy: Option<&str>) -> PyResult<Vec<Py<PyAny>>> {
        let params = if ccy.is_some() {
            Some(GetAssetValuationParams {
                ccy: ccy.map(String::from),
            })
        } else {
            None
        };

        self.runtime.block_on(async {
            self.client
                .get_asset_valuation(params)
                .await
                .map_err(to_py_err)
                .and_then(values_to_py_list)
        })
    }

    /// 查询划转状态。
    fn get_transfer_state(&self, trans_id: &str, type_: Option<&str>) -> PyResult<Vec<Py<PyAny>>> {
        let params = GetTransferStateParams {
            trans_id: trans_id.to_string(),
            r#type: type_.map(String::from),
        };

        self.runtime.block_on(async {
            self.client
                .get_transfer_state(params)
                .await
                .map_err(to_py_err)
                .and_then(values_to_py_list)
        })
    }

    /// 查询资金流水（资金账户）。
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

        self.runtime.block_on(async {
            self.client
                .get_funding_bills(params)
                .await
                .map_err(to_py_err)
                .and_then(values_to_py_list)
        })
    }

    /// 申购/赎回余币宝。
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

        self.runtime.block_on(async {
            self.client
                .purchase_redempt(request)
                .await
                .map_err(to_py_err)
                .and_then(values_to_py_list)
        })
    }

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

        self.runtime.block_on(async {
            self.client
                .get_deposit_lightning(params)
                .await
                .map_err(to_py_err)
                .and_then(values_to_py_list)
        })
    }

    /// 闪电网络提现。
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

        self.runtime.block_on(async {
            self.client
                .withdrawal_lightning(request)
                .await
                .map_err(to_py_err)
                .and_then(values_to_py_list)
        })
    }

    /// 取消提现。
    #[pyo3(signature = (wd_id=None))]
    fn cancel_withdrawal(&self, wd_id: Option<&str>) -> PyResult<Vec<Py<PyAny>>> {
        let params = CancelWithdrawalParams {
            wd_id: wd_id.map(String::from),
        };

        self.runtime.block_on(async {
            self.client
                .cancel_withdrawal(params)
                .await
                .map_err(to_py_err)
                .and_then(values_to_py_list)
        })
    }

    /// 查询充值/提现状态。
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

        self.runtime.block_on(async {
            self.client
                .get_deposit_withdraw_status(params)
                .await
                .map_err(to_py_err)
                .and_then(values_to_py_list)
        })
    }

    /// 设置借贷利率。
    fn set_lending_rate(&self, ccy: &str, rate: &str) -> PyResult<Vec<Py<PyAny>>> {
        let request = SetLendingRateRequest {
            ccy: ccy.to_string(),
            rate: rate.to_string(),
        };

        self.runtime.block_on(async {
            self.client
                .set_lending_rate(request)
                .await
                .map_err(to_py_err)
                .and_then(values_to_py_list)
        })
    }

    /// 借贷历史。
    #[pyo3(signature = (ccy=None, before=None, after=None, limit=None))]
    fn get_lending_history(
        &self,
        ccy: Option<&str>,
        before: Option<&str>,
        after: Option<&str>,
        limit: Option<&str>,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let params = if ccy.is_some() || before.is_some() || after.is_some() || limit.is_some() {
            Some(GetLendingHistoryParams {
                ccy: ccy.map(String::from),
                before: before.map(String::from),
                after: after.map(String::from),
                limit: limit.map(String::from),
            })
        } else {
            None
        };

        self.runtime.block_on(async {
            self.client
                .get_lending_history(params)
                .await
                .map_err(to_py_err)
                .and_then(values_to_py_list)
        })
    }

    /// 借贷利率历史。
    #[pyo3(signature = (ccy=None, after=None, before=None, limit=None))]
    fn get_lending_rate_history(
        &self,
        ccy: Option<&str>,
        after: Option<&str>,
        before: Option<&str>,
        limit: Option<&str>,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let params = if ccy.is_some() || after.is_some() || before.is_some() || limit.is_some() {
            Some(GetLendingRateHistoryParams {
                ccy: ccy.map(String::from),
                after: after.map(String::from),
                before: before.map(String::from),
                limit: limit.map(String::from),
            })
        } else {
            None
        };

        self.runtime.block_on(async {
            self.client
                .get_lending_rate_history(params)
                .await
                .map_err(to_py_err)
                .and_then(values_to_py_list)
        })
    }

    /// 借贷利率汇总。
    #[pyo3(signature = (ccy=None))]
    fn get_lending_rate_summary(&self, ccy: Option<&str>) -> PyResult<Vec<Py<PyAny>>> {
        let params = if ccy.is_some() {
            Some(GetLendingRateSummaryParams {
                ccy: ccy.map(String::from),
            })
        } else {
            None
        };

        self.runtime.block_on(async {
            self.client
                .get_lending_rate_summary(params)
                .await
                .map_err(to_py_err)
                .and_then(values_to_py_list)
        })
    }

    /// 小额资产兑换。
    #[pyo3(signature = (ccy=None))]
    fn convert_dust_assets(&self, ccy: Option<Vec<String>>) -> PyResult<Vec<Py<PyAny>>> {
        let request = ConvertDustAssetsRequest { ccy };

        self.runtime.block_on(async {
            self.client
                .convert_dust_assets(request)
                .await
                .map_err(to_py_err)
                .and_then(values_to_py_list)
        })
    }

    /// 余币宝余额。
    #[pyo3(signature = (ccy=None))]
    fn get_saving_balance(&self, ccy: Option<&str>) -> PyResult<Vec<Py<PyAny>>> {
        let params = if ccy.is_some() {
            Some(GetSavingBalanceParams {
                ccy: ccy.map(String::from),
            })
        } else {
            None
        };

        self.runtime.block_on(async {
            self.client
                .get_saving_balance(params)
                .await
                .map_err(to_py_err)
                .and_then(values_to_py_list)
        })
    }

    // ==================== Market API ====================

    /// 查询单个交易对行情。
    fn get_ticker(&self, inst_id: &str) -> PyResult<Option<PyTicker>> {
        self.runtime.block_on(async {
            self.client
                .get_ticker(inst_id)
                .await
                .map(|v| v.into_iter().next().map(PyTicker::from))
                .map_err(to_py_err)
        })
    }

    /// 查询指定品种的全部行情。
    fn get_tickers(&self, inst_type: &str) -> PyResult<Vec<PyTicker>> {
        let params = GetTickersParams {
            inst_type: inst_type.to_string(),
            uly: None,
            inst_family: None,
        };

        self.runtime.block_on(async {
            self.client
                .get_tickers(params)
                .await
                .map(|v| v.into_iter().map(PyTicker::from).collect())
                .map_err(to_py_err)
        })
    }

    /// 查询订单簿。
    #[pyo3(signature = (inst_id, sz=None))]
    fn get_orderbook(&self, inst_id: &str, sz: Option<&str>) -> PyResult<Option<PyOrderBook>> {
        let depth = sz.and_then(|v| v.parse::<u32>().ok());

        self.runtime.block_on(async {
            self.client
                .get_orderbook(inst_id, depth)
                .await
                .map(|mut v| v.pop().map(PyOrderBook::from))
                .map_err(to_py_err)
        })
    }

    /// 查询 K 线。
    #[pyo3(signature = (inst_id, bar=None, after=None, before=None, limit=None))]
    fn get_candles(
        &self,
        inst_id: &str,
        bar: Option<&str>,
        after: Option<&str>,
        before: Option<&str>,
        limit: Option<&str>,
    ) -> PyResult<Vec<PyCandle>> {
        let params = GetCandlesParams {
            inst_id: inst_id.to_string(),
            bar: bar.map(String::from),
            after: after.map(String::from),
            before: before.map(String::from),
            limit: limit.map(String::from),
        };

        self.runtime.block_on(async {
            self.client
                .get_candles(params)
                .await
                .map(|v| {
                    v.into_iter()
                        .filter_map(|arr| okx_core::types::Candle::from_array(&arr))
                        .map(PyCandle::from)
                        .collect()
                })
                .map_err(to_py_err)
        })
    }

    /// 查询历史 K 线。
    #[pyo3(signature = (inst_id, bar=None, after=None, before=None, limit=None))]
    fn get_history_candles(
        &self,
        inst_id: &str,
        bar: Option<&str>,
        after: Option<&str>,
        before: Option<&str>,
        limit: Option<&str>,
    ) -> PyResult<Vec<PyCandle>> {
        let params = GetCandlesParams {
            inst_id: inst_id.to_string(),
            bar: bar.map(String::from),
            after: after.map(String::from),
            before: before.map(String::from),
            limit: limit.map(String::from),
        };

        self.runtime.block_on(async {
            self.client
                .get_history_candles(params)
                .await
                .map(|v| {
                    v.into_iter()
                        .filter_map(|arr| okx_core::types::Candle::from_array(&arr))
                        .map(PyCandle::from)
                        .collect()
                })
                .map_err(to_py_err)
        })
    }

    /// 查询指数 K 线。
    #[pyo3(signature = (inst_id, bar=None, after=None, before=None, limit=None))]
    fn get_index_candles(
        &self,
        inst_id: &str,
        bar: Option<&str>,
        after: Option<&str>,
        before: Option<&str>,
        limit: Option<&str>,
    ) -> PyResult<Vec<PyCandle>> {
        let params = GetIndexCandlesParams {
            inst_id: inst_id.to_string(),
            bar: bar.map(String::from),
            after: after.map(String::from),
            before: before.map(String::from),
            limit: limit.map(String::from),
        };

        self.runtime.block_on(async {
            self.client
                .get_index_candles(params)
                .await
                .map(|v| {
                    v.into_iter()
                        .filter_map(|arr| okx_core::types::Candle::from_array(&arr))
                        .map(PyCandle::from)
                        .collect()
                })
                .map_err(to_py_err)
        })
    }

    /// 查询标记价格 K 线。
    #[pyo3(signature = (inst_id, bar=None, after=None, before=None, limit=None))]
    fn get_mark_price_candles(
        &self,
        inst_id: &str,
        bar: Option<&str>,
        after: Option<&str>,
        before: Option<&str>,
        limit: Option<&str>,
    ) -> PyResult<Vec<PyCandle>> {
        let params = GetMarkPriceCandlesParams {
            inst_id: inst_id.to_string(),
            bar: bar.map(String::from),
            after: after.map(String::from),
            before: before.map(String::from),
            limit: limit.map(String::from),
        };

        self.runtime.block_on(async {
            self.client
                .get_mark_price_candles(params)
                .await
                .map(|v| {
                    v.into_iter()
                        .filter_map(|arr| okx_core::types::Candle::from_array(&arr))
                        .map(PyCandle::from)
                        .collect()
                })
                .map_err(to_py_err)
        })
    }

    /// 查询最新成交。
    #[pyo3(signature = (inst_id, limit=None))]
    fn get_trades(&self, inst_id: &str, limit: Option<&str>) -> PyResult<Vec<PyPublicTrade>> {
        let parsed_limit = limit.and_then(|v| v.parse::<u32>().ok());

        self.runtime.block_on(async {
            self.client
                .get_trades(inst_id, parsed_limit)
                .await
                .map(|v| v.into_iter().map(PyPublicTrade::from).collect())
                .map_err(to_py_err)
        })
    }

    /// 查询历史成交。
    #[pyo3(signature = (inst_id, after=None, before=None, limit=None, type_=None))]
    fn get_history_trades(
        &self,
        inst_id: &str,
        after: Option<&str>,
        before: Option<&str>,
        limit: Option<&str>,
        type_: Option<&str>,
    ) -> PyResult<Vec<PyPublicTrade>> {
        let params = GetHistoryTradesParams {
            inst_id: inst_id.to_string(),
            after: after.map(String::from),
            before: before.map(String::from),
            limit: limit.map(String::from),
            r#type: type_.map(String::from),
        };

        self.runtime.block_on(async {
            self.client
                .get_history_trades(params)
                .await
                .map(|v| v.into_iter().map(PyPublicTrade::from).collect())
                .map_err(to_py_err)
        })
    }

    /// 轻量深度。
    fn get_orderbook_lite(&self, inst_id: &str) -> PyResult<Option<PyOrderBook>> {
        self.runtime.block_on(async {
            self.client
                .get_orderbook_lite(inst_id)
                .await
                .map(|mut v| v.pop().map(PyOrderBook::from))
                .map_err(to_py_err)
        })
    }

    /// 块交易行情（单个）。
    fn get_block_ticker(&self, inst_id: &str) -> PyResult<Vec<Py<PyAny>>> {
        self.runtime.block_on(async {
            self.client
                .get_block_ticker(inst_id)
                .await
                .map_err(to_py_err)
                .and_then(values_to_py_list)
        })
    }

    /// 块交易行情（列表）。
    fn get_block_tickers(
        &self,
        inst_type: &str,
        uly: Option<&str>,
        inst_family: Option<&str>,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let params = GetBlockTickersParams {
            inst_type: inst_type.to_string(),
            uly: uly.map(String::from),
            inst_family: inst_family.map(String::from),
        };

        self.runtime.block_on(async {
            self.client
                .get_block_tickers(params)
                .await
                .map_err(to_py_err)
                .and_then(values_to_py_list)
        })
    }

    /// 块交易成交。
    fn get_block_trades(&self, inst_id: &str) -> PyResult<Vec<Py<PyAny>>> {
        self.runtime.block_on(async {
            self.client
                .get_block_trades(inst_id)
                .await
                .map_err(to_py_err)
                .and_then(values_to_py_list)
        })
    }

    /// 期权系列成交。
    fn get_option_family_trades(&self, inst_family: &str) -> PyResult<Vec<Py<PyAny>>> {
        self.runtime.block_on(async {
            self.client
                .get_option_family_trades(inst_family)
                .await
                .map_err(to_py_err)
                .and_then(values_to_py_list)
        })
    }

    /// 查询指数行情。
    #[pyo3(signature = (quote_ccy=None, inst_id=None))]
    fn get_index_tickers(
        &self,
        quote_ccy: Option<&str>,
        inst_id: Option<&str>,
    ) -> PyResult<Vec<PyIndexTicker>> {
        let params = GetIndexTickersParams {
            quote_ccy: quote_ccy.map(String::from),
            inst_id: inst_id.map(String::from),
        };

        self.runtime.block_on(async {
            self.client
                .get_index_tickers(params)
                .await
                .map(|v| v.into_iter().map(PyIndexTicker::from).collect())
                .map_err(to_py_err)
        })
    }

    // ==================== Public API ====================

    /// 查询合约/币对列表。
    #[pyo3(signature = (inst_type, inst_id=None))]
    fn get_instruments(&self, inst_type: &str, inst_id: Option<&str>) -> PyResult<Vec<Py<PyAny>>> {
        let params = GetInstrumentsParams {
            inst_type: inst_type.to_string(),
            uly: None,
            inst_family: None,
            inst_id: inst_id.map(String::from),
        };

        self.runtime
            .block_on(async { self.client.get_instruments(params).await.map_err(to_py_err) })
            .and_then(|instruments| {
                Python::attach(|py| {
                    instruments
                        .into_iter()
                        .map(|inst| {
                            let dict = pyo3::types::PyDict::new(py);
                            dict.set_item("instId", &inst.inst_id)?;
                            dict.set_item("instType", &inst.inst_type)?;
                            dict.set_item("baseCcy", &inst.base_ccy)?;
                            dict.set_item("quoteCcy", &inst.quote_ccy)?;
                            dict.set_item("tickSz", &inst.tick_sz)?;
                            dict.set_item("lotSz", &inst.lot_sz)?;
                            dict.set_item("minSz", &inst.min_sz)?;
                            dict.set_item("state", &inst.state)?;
                            Ok(dict.unbind().into())
                        })
                        .collect::<PyResult<Vec<Py<PyAny>>>>()
                })
            })
    }

    /// 查询交割/行权历史。
    #[pyo3(signature = (inst_type, uly=None, inst_family=None, after=None, before=None, limit=None))]
    fn get_delivery_exercise_history(
        &self,
        inst_type: &str,
        uly: Option<&str>,
        inst_family: Option<&str>,
        after: Option<&str>,
        before: Option<&str>,
        limit: Option<&str>,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let params = GetDeliveryExerciseHistoryParams {
            inst_type: inst_type.to_string(),
            uly: uly.map(String::from),
            inst_family: inst_family.map(String::from),
            after: after.map(String::from),
            before: before.map(String::from),
            limit: limit.map(String::from),
        };

        self.runtime.block_on(async {
            self.client
                .get_delivery_exercise_history(params)
                .await
                .map_err(to_py_err)
                .and_then(values_to_py_list)
        })
    }

    /// 查询持仓总量。
    #[pyo3(signature = (inst_type, uly=None, inst_id=None, inst_family=None))]
    fn get_open_interest(
        &self,
        inst_type: &str,
        uly: Option<&str>,
        inst_id: Option<&str>,
        inst_family: Option<&str>,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let params = GetOpenInterestParams {
            inst_type: inst_type.to_string(),
            uly: uly.map(String::from),
            inst_id: inst_id.map(String::from),
            inst_family: inst_family.map(String::from),
        };

        self.runtime.block_on(async {
            self.client
                .get_open_interest(params)
                .await
                .map_err(to_py_err)
                .and_then(values_to_py_list)
        })
    }

    /// 查询杠杆分层（公共）。
    #[pyo3(signature = (inst_type, td_mode, uly=None, inst_id=None, ccy=None, tier=None, inst_family=None))]
    fn get_position_tiers(
        &self,
        inst_type: &str,
        td_mode: &str,
        uly: Option<&str>,
        inst_id: Option<&str>,
        ccy: Option<&str>,
        tier: Option<&str>,
        inst_family: Option<&str>,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let params = GetPositionTiersParams {
            inst_type: inst_type.to_string(),
            td_mode: td_mode.to_string(),
            uly: uly.map(String::from),
            inst_id: inst_id.map(String::from),
            ccy: ccy.map(String::from),
            tier: tier.map(String::from),
            inst_family: inst_family.map(String::from),
        };

        self.runtime.block_on(async {
            self.client
                .get_position_tiers(params)
                .await
                .map_err(to_py_err)
                .and_then(values_to_py_list)
        })
    }

    /// 查询资金费率。
    fn get_funding_rate(&self, inst_id: &str) -> PyResult<Option<PyFundingRate>> {
        self.runtime.block_on(async {
            self.client
                .get_funding_rate(inst_id)
                .await
                .map(|v| v.into_iter().next().map(PyFundingRate::from))
                .map_err(to_py_err)
        })
    }

    /// 查询资金费率历史。
    #[pyo3(signature = (inst_id, after=None, before=None, limit=None))]
    fn get_funding_rate_history(
        &self,
        inst_id: &str,
        after: Option<&str>,
        before: Option<&str>,
        limit: Option<&str>,
    ) -> PyResult<Vec<PyFundingRate>> {
        let params = GetFundingRateHistoryParams {
            inst_id: inst_id.to_string(),
            after: after.map(String::from),
            before: before.map(String::from),
            limit: limit.map(String::from),
        };

        self.runtime.block_on(async {
            self.client
                .get_funding_rate_history(params)
                .await
                .map(|v| v.into_iter().map(PyFundingRate::from).collect())
                .map_err(to_py_err)
        })
    }

    /// 查询标记价格。
    #[pyo3(signature = (inst_type, uly=None, inst_family=None, inst_id=None))]
    fn get_mark_price(
        &self,
        inst_type: &str,
        uly: Option<&str>,
        inst_family: Option<&str>,
        inst_id: Option<&str>,
    ) -> PyResult<Vec<PyMarkPrice>> {
        let params = GetMarkPriceParams {
            inst_type: inst_type.to_string(),
            uly: uly.map(String::from),
            inst_family: inst_family.map(String::from),
            inst_id: inst_id.map(String::from),
        };

        self.runtime.block_on(async {
            self.client
                .get_mark_price(params)
                .await
                .map(|v| v.into_iter().map(PyMarkPrice::from).collect())
                .map_err(to_py_err)
        })
    }

    /// 获取服务器时间戳（毫秒）。
    fn get_system_time(&self) -> PyResult<String> {
        self.runtime.block_on(async {
            self.client
                .get_system_time()
                .await
                .map(|v| v.first().map(|t| t.ts.clone()).unwrap_or_default())
                .map_err(to_py_err)
        })
    }

    fn __repr__(&self) -> String {
        format!(
            "OkxClient(simulated={})",
            self.client.config().is_simulated()
        )
    }
}
