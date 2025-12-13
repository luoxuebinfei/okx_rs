//! Account 域绑定的同步/异步共享实现。

use pyo3::prelude::*;

use okx_rest::api::account::{
    AdjustmentMarginRequest, AmendFixLoanBorrowingOrderRequest, BorrowRepayHistoryParams,
    BorrowRepayRequest, FixLoanBorrowingOrderRequest, FixLoanManualReborrowRequest,
    GetAccountPositionTiersParams, GetBillsArchiveParams, GetBillsParams, GetFeeRatesParams,
    GetFixLoanBorrowingOrdersListParams, GetFixLoanBorrowingQuoteParams, GetGreeksParams,
    GetInterestAccruedParams, GetInterestLimitsParams, GetLeverageInfoParams,
    GetMaxAvailSizeParams, GetMaxLoanParams, GetMaxSizeParams, GetPositionsHistoryParams,
    GetSimulatedMarginParams, GetVipInterestParams, GetVipLoanOrderDetailParams,
    GetVipLoanOrderListParams, PositionBuilderRequest, RepayFixLoanBorrowingOrderRequest,
    SetAccountLevelRequest, SetAutoLoanRequest, SetAutoRepayRequest, SetGreeksRequest,
    SetIsolatedModeRequest, SetLeverageRequest, SetRiskOffsetTypeRequest,
    SpotBorrowRepayHistoryParams, SpotManualBorrowRepayRequest,
};
use okx_rest::AccountApi;

use crate::types::{
    PyAccountConfig, PyAccountPositionRisk, PyBalance, PyFeeRates, PyLeverageInfo, PyMaxAvailSize,
    PyMaxSize, PyPosition, PySetLeverageResult, PySetPositionModeResult,
};
use crate::{
    map_values, parse_json_array, parse_json_value, to_py_err, PyAsyncOkxClient, PyOkxClient,
};

pub(crate) mod sync {
    use super::*;

    pub(crate) fn get_balance(client: &PyOkxClient, ccy: Option<&str>) -> PyResult<Vec<PyBalance>> {
        client.block_on_allow_threads(async {
            client
                .rest_client()
                .get_balance(ccy)
                .await
                .map(|v| v.into_iter().map(PyBalance::from).collect())
        })
    }

    pub(crate) fn get_positions(
        client: &PyOkxClient,
        inst_type: Option<&str>,
        inst_id: Option<&str>,
    ) -> PyResult<Vec<PyPosition>> {
        let params = if inst_type.is_some() || inst_id.is_some() {
            Some(okx_rest::api::account::GetPositionsParams {
                inst_type: inst_type.map(String::from),
                inst_id: inst_id.map(String::from),
                pos_id: None,
            })
        } else {
            None
        };

        client.block_on_allow_threads(async {
            client
                .rest_client()
                .get_positions(params)
                .await
                .map(|v| v.into_iter().map(PyPosition::from).collect())
        })
    }

    pub(crate) fn get_account_config(client: &PyOkxClient) -> PyResult<Vec<PyAccountConfig>> {
        client.block_on_allow_threads(async {
            client
                .rest_client()
                .get_account_config()
                .await
                .map(|v| v.into_iter().map(PyAccountConfig::from).collect())
        })
    }

    pub(crate) fn set_leverage(
        client: &PyOkxClient,
        request: SetLeverageRequest,
    ) -> PyResult<Option<PySetLeverageResult>> {
        client.block_on_allow_threads(async {
            client
                .rest_client()
                .set_leverage(request)
                .await
                .map(|mut v| v.pop().map(PySetLeverageResult::from))
        })
    }

    pub(crate) fn get_leverage_info(
        client: &PyOkxClient,
        params: GetLeverageInfoParams,
    ) -> PyResult<Vec<PyLeverageInfo>> {
        client.block_on_allow_threads(async {
            client
                .rest_client()
                .get_leverage_info(params)
                .await
                .map(|v| v.into_iter().map(PyLeverageInfo::from).collect())
        })
    }

    pub(crate) fn get_max_size(
        client: &PyOkxClient,
        params: GetMaxSizeParams,
    ) -> PyResult<Vec<PyMaxSize>> {
        client.block_on_allow_threads(async {
            client
                .rest_client()
                .get_max_size(params)
                .await
                .map(|v| v.into_iter().map(PyMaxSize::from).collect())
        })
    }

    pub(crate) fn get_max_avail_size(
        client: &PyOkxClient,
        params: GetMaxAvailSizeParams,
    ) -> PyResult<Vec<PyMaxAvailSize>> {
        client.block_on_allow_threads(async {
            client
                .rest_client()
                .get_max_avail_size(params)
                .await
                .map(|v| v.into_iter().map(PyMaxAvailSize::from).collect())
        })
    }

    pub(crate) fn get_max_loan(
        client: &PyOkxClient,
        params: GetMaxLoanParams,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let res = client
            .block_on_allow_threads(async { client.rest_client().get_max_loan(params).await })?;
        map_values(Ok(res))
    }

    pub(crate) fn get_fee_rates(
        client: &PyOkxClient,
        params: GetFeeRatesParams,
    ) -> PyResult<Vec<PyFeeRates>> {
        client.block_on_allow_threads(async {
            client
                .rest_client()
                .get_fee_rates(params)
                .await
                .map(|v| v.into_iter().map(PyFeeRates::from).collect())
        })
    }

    pub(crate) fn get_max_withdrawal(
        client: &PyOkxClient,
        ccy: Option<&str>,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let res = client
            .block_on_allow_threads(async { client.rest_client().get_max_withdrawal(ccy).await })?;
        map_values(Ok(res))
    }

    pub(crate) fn get_interest_rate(
        client: &PyOkxClient,
        ccy: Option<&str>,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let res = client
            .block_on_allow_threads(async { client.rest_client().get_interest_rate(ccy).await })?;
        map_values(Ok(res))
    }

    pub(crate) fn set_position_mode(
        client: &PyOkxClient,
        pos_mode: &str,
    ) -> PyResult<Option<PySetPositionModeResult>> {
        client.block_on_allow_threads(async {
            client
                .rest_client()
                .set_position_mode(pos_mode)
                .await
                .map(|mut v| v.pop().map(PySetPositionModeResult::from))
        })
    }

    pub(crate) fn get_account_position_risk(
        client: &PyOkxClient,
    ) -> PyResult<Vec<PyAccountPositionRisk>> {
        client.block_on_allow_threads(async {
            client
                .rest_client()
                .get_account_position_risk()
                .await
                .map(|v| {
                    v.into_iter()
                        .map(PyAccountPositionRisk::from)
                        .collect::<Vec<_>>()
                })
        })
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn get_positions_history(
        client: &PyOkxClient,
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

        let res = client.block_on_allow_threads(async {
            client.rest_client().get_positions_history(params).await
        })?;
        map_values(Ok(res))
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn get_account_bills(
        client: &PyOkxClient,
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

        let res = client.block_on_allow_threads(async {
            client.rest_client().get_account_bills(params).await
        })?;
        map_values(Ok(res))
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn get_account_bills_archive(
        client: &PyOkxClient,
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

        let res = client.block_on_allow_threads(async {
            client.rest_client().get_account_bills_archive(params).await
        })?;
        map_values(Ok(res))
    }

    pub(crate) fn set_greeks(client: &PyOkxClient, greeks_type: &str) -> PyResult<Vec<Py<PyAny>>> {
        let request = SetGreeksRequest {
            greeks_type: greeks_type.to_string(),
        };
        let res = client
            .block_on_allow_threads(async { client.rest_client().set_greeks(request).await })?;
        map_values(Ok(res))
    }

    pub(crate) fn set_isolated_mode(
        client: &PyOkxClient,
        iso_mode: &str,
        type_: &str,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let request = SetIsolatedModeRequest {
            iso_mode: iso_mode.to_string(),
            r#type: type_.to_string(),
        };
        let res = client.block_on_allow_threads(async {
            client.rest_client().set_isolated_mode(request).await
        })?;
        map_values(Ok(res))
    }

    pub(crate) fn adjustment_margin(
        client: &PyOkxClient,
        inst_id: &str,
        pos_side: &str,
        type_: &str,
        amt: &str,
        loan_trans: Option<bool>,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let request = AdjustmentMarginRequest {
            inst_id: inst_id.to_string(),
            pos_side: pos_side.to_string(),
            r#type: type_.to_string(),
            amt: amt.to_string(),
            loan_trans,
        };
        let res = client.block_on_allow_threads(async {
            client.rest_client().adjustment_margin(request).await
        })?;
        map_values(Ok(res))
    }

    pub(crate) fn set_risk_offset_type(
        client: &PyOkxClient,
        type_: &str,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let request = SetRiskOffsetTypeRequest {
            r#type: type_.to_string(),
        };
        let res = client.block_on_allow_threads(async {
            client.rest_client().set_risk_offset_type(request).await
        })?;
        map_values(Ok(res))
    }

    pub(crate) fn set_auto_loan(
        client: &PyOkxClient,
        auto_loan: Option<&str>,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let request = SetAutoLoanRequest {
            auto_loan: auto_loan.map(String::from),
        };
        let res = client
            .block_on_allow_threads(async { client.rest_client().set_auto_loan(request).await })?;
        map_values(Ok(res))
    }

    pub(crate) fn activate_option(client: &PyOkxClient) -> PyResult<Vec<Py<PyAny>>> {
        let res = client
            .block_on_allow_threads(async { client.rest_client().activate_option().await })?;
        map_values(Ok(res))
    }

    pub(crate) fn set_auto_repay(
        client: &PyOkxClient,
        auto_repay: Option<bool>,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let request = SetAutoRepayRequest { auto_repay };
        let res = client
            .block_on_allow_threads(async { client.rest_client().set_auto_repay(request).await })?;
        map_values(Ok(res))
    }

    pub(crate) fn get_interest_limits(
        client: &PyOkxClient,
        type_: Option<&str>,
        ccy: Option<&str>,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let params = if type_.is_some() || ccy.is_some() {
            Some(GetInterestLimitsParams {
                r#type: type_.map(String::from),
                ccy: ccy.map(String::from),
            })
        } else {
            None
        };

        let res = client.block_on_allow_threads(async {
            client.rest_client().get_interest_limits(params).await
        })?;
        map_values(Ok(res))
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn get_vip_loan_order_list(
        client: &PyOkxClient,
        ord_id: Option<&str>,
        state: Option<&str>,
        ccy: Option<&str>,
        after: Option<&str>,
        before: Option<&str>,
        limit: Option<&str>,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let params = if ord_id.is_some()
            || state.is_some()
            || ccy.is_some()
            || after.is_some()
            || before.is_some()
            || limit.is_some()
        {
            Some(GetVipLoanOrderListParams {
                ord_id: ord_id.map(String::from),
                state: state.map(String::from),
                ccy: ccy.map(String::from),
                after: after.map(String::from),
                before: before.map(String::from),
                limit: limit.map(String::from),
            })
        } else {
            None
        };

        let res = client.block_on_allow_threads(async {
            client.rest_client().get_vip_loan_order_list(params).await
        })?;
        map_values(Ok(res))
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn get_vip_loan_order_detail(
        client: &PyOkxClient,
        ccy: Option<&str>,
        ord_id: Option<&str>,
        after: Option<&str>,
        before: Option<&str>,
        limit: Option<&str>,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let params = if ccy.is_some()
            || ord_id.is_some()
            || after.is_some()
            || before.is_some()
            || limit.is_some()
        {
            Some(GetVipLoanOrderDetailParams {
                ccy: ccy.map(String::from),
                ord_id: ord_id.map(String::from),
                after: after.map(String::from),
                before: before.map(String::from),
                limit: limit.map(String::from),
            })
        } else {
            None
        };

        let res = client.block_on_allow_threads(async {
            client.rest_client().get_vip_loan_order_detail(params).await
        })?;
        map_values(Ok(res))
    }

    pub(crate) fn get_fix_loan_borrowing_limit(client: &PyOkxClient) -> PyResult<Vec<Py<PyAny>>> {
        let res = client.block_on_allow_threads(async {
            client.rest_client().get_fix_loan_borrowing_limit().await
        })?;
        map_values(Ok(res))
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn get_fix_loan_borrowing_quote(
        client: &PyOkxClient,
        type_: Option<&str>,
        ccy: Option<&str>,
        amt: Option<&str>,
        max_rate: Option<&str>,
        term: Option<&str>,
        ord_id: Option<&str>,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let params = if type_.is_some()
            || ccy.is_some()
            || amt.is_some()
            || max_rate.is_some()
            || term.is_some()
            || ord_id.is_some()
        {
            Some(GetFixLoanBorrowingQuoteParams {
                r#type: type_.map(String::from),
                ccy: ccy.map(String::from),
                amt: amt.map(String::from),
                max_rate: max_rate.map(String::from),
                term: term.map(String::from),
                ord_id: ord_id.map(String::from),
            })
        } else {
            None
        };

        let res = client.block_on_allow_threads(async {
            client
                .rest_client()
                .get_fix_loan_borrowing_quote(params)
                .await
        })?;
        map_values(Ok(res))
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn place_fix_loan_borrowing_order(
        client: &PyOkxClient,
        ccy: Option<&str>,
        amt: Option<&str>,
        max_rate: Option<&str>,
        term: Option<&str>,
        reborrow: Option<bool>,
        reborrow_rate: Option<&str>,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let request = FixLoanBorrowingOrderRequest {
            ccy: ccy.map(String::from),
            amt: amt.map(String::from),
            max_rate: max_rate.map(String::from),
            term: term.map(String::from),
            reborrow,
            reborrow_rate: reborrow_rate.map(String::from),
        };

        let res = client.block_on_allow_threads(async {
            client
                .rest_client()
                .place_fix_loan_borrowing_order(request)
                .await
        })?;
        map_values(Ok(res))
    }

    pub(crate) fn amend_fix_loan_borrowing_order(
        client: &PyOkxClient,
        ord_id: Option<&str>,
        reborrow: Option<bool>,
        renew_max_rate: Option<&str>,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let request = AmendFixLoanBorrowingOrderRequest {
            ord_id: ord_id.map(String::from),
            reborrow,
            renew_max_rate: renew_max_rate.map(String::from),
        };

        let res = client.block_on_allow_threads(async {
            client
                .rest_client()
                .amend_fix_loan_borrowing_order(request)
                .await
        })?;
        map_values(Ok(res))
    }

    pub(crate) fn fix_loan_manual_reborrow(
        client: &PyOkxClient,
        ord_id: Option<&str>,
        max_rate: Option<&str>,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let request = FixLoanManualReborrowRequest {
            ord_id: ord_id.map(String::from),
            max_rate: max_rate.map(String::from),
        };

        let res = client.block_on_allow_threads(async {
            client.rest_client().fix_loan_manual_reborrow(request).await
        })?;
        map_values(Ok(res))
    }

    pub(crate) fn repay_fix_loan_borrowing_order(
        client: &PyOkxClient,
        ord_id: Option<&str>,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let request = RepayFixLoanBorrowingOrderRequest {
            ord_id: ord_id.map(String::from),
        };

        let res = client.block_on_allow_threads(async {
            client
                .rest_client()
                .repay_fix_loan_borrowing_order(request)
                .await
        })?;
        map_values(Ok(res))
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn get_fix_loan_borrowing_orders_list(
        client: &PyOkxClient,
        ord_id: Option<&str>,
        ccy: Option<&str>,
        state: Option<&str>,
        after: Option<&str>,
        before: Option<&str>,
        limit: Option<&str>,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let params = if ord_id.is_some()
            || ccy.is_some()
            || state.is_some()
            || after.is_some()
            || before.is_some()
            || limit.is_some()
        {
            Some(GetFixLoanBorrowingOrdersListParams {
                ord_id: ord_id.map(String::from),
                ccy: ccy.map(String::from),
                state: state.map(String::from),
                after: after.map(String::from),
                before: before.map(String::from),
                limit: limit.map(String::from),
            })
        } else {
            None
        };

        let res = client.block_on_allow_threads(async {
            client
                .rest_client()
                .get_fix_loan_borrowing_orders_list(params)
                .await
        })?;
        map_values(Ok(res))
    }

    pub(crate) fn set_account_level(
        client: &PyOkxClient,
        acct_lv: &str,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let request = SetAccountLevelRequest {
            acct_lv: acct_lv.to_string(),
        };
        let res = client.block_on_allow_threads(async {
            client.rest_client().set_account_level(request).await
        })?;
        map_values(Ok(res))
    }

    pub(crate) fn borrow_repay(
        client: &PyOkxClient,
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

        let res = client
            .block_on_allow_threads(async { client.rest_client().borrow_repay(request).await })?;
        map_values(Ok(res))
    }

    pub(crate) fn get_borrow_repay_history(
        client: &PyOkxClient,
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

        let res = client.block_on_allow_threads(async {
            client.rest_client().get_borrow_repay_history(params).await
        })?;
        map_values(Ok(res))
    }

    pub(crate) fn spot_manual_borrow_repay(
        client: &PyOkxClient,
        ccy: Option<&str>,
        side: Option<&str>,
        amt: Option<&str>,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let request = SpotManualBorrowRepayRequest {
            ccy: ccy.map(String::from),
            side: side.map(String::from),
            amt: amt.map(String::from),
        };

        let res = client.block_on_allow_threads(async {
            client.rest_client().spot_manual_borrow_repay(request).await
        })?;
        map_values(Ok(res))
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn spot_borrow_repay_history(
        client: &PyOkxClient,
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

        let res = client.block_on_allow_threads(async {
            client.rest_client().spot_borrow_repay_history(params).await
        })?;
        map_values(Ok(res))
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn get_interest_accrued(
        client: &PyOkxClient,
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

        let res = client.block_on_allow_threads(async {
            client.rest_client().get_interest_accrued(params).await
        })?;
        map_values(Ok(res))
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn get_vip_interest_accrued(
        client: &PyOkxClient,
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

        let res = client.block_on_allow_threads(async {
            client.rest_client().get_vip_interest_accrued(params).await
        })?;
        map_values(Ok(res))
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn get_vip_interest_deducted(
        client: &PyOkxClient,
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

        let res = client.block_on_allow_threads(async {
            client.rest_client().get_vip_interest_deducted(params).await
        })?;
        map_values(Ok(res))
    }

    pub(crate) fn get_simulated_margin(
        client: &PyOkxClient,
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

        let res = client.block_on_allow_threads(async {
            client.rest_client().get_simulated_margin(params).await
        })?;
        map_values(Ok(res))
    }

    pub(crate) fn get_account_position_tiers(
        client: &PyOkxClient,
        inst_type: &str,
        uly: Option<&str>,
        inst_family: Option<&str>,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let params = GetAccountPositionTiersParams {
            inst_type: inst_type.to_string(),
            uly: uly.map(String::from),
            inst_family: inst_family.map(String::from),
        };

        let res = client.block_on_allow_threads(async {
            client
                .rest_client()
                .get_account_position_tiers(params)
                .await
        })?;
        map_values(Ok(res))
    }

    pub(crate) fn get_greeks(client: &PyOkxClient, ccy: Option<&str>) -> PyResult<Vec<Py<PyAny>>> {
        let params = GetGreeksParams {
            ccy: ccy.map(String::from),
        };
        let res = client
            .block_on_allow_threads(async { client.rest_client().get_greeks(params).await })?;
        map_values(Ok(res))
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn position_builder(
        client: &PyOkxClient,
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

        let res = client.block_on_allow_threads(async {
            client.rest_client().position_builder(request).await
        })?;
        map_values(Ok(res))
    }
}

pub(crate) mod async_api {
    use super::*;

    pub(crate) fn get_balance<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        ccy: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            rest.get_balance(ccy.as_deref())
                .await
                .map(|v| v.into_iter().map(PyBalance::from).collect::<Vec<_>>())
                .map_err(to_py_err)
        })
    }

    pub(crate) fn get_positions<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        inst_type: Option<String>,
        inst_id: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        let params = if inst_type.is_some() || inst_id.is_some() {
            Some(okx_rest::api::account::GetPositionsParams {
                inst_type,
                inst_id,
                pos_id: None,
            })
        } else {
            None
        };

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            rest.get_positions(params)
                .await
                .map(|v| v.into_iter().map(PyPosition::from).collect::<Vec<_>>())
                .map_err(to_py_err)
        })
    }

    pub(crate) fn get_account_config<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            rest.get_account_config()
                .await
                .map(|v| v.into_iter().map(PyAccountConfig::from).collect::<Vec<_>>())
                .map_err(to_py_err)
        })
    }

    pub(crate) fn set_leverage<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        request: SetLeverageRequest,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            rest.set_leverage(request)
                .await
                .map(|mut v| v.pop().map(PySetLeverageResult::from))
                .map_err(to_py_err)
        })
    }

    pub(crate) fn get_leverage_info<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        params: GetLeverageInfoParams,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            rest.get_leverage_info(params)
                .await
                .map(|v| v.into_iter().map(PyLeverageInfo::from).collect::<Vec<_>>())
                .map_err(to_py_err)
        })
    }

    pub(crate) fn get_max_size<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        params: GetMaxSizeParams,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            rest.get_max_size(params)
                .await
                .map(|mut v| v.pop().map(PyMaxSize::from))
                .map_err(to_py_err)
        })
    }

    pub(crate) fn get_max_avail_size<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        params: GetMaxAvailSizeParams,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            rest.get_max_avail_size(params)
                .await
                .map(|mut v| v.pop().map(PyMaxAvailSize::from))
                .map_err(to_py_err)
        })
    }

    pub(crate) fn get_max_loan<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        params: GetMaxLoanParams,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(rest.get_max_loan(params).await)
        })
    }

    pub(crate) fn get_fee_rates<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        params: GetFeeRatesParams,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            rest.get_fee_rates(params)
                .await
                .map(|v| v.into_iter().map(PyFeeRates::from).collect::<Vec<_>>())
                .map_err(to_py_err)
        })
    }

    pub(crate) fn get_max_withdrawal<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        ccy: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(rest.get_max_withdrawal(ccy.as_deref()).await)
        })
    }

    pub(crate) fn get_interest_rate<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        ccy: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(rest.get_interest_rate(ccy.as_deref()).await)
        })
    }

    pub(crate) fn set_position_mode<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        pos_mode: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            rest.set_position_mode(&pos_mode)
                .await
                .map(|mut v| v.pop().map(PySetPositionModeResult::from))
                .map_err(to_py_err)
        })
    }

    pub(crate) fn get_account_position_risk<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            rest.get_account_position_risk()
                .await
                .map(|v| {
                    v.into_iter()
                        .map(PyAccountPositionRisk::from)
                        .collect::<Vec<_>>()
                })
                .map_err(to_py_err)
        })
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn get_positions_history<'py>(
        client: &PyAsyncOkxClient,
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
        let rest = client.rest_client();
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
            map_values(rest.get_positions_history(params).await)
        })
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn get_account_bills<'py>(
        client: &PyAsyncOkxClient,
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
        let rest = client.rest_client();
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
            map_values(rest.get_account_bills(params).await)
        })
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn get_account_bills_archive<'py>(
        client: &PyAsyncOkxClient,
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
        let rest = client.rest_client();
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
            map_values(rest.get_account_bills_archive(params).await)
        })
    }

    pub(crate) fn set_greeks<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        greeks_type: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        let request = SetGreeksRequest { greeks_type };

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(rest.set_greeks(request).await)
        })
    }

    pub(crate) fn set_isolated_mode<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        iso_mode: String,
        type_: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        let request = SetIsolatedModeRequest {
            iso_mode,
            r#type: type_,
        };

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(rest.set_isolated_mode(request).await)
        })
    }

    pub(crate) fn adjustment_margin<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        inst_id: String,
        pos_side: String,
        type_: String,
        amt: String,
        loan_trans: Option<bool>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        let request = AdjustmentMarginRequest {
            inst_id,
            pos_side,
            r#type: type_,
            amt,
            loan_trans,
        };

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(rest.adjustment_margin(request).await)
        })
    }

    pub(crate) fn set_risk_offset_type<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        type_: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        let request = SetRiskOffsetTypeRequest { r#type: type_ };

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(rest.set_risk_offset_type(request).await)
        })
    }

    pub(crate) fn set_auto_loan<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        auto_loan: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        let request = SetAutoLoanRequest { auto_loan };

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(rest.set_auto_loan(request).await)
        })
    }

    pub(crate) fn activate_option<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(rest.activate_option().await)
        })
    }

    pub(crate) fn set_auto_repay<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        auto_repay: Option<bool>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        let request = SetAutoRepayRequest { auto_repay };

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(rest.set_auto_repay(request).await)
        })
    }

    pub(crate) fn get_interest_limits<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        type_: Option<String>,
        ccy: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        let params = if type_.is_some() || ccy.is_some() {
            Some(GetInterestLimitsParams { r#type: type_, ccy })
        } else {
            None
        };

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(rest.get_interest_limits(params).await)
        })
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn get_vip_loan_order_list<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        ord_id: Option<String>,
        state: Option<String>,
        ccy: Option<String>,
        after: Option<String>,
        before: Option<String>,
        limit: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        let params = if ord_id.is_some()
            || state.is_some()
            || ccy.is_some()
            || after.is_some()
            || before.is_some()
            || limit.is_some()
        {
            Some(GetVipLoanOrderListParams {
                ord_id,
                state,
                ccy,
                after,
                before,
                limit,
            })
        } else {
            None
        };

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(rest.get_vip_loan_order_list(params).await)
        })
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn get_vip_loan_order_detail<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        ccy: Option<String>,
        ord_id: Option<String>,
        after: Option<String>,
        before: Option<String>,
        limit: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        let params = if ccy.is_some()
            || ord_id.is_some()
            || after.is_some()
            || before.is_some()
            || limit.is_some()
        {
            Some(GetVipLoanOrderDetailParams {
                ccy,
                ord_id,
                after,
                before,
                limit,
            })
        } else {
            None
        };

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(rest.get_vip_loan_order_detail(params).await)
        })
    }

    pub(crate) fn get_fix_loan_borrowing_limit<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(rest.get_fix_loan_borrowing_limit().await)
        })
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn get_fix_loan_borrowing_quote<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        type_: Option<String>,
        ccy: Option<String>,
        amt: Option<String>,
        max_rate: Option<String>,
        term: Option<String>,
        ord_id: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        let params = if type_.is_some()
            || ccy.is_some()
            || amt.is_some()
            || max_rate.is_some()
            || term.is_some()
            || ord_id.is_some()
        {
            Some(GetFixLoanBorrowingQuoteParams {
                r#type: type_,
                ccy,
                amt,
                max_rate,
                term,
                ord_id,
            })
        } else {
            None
        };

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(rest.get_fix_loan_borrowing_quote(params).await)
        })
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn place_fix_loan_borrowing_order<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        ccy: Option<String>,
        amt: Option<String>,
        max_rate: Option<String>,
        term: Option<String>,
        reborrow: Option<bool>,
        reborrow_rate: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        let request = FixLoanBorrowingOrderRequest {
            ccy,
            amt,
            max_rate,
            term,
            reborrow,
            reborrow_rate,
        };

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(rest.place_fix_loan_borrowing_order(request).await)
        })
    }

    pub(crate) fn amend_fix_loan_borrowing_order<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        ord_id: Option<String>,
        reborrow: Option<bool>,
        renew_max_rate: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        let request = AmendFixLoanBorrowingOrderRequest {
            ord_id,
            reborrow,
            renew_max_rate,
        };

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(rest.amend_fix_loan_borrowing_order(request).await)
        })
    }

    pub(crate) fn fix_loan_manual_reborrow<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        ord_id: Option<String>,
        max_rate: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        let request = FixLoanManualReborrowRequest { ord_id, max_rate };

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(rest.fix_loan_manual_reborrow(request).await)
        })
    }

    pub(crate) fn repay_fix_loan_borrowing_order<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        ord_id: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        let request = RepayFixLoanBorrowingOrderRequest { ord_id };

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(rest.repay_fix_loan_borrowing_order(request).await)
        })
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn get_fix_loan_borrowing_orders_list<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        ord_id: Option<String>,
        ccy: Option<String>,
        state: Option<String>,
        after: Option<String>,
        before: Option<String>,
        limit: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        let params = if ord_id.is_some()
            || ccy.is_some()
            || state.is_some()
            || after.is_some()
            || before.is_some()
            || limit.is_some()
        {
            Some(GetFixLoanBorrowingOrdersListParams {
                ord_id,
                ccy,
                state,
                after,
                before,
                limit,
            })
        } else {
            None
        };

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(rest.get_fix_loan_borrowing_orders_list(params).await)
        })
    }

    pub(crate) fn set_account_level<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        acct_lv: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        let request = SetAccountLevelRequest { acct_lv };

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(rest.set_account_level(request).await)
        })
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn borrow_repay<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        ccy: Option<String>,
        side: Option<String>,
        amt: Option<String>,
        ord_id: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        let request = BorrowRepayRequest {
            ccy,
            side,
            amt,
            ord_id,
        };

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(rest.borrow_repay(request).await)
        })
    }

    pub(crate) fn get_borrow_repay_history<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        ccy: Option<String>,
        after: Option<String>,
        before: Option<String>,
        limit: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
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
            map_values(rest.get_borrow_repay_history(params).await)
        })
    }

    pub(crate) fn spot_manual_borrow_repay<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        ccy: Option<String>,
        side: Option<String>,
        amt: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        let request = SpotManualBorrowRepayRequest { ccy, side, amt };

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(rest.spot_manual_borrow_repay(request).await)
        })
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn spot_borrow_repay_history<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        ccy: Option<String>,
        type_: Option<String>,
        after: Option<String>,
        before: Option<String>,
        limit: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
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
            map_values(rest.spot_borrow_repay_history(params).await)
        })
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn get_interest_accrued<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        inst_id: Option<String>,
        ccy: Option<String>,
        mgn_mode: Option<String>,
        after: Option<String>,
        before: Option<String>,
        limit: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        let params = GetInterestAccruedParams {
            inst_id,
            ccy,
            mgn_mode,
            after,
            before,
            limit,
        };

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(rest.get_interest_accrued(params).await)
        })
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn get_vip_interest_accrued<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        ccy: Option<String>,
        ord_id: Option<String>,
        after: Option<String>,
        before: Option<String>,
        limit: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        let params = GetVipInterestParams {
            ccy,
            ord_id,
            after,
            before,
            limit,
        };

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(rest.get_vip_interest_accrued(params).await)
        })
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn get_vip_interest_deducted<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        ccy: Option<String>,
        ord_id: Option<String>,
        after: Option<String>,
        before: Option<String>,
        limit: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        let params = GetVipInterestParams {
            ccy,
            ord_id,
            after,
            before,
            limit,
        };

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(rest.get_vip_interest_deducted(params).await)
        })
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn get_simulated_margin<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        inst_type: String,
        incl_real_pos: Option<bool>,
        spot_offset_type: Option<String>,
        sim_pos_json: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        let sim_pos = parse_json_array(sim_pos_json.as_deref(), "sim_pos")?;
        let params = GetSimulatedMarginParams {
            inst_type,
            incl_real_pos,
            spot_offset_type,
            sim_pos,
        };

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(rest.get_simulated_margin(params).await)
        })
    }

    pub(crate) fn get_account_position_tiers<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        inst_type: String,
        uly: Option<String>,
        inst_family: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        let params = GetAccountPositionTiersParams {
            inst_type,
            uly,
            inst_family,
        };

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(rest.get_account_position_tiers(params).await)
        })
    }

    pub(crate) fn get_greeks<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        ccy: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        let params = GetGreeksParams { ccy };

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(rest.get_greeks(params).await)
        })
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn position_builder<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        acct_lv: Option<String>,
        incl_real_pos_and_eq: Option<bool>,
        lever: Option<String>,
        greeks_type: Option<String>,
        sim_pos_json: Option<String>,
        sim_asset_json: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
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
            map_values(rest.position_builder(request).await)
        })
    }
}
