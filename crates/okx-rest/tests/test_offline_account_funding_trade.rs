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
    self, AdjustmentMarginRequest, AmendFixLoanBorrowingOrderRequest, BorrowRepayRequest,
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
    self, CancelWithdrawalParams, ConvertDustAssetsRequest, GetAssetValuationParams,
    GetDepositHistoryParams, GetDepositLightningParams, GetDepositWithdrawStatusParams,
    GetFundingBillsParams, GetLendingHistoryParams, GetLendingRateHistoryParams,
    GetLendingRateSummaryParams, GetSavingBalanceParams, GetTransferStateParams,
    GetWithdrawalHistoryParams, PurchaseRedemptRequest, SetLendingRateRequest,
    WithdrawalLightningRequest,
};
use okx_rest::api::trade::{
    self, AmendAlgoOrderRequest, ClosePositionRequest, GetAlgoOrderDetailsParams,
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

fn expect_http_error(err: OkxError) -> String {
    match err {
        OkxError::Http(msg) => msg,
        other => panic!("预期 HTTP 错误，实际为: {other:?}"),
    }
}

#[tokio::test]
async fn account_api_offline_covers_all_methods() {
    let client = dummy_client();

    assert!(
        expect_http_error(client.get_balance(Some("USDT")).await.unwrap_err())
            .contains(account::endpoints::BALANCE)
    );

    let pos = GetPositionsParams {
        inst_type: Some("SWAP".into()),
        inst_id: Some("BTC-USDT-SWAP".into()),
        pos_id: None,
    };
    assert!(
        expect_http_error(client.get_positions(Some(pos)).await.unwrap_err())
            .contains(account::endpoints::POSITIONS)
    );

    let acc_inst = GetAccountInstrumentsParams {
        inst_type: "SPOT".into(),
        inst_family: None,
        inst_id: None,
    };
    assert!(
        expect_http_error(client.get_account_instruments(acc_inst).await.unwrap_err())
            .contains(account::endpoints::INSTRUMENTS)
    );

    assert!(
        expect_http_error(client.get_account_config().await.unwrap_err())
            .contains(account::endpoints::CONFIG)
    );

    let leverage = SetLeverageRequest {
        inst_id: Some("BTC-USDT-SWAP".into()),
        ccy: None,
        lever: "5".into(),
        mgn_mode: "cross".into(),
        pos_side: None,
    };
    assert!(
        expect_http_error(client.set_leverage(leverage).await.unwrap_err())
            .contains(account::endpoints::SET_LEVERAGE)
    );

    let lever_info = GetLeverageInfoParams {
        mgn_mode: "cross".into(),
        ccy: Some("USDT".into()),
        inst_id: None,
    };
    assert!(
        expect_http_error(client.get_leverage_info(lever_info).await.unwrap_err())
            .contains(account::endpoints::LEVERAGE_INFO)
    );

    let max_size = GetMaxSizeParams {
        inst_id: "BTC-USDT".into(),
        td_mode: "cash".into(),
        ccy: None,
        px: Some("100".into()),
        leverage: None,
    };
    assert!(
        expect_http_error(client.get_max_size(max_size).await.unwrap_err())
            .contains(account::endpoints::MAX_SIZE)
    );

    let max_avail = GetMaxAvailSizeParams {
        inst_id: "BTC-USDT".into(),
        td_mode: "cash".into(),
        ccy: None,
        reduce_only: Some(false),
        quick_mgn_type: None,
    };
    assert!(
        expect_http_error(client.get_max_avail_size(max_avail).await.unwrap_err())
            .contains(account::endpoints::MAX_AVAIL_SIZE)
    );

    let max_loan = GetMaxLoanParams {
        inst_id: "BTC-USDT".into(),
        mgn_mode: "cross".into(),
        mgn_ccy: Some("USDT".into()),
    };
    assert!(
        expect_http_error(client.get_max_loan(max_loan).await.unwrap_err())
            .contains(account::endpoints::MAX_LOAN)
    );

    let fee = okx_rest::api::account::GetFeeRatesParams {
        inst_type: "SPOT".into(),
        inst_id: Some("BTC-USDT".into()),
        uly: None,
        inst_family: None,
    };
    assert!(
        expect_http_error(client.get_fee_rates(fee).await.unwrap_err())
            .contains(account::endpoints::TRADE_FEE)
    );

    assert!(expect_http_error(
        client
            .set_position_mode("long_short_mode")
            .await
            .unwrap_err()
    )
    .contains(account::endpoints::SET_POSITION_MODE));

    assert!(
        expect_http_error(client.get_account_position_risk().await.unwrap_err())
            .contains(account::endpoints::POSITION_RISK)
    );
    assert!(
        expect_http_error(client.get_account_risk_state().await.unwrap_err())
            .contains(account::endpoints::RISK_STATE)
    );

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
    assert!(
        expect_http_error(client.get_positions_history(Some(hist)).await.unwrap_err())
            .contains(account::endpoints::POSITIONS_HISTORY)
    );

    assert!(
        expect_http_error(client.get_max_withdrawal(Some("USDT")).await.unwrap_err())
            .contains(account::endpoints::MAX_WITHDRAWAL)
    );

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
    assert!(
        expect_http_error(client.get_account_bills(Some(bills)).await.unwrap_err())
            .contains(account::endpoints::BILLS)
    );

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
    assert!(
        expect_http_error(client.get_account_bills_archive(archive).await.unwrap_err())
            .contains(account::endpoints::BILLS_ARCHIVE)
    );

    let set_greeks = SetGreeksRequest {
        greeks_type: "PA".into(),
    };
    assert!(
        expect_http_error(client.set_greeks(set_greeks).await.unwrap_err())
            .contains(account::endpoints::SET_GREEKS)
    );

    let isolated = okx_rest::api::account::SetIsolatedModeRequest {
        iso_mode: "automatic".into(),
        r#type: "MARGIN".into(),
    };
    assert!(
        expect_http_error(client.set_isolated_mode(isolated).await.unwrap_err())
            .contains(account::endpoints::SET_ISOLATED_MODE)
    );

    let level = SetAccountLevelRequest {
        acct_lv: "2".into(),
    };
    assert!(
        expect_http_error(client.set_account_level(level).await.unwrap_err())
            .contains(account::endpoints::SET_ACCOUNT_LEVEL)
    );

    let borrow = BorrowRepayRequest {
        ccy: Some("USDT".into()),
        side: Some("borrow".into()),
        amt: Some("1".into()),
        ord_id: None,
    };
    assert!(
        expect_http_error(client.borrow_repay(borrow).await.unwrap_err())
            .contains(account::endpoints::BORROW_REPAY)
    );

    assert!(
        expect_http_error(client.get_borrow_repay_history(None).await.unwrap_err())
            .contains(account::endpoints::BORROW_REPAY_HISTORY)
    );

    let spot = SpotManualBorrowRepayRequest {
        ccy: Some("USDT".into()),
        side: Some("borrow".into()),
        amt: Some("1".into()),
    };
    assert!(
        expect_http_error(client.spot_manual_borrow_repay(spot).await.unwrap_err())
            .contains(account::endpoints::SPOT_MANUAL_BORROW_REPAY)
    );

    let spot_hist = SpotBorrowRepayHistoryParams {
        ccy: Some("USDT".into()),
        r#type: Some("borrow".into()),
        after: None,
        before: None,
        limit: Some("1".into()),
    };
    assert!(expect_http_error(
        client
            .spot_borrow_repay_history(Some(spot_hist))
            .await
            .unwrap_err()
    )
    .contains(account::endpoints::SPOT_BORROW_REPAY_HISTORY));

    let adj = AdjustmentMarginRequest {
        inst_id: "BTC-USDT-SWAP".into(),
        pos_side: "net".into(),
        r#type: "add".into(),
        amt: "1".into(),
        loan_trans: Some(false),
    };
    assert!(
        expect_http_error(client.adjustment_margin(adj).await.unwrap_err())
            .contains(account::endpoints::ADJUSTMENT_MARGIN)
    );

    let risk = SetRiskOffsetTypeRequest { r#type: "1".into() };
    assert!(
        expect_http_error(client.set_risk_offset_type(risk).await.unwrap_err())
            .contains(account::endpoints::SET_RISK_OFFSET_TYPE)
    );

    let auto_loan = SetAutoLoanRequest {
        auto_loan: Some("true".into()),
    };
    assert!(
        expect_http_error(client.set_auto_loan(auto_loan).await.unwrap_err())
            .contains(account::endpoints::SET_AUTO_LOAN)
    );

    assert!(
        expect_http_error(client.activate_option().await.unwrap_err())
            .contains(account::endpoints::ACTIVATE_OPTION)
    );

    let auto_repay = SetAutoRepayRequest {
        auto_repay: Some(true),
    };
    assert!(
        expect_http_error(client.set_auto_repay(auto_repay).await.unwrap_err())
            .contains(account::endpoints::SET_AUTO_REPAY)
    );

    let interest_limits = GetInterestLimitsParams {
        r#type: Some("1".into()),
        ccy: Some("USDT".into()),
    };
    assert!(expect_http_error(
        client
            .get_interest_limits(Some(interest_limits))
            .await
            .unwrap_err()
    )
    .contains(account::endpoints::INTEREST_LIMITS));

    let vip_list = GetVipLoanOrderListParams {
        ord_id: Some("1".into()),
        state: Some("filled".into()),
        ccy: Some("USDT".into()),
        after: Some("10".into()),
        before: Some("1".into()),
        limit: Some("20".into()),
    };
    assert!(expect_http_error(
        client
            .get_vip_loan_order_list(Some(vip_list))
            .await
            .unwrap_err()
    )
    .contains(account::endpoints::VIP_LOAN_ORDER_LIST));

    let vip_detail = GetVipLoanOrderDetailParams {
        ccy: Some("USDT".into()),
        ord_id: Some("2".into()),
        after: None,
        before: None,
        limit: None,
    };
    assert!(expect_http_error(
        client
            .get_vip_loan_order_detail(Some(vip_detail))
            .await
            .unwrap_err()
    )
    .contains(account::endpoints::VIP_LOAN_ORDER_DETAIL));

    assert!(
        expect_http_error(client.get_fix_loan_borrowing_limit().await.unwrap_err())
            .contains(account::endpoints::FIX_LOAN_BORROWING_LIMIT)
    );

    let quote = GetFixLoanBorrowingQuoteParams {
        r#type: Some("1".into()),
        ccy: Some("USDT".into()),
        amt: Some("10".into()),
        max_rate: Some("0.02".into()),
        term: Some("7".into()),
        ord_id: Some("123".into()),
    };
    assert!(expect_http_error(
        client
            .get_fix_loan_borrowing_quote(Some(quote))
            .await
            .unwrap_err()
    )
    .contains(account::endpoints::FIX_LOAN_BORROWING_QUOTE));

    let order = FixLoanBorrowingOrderRequest {
        ccy: Some("USDT".into()),
        amt: Some("10".into()),
        max_rate: Some("0.02".into()),
        term: Some("7".into()),
        reborrow: Some(true),
        reborrow_rate: Some("0.03".into()),
    };
    assert!(expect_http_error(
        client
            .place_fix_loan_borrowing_order(order)
            .await
            .unwrap_err()
    )
    .contains(account::endpoints::FIX_LOAN_BORROWING_ORDER));

    let amend = AmendFixLoanBorrowingOrderRequest {
        ord_id: Some("123".into()),
        reborrow: Some(false),
        renew_max_rate: Some("0.01".into()),
    };
    assert!(expect_http_error(
        client
            .amend_fix_loan_borrowing_order(amend)
            .await
            .unwrap_err()
    )
    .contains(account::endpoints::FIX_LOAN_AMEND_BORROWING_ORDER));

    let manual = FixLoanManualReborrowRequest {
        ord_id: Some("123".into()),
        max_rate: Some("0.03".into()),
    };
    assert!(
        expect_http_error(client.fix_loan_manual_reborrow(manual).await.unwrap_err())
            .contains(account::endpoints::FIX_LOAN_MANUAL_REBORROW)
    );

    let repay = RepayFixLoanBorrowingOrderRequest {
        ord_id: Some("123".into()),
    };
    assert!(expect_http_error(
        client
            .repay_fix_loan_borrowing_order(repay)
            .await
            .unwrap_err()
    )
    .contains(account::endpoints::FIX_LOAN_REPAY_BORROWING_ORDER));

    let list = GetFixLoanBorrowingOrdersListParams {
        ord_id: Some("123".into()),
        ccy: Some("USDT".into()),
        state: Some("filled".into()),
        after: None,
        before: None,
        limit: Some("1".into()),
    };
    assert!(expect_http_error(
        client
            .get_fix_loan_borrowing_orders_list(Some(list))
            .await
            .unwrap_err()
    )
    .contains(account::endpoints::FIX_LOAN_BORROWING_ORDERS_LIST));

    let accrued = GetInterestAccruedParams {
        inst_id: None,
        ccy: Some("USDT".into()),
        mgn_mode: Some("cross".into()),
        after: None,
        before: None,
        limit: Some("1".into()),
    };
    assert!(
        expect_http_error(client.get_interest_accrued(accrued).await.unwrap_err())
            .contains(account::endpoints::INTEREST_ACCRUED)
    );

    assert!(
        expect_http_error(client.get_interest_rate(Some("USDT")).await.unwrap_err())
            .contains(account::endpoints::INTEREST_RATE)
    );

    let vip_interest = GetVipInterestParams {
        ccy: Some("USDT".into()),
        ord_id: Some("1".into()),
        after: None,
        before: None,
        limit: Some("1".into()),
    };
    assert!(expect_http_error(
        client
            .get_vip_interest_accrued(vip_interest)
            .await
            .unwrap_err()
    )
    .contains(account::endpoints::VIP_INTEREST_ACCRUED));

    let vip_interest = GetVipInterestParams {
        ccy: Some("USDT".into()),
        ord_id: Some("1".into()),
        after: None,
        before: None,
        limit: Some("1".into()),
    };
    assert!(expect_http_error(
        client
            .get_vip_interest_deducted(vip_interest)
            .await
            .unwrap_err()
    )
    .contains(account::endpoints::VIP_INTEREST_DEDUCTED));

    let simulated = GetSimulatedMarginParams {
        inst_type: "SWAP".into(),
        incl_real_pos: Some(false),
        spot_offset_type: None,
        sim_pos: None,
    };
    assert!(
        expect_http_error(client.get_simulated_margin(simulated).await.unwrap_err())
            .contains(account::endpoints::SIMULATED_MARGIN)
    );

    let tiers = okx_rest::api::account::GetAccountPositionTiersParams {
        inst_type: "SWAP".into(),
        uly: None,
        inst_family: None,
    };
    assert!(
        expect_http_error(client.get_account_position_tiers(tiers).await.unwrap_err())
            .contains(account::endpoints::ACCOUNT_POSITION_TIERS)
    );

    let greeks = okx_rest::api::account::GetGreeksParams {
        ccy: Some("USDT".into()),
    };
    assert!(
        expect_http_error(client.get_greeks(greeks).await.unwrap_err())
            .contains(account::endpoints::GREEKS)
    );

    let builder = PositionBuilderRequest {
        acct_lv: Some("2".into()),
        incl_real_pos_and_eq: Some(false),
        lever: Some("5".into()),
        greeks_type: Some("PA".into()),
        sim_pos: Some(json!([])),
        sim_asset: Some(json!([])),
    };
    assert!(
        expect_http_error(client.position_builder(builder).await.unwrap_err())
            .contains(account::endpoints::POSITION_BUILDER)
    );
}

#[tokio::test]
async fn funding_api_offline_covers_all_methods() {
    let client = dummy_client();

    assert!(
        expect_http_error(client.get_asset_balances(Some("USDT")).await.unwrap_err())
            .contains(funding::endpoints::BALANCES)
    );

    assert!(
        expect_http_error(client.get_deposit_address("USDT").await.unwrap_err())
            .contains(funding::endpoints::DEPOSIT_ADDRESS)
    );

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
    assert!(expect_http_error(
        client
            .get_deposit_history(Some(dep_hist))
            .await
            .unwrap_err()
    )
    .contains(funding::endpoints::DEPOSIT_HISTORY));

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
    assert!(expect_http_error(
        client
            .get_withdrawal_history(Some(wd_hist))
            .await
            .unwrap_err()
    )
    .contains(funding::endpoints::WITHDRAWAL_HISTORY));

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
    assert!(
        expect_http_error(client.funds_transfer(transfer).await.unwrap_err())
            .contains(funding::endpoints::TRANSFER)
    );

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
    assert!(
        expect_http_error(client.withdrawal(withdrawal).await.unwrap_err())
            .contains(funding::endpoints::WITHDRAWAL)
    );

    assert!(
        expect_http_error(client.get_currencies(Some("USDT")).await.unwrap_err())
            .contains(funding::endpoints::CURRENCIES)
    );

    assert!(expect_http_error(
        client
            .get_non_tradable_assets(Some("USDT"))
            .await
            .unwrap_err()
    )
    .contains(funding::endpoints::NON_TRADABLE_ASSETS));

    let valuation = GetAssetValuationParams {
        ccy: Some("USDT".into()),
    };
    assert!(expect_http_error(
        client
            .get_asset_valuation(Some(valuation))
            .await
            .unwrap_err()
    )
    .contains(funding::endpoints::ASSET_VALUATION));

    let transfer_state = GetTransferStateParams {
        trans_id: "1".into(),
        r#type: Some("0".into()),
    };
    assert!(
        expect_http_error(client.get_transfer_state(transfer_state).await.unwrap_err())
            .contains(funding::endpoints::TRANSFER_STATE)
    );

    let bills = GetFundingBillsParams {
        ccy: Some("USDT".into()),
        r#type: None,
        after: None,
        before: None,
        limit: Some("1".into()),
    };
    assert!(
        expect_http_error(client.get_funding_bills(Some(bills)).await.unwrap_err())
            .contains(funding::endpoints::BILLS)
    );

    let pr = PurchaseRedemptRequest {
        ccy: "USDT".into(),
        amt: "1".into(),
        side: "purchase".into(),
        rate: None,
    };
    assert!(
        expect_http_error(client.purchase_redempt(pr).await.unwrap_err())
            .contains(funding::endpoints::PURCHASE_REDEMPT)
    );

    let dep_ln = GetDepositLightningParams {
        ccy: "BTC".into(),
        amt: "1".into(),
        to: None,
    };
    assert!(
        expect_http_error(client.get_deposit_lightning(dep_ln).await.unwrap_err())
            .contains(funding::endpoints::DEPOSIT_LIGHTNING)
    );

    let wd_ln = WithdrawalLightningRequest {
        ccy: "BTC".into(),
        invoice: "invoice".into(),
        memo: None,
    };
    assert!(
        expect_http_error(client.withdrawal_lightning(wd_ln).await.unwrap_err())
            .contains(funding::endpoints::WITHDRAWAL_LIGHTNING)
    );

    let cancel = CancelWithdrawalParams {
        wd_id: Some("1".into()),
    };
    assert!(
        expect_http_error(client.cancel_withdrawal(cancel).await.unwrap_err())
            .contains(funding::endpoints::CANCEL_WITHDRAWAL)
    );

    let status = GetDepositWithdrawStatusParams {
        wd_id: Some("1".into()),
        tx_id: None,
        ccy: Some("USDT".into()),
        to: None,
        chain: None,
    };
    assert!(expect_http_error(
        client
            .get_deposit_withdraw_status(status)
            .await
            .unwrap_err()
    )
    .contains(funding::endpoints::DEPOSIT_WITHDRAW_STATUS));

    let set_rate = SetLendingRateRequest {
        ccy: "USDT".into(),
        rate: "0.01".into(),
    };
    assert!(
        expect_http_error(client.set_lending_rate(set_rate).await.unwrap_err())
            .contains(funding::endpoints::SET_LENDING_RATE)
    );

    let lend_hist = GetLendingHistoryParams {
        ccy: Some("USDT".into()),
        after: None,
        before: None,
        limit: Some("1".into()),
    };
    assert!(expect_http_error(
        client
            .get_lending_history(Some(lend_hist))
            .await
            .unwrap_err()
    )
    .contains(funding::endpoints::LENDING_HISTORY));

    let rate_hist = GetLendingRateHistoryParams {
        ccy: Some("USDT".into()),
        after: None,
        before: None,
        limit: Some("1".into()),
    };
    assert!(expect_http_error(
        client
            .get_lending_rate_history(Some(rate_hist))
            .await
            .unwrap_err()
    )
    .contains(funding::endpoints::LENDING_RATE_HISTORY));

    let summary = GetLendingRateSummaryParams {
        ccy: Some("USDT".into()),
    };
    assert!(expect_http_error(
        client
            .get_lending_rate_summary(Some(summary))
            .await
            .unwrap_err()
    )
    .contains(funding::endpoints::LENDING_RATE_SUMMARY));

    let dust = ConvertDustAssetsRequest {
        ccy: Some(vec!["BTC".into()]),
    };
    assert!(
        expect_http_error(client.convert_dust_assets(dust).await.unwrap_err())
            .contains(funding::endpoints::CONVERT_DUST_ASSETS)
    );

    let saving = GetSavingBalanceParams {
        ccy: Some("USDT".into()),
    };
    assert!(
        expect_http_error(client.get_saving_balance(Some(saving)).await.unwrap_err())
            .contains(funding::endpoints::SAVING_BALANCE)
    );
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
    assert!(
        expect_http_error(client.place_order(place).await.unwrap_err())
            .contains(trade::endpoints::PLACE_ORDER)
    );

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
    assert!(
        expect_http_error(client.place_batch_orders(batch).await.unwrap_err())
            .contains(trade::endpoints::PLACE_BATCH_ORDERS)
    );

    let cancel = CancelOrderRequest {
        inst_id: "BTC-USDT".into(),
        ord_id: Some("1".into()),
        cl_ord_id: None,
    };
    assert!(
        expect_http_error(client.cancel_order(cancel).await.unwrap_err())
            .contains(trade::endpoints::CANCEL_ORDER)
    );

    let cancel_batch = vec![CancelOrderRequest {
        inst_id: "BTC-USDT".into(),
        ord_id: Some("1".into()),
        cl_ord_id: None,
    }];
    assert!(
        expect_http_error(client.cancel_batch_orders(cancel_batch).await.unwrap_err())
            .contains(trade::endpoints::CANCEL_BATCH_ORDERS)
    );

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
    assert!(
        expect_http_error(client.amend_order(amend).await.unwrap_err())
            .contains(trade::endpoints::AMEND_ORDER)
    );

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
    assert!(
        expect_http_error(client.amend_batch_orders(amend_batch).await.unwrap_err())
            .contains(trade::endpoints::AMEND_BATCH_ORDERS)
    );

    let order = GetOrderParams {
        inst_id: "BTC-USDT".into(),
        ord_id: Some("1".into()),
        cl_ord_id: None,
    };
    assert!(
        expect_http_error(client.get_order(order).await.unwrap_err())
            .contains(trade::endpoints::GET_ORDER)
    );

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
    assert!(
        expect_http_error(client.get_orders_pending(Some(pending)).await.unwrap_err())
            .contains(trade::endpoints::ORDERS_PENDING)
    );

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
    assert!(
        expect_http_error(client.get_orders_history(hist).await.unwrap_err())
            .contains(trade::endpoints::ORDERS_HISTORY)
    );

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
    assert!(expect_http_error(
        client
            .get_orders_history_archive(hist_archive)
            .await
            .unwrap_err()
    )
    .contains(trade::endpoints::ORDERS_HISTORY_ARCHIVE));

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
    assert!(
        expect_http_error(client.get_fills(Some(fills)).await.unwrap_err())
            .contains(trade::endpoints::FILLS)
    );

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
    assert!(
        expect_http_error(client.get_fills_history(fills_hist).await.unwrap_err())
            .contains(trade::endpoints::FILLS_HISTORY)
    );

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
    assert!(
        expect_http_error(client.place_algo_order(algo).await.unwrap_err())
            .contains(trade::endpoints::PLACE_ALGO_ORDER)
    );

    let cancel_algo = vec![CancelAlgoOrderRequest {
        inst_id: "BTC-USDT".into(),
        algo_id: "1".into(),
    }];
    assert!(
        expect_http_error(client.cancel_algo_orders(cancel_algo).await.unwrap_err())
            .contains(trade::endpoints::CANCEL_ALGO_ORDERS)
    );

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
    assert!(
        expect_http_error(client.amend_algo_order(amend_algo).await.unwrap_err())
            .contains(trade::endpoints::AMEND_ALGO_ORDER)
    );

    let algo_pending = GetAlgoOrdersParams {
        ord_type: "conditional".into(),
        algo_id: None,
        inst_type: None,
        inst_id: None,
        after: None,
        before: None,
        limit: Some("1".into()),
    };
    assert!(expect_http_error(
        client
            .get_algo_orders_pending(algo_pending)
            .await
            .unwrap_err()
    )
    .contains(trade::endpoints::ALGO_ORDERS_PENDING));

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
    assert!(
        expect_http_error(client.get_algo_orders_history(algo_hist).await.unwrap_err())
            .contains(trade::endpoints::ALGO_ORDERS_HISTORY)
    );

    let details = GetAlgoOrderDetailsParams {
        algo_id: Some("1".into()),
        algo_cl_ord_id: None,
    };
    assert!(
        expect_http_error(client.get_algo_order_details(details).await.unwrap_err())
            .contains(trade::endpoints::ALGO_ORDER_DETAILS)
    );

    let close = ClosePositionRequest {
        inst_id: "BTC-USDT-SWAP".into(),
        mgn_mode: "cross".into(),
        pos_side: Some("net".into()),
        ccy: None,
        auto_cancel: None,
        cl_ord_id: None,
        tag: None,
    };
    assert!(
        expect_http_error(client.close_position(close).await.unwrap_err())
            .contains(trade::endpoints::CLOSE_POSITION)
    );

    assert!(expect_http_error(
        client
            .mass_cancel(json!({"instType":"SWAP"}))
            .await
            .unwrap_err()
    )
    .contains(trade::endpoints::MASS_CANCEL));
    assert!(expect_http_error(
        client
            .cancel_all_after(json!({"timeOut":"1"}))
            .await
            .unwrap_err()
    )
    .contains(trade::endpoints::CANCEL_ALL_AFTER));
    assert!(expect_http_error(
        client
            .order_precheck(json!({"instId":"BTC-USDT"}))
            .await
            .unwrap_err()
    )
    .contains(trade::endpoints::ORDER_PRECHECK));

    assert!(expect_http_error(
        client
            .get_one_click_repay_currency_list_v2()
            .await
            .unwrap_err()
    )
    .contains(trade::endpoints::ONE_CLICK_REPAY_CURRENCY_LIST_V2));

    let repay = OneClickRepayV2Request {
        debt_ccy: "BTC".into(),
        repay_ccy_list: vec!["USDT".into()],
    };
    assert!(
        expect_http_error(client.one_click_repay_v2(repay).await.unwrap_err())
            .contains(trade::endpoints::ONE_CLICK_REPAY_V2)
    );

    let repay_hist = OneClickRepayHistoryV2Params {
        after: Some("1".into()),
        before: Some("2".into()),
        limit: Some("1".into()),
    };
    assert!(expect_http_error(
        client
            .get_one_click_repay_history_v2(Some(repay_hist))
            .await
            .unwrap_err()
    )
    .contains(trade::endpoints::ONE_CLICK_REPAY_HISTORY_V2));

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
