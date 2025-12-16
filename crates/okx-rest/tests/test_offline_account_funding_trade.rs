//! 覆盖率补齐：离线环境下覆盖 `account/funding/trade` 三个仍偏低的模块实现体。
//!
//! 约束：
//! - 不依赖外网；使用本地不可达地址触发快速 HTTP 错误
//! - 以“调用到实现体”为目标，确保每个 `impl *Api for OkxRestClient` 的方法至少被执行一次

#![allow(missing_docs)]

use okx_core::types::{
    AmendOrderRequest, AttachAlgoOrdRequest, CancelAlgoOrderRequest, CancelOrderRequest,
    FundsTransferRequest, PlaceAlgoOrderRequest, PlaceOrderRequest, WithdrawalRequest,
};
use okx_core::{Config, Credentials};
use okx_rest::api::account::{
    AdjustmentMarginRequest, AmendFixLoanBorrowingOrderRequest, BorrowRepayRequest,
    FixLoanBorrowingOrderRequest, FixLoanManualReborrowRequest, GetAccountInstrumentsParams,
    GetBillsArchiveParams, GetBillsParams, GetFixLoanBorrowingOrdersListParams,
    GetFixLoanBorrowingQuoteParams, GetInterestAccruedParams, GetInterestLimitsParams,
    GetLeverageInfoParams, GetMaxAvailSizeParams, GetMaxLoanParams, GetMaxSizeParams,
    GetPositionsHistoryParams, GetPositionsParams, GetSimulatedMarginParams, GetVipInterestParams,
    GetVipLoanOrderDetailParams, GetVipLoanOrderListParams, PositionBuilderRequest,
    RepayFixLoanBorrowingOrderRequest, SetAccountLevelRequest, SetAutoLoanRequest,
    SetAutoRepayRequest, SetGreeksRequest, SetLeverageRequest, SetRiskOffsetTypeRequest,
    SpotBorrowRepayHistoryParams, SpotManualBorrowRepayRequest,
};
use okx_rest::api::funding::{
    CancelWithdrawalParams, ConvertDustAssetsRequest, GetAssetValuationParams,
    GetDepositHistoryParams, GetDepositLightningParams, GetDepositWithdrawStatusParams,
    GetFundingBillsParams, GetLendingHistoryParams, GetLendingRateHistoryParams,
    GetLendingRateSummaryParams, GetSavingBalanceParams, GetTransferStateParams,
    GetWithdrawalHistoryParams, PurchaseRedemptRequest, SetLendingRateRequest,
    WithdrawalLightningRequest,
};
use okx_rest::api::trade::{
    AmendAlgoOrderRequest, ClosePositionRequest, GetAlgoOrderDetailsParams,
    GetAlgoOrdersHistoryParams, GetAlgoOrdersParams, GetFillsHistoryParams, GetFillsParams,
    GetOrderParams, GetOrdersHistoryArchiveParams, GetOrdersHistoryParams, GetOrdersPendingParams,
    OneClickRepayHistoryV2Params, OneClickRepayV2Request,
};
use okx_rest::{AccountApi, FundingApi, OkxError, OkxRestClient, TradeApi};
use serde_json::{json, to_value};

fn dummy_client() -> OkxRestClient {
    let creds = Credentials::new("key", "secret", "pass");
    let config = Config::new(creds)
        .with_rest_url("http://127.0.0.1:9")
        .with_timeout_secs(1);
    OkxRestClient::new(config)
}

fn expect_http_error(err: OkxError) {
    match err {
        OkxError::Http(_) | OkxError::HttpStatus { .. } => {}
        other => panic!("预期 HTTP 错误，实际为: {other:?}"),
    }
}

#[tokio::test]
async fn account_api_offline_covers_all_methods() {
    let client = dummy_client();

    expect_http_error(client.get_balance(Some("USDT")).await.unwrap_err());

    let pos = GetPositionsParams {
        inst_type: Some("SWAP".into()),
        inst_id: Some("BTC-USDT-SWAP".into()),
        pos_id: None,
    };
    expect_http_error(client.get_positions(Some(pos)).await.unwrap_err());

    let acc_inst = GetAccountInstrumentsParams {
        inst_type: "SPOT".into(),
        inst_family: None,
        inst_id: None,
    };
    expect_http_error(client.get_account_instruments(acc_inst).await.unwrap_err());

    expect_http_error(client.get_account_config().await.unwrap_err());

    let leverage = SetLeverageRequest {
        inst_id: Some("BTC-USDT-SWAP".into()),
        ccy: None,
        lever: "5".into(),
        mgn_mode: "cross".into(),
        pos_side: None,
    };
    expect_http_error(client.set_leverage(leverage).await.unwrap_err());

    let lever_info = GetLeverageInfoParams {
        mgn_mode: "cross".into(),
        ccy: Some("USDT".into()),
        inst_id: None,
    };
    expect_http_error(client.get_leverage_info(lever_info).await.unwrap_err());

    let max_size = GetMaxSizeParams {
        inst_id: "BTC-USDT".into(),
        td_mode: "cash".into(),
        ccy: None,
        px: Some("100".into()),
        leverage: None,
    };
    expect_http_error(client.get_max_size(max_size).await.unwrap_err());

    let max_avail = GetMaxAvailSizeParams {
        inst_id: "BTC-USDT".into(),
        td_mode: "cash".into(),
        ccy: None,
        reduce_only: Some(false),
        quick_mgn_type: None,
    };
    expect_http_error(client.get_max_avail_size(max_avail).await.unwrap_err());

    let max_loan = GetMaxLoanParams {
        inst_id: "BTC-USDT".into(),
        mgn_mode: "cross".into(),
        mgn_ccy: Some("USDT".into()),
    };
    expect_http_error(client.get_max_loan(max_loan).await.unwrap_err());

    let fee = okx_rest::api::account::GetFeeRatesParams {
        inst_type: "SPOT".into(),
        inst_id: Some("BTC-USDT".into()),
        uly: None,
        inst_family: None,
    };
    expect_http_error(client.get_fee_rates(fee).await.unwrap_err());

    expect_http_error(
        client
            .set_position_mode("long_short_mode")
            .await
            .unwrap_err(),
    );

    expect_http_error(client.get_account_position_risk().await.unwrap_err());
    expect_http_error(client.get_account_risk_state().await.unwrap_err());

    let hist = GetPositionsHistoryParams {
        inst_type: None,
        inst_id: None,
        mgn_mode: None,
        r#type: None,
        pos_id: None,
        after: None,
        before: None,
        limit: None,
    };
    expect_http_error(client.get_positions_history(Some(hist)).await.unwrap_err());

    expect_http_error(client.get_max_withdrawal(Some("USDT")).await.unwrap_err());

    let bills = GetBillsParams {
        inst_type: None,
        ccy: None,
        mgn_mode: None,
        ct_type: None,
        r#type: None,
        sub_type: None,
        after: None,
        before: None,
        limit: None,
    };
    expect_http_error(client.get_account_bills(Some(bills)).await.unwrap_err());

    let archive = GetBillsArchiveParams {
        inst_type: None,
        ccy: None,
        mgn_mode: None,
        ct_type: None,
        r#type: None,
        sub_type: None,
        after: None,
        before: None,
        begin: None,
        end: None,
        limit: None,
    };
    expect_http_error(client.get_account_bills_archive(archive).await.unwrap_err());

    let set_greeks = SetGreeksRequest {
        greeks_type: "PA".into(),
    };
    expect_http_error(client.set_greeks(set_greeks).await.unwrap_err());

    let isolated = okx_rest::api::account::SetIsolatedModeRequest {
        iso_mode: "automatic".into(),
        r#type: "MARGIN".into(),
    };
    expect_http_error(client.set_isolated_mode(isolated).await.unwrap_err());

    let level = SetAccountLevelRequest {
        acct_lv: "2".into(),
    };
    expect_http_error(client.set_account_level(level).await.unwrap_err());

    let borrow = BorrowRepayRequest {
        ccy: Some("USDT".into()),
        side: Some("borrow".into()),
        amt: Some("1".into()),
        ord_id: None,
    };
    expect_http_error(client.borrow_repay(borrow).await.unwrap_err());

    expect_http_error(client.get_borrow_repay_history(None).await.unwrap_err());

    let spot = SpotManualBorrowRepayRequest {
        ccy: Some("USDT".into()),
        side: Some("borrow".into()),
        amt: Some("1".into()),
    };
    expect_http_error(client.spot_manual_borrow_repay(spot).await.unwrap_err());

    let spot_hist = SpotBorrowRepayHistoryParams {
        ccy: Some("USDT".into()),
        r#type: Some("borrow".into()),
        after: None,
        before: None,
        limit: Some("1".into()),
    };
    expect_http_error(
        client
            .spot_borrow_repay_history(Some(spot_hist))
            .await
            .unwrap_err(),
    );

    let adj = AdjustmentMarginRequest {
        inst_id: "BTC-USDT-SWAP".into(),
        pos_side: "net".into(),
        r#type: "add".into(),
        amt: "1".into(),
        loan_trans: Some(false),
    };
    expect_http_error(client.adjustment_margin(adj).await.unwrap_err());

    let risk = SetRiskOffsetTypeRequest { r#type: "1".into() };
    expect_http_error(client.set_risk_offset_type(risk).await.unwrap_err());

    let auto_loan = SetAutoLoanRequest {
        auto_loan: Some("true".into()),
    };
    expect_http_error(client.set_auto_loan(auto_loan).await.unwrap_err());

    expect_http_error(client.activate_option().await.unwrap_err());

    let auto_repay = SetAutoRepayRequest {
        auto_repay: Some(true),
    };
    expect_http_error(client.set_auto_repay(auto_repay).await.unwrap_err());

    let interest_limits = GetInterestLimitsParams {
        r#type: Some("1".into()),
        ccy: Some("USDT".into()),
    };
    expect_http_error(
        client
            .get_interest_limits(Some(interest_limits))
            .await
            .unwrap_err(),
    );

    let vip_list = GetVipLoanOrderListParams {
        ord_id: Some("1".into()),
        state: Some("filled".into()),
        ccy: Some("USDT".into()),
        after: Some("10".into()),
        before: Some("1".into()),
        limit: Some("20".into()),
    };
    expect_http_error(
        client
            .get_vip_loan_order_list(Some(vip_list))
            .await
            .unwrap_err(),
    );

    let vip_detail = GetVipLoanOrderDetailParams {
        ccy: Some("USDT".into()),
        ord_id: Some("2".into()),
        after: None,
        before: None,
        limit: None,
    };
    expect_http_error(
        client
            .get_vip_loan_order_detail(Some(vip_detail))
            .await
            .unwrap_err(),
    );

    expect_http_error(client.get_fix_loan_borrowing_limit().await.unwrap_err());

    let quote = GetFixLoanBorrowingQuoteParams {
        r#type: Some("1".into()),
        ccy: Some("USDT".into()),
        amt: Some("10".into()),
        max_rate: Some("0.02".into()),
        term: Some("7".into()),
        ord_id: Some("123".into()),
    };
    expect_http_error(
        client
            .get_fix_loan_borrowing_quote(Some(quote))
            .await
            .unwrap_err(),
    );

    let order = FixLoanBorrowingOrderRequest {
        ccy: Some("USDT".into()),
        amt: Some("10".into()),
        max_rate: Some("0.02".into()),
        term: Some("7".into()),
        reborrow: Some(true),
        reborrow_rate: Some("0.03".into()),
    };
    expect_http_error(
        client
            .place_fix_loan_borrowing_order(order)
            .await
            .unwrap_err(),
    );

    let amend = AmendFixLoanBorrowingOrderRequest {
        ord_id: Some("123".into()),
        reborrow: Some(false),
        renew_max_rate: Some("0.01".into()),
    };
    expect_http_error(
        client
            .amend_fix_loan_borrowing_order(amend)
            .await
            .unwrap_err(),
    );

    let manual = FixLoanManualReborrowRequest {
        ord_id: Some("123".into()),
        max_rate: Some("0.03".into()),
    };
    expect_http_error(client.fix_loan_manual_reborrow(manual).await.unwrap_err());

    let repay = RepayFixLoanBorrowingOrderRequest {
        ord_id: Some("123".into()),
    };
    expect_http_error(
        client
            .repay_fix_loan_borrowing_order(repay)
            .await
            .unwrap_err(),
    );

    let list = GetFixLoanBorrowingOrdersListParams {
        ord_id: Some("123".into()),
        ccy: Some("USDT".into()),
        state: Some("filled".into()),
        after: None,
        before: None,
        limit: Some("1".into()),
    };
    expect_http_error(
        client
            .get_fix_loan_borrowing_orders_list(Some(list))
            .await
            .unwrap_err(),
    );

    let accrued = GetInterestAccruedParams {
        inst_id: None,
        ccy: Some("USDT".into()),
        mgn_mode: Some("cross".into()),
        after: None,
        before: None,
        limit: Some("1".into()),
    };
    expect_http_error(client.get_interest_accrued(accrued).await.unwrap_err());

    expect_http_error(client.get_interest_rate(Some("USDT")).await.unwrap_err());

    let vip_interest = GetVipInterestParams {
        ccy: Some("USDT".into()),
        ord_id: Some("1".into()),
        after: None,
        before: None,
        limit: Some("1".into()),
    };
    expect_http_error(
        client
            .get_vip_interest_accrued(vip_interest)
            .await
            .unwrap_err(),
    );

    let vip_interest = GetVipInterestParams {
        ccy: Some("USDT".into()),
        ord_id: Some("1".into()),
        after: None,
        before: None,
        limit: Some("1".into()),
    };
    expect_http_error(
        client
            .get_vip_interest_deducted(vip_interest)
            .await
            .unwrap_err(),
    );

    let simulated = GetSimulatedMarginParams {
        inst_type: "SWAP".into(),
        incl_real_pos: Some(false),
        spot_offset_type: None,
        sim_pos: None,
    };
    expect_http_error(client.get_simulated_margin(simulated).await.unwrap_err());

    let tiers = okx_rest::api::account::GetAccountPositionTiersParams {
        inst_type: "SWAP".into(),
        uly: None,
        inst_family: None,
    };
    expect_http_error(client.get_account_position_tiers(tiers).await.unwrap_err());

    let greeks = okx_rest::api::account::GetGreeksParams {
        ccy: Some("USDT".into()),
    };
    expect_http_error(client.get_greeks(greeks).await.unwrap_err());

    let builder = PositionBuilderRequest {
        acct_lv: Some("2".into()),
        incl_real_pos_and_eq: Some(false),
        lever: Some("5".into()),
        greeks_type: Some("PA".into()),
        sim_pos: Some(json!([])),
        sim_asset: Some(json!([])),
    };
    expect_http_error(client.position_builder(builder).await.unwrap_err());
}

#[tokio::test]
async fn funding_api_offline_covers_all_methods() {
    let client = dummy_client();

    expect_http_error(client.get_asset_balances(Some("USDT")).await.unwrap_err());

    expect_http_error(client.get_deposit_address("USDT").await.unwrap_err());

    let dep_hist = GetDepositHistoryParams {
        ccy: Some("USDT".into()),
        dep_id: None,
        tx_id: None,
        r#type: None,
        state: None,
        after: None,
        before: None,
        limit: Some("1".into()),
    };
    expect_http_error(
        client
            .get_deposit_history(Some(dep_hist))
            .await
            .unwrap_err(),
    );

    let wd_hist = GetWithdrawalHistoryParams {
        ccy: Some("USDT".into()),
        wd_id: None,
        client_id: None,
        tx_id: None,
        r#type: None,
        state: None,
        after: None,
        before: None,
        limit: Some("1".into()),
    };
    expect_http_error(
        client
            .get_withdrawal_history(Some(wd_hist))
            .await
            .unwrap_err(),
    );

    let transfer = FundsTransferRequest {
        ccy: "USDT".into(),
        amt: "1".into(),
        from: "6".into(),
        to: "18".into(),
        r#type: Some("0".into()),
        sub_acct: None,
        inst_id: None,
        to_inst_id: None,
        loan_trans: None,
    };
    expect_http_error(client.funds_transfer(transfer).await.unwrap_err());

    let withdrawal = WithdrawalRequest {
        ccy: "USDT".into(),
        amt: "1".into(),
        dest: "4".into(),
        to_addr: "addr".into(),
        chain: None,
        area_code: None,
        client_id: None,
        fee: Some("0.1".into()),
    };
    expect_http_error(client.withdrawal(withdrawal).await.unwrap_err());

    expect_http_error(client.get_currencies(Some("USDT")).await.unwrap_err());

    expect_http_error(
        client
            .get_non_tradable_assets(Some("USDT"))
            .await
            .unwrap_err(),
    );

    let valuation = GetAssetValuationParams {
        ccy: Some("USDT".into()),
    };
    expect_http_error(
        client
            .get_asset_valuation(Some(valuation))
            .await
            .unwrap_err(),
    );

    let transfer_state = GetTransferStateParams {
        trans_id: "1".into(),
        r#type: Some("0".into()),
    };
    expect_http_error(client.get_transfer_state(transfer_state).await.unwrap_err());

    let bills = GetFundingBillsParams {
        ccy: Some("USDT".into()),
        r#type: None,
        after: None,
        before: None,
        limit: Some("1".into()),
    };
    expect_http_error(client.get_funding_bills(Some(bills)).await.unwrap_err());

    let pr = PurchaseRedemptRequest {
        ccy: "USDT".into(),
        amt: "1".into(),
        side: "purchase".into(),
        rate: None,
    };
    expect_http_error(client.purchase_redempt(pr).await.unwrap_err());

    let dep_ln = GetDepositLightningParams {
        ccy: "BTC".into(),
        amt: "1".into(),
        to: None,
    };
    expect_http_error(client.get_deposit_lightning(dep_ln).await.unwrap_err());

    let wd_ln = WithdrawalLightningRequest {
        ccy: "BTC".into(),
        invoice: "invoice".into(),
        memo: None,
    };
    expect_http_error(client.withdrawal_lightning(wd_ln).await.unwrap_err());

    let cancel = CancelWithdrawalParams {
        wd_id: Some("1".into()),
    };
    expect_http_error(client.cancel_withdrawal(cancel).await.unwrap_err());

    let status = GetDepositWithdrawStatusParams {
        wd_id: Some("1".into()),
        tx_id: None,
        ccy: Some("USDT".into()),
        to: None,
        chain: None,
    };
    expect_http_error(
        client
            .get_deposit_withdraw_status(status)
            .await
            .unwrap_err(),
    );

    let set_rate = SetLendingRateRequest {
        ccy: "USDT".into(),
        rate: "0.01".into(),
    };
    expect_http_error(client.set_lending_rate(set_rate).await.unwrap_err());

    let lend_hist = GetLendingHistoryParams {
        ccy: Some("USDT".into()),
        after: None,
        before: None,
        limit: Some("1".into()),
    };
    expect_http_error(
        client
            .get_lending_history(Some(lend_hist))
            .await
            .unwrap_err(),
    );

    let rate_hist = GetLendingRateHistoryParams {
        ccy: Some("USDT".into()),
        after: None,
        before: None,
        limit: Some("1".into()),
    };
    expect_http_error(
        client
            .get_lending_rate_history(Some(rate_hist))
            .await
            .unwrap_err(),
    );

    let summary = GetLendingRateSummaryParams {
        ccy: Some("USDT".into()),
    };
    expect_http_error(
        client
            .get_lending_rate_summary(Some(summary))
            .await
            .unwrap_err(),
    );

    let dust = ConvertDustAssetsRequest {
        ccy: Some(vec!["BTC".into()]),
    };
    expect_http_error(client.convert_dust_assets(dust).await.unwrap_err());

    let saving = GetSavingBalanceParams {
        ccy: Some("USDT".into()),
    };
    expect_http_error(client.get_saving_balance(Some(saving)).await.unwrap_err());
}

#[tokio::test]
async fn trade_api_offline_covers_all_methods() {
    let client = dummy_client();

    let place = PlaceOrderRequest {
        inst_id: "BTC-USDT".into(),
        td_mode: "cash".into(),
        side: "buy".into(),
        ord_type: "limit".into(),
        sz: "1".into(),
        ccy: None,
        cl_ord_id: None,
        tag: None,
        pos_side: None,
        px: Some("100".into()),
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
    expect_http_error(client.place_order(place).await.unwrap_err());

    let batch = vec![PlaceOrderRequest {
        inst_id: "BTC-USDT".into(),
        td_mode: "cash".into(),
        side: "buy".into(),
        ord_type: "limit".into(),
        sz: "1".into(),
        ccy: None,
        cl_ord_id: None,
        tag: None,
        pos_side: None,
        px: Some("100".into()),
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
    }];
    expect_http_error(client.place_batch_orders(batch).await.unwrap_err());

    let cancel = CancelOrderRequest {
        inst_id: "BTC-USDT".into(),
        ord_id: Some("1".into()),
        cl_ord_id: None,
    };
    expect_http_error(client.cancel_order(cancel).await.unwrap_err());

    let cancel_batch = vec![CancelOrderRequest {
        inst_id: "BTC-USDT".into(),
        ord_id: Some("1".into()),
        cl_ord_id: None,
    }];
    expect_http_error(client.cancel_batch_orders(cancel_batch).await.unwrap_err());

    let amend = AmendOrderRequest {
        inst_id: "BTC-USDT".into(),
        ord_id: Some("1".into()),
        cl_ord_id: None,
        req_id: None,
        new_sz: Some("2".into()),
        new_px: Some("101".into()),
        new_tp_trigger_px: None,
        new_tp_ord_px: None,
        new_sl_trigger_px: None,
        new_sl_ord_px: None,
        new_tp_trigger_px_type: None,
        new_sl_trigger_px_type: None,
    };
    expect_http_error(client.amend_order(amend).await.unwrap_err());

    let amend_batch = vec![AmendOrderRequest {
        inst_id: "BTC-USDT".into(),
        ord_id: Some("1".into()),
        cl_ord_id: None,
        req_id: None,
        new_sz: Some("2".into()),
        new_px: Some("101".into()),
        new_tp_trigger_px: None,
        new_tp_ord_px: None,
        new_sl_trigger_px: None,
        new_sl_ord_px: None,
        new_tp_trigger_px_type: None,
        new_sl_trigger_px_type: None,
    }];
    expect_http_error(client.amend_batch_orders(amend_batch).await.unwrap_err());

    let order = GetOrderParams {
        inst_id: "BTC-USDT".into(),
        ord_id: Some("1".into()),
        cl_ord_id: None,
    };
    expect_http_error(client.get_order(order).await.unwrap_err());

    let pending = GetOrdersPendingParams {
        inst_type: Some("SPOT".into()),
        uly: None,
        inst_family: None,
        inst_id: Some("BTC-USDT".into()),
        ord_type: None,
        state: None,
        after: None,
        before: None,
        limit: Some("1".into()),
    };
    expect_http_error(client.get_orders_pending(Some(pending)).await.unwrap_err());

    let hist = GetOrdersHistoryParams {
        inst_type: "SPOT".into(),
        uly: None,
        inst_family: None,
        inst_id: Some("BTC-USDT".into()),
        ord_type: None,
        state: None,
        category: None,
        after: None,
        before: None,
        begin: None,
        end: None,
        limit: Some("1".into()),
    };
    expect_http_error(client.get_orders_history(hist).await.unwrap_err());

    let hist_archive = GetOrdersHistoryArchiveParams {
        inst_type: "SPOT".into(),
        uly: None,
        inst_family: None,
        inst_id: Some("BTC-USDT".into()),
        ord_type: None,
        state: None,
        category: None,
        after: None,
        before: None,
        begin: None,
        end: None,
        limit: Some("1".into()),
    };
    expect_http_error(
        client
            .get_orders_history_archive(hist_archive)
            .await
            .unwrap_err(),
    );

    let fills = GetFillsParams {
        inst_type: None,
        uly: None,
        inst_family: None,
        inst_id: None,
        ord_id: None,
        after: None,
        before: None,
        begin: None,
        end: None,
        limit: Some("1".into()),
    };
    expect_http_error(client.get_fills(Some(fills)).await.unwrap_err());

    let fills_hist = GetFillsHistoryParams {
        inst_type: "SPOT".into(),
        uly: None,
        inst_family: None,
        inst_id: None,
        ord_id: None,
        after: None,
        before: None,
        limit: Some("1".into()),
    };
    expect_http_error(client.get_fills_history(fills_hist).await.unwrap_err());

    let algo = PlaceAlgoOrderRequest {
        inst_id: "BTC-USDT".into(),
        td_mode: "cash".into(),
        side: "buy".into(),
        ord_type: "conditional".into(),
        sz: "1".into(),
        ccy: None,
        pos_side: None,
        reduce_only: None,
        tgt_ccy: None,
        algo_cl_ord_id: Some("algo-1".into()),
        trigger_px: Some("100".into()),
        order_px: Some("100".into()),
        trigger_px_type: None,
        tp_trigger_px: None,
        tp_ord_px: None,
        tp_trigger_px_type: None,
        sl_trigger_px: None,
        sl_ord_px: None,
        sl_trigger_px_type: None,
        callback_ratio: None,
        callback_spread: None,
        active_px: None,
    };
    expect_http_error(client.place_algo_order(algo).await.unwrap_err());

    let cancel_algo = vec![CancelAlgoOrderRequest {
        inst_id: "BTC-USDT".into(),
        algo_id: "1".into(),
    }];
    expect_http_error(client.cancel_algo_orders(cancel_algo).await.unwrap_err());

    let amend_algo = AmendAlgoOrderRequest {
        inst_id: Some("BTC-USDT".into()),
        algo_id: Some("1".into()),
        algo_cl_ord_id: None,
        cxl_on_fail: None,
        req_id: None,
        new_sz: Some("2".into()),
        new_tp_trigger_px: None,
        new_tp_ord_px: None,
        new_sl_trigger_px: None,
        new_sl_ord_px: None,
        new_tp_trigger_px_type: None,
        new_sl_trigger_px_type: None,
    };
    expect_http_error(client.amend_algo_order(amend_algo).await.unwrap_err());

    let algo_pending = GetAlgoOrdersParams {
        ord_type: "conditional".into(),
        algo_id: None,
        inst_type: None,
        inst_id: None,
        after: None,
        before: None,
        limit: Some("1".into()),
    };
    expect_http_error(
        client
            .get_algo_orders_pending(algo_pending)
            .await
            .unwrap_err(),
    );

    let algo_hist = GetAlgoOrdersHistoryParams {
        ord_type: "conditional".into(),
        state: Some("effective".into()),
        algo_id: None,
        inst_type: None,
        inst_id: None,
        after: None,
        before: None,
        limit: Some("1".into()),
    };
    expect_http_error(client.get_algo_orders_history(algo_hist).await.unwrap_err());

    let details = GetAlgoOrderDetailsParams {
        algo_id: Some("1".into()),
        algo_cl_ord_id: None,
    };
    expect_http_error(client.get_algo_order_details(details).await.unwrap_err());

    let close = ClosePositionRequest {
        inst_id: "BTC-USDT-SWAP".into(),
        mgn_mode: "cross".into(),
        pos_side: Some("net".into()),
        ccy: None,
        auto_cancel: None,
        cl_ord_id: None,
        tag: None,
    };
    expect_http_error(client.close_position(close).await.unwrap_err());

    expect_http_error(
        client
            .mass_cancel(json!({"instType":"SWAP"}))
            .await
            .unwrap_err(),
    );
    expect_http_error(
        client
            .cancel_all_after(json!({"timeOut":"1"}))
            .await
            .unwrap_err(),
    );
    expect_http_error(
        client
            .order_precheck(json!({"instId":"BTC-USDT"}))
            .await
            .unwrap_err(),
    );

    expect_http_error(
        client
            .get_one_click_repay_currency_list_v2()
            .await
            .unwrap_err(),
    );

    let repay = OneClickRepayV2Request {
        debt_ccy: "BTC".into(),
        repay_ccy_list: vec!["USDT".into()],
    };
    expect_http_error(client.one_click_repay_v2(repay).await.unwrap_err());

    let repay_hist = OneClickRepayHistoryV2Params {
        after: Some("1".into()),
        before: Some("2".into()),
        limit: Some("1".into()),
    };
    expect_http_error(
        client
            .get_one_click_repay_history_v2(Some(repay_hist))
            .await
            .unwrap_err(),
    );

    // 附加覆盖：attach_algo_ords 序列化分支
    let attach = AttachAlgoOrdRequest {
        attach_algo_cl_ord_id: Some("a1".into()),
        tp_trigger_px: Some("110".into()),
        tp_trigger_px_type: None,
        tp_ord_px: Some("111".into()),
        sl_trigger_px: Some("90".into()),
        sl_trigger_px_type: None,
        sl_ord_px: Some("89".into()),
        sz: Some("1".into()),
    };
    let place = PlaceOrderRequest {
        inst_id: "BTC-USDT".into(),
        td_mode: "cash".into(),
        side: "buy".into(),
        ord_type: "limit".into(),
        sz: "1".into(),
        ccy: None,
        cl_ord_id: None,
        tag: None,
        pos_side: None,
        px: Some("100".into()),
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
        attach_algo_ords: Some(vec![attach]),
    };
    let body = to_value(&place).expect("序列化下单请求失败");
    assert!(body.get("attachAlgoOrds").is_some());
    let _ = client.place_order(place).await.unwrap_err();
}
