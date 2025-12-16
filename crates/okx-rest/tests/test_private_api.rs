#![allow(missing_docs)]

use okx_core::types::EstimateQuoteParams;
use okx_core::types::{
    AmendOrderRequest, AttachAlgoOrdRequest, CancelAlgoOrderRequest, CancelOrderRequest,
    PlaceAlgoOrderRequest,
};
use okx_core::{
    types::{FundsTransferRequest, PlaceOrderRequest, WithdrawalRequest},
    Config, Credentials,
};
use okx_rest::api::account::{
    self, AdjustmentMarginRequest, AmendFixLoanBorrowingOrderRequest, FixLoanBorrowingOrderRequest,
    FixLoanManualReborrowRequest, GetFeeRatesParams, GetFixLoanBorrowingOrdersListParams,
    GetFixLoanBorrowingQuoteParams, GetInterestLimitsParams, GetLeverageInfoParams,
    GetMaxAvailSizeParams, GetMaxSizeParams, GetPositionsParams, GetVipLoanOrderDetailParams,
    GetVipLoanOrderListParams, RepayFixLoanBorrowingOrderRequest, SetAutoLoanRequest,
    SetAutoRepayRequest, SetLeverageRequest, SetRiskOffsetTypeRequest,
};
use okx_rest::api::block_rfq;
use okx_rest::api::broker;
use okx_rest::api::convert;
use okx_rest::api::copy_trading;
use okx_rest::api::finance;
use okx_rest::api::funding::{self, GetDepositHistoryParams, GetWithdrawalHistoryParams};
use okx_rest::api::grid;
use okx_rest::api::status;
use okx_rest::api::subaccount::{self, SubaccountTransferRequest};
use okx_rest::api::trade::{
    ClosePositionRequest, GetAlgoOrdersHistoryParams, GetAlgoOrdersParams, GetFillsParams,
    GetOrderParams, GetOrdersHistoryParams, GetOrdersPendingParams,
};
use okx_rest::api::trading_data;
use okx_rest::api::{funding::endpoints as funding_ep, spread, trade};
use okx_rest::{
    AccountApi, BlockRfqApi, BrokerApi, ConvertApi, CopyTradingApi, FinanceApi, FundingApi,
    GridApi, OkxError, OkxRestClient, SpreadApi, StatusApi, SubaccountApi, TradeApi,
    TradingDataApi,
};
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
        OkxError::HttpStatus { status, body } => format!("HTTP {status}: {body}"),
        other => panic!("预期 HTTP 错误，实际为: {other:?}"),
    }
}

#[tokio::test]
async fn account_methods_build_params_and_return_http_error() {
    let client = dummy_client();

    let msg = expect_http_error(client.get_balance(Some("USDT")).await.unwrap_err());
    assert!(msg.contains(account::endpoints::BALANCE));

    let params = GetPositionsParams {
        inst_type: Some("SWAP".into()),
        inst_id: Some("BTC-USDT-SWAP".into()),
        pos_id: None,
    };
    let msg = expect_http_error(client.get_positions(Some(params)).await.unwrap_err());
    assert!(msg.contains(account::endpoints::POSITIONS));

    let leverage_req = SetLeverageRequest {
        inst_id: Some("BTC-USDT-SWAP".into()),
        ccy: None,
        lever: "5".into(),
        mgn_mode: "cross".into(),
        pos_side: None,
    };
    let body = to_value(&leverage_req).expect("序列化 setLeverage 请求失败");
    assert_eq!(body["lever"], "5");
    let msg = expect_http_error(client.set_leverage(leverage_req).await.unwrap_err());
    assert!(msg.contains(account::endpoints::SET_LEVERAGE));

    let lever_info = GetLeverageInfoParams {
        mgn_mode: "cross".into(),
        ccy: Some("USDT".into()),
        inst_id: None,
    };
    let msg = expect_http_error(client.get_leverage_info(lever_info).await.unwrap_err());
    assert!(msg.contains(account::endpoints::LEVERAGE_INFO));

    let msg = expect_http_error(client.get_account_config().await.unwrap_err());
    assert!(msg.contains(account::endpoints::CONFIG));

    let msg = expect_http_error(client.get_positions_history(None).await.unwrap_err());
    assert!(msg.contains(account::endpoints::POSITIONS_HISTORY));

    let msg = expect_http_error(client.get_max_withdrawal(None).await.unwrap_err());
    assert!(msg.contains(account::endpoints::MAX_WITHDRAWAL));

    let msg = expect_http_error(client.get_account_bills(None).await.unwrap_err());
    assert!(msg.contains(account::endpoints::BILLS));

    let msg = expect_http_error(client.get_borrow_repay_history(None).await.unwrap_err());
    assert!(msg.contains(account::endpoints::BORROW_REPAY_HISTORY));

    let msg = expect_http_error(client.spot_borrow_repay_history(None).await.unwrap_err());
    assert!(msg.contains(account::endpoints::SPOT_BORROW_REPAY_HISTORY));

    let msg = expect_http_error(client.get_interest_rate(None).await.unwrap_err());
    assert!(msg.contains(account::endpoints::INTEREST_RATE));
}

#[tokio::test]
async fn account_margin_and_risk_settings_paths() {
    let client = dummy_client();

    let adj = AdjustmentMarginRequest {
        inst_id: "BTC-USDT-SWAP".into(),
        pos_side: "net".into(),
        r#type: "add".into(),
        amt: "1".into(),
        loan_trans: Some(false),
    };
    let body = to_value(&adj).expect("序列化 adjustment_margin 请求失败");
    assert_eq!(body["instId"], "BTC-USDT-SWAP");
    let msg = expect_http_error(client.adjustment_margin(adj).await.unwrap_err());
    assert!(msg.contains(account::endpoints::ADJUSTMENT_MARGIN));

    let risk = SetRiskOffsetTypeRequest { r#type: "1".into() };
    let msg = expect_http_error(client.set_risk_offset_type(risk).await.unwrap_err());
    assert!(msg.contains(account::endpoints::SET_RISK_OFFSET_TYPE));

    let auto = SetAutoLoanRequest {
        auto_loan: Some("true".into()),
    };
    let body = to_value(&auto).expect("序列化 set_auto_loan 请求失败");
    assert_eq!(body["autoLoan"], "true");
    let msg = expect_http_error(client.set_auto_loan(auto).await.unwrap_err());
    assert!(msg.contains(account::endpoints::SET_AUTO_LOAN));
}

#[tokio::test]
async fn account_new_endpoints_paths() {
    let client = dummy_client();

    let msg = expect_http_error(client.activate_option().await.unwrap_err());
    assert!(msg.contains(account::endpoints::ACTIVATE_OPTION));

    let auto_repay = SetAutoRepayRequest {
        auto_repay: Some(true),
    };
    let body = to_value(&auto_repay).expect("序列化 set_auto_repay 请求失败");
    assert_eq!(body["autoRepay"], true);
    let msg = expect_http_error(client.set_auto_repay(auto_repay).await.unwrap_err());
    assert!(msg.contains(account::endpoints::SET_AUTO_REPAY));

    let interest_limits = GetInterestLimitsParams {
        r#type: Some("1".into()),
        ccy: Some("USDT".into()),
    };
    let body = to_value(&interest_limits).expect("序列化 interest_limits 请求失败");
    assert_eq!(body["type"], "1");
    let msg = expect_http_error(
        client
            .get_interest_limits(Some(interest_limits))
            .await
            .unwrap_err(),
    );
    assert!(msg.contains(account::endpoints::INTEREST_LIMITS));

    let vip_list = GetVipLoanOrderListParams {
        ord_id: Some("1".into()),
        state: Some("filled".into()),
        ccy: Some("USDT".into()),
        after: Some("10".into()),
        before: Some("1".into()),
        limit: Some("20".into()),
    };
    let body = to_value(&vip_list).expect("序列化 vip loan list 失败");
    assert_eq!(body["ordId"], "1");
    let msg = expect_http_error(
        client
            .get_vip_loan_order_list(Some(vip_list))
            .await
            .unwrap_err(),
    );
    assert!(msg.contains(account::endpoints::VIP_LOAN_ORDER_LIST));

    let vip_detail = GetVipLoanOrderDetailParams {
        ccy: Some("USDT".into()),
        ord_id: Some("2".into()),
        after: None,
        before: None,
        limit: None,
    };
    let body = to_value(&vip_detail).expect("序列化 vip loan detail 失败");
    assert_eq!(body["ordId"], "2");
    let msg = expect_http_error(
        client
            .get_vip_loan_order_detail(Some(vip_detail))
            .await
            .unwrap_err(),
    );
    assert!(msg.contains(account::endpoints::VIP_LOAN_ORDER_DETAIL));

    let msg = expect_http_error(client.get_fix_loan_borrowing_limit().await.unwrap_err());
    assert!(msg.contains(account::endpoints::FIX_LOAN_BORROWING_LIMIT));

    let quote_params = GetFixLoanBorrowingQuoteParams {
        r#type: Some("1".into()),
        ccy: Some("USDT".into()),
        amt: Some("10".into()),
        max_rate: Some("0.02".into()),
        term: Some("7".into()),
        ord_id: Some("123".into()),
    };
    let body = to_value(&quote_params).expect("序列化 fix loan quote 失败");
    assert_eq!(body["maxRate"], "0.02");
    let msg = expect_http_error(
        client
            .get_fix_loan_borrowing_quote(Some(quote_params))
            .await
            .unwrap_err(),
    );
    assert!(msg.contains(account::endpoints::FIX_LOAN_BORROWING_QUOTE));

    let order_req = FixLoanBorrowingOrderRequest {
        ccy: Some("USDT".into()),
        amt: Some("10".into()),
        max_rate: Some("0.02".into()),
        term: Some("7".into()),
        reborrow: Some(true),
        reborrow_rate: Some("0.03".into()),
    };
    let body = to_value(&order_req).expect("序列化 fix loan order 失败");
    assert_eq!(body["reborrow"], true);
    let msg = expect_http_error(
        client
            .place_fix_loan_borrowing_order(order_req)
            .await
            .unwrap_err(),
    );
    assert!(msg.contains(account::endpoints::FIX_LOAN_BORROWING_ORDER));

    let amend_req = AmendFixLoanBorrowingOrderRequest {
        ord_id: Some("123".into()),
        reborrow: Some(false),
        renew_max_rate: Some("0.01".into()),
    };
    let body = to_value(&amend_req).expect("序列化 amend fix loan 失败");
    assert_eq!(body["renewMaxRate"], "0.01");
    let msg = expect_http_error(
        client
            .amend_fix_loan_borrowing_order(amend_req)
            .await
            .unwrap_err(),
    );
    assert!(msg.contains(account::endpoints::FIX_LOAN_AMEND_BORROWING_ORDER));

    let manual_req = FixLoanManualReborrowRequest {
        ord_id: Some("123".into()),
        max_rate: Some("0.02".into()),
    };
    let msg = expect_http_error(
        client
            .fix_loan_manual_reborrow(manual_req)
            .await
            .unwrap_err(),
    );
    assert!(msg.contains(account::endpoints::FIX_LOAN_MANUAL_REBORROW));

    let repay_req = RepayFixLoanBorrowingOrderRequest {
        ord_id: Some("123".into()),
    };
    let msg = expect_http_error(
        client
            .repay_fix_loan_borrowing_order(repay_req)
            .await
            .unwrap_err(),
    );
    assert!(msg.contains(account::endpoints::FIX_LOAN_REPAY_BORROWING_ORDER));

    let list_params = GetFixLoanBorrowingOrdersListParams {
        ord_id: Some("123".into()),
        ccy: Some("USDT".into()),
        state: Some("filled".into()),
        after: Some("1".into()),
        before: Some("0".into()),
        limit: Some("10".into()),
    };
    let body = to_value(&list_params).expect("序列化 fix loan list 失败");
    assert_eq!(body["state"], "filled");
    let msg = expect_http_error(
        client
            .get_fix_loan_borrowing_orders_list(Some(list_params))
            .await
            .unwrap_err(),
    );
    assert!(msg.contains(account::endpoints::FIX_LOAN_BORROWING_ORDERS_LIST));
}

#[tokio::test]
async fn finance_simple_earn_endpoints_paths() {
    let client = dummy_client();

    let offers_params = json!({"ccy": "USDT"});
    let msg = expect_http_error(
        client
            .simple_earn_get_lending_offers(Some(offers_params))
            .await
            .unwrap_err(),
    );
    assert!(msg.contains(finance::endpoints::SIMPLE_EARN_LENDING_OFFERS));

    let apy_params = json!({"ccy": "USDT", "term": "7"});
    let msg = expect_http_error(
        client
            .simple_earn_get_lending_apy_history(Some(apy_params))
            .await
            .unwrap_err(),
    );
    assert!(msg.contains(finance::endpoints::SIMPLE_EARN_LENDING_APY_HISTORY));

    let pending_params = json!({"ccy": "USDT"});
    let msg = expect_http_error(
        client
            .simple_earn_get_pending_lending_volume(Some(pending_params))
            .await
            .unwrap_err(),
    );
    assert!(msg.contains(finance::endpoints::SIMPLE_EARN_PENDING_LENDING_VOLUME));

    let place_body = json!({
        "ccy": "USDT",
        "amt": "10",
        "rate": "0.02",
        "term": "7",
        "autoRenewal": true
    });
    assert_eq!(place_body["rate"], "0.02");
    let msg = expect_http_error(
        client
            .simple_earn_place_lending_order(place_body)
            .await
            .unwrap_err(),
    );
    assert!(msg.contains(finance::endpoints::SIMPLE_EARN_LENDING_ORDER));

    let amend_body = json!({
        "ordId": "123",
        "changeAmt": "5",
        "rate": "0.01",
        "autoRenewal": false
    });
    assert_eq!(amend_body["changeAmt"], "5");
    let msg = expect_http_error(
        client
            .simple_earn_amend_lending_order(amend_body)
            .await
            .unwrap_err(),
    );
    assert!(msg.contains(finance::endpoints::SIMPLE_EARN_AMEND_LENDING_ORDER));

    let list_params = json!({"ccy": "USDT", "state": "ongoing"});
    let msg = expect_http_error(
        client
            .simple_earn_get_lending_orders_list(Some(list_params))
            .await
            .unwrap_err(),
    );
    assert!(msg.contains(finance::endpoints::SIMPLE_EARN_LENDING_ORDERS_LIST));

    let sub_params = json!({"ordId": "123"});
    let msg = expect_http_error(
        client
            .simple_earn_get_lending_sub_orders(Some(sub_params))
            .await
            .unwrap_err(),
    );
    assert!(msg.contains(finance::endpoints::SIMPLE_EARN_LENDING_SUB_ORDERS));
}

#[tokio::test]
async fn funding_methods_serialize_and_fail_fast() {
    let client = dummy_client();

    let msg = expect_http_error(client.get_currencies(Some("USDT")).await.unwrap_err());
    assert!(msg.contains(funding::endpoints::CURRENCIES));

    let balances = client.get_asset_balances(Some("USDT")).await.unwrap_err();
    assert!(expect_http_error(balances).contains(funding_ep::BALANCES));

    let transfer_req = FundsTransferRequest {
        ccy: "USDT".into(),
        amt: "1".into(),
        from: "6".into(),
        to: "18".into(),
        r#type: None,
        sub_acct: None,
        inst_id: None,
        to_inst_id: None,
        loan_trans: None,
    };
    let body = to_value(&transfer_req).expect("序列化 transfer 请求失败");
    assert_eq!(body["from"], "6");
    let msg = expect_http_error(client.funds_transfer(transfer_req).await.unwrap_err());
    assert!(msg.contains(funding::endpoints::TRANSFER));

    let msg = expect_http_error(client.get_deposit_address("USDT").await.unwrap_err());
    assert!(msg.contains(funding::endpoints::DEPOSIT_ADDRESS));

    let deposit_params = GetDepositHistoryParams::default();
    let msg = expect_http_error(
        client
            .get_deposit_history(Some(deposit_params))
            .await
            .unwrap_err(),
    );
    assert!(msg.contains(funding::endpoints::DEPOSIT_HISTORY));

    let withdraw_params = GetWithdrawalHistoryParams::default();
    let msg = expect_http_error(
        client
            .get_withdrawal_history(Some(withdraw_params))
            .await
            .unwrap_err(),
    );
    assert!(msg.contains(funding::endpoints::WITHDRAWAL_HISTORY));

    let msg = expect_http_error(client.get_non_tradable_assets(None).await.unwrap_err());
    assert!(msg.contains(funding::endpoints::NON_TRADABLE_ASSETS));

    let msg = expect_http_error(client.get_asset_valuation(None).await.unwrap_err());
    assert!(msg.contains(funding::endpoints::ASSET_VALUATION));

    let msg = expect_http_error(client.get_funding_bills(None).await.unwrap_err());
    assert!(msg.contains(funding::endpoints::BILLS));

    let msg = expect_http_error(client.get_lending_history(None).await.unwrap_err());
    assert!(msg.contains(funding::endpoints::LENDING_HISTORY));

    let msg = expect_http_error(client.get_lending_rate_history(None).await.unwrap_err());
    assert!(msg.contains(funding::endpoints::LENDING_RATE_HISTORY));

    let msg = expect_http_error(client.get_lending_rate_summary(None).await.unwrap_err());
    assert!(msg.contains(funding::endpoints::LENDING_RATE_SUMMARY));

    let msg = expect_http_error(client.get_saving_balance(None).await.unwrap_err());
    assert!(msg.contains(funding::endpoints::SAVING_BALANCE));
}

#[tokio::test]
async fn trade_place_order_builds_body_and_errors_with_path() {
    let client = dummy_client();

    let order_req = PlaceOrderRequest {
        inst_id: "BTC-USDT".into(),
        td_mode: "cash".into(),
        side: "buy".into(),
        ord_type: "market".into(),
        sz: "1".into(),
        ccy: None,
        cl_ord_id: None,
        tag: None,
        pos_side: None,
        px: None,
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

    let body = to_value(&order_req).expect("序列化 order 请求失败");
    assert_eq!(body["ordType"], "market");
    let msg = expect_http_error(client.place_order(order_req).await.unwrap_err());
    assert!(msg.contains(trade::endpoints::PLACE_ORDER));
}

#[tokio::test]
async fn trade_place_order_covers_all_optional_fields() {
    let client = dummy_client();
    let long_cl_id = "1234567890abcdef1234567890abcdef".to_string();
    let short_tag = "abcdefghijklmnop".to_string();

    let order_req = PlaceOrderRequest {
        inst_id: "BTC-USDT".into(),
        td_mode: "cross".into(),
        side: "sell".into(),
        ord_type: "limit".into(),
        sz: "1.23456789".into(),
        ccy: Some("USDT".into()),
        cl_ord_id: Some(long_cl_id.clone()),
        tag: Some(short_tag.clone()),
        pos_side: Some("long".into()),
        px: Some("20001.1234".into()),
        reduce_only: Some(true),
        tgt_ccy: Some("base_ccy".into()),
        tp_trigger_px: Some("21000".into()),
        tp_ord_px: Some("20990".into()),
        sl_trigger_px: Some("19000".into()),
        sl_ord_px: Some("18990".into()),
        tp_trigger_px_type: Some("last".into()),
        sl_trigger_px_type: Some("index".into()),
        quick_mgn_type: Some("manual".into()),
        stp_id: Some("999".into()),
        stp_mode: Some("cancel_maker".into()),
        attach_algo_ords: Some(vec![
            AttachAlgoOrdRequest {
                attach_algo_cl_ord_id: Some("tp-1".into()),
                tp_trigger_px: Some("21111".into()),
                tp_trigger_px_type: Some("mark".into()),
                tp_ord_px: Some("21110".into()),
                sl_trigger_px: None,
                sl_trigger_px_type: None,
                sl_ord_px: None,
                sz: Some("0.5".into()),
            },
            AttachAlgoOrdRequest {
                attach_algo_cl_ord_id: Some("sl-1".into()),
                tp_trigger_px: None,
                tp_trigger_px_type: None,
                tp_ord_px: None,
                sl_trigger_px: Some("18888".into()),
                sl_trigger_px_type: Some("last".into()),
                sl_ord_px: Some("18880".into()),
                sz: Some("0.7".into()),
            },
        ]),
    };

    let body = to_value(&order_req).expect("序列化 order 请求失败");
    assert_eq!(body["instId"], "BTC-USDT");
    assert_eq!(body["tdMode"], "cross");
    assert_eq!(body["ordType"], "limit");
    assert_eq!(body["px"], "20001.1234");
    assert_eq!(body["clOrdId"].as_str().unwrap().len(), 32);
    assert_eq!(body["tag"], short_tag);
    assert_eq!(body["posSide"], "long");
    assert!(
        body["reduceOnly"].as_bool().unwrap(),
        "reduceOnly 应为布尔值"
    );
    assert_eq!(body["tgtCcy"], "base_ccy");
    assert_eq!(body["tpTriggerPxType"], "last");
    assert_eq!(body["slTriggerPxType"], "index");
    assert_eq!(body["stpId"], "999");
    assert_eq!(body["stpMode"], "cancel_maker");

    let attach = body["attachAlgoOrds"]
        .as_array()
        .expect("附带算法订单应存在");
    assert_eq!(attach.len(), 2);
    assert_eq!(attach[0]["tpTriggerPx"], "21111");
    assert_eq!(attach[0]["tpOrdPx"], "21110");
    assert_eq!(attach[0]["sz"], "0.5");
    assert_eq!(attach[1]["slTriggerPx"], "18888");
    assert_eq!(attach[1]["slOrdPx"], "18880");
    assert_eq!(attach[1]["sz"], "0.7");

    let msg = expect_http_error(client.place_order(order_req).await.unwrap_err());
    assert!(
        msg.contains(trade::endpoints::PLACE_ORDER),
        "错误应包含下单路径，实际: {msg}"
    );
}

#[tokio::test]
async fn trade_other_operations_build_and_use_correct_paths() {
    let client = dummy_client();

    let get_order_params = GetOrderParams {
        inst_id: "BTC-USDT".into(),
        ord_id: Some("123".into()),
        cl_ord_id: None,
    };
    let msg = expect_http_error(client.get_order(get_order_params).await.unwrap_err());
    assert!(msg.contains(trade::endpoints::GET_ORDER));

    let pending_params = GetOrdersPendingParams::default();
    let msg = expect_http_error(
        client
            .get_orders_pending(Some(pending_params))
            .await
            .unwrap_err(),
    );
    assert!(msg.contains(trade::endpoints::ORDERS_PENDING));

    let cancel_req = CancelOrderRequest {
        inst_id: "BTC-USDT".into(),
        ord_id: None,
        cl_ord_id: Some("cid-1".into()),
    };
    let body = to_value(&cancel_req).expect("序列化 cancel 请求失败");
    assert_eq!(body["clOrdId"], "cid-1");
    let msg = expect_http_error(client.cancel_order(cancel_req).await.unwrap_err());
    assert!(msg.contains(trade::endpoints::CANCEL_ORDER));

    let amend_req = AmendOrderRequest {
        inst_id: "BTC-USDT".into(),
        ord_id: Some("123".into()),
        cl_ord_id: None,
        req_id: None,
        new_sz: Some("2".into()),
        new_px: None,
        new_tp_trigger_px: None,
        new_tp_ord_px: None,
        new_sl_trigger_px: None,
        new_sl_ord_px: None,
        new_tp_trigger_px_type: None,
        new_sl_trigger_px_type: None,
    };
    let body = to_value(&amend_req).expect("序列化 amend 请求失败");
    assert_eq!(body["newSz"], "2");
    let msg = expect_http_error(client.amend_order(amend_req).await.unwrap_err());
    assert!(msg.contains(trade::endpoints::AMEND_ORDER));
}

#[tokio::test]
async fn trade_batch_algo_and_close_position_paths() {
    let client = dummy_client();

    let batch_req = vec![PlaceOrderRequest {
        inst_id: "BTC-USDT".into(),
        td_mode: "cash".into(),
        side: "sell".into(),
        ord_type: "limit".into(),
        sz: "1".into(),
        ccy: None,
        cl_ord_id: Some("cid-1".into()),
        tag: Some("t".into()),
        pos_side: None,
        px: Some("20000".into()),
        reduce_only: Some(false),
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
    let msg = expect_http_error(client.place_batch_orders(batch_req).await.unwrap_err());
    assert!(msg.contains(trade::endpoints::PLACE_BATCH_ORDERS));

    let cancel_batch = vec![CancelOrderRequest {
        inst_id: "BTC-USDT".into(),
        ord_id: Some("123".into()),
        cl_ord_id: None,
    }];
    let msg = expect_http_error(client.cancel_batch_orders(cancel_batch).await.unwrap_err());
    assert!(msg.contains(trade::endpoints::CANCEL_BATCH_ORDERS));

    let history_params = GetOrdersHistoryParams {
        inst_type: "SPOT".into(),
        uly: None,
        inst_family: None,
        inst_id: Some("BTC-USDT".into()),
        ord_type: Some("limit".into()),
        state: Some("filled".into()),
        category: Some("twap".into()),
        after: None,
        before: Some("10".into()),
        begin: None,
        end: Some("20".into()),
        limit: Some("20".into()),
    };
    let msg = expect_http_error(client.get_orders_history(history_params).await.unwrap_err());
    assert!(msg.contains(trade::endpoints::ORDERS_HISTORY));

    let fills_params = GetFillsParams {
        inst_type: Some("SPOT".into()),
        uly: None,
        inst_family: None,
        inst_id: Some("BTC-USDT".into()),
        ord_id: Some("123".into()),
        after: Some("1".into()),
        before: Some("2".into()),
        begin: Some("1000".into()),
        end: Some("2000".into()),
        limit: Some("50".into()),
    };
    let msg = expect_http_error(client.get_fills(Some(fills_params)).await.unwrap_err());
    assert!(msg.contains(trade::endpoints::FILLS));

    let algo_req = PlaceAlgoOrderRequest {
        inst_id: "BTC-USDT".into(),
        td_mode: "cash".into(),
        side: "buy".into(),
        ord_type: "trigger".into(),
        sz: "1".into(),
        ccy: None,
        pos_side: None,
        reduce_only: None,
        tgt_ccy: None,
        algo_cl_ord_id: Some("algo-1".into()),
        trigger_px: Some("20000".into()),
        order_px: Some("20010".into()),
        trigger_px_type: Some("last".into()),
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
    let msg = expect_http_error(client.place_algo_order(algo_req).await.unwrap_err());
    assert!(msg.contains(trade::endpoints::PLACE_ALGO_ORDER));

    let cancel_algo = vec![CancelAlgoOrderRequest {
        algo_id: "1".into(),
        inst_id: "BTC-USDT".into(),
    }];
    let msg = expect_http_error(client.cancel_algo_orders(cancel_algo).await.unwrap_err());
    assert!(msg.contains(trade::endpoints::CANCEL_ALGO_ORDERS));

    let algo_pending = GetAlgoOrdersParams {
        ord_type: "trigger".into(),
        algo_id: Some("1".into()),
        inst_type: Some("SPOT".into()),
        inst_id: Some("BTC-USDT".into()),
        after: None,
        before: Some("10".into()),
        limit: Some("10".into()),
    };
    let msg = expect_http_error(
        client
            .get_algo_orders_pending(algo_pending)
            .await
            .unwrap_err(),
    );
    assert!(msg.contains(trade::endpoints::ALGO_ORDERS_PENDING));

    let algo_history = GetAlgoOrdersHistoryParams {
        ord_type: "trigger".into(),
        state: Some("canceled".into()),
        algo_id: Some("1".into()),
        inst_type: None,
        inst_id: Some("BTC-USDT".into()),
        after: Some("1".into()),
        before: Some("2".into()),
        limit: Some("5".into()),
    };
    let msg = expect_http_error(
        client
            .get_algo_orders_history(algo_history)
            .await
            .unwrap_err(),
    );
    assert!(msg.contains(trade::endpoints::ALGO_ORDERS_HISTORY));

    let close_req = ClosePositionRequest {
        inst_id: "BTC-USDT-SWAP".into(),
        mgn_mode: "cross".into(),
        pos_side: Some("long".into()),
        ccy: None,
        auto_cancel: Some(true),
        cl_ord_id: Some("cid-close".into()),
        tag: Some("close".into()),
    };
    let msg = expect_http_error(client.close_position(close_req).await.unwrap_err());
    assert!(msg.contains(trade::endpoints::CLOSE_POSITION));
}

#[tokio::test]
async fn account_additional_calls_cover_params_and_paths() {
    let client = dummy_client();

    let max_size = GetMaxSizeParams {
        inst_id: "BTC-USDT".into(),
        td_mode: "cash".into(),
        ccy: Some("USDT".into()),
        px: Some("20000".into()),
        leverage: Some("3".into()),
    };
    let body = to_value(&max_size).unwrap();
    assert_eq!(body["tdMode"], "cash");
    let msg = expect_http_error(client.get_max_size(max_size).await.unwrap_err());
    assert!(msg.contains(account::endpoints::MAX_SIZE));

    let max_avail = GetMaxAvailSizeParams {
        inst_id: "BTC-USDT".into(),
        td_mode: "cross".into(),
        ccy: None,
        reduce_only: Some(true),
        quick_mgn_type: Some("manual".into()),
    };
    let body = to_value(&max_avail).unwrap();
    assert_eq!(body["reduceOnly"], true);
    let msg = expect_http_error(client.get_max_avail_size(max_avail).await.unwrap_err());
    assert!(msg.contains(account::endpoints::MAX_AVAIL_SIZE));

    let fee_params = GetFeeRatesParams {
        inst_type: "SWAP".into(),
        inst_id: Some("BTC-USDT-SWAP".into()),
        uly: None,
        inst_family: Some("BTC-USD".into()),
    };
    let msg = expect_http_error(client.get_fee_rates(fee_params).await.unwrap_err());
    assert!(msg.contains(account::endpoints::TRADE_FEE));

    let msg = expect_http_error(
        client
            .set_position_mode("long_short_mode")
            .await
            .unwrap_err(),
    );
    assert!(msg.contains(account::endpoints::SET_POSITION_MODE));

    let msg = expect_http_error(client.get_account_position_risk().await.unwrap_err());
    assert!(msg.contains(account::endpoints::POSITION_RISK));
}

#[tokio::test]
async fn funding_more_paths_and_requests_are_serialized() {
    let client = dummy_client();

    let msg = expect_http_error(client.get_deposit_address("USDT").await.unwrap_err());
    assert!(msg.contains(funding_ep::DEPOSIT_ADDRESS));

    let dep_params = GetDepositHistoryParams {
        ccy: Some("USDT".into()),
        dep_id: Some("1".into()),
        tx_id: None,
        r#type: Some("3".into()),
        state: Some("1".into()),
        after: Some("10".into()),
        before: None,
        limit: Some("50".into()),
    };
    let msg = expect_http_error(
        client
            .get_deposit_history(Some(dep_params))
            .await
            .unwrap_err(),
    );
    assert!(msg.contains(funding_ep::DEPOSIT_HISTORY));

    let wd_params = GetWithdrawalHistoryParams {
        ccy: Some("USDT".into()),
        wd_id: Some("2".into()),
        client_id: None,
        tx_id: None,
        r#type: Some("4".into()),
        state: Some("-3".into()),
        after: None,
        before: Some("20".into()),
        limit: Some("20".into()),
    };
    let msg = expect_http_error(
        client
            .get_withdrawal_history(Some(wd_params))
            .await
            .unwrap_err(),
    );
    assert!(msg.contains(funding_ep::WITHDRAWAL_HISTORY));

    let withdraw_req = WithdrawalRequest {
        ccy: "USDT".into(),
        amt: "5".into(),
        dest: "4".into(),
        to_addr: "internal".into(),
        chain: Some("USDT-ERC20".into()),
        area_code: None,
        client_id: Some("cid".into()),
        fee: Some("0.1".into()),
    };
    let body = to_value(&withdraw_req).unwrap();
    assert_eq!(body["dest"], "4");
    let msg = expect_http_error(client.withdrawal(withdraw_req).await.unwrap_err());
    assert!(msg.contains(funding_ep::WITHDRAWAL));
}

#[tokio::test]
async fn trade_mass_cancel_and_precheck_paths() {
    let client = dummy_client();

    let mass_cancel_body = json!({
        "instType": "SWAP",
        "instId": "BTC-USDT-SWAP",
    });
    let msg = expect_http_error(client.mass_cancel(mass_cancel_body).await.unwrap_err());
    assert!(msg.contains(trade::endpoints::MASS_CANCEL));

    let cancel_all_after_body = json!({
        "timeOut": "10"
    });
    let msg = expect_http_error(
        client
            .cancel_all_after(cancel_all_after_body)
            .await
            .unwrap_err(),
    );
    assert!(msg.contains(trade::endpoints::CANCEL_ALL_AFTER));

    let precheck_body = json!({
        "instId": "BTC-USDT",
        "tdMode": "cash",
        "side": "buy",
        "ordType": "limit",
        "px": "20000",
        "sz": "1"
    });
    let msg = expect_http_error(client.order_precheck(precheck_body).await.unwrap_err());
    assert!(msg.contains(trade::endpoints::ORDER_PRECHECK));
}

#[tokio::test]
async fn spread_paths_cover_basic_calls() {
    let client = dummy_client();

    let place = json!({
        "sprdId": "spread1",
        "side": "buy",
        "ordType": "limit",
        "px": "1",
        "sz": "1"
    });
    let msg = expect_http_error(client.spread_place_order(place).await.unwrap_err());
    assert!(msg.contains(spread::endpoints::PLACE_ORDER));

    let cancel = json!({ "sprdId": "spread1" });
    let msg = expect_http_error(client.spread_cancel_order(cancel).await.unwrap_err());
    assert!(msg.contains(spread::endpoints::CANCEL_ORDER));

    let mass = json!({ "instId": "BTC-USDT-SPRD" });
    let msg = expect_http_error(client.spread_cancel_all_orders(mass).await.unwrap_err());
    assert!(msg.contains(spread::endpoints::CANCEL_ALL_ORDERS));

    let details = json!({ "sprdId": "spread1" });
    let msg = expect_http_error(client.spread_get_order_details(details).await.unwrap_err());
    assert!(msg.contains(spread::endpoints::ORDER_DETAILS));
}

#[tokio::test]
async fn convert_methods_build_params_and_return_http_error() {
    let client = dummy_client();

    let msg = expect_http_error(client.get_convert_currencies().await.unwrap_err());
    assert!(msg.contains(convert::endpoints::CONVERT_CURRENCIES));

    let estimate = EstimateQuoteParams {
        base_ccy: "BTC".into(),
        quote_ccy: "USDT".into(),
        side: "sell".into(),
        rfq_sz: "0.1".into(),
        rfq_sz_ccy: "BTC".into(),
        cl_q_req_id: Some("req-1".into()),
        tag: Some("tag-1".into()),
    };
    let body = to_value(&estimate).expect("序列化报价失败");
    assert_eq!(body["rfqSzCcy"], "BTC");
    let msg = expect_http_error(client.estimate_convert_quote(estimate).await.unwrap_err());
    assert!(msg.contains(convert::endpoints::CONVERT_ESTIMATE_QUOTE));

    let trade_req = okx_core::types::ConvertTradeRequest {
        quote_id: "qid".into(),
        base_ccy: "BTC".into(),
        quote_ccy: "USDT".into(),
        side: "sell".into(),
        sz: "0.1".into(),
        sz_ccy: "BTC".into(),
        cl_t_req_id: None,
        tag: None,
    };
    let msg = expect_http_error(client.convert_trade(trade_req).await.unwrap_err());
    assert!(msg.contains(convert::endpoints::CONVERT_TRADE));

    let msg = expect_http_error(
        client
            .get_easy_convert_history(Some("1"), Some("2"), Some(10))
            .await
            .unwrap_err(),
    );
    assert!(msg.contains(convert::endpoints::EASY_CONVERT_HISTORY));

    let msg = expect_http_error(
        client
            .get_one_click_repay_history(Some("1"), Some("2"), Some(10))
            .await
            .unwrap_err(),
    );
    assert!(msg.contains(convert::endpoints::ONE_CLICK_REPAY_HISTORY));
}

#[tokio::test]
async fn system_status_path_builds() {
    let client = dummy_client();
    let msg = expect_http_error(client.get_system_status(Some("1")).await.unwrap_err());
    assert!(msg.contains(status::endpoints::SYSTEM_STATUS));
}

#[tokio::test]
async fn subaccount_methods_build_params_and_return_http_error() {
    let client = dummy_client();

    let msg = expect_http_error(client.get_subaccount_balance("sub1").await.unwrap_err());
    assert!(msg.contains(subaccount::endpoints::BALANCE));

    let msg = expect_http_error(client.get_subaccount_list(None).await.unwrap_err());
    assert!(msg.contains(subaccount::endpoints::LIST));

    let transfer = SubaccountTransferRequest {
        ccy: "USDT".into(),
        amt: "10".into(),
        froms: "6".into(),
        to: "18".into(),
        from_sub_account: "sub1".into(),
        to_sub_account: "sub2".into(),
        loan_trans: Some(false),
        omit_pos_risk: Some(false),
    };
    let body = to_value(&transfer).expect("序列化子账户划转失败");
    assert_eq!(body["fromSubAccount"], "sub1");
    let msg = expect_http_error(client.subaccount_transfer(transfer).await.unwrap_err());
    assert!(msg.contains(subaccount::endpoints::TRANSFER));

    let msg = expect_http_error(client.get_entrust_subaccount_list(None).await.unwrap_err());
    assert!(msg.contains(subaccount::endpoints::ENTRUST_LIST));

    let msg = expect_http_error(client.get_funding_balance("sub1", None).await.unwrap_err());
    assert!(msg.contains(subaccount::endpoints::FUNDING_BALANCE));

    let msg = expect_http_error(
        client
            .get_affiliate_rebate_info("key123")
            .await
            .unwrap_err(),
    );
    assert!(msg.contains(subaccount::endpoints::AFFILIATE_REBATE));
}

#[tokio::test]
async fn rfq_paths_cover_basic_calls() {
    let client = dummy_client();

    let rfq_body = json!({ "instId": "BTC-USDT" });
    let msg = expect_http_error(client.create_rfq(rfq_body).await.unwrap_err());
    assert!(msg.contains(block_rfq::endpoints::CREATE_RFQ));

    let quote_body = json!({ "rfqId": "1", "quoteSide": "buy" });
    let msg = expect_http_error(client.create_quote(quote_body).await.unwrap_err());
    assert!(msg.contains(block_rfq::endpoints::CREATE_QUOTE));

    let cancel_quote_body = json!({ "quoteId": "1" });
    let msg = expect_http_error(client.cancel_quote(cancel_quote_body).await.unwrap_err());
    assert!(msg.contains(block_rfq::endpoints::CANCEL_QUOTE));

    let mmp_body = json!({ "timeInterval": "1000", "frozenInterval": "1000", "qtyLimit": "1" });
    let msg = expect_http_error(client.set_mmp_config(mmp_body).await.unwrap_err());
    assert!(msg.contains(block_rfq::endpoints::SET_MMP));

    let msg = expect_http_error(client.get_mmp_config().await.unwrap_err());
    assert!(msg.contains(block_rfq::endpoints::GET_MMP_CONFIG));

    let set_marker = json!({"instFamily": "BTC-USD"});
    let msg = expect_http_error(client.set_marker_instrument(set_marker).await.unwrap_err());
    assert!(msg.contains(block_rfq::endpoints::SET_MARKER_INSTRUMENT));
}

#[tokio::test]
async fn copy_trading_paths_cover_basic_calls() {
    let client = dummy_client();

    let params = serde_json::json!({"instId": "BTC-USDT"});
    let msg = expect_http_error(
        client
            .get_existing_lead_positions(Some(params))
            .await
            .unwrap_err(),
    );
    assert!(msg.contains(copy_trading::endpoints::EXISTING_LEAD_POSITIONS));

    let stop_order = serde_json::json!({"subPosId": "1", "tpTriggerPx": "20000"});
    let msg = expect_http_error(client.place_lead_stop_order(stop_order).await.unwrap_err());
    assert!(msg.contains(copy_trading::endpoints::PLACE_LEAD_STOP_ORDER));

    let close_pos = serde_json::json!({"subPosId": "1"});
    let msg = expect_http_error(client.close_lead_position(close_pos).await.unwrap_err());
    assert!(msg.contains(copy_trading::endpoints::CLOSE_LEAD_POSITION));

    let profit_params = serde_json::json!({"after": "1"});
    let msg = expect_http_error(
        client
            .get_profit_sharing_details(Some(profit_params))
            .await
            .unwrap_err(),
    );
    assert!(msg.contains(copy_trading::endpoints::PROFIT_SHARING_DETAILS));

    let msg = expect_http_error(client.get_total_profit_sharing().await.unwrap_err());
    assert!(msg.contains(copy_trading::endpoints::TOTAL_PROFIT_SHARING));
}

#[tokio::test]
async fn grid_paths_cover_basic_calls() {
    let client = dummy_client();

    let grid_order = json!({"instId": "BTC-USDT-SWAP", "algoOrderType": "grid", "maxPx": "30000", "minPx": "20000", "gridNum": "5", "runType": "2"});
    let msg = expect_http_error(client.grid_order_algo(grid_order).await.unwrap_err());
    assert!(msg.contains(grid::endpoints::GRID_ORDER_ALGO));

    let amend = json!({"algoId": "123", "instId": "BTC-USDT-SWAP"});
    let msg = expect_http_error(client.grid_amend_order_algo(amend).await.unwrap_err());
    assert!(msg.contains(grid::endpoints::GRID_AMEND_ORDER_ALGO));

    let stop = json!({"algoId": "123", "instId": "BTC-USDT-SWAP", "stopType": "1"});
    let msg = expect_http_error(client.grid_stop_order_algo(stop).await.unwrap_err());
    assert!(msg.contains(grid::endpoints::GRID_STOP_ORDER_ALGO));

    let pending = json!({"algoOrderType": "grid"});
    let msg = expect_http_error(
        client
            .grid_orders_algo_pending(Some(pending))
            .await
            .unwrap_err(),
    );
    assert!(msg.contains(grid::endpoints::GRID_ORDERS_ALGO_PENDING));

    let history = json!({"algoOrderType": "grid"});
    let msg = expect_http_error(
        client
            .grid_orders_algo_history(Some(history))
            .await
            .unwrap_err(),
    );
    assert!(msg.contains(grid::endpoints::GRID_ORDERS_ALGO_HISTORY));

    let details = json!({"algoOrderType": "grid", "algoId": "123"});
    let msg = expect_http_error(client.grid_orders_algo_details(details).await.unwrap_err());
    assert!(msg.contains(grid::endpoints::GRID_ORDERS_ALGO_DETAILS));

    let sub = json!({"algoId": "123", "algoOrderType": "grid"});
    let msg = expect_http_error(client.grid_sub_orders(sub).await.unwrap_err());
    assert!(msg.contains(grid::endpoints::GRID_SUB_ORDERS));

    let pos = json!({"algoOrderType": "grid"});
    let msg = expect_http_error(client.grid_positions(Some(pos)).await.unwrap_err());
    assert!(msg.contains(grid::endpoints::GRID_POSITIONS));

    let withdraw = json!({"algoId": "123"});
    let msg = expect_http_error(client.grid_withdraw_income(withdraw).await.unwrap_err());
    assert!(msg.contains(grid::endpoints::GRID_WITHDRAW_INCOME));

    let compute = json!({"algoId": "123", "type": "add", "amt": "100"});
    let msg = expect_http_error(
        client
            .grid_compute_margin_balance(compute)
            .await
            .unwrap_err(),
    );
    assert!(msg.contains(grid::endpoints::GRID_COMPUTE_MARGIN_BALANCE));

    let margin = json!({"algoId": "123", "type": "add", "amt": "100"});
    let msg = expect_http_error(client.grid_margin_balance(margin).await.unwrap_err());
    assert!(msg.contains(grid::endpoints::GRID_MARGIN_BALANCE));

    let ai = json!({"algoOrderType": "grid", "instId": "BTC-USDT"});
    let msg = expect_http_error(client.grid_ai_param(Some(ai)).await.unwrap_err());
    assert!(msg.contains(grid::endpoints::GRID_AI_PARAM));

    let recur_order = json!({"algoOrderType": "recurring", "ruleType": "1", "baseCcy": "USDT", "quoteCcy": "BTC", "amt": "100", "investmentType": "1", "period": "daily"});
    let msg = expect_http_error(
        client
            .place_recurring_buy_order(recur_order)
            .await
            .unwrap_err(),
    );
    assert!(msg.contains(grid::endpoints::PLACE_RECURRING_BUY_ORDER));

    let amend_recur = json!({"algoId": "123", "amt": "200"});
    let msg = expect_http_error(
        client
            .amend_recurring_buy_order(amend_recur)
            .await
            .unwrap_err(),
    );
    assert!(msg.contains(grid::endpoints::AMEND_RECURRING_BUY_ORDER));

    let stop_recur = json!({"algoId": "123"});
    let msg = expect_http_error(
        client
            .stop_recurring_buy_order(stop_recur)
            .await
            .unwrap_err(),
    );
    assert!(msg.contains(grid::endpoints::STOP_RECURRING_BUY_ORDER));

    let list = json!({"algoOrderType": "recurring"});
    let msg = expect_http_error(
        client
            .get_recurring_buy_order_list(Some(list))
            .await
            .unwrap_err(),
    );
    assert!(msg.contains(grid::endpoints::GET_RECURRING_BUY_ORDER_LIST));

    let hist = json!({"algoOrderType": "recurring"});
    let msg = expect_http_error(
        client
            .get_recurring_buy_order_history(Some(hist))
            .await
            .unwrap_err(),
    );
    assert!(msg.contains(grid::endpoints::GET_RECURRING_BUY_ORDER_HISTORY));

    let det = json!({"algoId": "123"});
    let msg = expect_http_error(
        client
            .get_recurring_buy_order_details(det)
            .await
            .unwrap_err(),
    );
    assert!(msg.contains(grid::endpoints::GET_RECURRING_BUY_ORDER_DETAILS));

    let sub_ord = json!({"algoId": "123"});
    let msg = expect_http_error(
        client
            .get_recurring_buy_sub_orders(sub_ord)
            .await
            .unwrap_err(),
    );
    assert!(msg.contains(grid::endpoints::GET_RECURRING_BUY_SUB_ORDERS));
}

#[tokio::test]
async fn finance_paths_cover_basic_calls() {
    let client = dummy_client();

    let offers_params = json!({"ccy": "USDT"});
    let msg = expect_http_error(
        client
            .defi_get_offers(Some(offers_params))
            .await
            .unwrap_err(),
    );
    assert!(msg.contains(finance::endpoints::DEFI_OFFERS));

    let purchase_body = json!({"ccy": "USDT", "amt": "10", "productId": "pid"});
    let msg = expect_http_error(client.defi_purchase(purchase_body).await.unwrap_err());
    assert!(msg.contains(finance::endpoints::DEFI_PURCHASE));

    let redeem_body = json!({"ordId": "123", "protocolType": "staking"});
    let msg = expect_http_error(client.defi_redeem(redeem_body).await.unwrap_err());
    assert!(msg.contains(finance::endpoints::DEFI_REDEEM));

    let cancel_body = json!({"ordId": "123", "protocolType": "staking"});
    let msg = expect_http_error(client.defi_cancel(cancel_body).await.unwrap_err());
    assert!(msg.contains(finance::endpoints::DEFI_CANCEL));

    let active_params = json!({"protocolType": "staking"});
    let msg = expect_http_error(
        client
            .defi_orders_active(Some(active_params))
            .await
            .unwrap_err(),
    );
    assert!(msg.contains(finance::endpoints::DEFI_ORDERS_ACTIVE));

    let history_params = json!({"protocolType": "staking"});
    let msg = expect_http_error(
        client
            .defi_orders_history(Some(history_params))
            .await
            .unwrap_err(),
    );
    assert!(msg.contains(finance::endpoints::DEFI_ORDERS_HISTORY));

    let balance_params = json!({"ccy": "USDT"});
    let msg = expect_http_error(
        client
            .saving_balance(Some(balance_params))
            .await
            .unwrap_err(),
    );
    assert!(msg.contains(finance::endpoints::SAVING_BALANCE));

    let saving_body = json!({"ccy": "USDT", "amt": "5", "side": "purchase"});
    let msg = expect_http_error(
        client
            .saving_purchase_redemption(saving_body)
            .await
            .unwrap_err(),
    );
    assert!(msg.contains(finance::endpoints::SAVING_PURCHASE_REDEMPTION));

    let rate_body = json!({"ccy": "USDT", "rate": "0.01"});
    let msg = expect_http_error(client.saving_set_lending_rate(rate_body).await.unwrap_err());
    assert!(msg.contains(finance::endpoints::SAVING_SET_LENDING_RATE));

    let lending_params = json!({"ccy": "USDT"});
    let msg = expect_http_error(
        client
            .saving_lending_history(Some(lending_params))
            .await
            .unwrap_err(),
    );
    assert!(msg.contains(finance::endpoints::SAVING_LENDING_HISTORY));

    let public_params = json!({"ccy": "USDT"});
    let msg = expect_http_error(
        client
            .saving_public_lending_rate(Some(public_params))
            .await
            .unwrap_err(),
    );
    assert!(msg.contains(finance::endpoints::SAVING_PUBLIC_LENDING_RATE));

    let earn_offers = json!({"ccy": "USDT"});
    let msg = expect_http_error(
        client
            .simple_earn_get_lending_offers(Some(earn_offers))
            .await
            .unwrap_err(),
    );
    assert!(msg.contains(finance::endpoints::SIMPLE_EARN_LENDING_OFFERS));

    let apr_params = json!({"ccy": "USDT"});
    let msg = expect_http_error(
        client
            .simple_earn_get_lending_apy_history(Some(apr_params))
            .await
            .unwrap_err(),
    );
    assert!(msg.contains(finance::endpoints::SIMPLE_EARN_LENDING_APY_HISTORY));

    let pending_params = json!({"ccy": "USDT"});
    let msg = expect_http_error(
        client
            .simple_earn_get_pending_lending_volume(Some(pending_params))
            .await
            .unwrap_err(),
    );
    assert!(msg.contains(finance::endpoints::SIMPLE_EARN_PENDING_LENDING_VOLUME));

    let list_params = json!({"ccy": "USDT"});
    let msg = expect_http_error(
        client
            .simple_earn_get_lending_orders_list(Some(list_params))
            .await
            .unwrap_err(),
    );
    assert!(msg.contains(finance::endpoints::SIMPLE_EARN_LENDING_ORDERS_LIST));

    let sub_orders_params = json!({"ordId": "123"});
    let msg = expect_http_error(
        client
            .simple_earn_get_lending_sub_orders(Some(sub_orders_params))
            .await
            .unwrap_err(),
    );
    assert!(msg.contains(finance::endpoints::SIMPLE_EARN_LENDING_SUB_ORDERS));

    let simple_earn_body = json!({"ccy": "USDT", "amt": "10", "rate": "0.02", "term": "7"});
    let msg = expect_http_error(
        client
            .simple_earn_place_lending_order(simple_earn_body)
            .await
            .unwrap_err(),
    );
    assert!(msg.contains(finance::endpoints::SIMPLE_EARN_LENDING_ORDER));

    let amend_body = json!({"ordId": "123", "changeAmt": "5"});
    let msg = expect_http_error(
        client
            .simple_earn_amend_lending_order(amend_body)
            .await
            .unwrap_err(),
    );
    assert!(msg.contains(finance::endpoints::SIMPLE_EARN_AMEND_LENDING_ORDER));
}

#[tokio::test]
async fn broker_methods_build_params_and_return_http_error() {
    let client = dummy_client();

    let rebate_params = json!({"begin": "1609459200000", "end": "1612137600000"});
    let msg = expect_http_error(
        client
            .fd_rebate_per_orders(rebate_params.clone())
            .await
            .unwrap_err(),
    );
    assert!(msg.contains(broker::endpoints::FD_REBATE_PER_ORDERS));

    let msg = expect_http_error(
        client
            .fd_get_rebate_per_orders(rebate_params)
            .await
            .unwrap_err(),
    );
    assert!(msg.contains(broker::endpoints::FD_GET_REBATE_PER_ORDERS));
}

#[tokio::test]
async fn trading_data_methods_build_params_and_return_http_error() {
    let client = dummy_client();

    let msg = expect_http_error(client.get_support_coin().await.unwrap_err());
    assert!(msg.contains(trading_data::endpoints::SUPPORT_COIN));

    let params = json!({"ccy": "BTC", "instType": "SWAP"});
    let msg = expect_http_error(
        client
            .get_taker_volume(Some(params.clone()))
            .await
            .unwrap_err(),
    );
    assert!(msg.contains(trading_data::endpoints::TAKER_VOLUME));

    let msg = expect_http_error(
        client
            .get_margin_lending_ratio(Some(params.clone()))
            .await
            .unwrap_err(),
    );
    assert!(msg.contains(trading_data::endpoints::MARGIN_LENDING_RATIO));

    let msg = expect_http_error(
        client
            .get_long_short_ratio(Some(params.clone()))
            .await
            .unwrap_err(),
    );
    assert!(msg.contains(trading_data::endpoints::LONG_SHORT_RATIO));

    let msg = expect_http_error(
        client
            .get_contracts_open_interest_volume(Some(params.clone()))
            .await
            .unwrap_err(),
    );
    assert!(msg.contains(trading_data::endpoints::CONTRACTS_INTEREST_VOLUME));

    let msg = expect_http_error(
        client
            .get_options_open_interest_volume(Some(params.clone()))
            .await
            .unwrap_err(),
    );
    assert!(msg.contains(trading_data::endpoints::OPTIONS_INTEREST_VOLUME));

    let msg = expect_http_error(
        client
            .get_put_call_ratio(Some(params.clone()))
            .await
            .unwrap_err(),
    );
    assert!(msg.contains(trading_data::endpoints::PUT_CALL_RATIO));

    let msg = expect_http_error(
        client
            .get_open_interest_volume_expiry(Some(params.clone()))
            .await
            .unwrap_err(),
    );
    assert!(msg.contains(trading_data::endpoints::OPEN_INTEREST_VOLUME_EXPIRY));

    let msg = expect_http_error(
        client
            .get_interest_volume_strike(Some(params.clone()))
            .await
            .unwrap_err(),
    );
    assert!(msg.contains(trading_data::endpoints::INTEREST_VOLUME_STRIKE));

    let msg = expect_http_error(client.get_taker_flow(Some(params)).await.unwrap_err());
    assert!(msg.contains(trading_data::endpoints::TAKER_FLOW));
}
