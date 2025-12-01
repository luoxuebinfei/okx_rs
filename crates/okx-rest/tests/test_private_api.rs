#![allow(missing_docs)]

use okx_core::types::{
    AmendOrderRequest, AttachAlgoOrdRequest, CancelAlgoOrderRequest, CancelOrderRequest,
    PlaceAlgoOrderRequest,
};
use okx_core::{
    types::{FundsTransferRequest, PlaceOrderRequest, WithdrawalRequest},
    Config, Credentials,
};
use okx_rest::api::account::{
    self, GetFeeRatesParams, GetLeverageInfoParams, GetMaxAvailSizeParams, GetMaxSizeParams,
    GetPositionsParams, SetLeverageRequest,
};
use okx_rest::api::funding::{self, GetDepositHistoryParams, GetWithdrawalHistoryParams};
use okx_rest::api::trade::{
    ClosePositionRequest, GetAlgoOrdersHistoryParams, GetAlgoOrdersParams, GetFillsParams,
    GetOrderParams, GetOrdersHistoryParams, GetOrdersPendingParams,
};
use okx_rest::api::{funding::endpoints as funding_ep, trade};
use okx_rest::{AccountApi, FundingApi, OkxError, OkxRestClient, TradeApi};
use serde_json::to_value;

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
