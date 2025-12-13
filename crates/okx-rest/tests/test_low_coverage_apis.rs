//! 覆盖率补齐：离线环境下覆盖更多 API 模块代码路径。
//!
//! 目标：
//! - 不依赖外网（使用本地不可达地址触发快速 HTTP 错误）
//! - 重点覆盖 `target/llvm-cov/html/html/index.html` 中行覆盖率较低的模块

use okx_core::{Config, Credentials};
use okx_rest::api::{
    block_rfq, convert, copy_trading, finance, market, public, spread, subaccount,
};
use okx_rest::{
    BlockRfqApi, ConvertApi, CopyTradingApi, FinanceApi, MarketApi, OkxError, OkxRestClient,
    PublicApi, SpreadApi, SubaccountApi,
};
use serde_json::json;

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
async fn public_api_offline_covers_all_methods_and_validation() {
    let client = dummy_client();

    let instruments = public::GetInstrumentsParams {
        inst_type: "SPOT".to_string(),
        uly: None,
        inst_family: None,
        inst_id: None,
    };
    let msg = expect_http_error(client.get_instruments(instruments).await.unwrap_err());
    assert!(msg.contains(public::endpoints::INSTRUMENTS));

    let msg = expect_http_error(client.get_funding_rate("BTC-USD-SWAP").await.unwrap_err());
    assert!(msg.contains(public::endpoints::FUNDING_RATE));

    let history = public::GetFundingRateHistoryParams {
        inst_id: "BTC-USD-SWAP".to_string(),
        after: Some("1".to_string()),
        before: None,
        limit: Some("2".to_string()),
    };
    let msg = expect_http_error(client.get_funding_rate_history(history).await.unwrap_err());
    assert!(msg.contains(public::endpoints::FUNDING_RATE_HISTORY));

    let mark = public::GetMarkPriceParams {
        inst_type: "SWAP".to_string(),
        uly: None,
        inst_family: None,
        inst_id: Some("BTC-USD-SWAP".to_string()),
    };
    let msg = expect_http_error(client.get_mark_price(mark).await.unwrap_err());
    assert!(msg.contains(public::endpoints::MARK_PRICE));

    let msg = expect_http_error(client.get_system_time().await.unwrap_err());
    assert!(msg.contains(public::endpoints::TIME));

    let bands = public::GetInstrumentTickBandsParams {
        inst_type: "OPTION".to_string(),
        inst_family: Some("BTC-USD".to_string()),
    };
    let msg = expect_http_error(client.get_instrument_tick_bands(bands).await.unwrap_err());
    assert!(msg.contains(public::endpoints::INSTRUMENT_TICK_BANDS));

    // 校验分支：inst_id 与 inst_family 均为空时直接返回参数错误
    let invalid = public::GetOptionTradesParams {
        inst_id: None,
        inst_family: None,
        opt_type: None,
    };
    let err = client.get_option_trades(invalid).await.unwrap_err();
    assert!(matches!(err, OkxError::Other(_)));
    assert!(err
        .to_string()
        .contains("Either inst_id or inst_family must be provided"));

    // 只有 inst_family：应保留 instFamily 查询参数
    let only_family = public::GetOptionTradesParams {
        inst_id: None,
        inst_family: Some("BTC-USD".to_string()),
        opt_type: Some("C".to_string()),
    };
    let msg = expect_http_error(client.get_option_trades(only_family).await.unwrap_err());
    assert!(msg.contains(public::endpoints::OPTION_TRADES));
    assert!(msg.contains("instFamily="));

    // 同时提供 inst_id 与 inst_family：应优先使用 inst_id 并清空 inst_family
    let both = public::GetOptionTradesParams {
        inst_id: Some("BTC-USD-240628-40000-C".to_string()),
        inst_family: Some("BTC-USD".to_string()),
        opt_type: Some("C".to_string()),
    };
    let msg = expect_http_error(client.get_option_trades(both).await.unwrap_err());
    assert!(msg.contains(public::endpoints::OPTION_TRADES));
    assert!(msg.contains("instId="));
    assert!(!msg.contains("instFamily="));

    let delivery = public::GetDeliveryExerciseHistoryParams {
        inst_type: "FUTURES".to_string(),
        uly: Some("BTC-USD".to_string()),
        inst_family: None,
        after: Some("1".to_string()),
        before: None,
        limit: Some("10".to_string()),
    };
    let msg = expect_http_error(
        client
            .get_delivery_exercise_history(delivery)
            .await
            .unwrap_err(),
    );
    assert!(msg.contains(public::endpoints::DELIVERY_EXERCISE_HISTORY));

    let open_interest = public::GetOpenInterestParams {
        inst_type: "SWAP".to_string(),
        uly: None,
        inst_id: Some("BTC-USD-SWAP".to_string()),
        inst_family: None,
    };
    let msg = expect_http_error(client.get_open_interest(open_interest).await.unwrap_err());
    assert!(msg.contains(public::endpoints::OPEN_INTEREST));

    let tiers = public::GetPositionTiersParams {
        inst_type: "SWAP".to_string(),
        td_mode: "cross".to_string(),
        uly: Some("BTC-USD".to_string()),
        inst_id: None,
        ccy: None,
        tier: Some("1".to_string()),
        inst_family: None,
    };
    let msg = expect_http_error(client.get_position_tiers(tiers).await.unwrap_err());
    assert!(msg.contains(public::endpoints::POSITION_TIERS));

    let price_limit = public::GetPriceLimitParams {
        inst_id: "BTC-USD-SWAP".to_string(),
    };
    let msg = expect_http_error(client.get_price_limit(price_limit).await.unwrap_err());
    assert!(msg.contains(public::endpoints::PRICE_LIMIT));

    let opt_summary = public::GetOptSummaryParams {
        uly: Some("BTC-USD".to_string()),
        exp_time: Some("250628".to_string()),
        inst_family: None,
    };
    let msg = expect_http_error(client.get_opt_summary(opt_summary).await.unwrap_err());
    assert!(msg.contains(public::endpoints::OPT_SUMMARY));

    let estimated = public::GetEstimatedPriceParams {
        inst_id: "BTC-USD-SWAP".to_string(),
    };
    let msg = expect_http_error(client.get_estimated_price(estimated).await.unwrap_err());
    assert!(msg.contains(public::endpoints::ESTIMATED_PRICE));

    let discount = public::GetDiscountQuotaParams {
        ccy: Some("BTC".to_string()),
    };
    let msg = expect_http_error(
        client
            .get_discount_interest_free_quota(discount)
            .await
            .unwrap_err(),
    );
    assert!(msg.contains(public::endpoints::DISCOUNT_RATE_INTEREST_FREE_QUOTA));

    let msg = expect_http_error(client.get_interest_rate_loan_quota().await.unwrap_err());
    assert!(msg.contains(public::endpoints::INTEREST_RATE_LOAN_QUOTA));

    let msg = expect_http_error(client.get_vip_interest_rate_loan_quota().await.unwrap_err());
    assert!(msg.contains(public::endpoints::VIP_INTEREST_RATE_LOAN_QUOTA));

    let underlying = public::GetUnderlyingParams {
        inst_type: Some("SWAP".to_string()),
    };
    let msg = expect_http_error(client.get_underlying(underlying).await.unwrap_err());
    assert!(msg.contains(public::endpoints::UNDERLYING));

    let insurance = public::GetInsuranceFundParams {
        inst_type: Some("SWAP".to_string()),
        r#type: Some("all".to_string()),
        uly: Some("BTC-USD".to_string()),
        ccy: None,
        before: None,
        after: Some("1".to_string()),
        limit: Some("1".to_string()),
        inst_family: None,
    };
    let msg = expect_http_error(client.get_insurance_fund(insurance).await.unwrap_err());
    assert!(msg.contains(public::endpoints::INSURANCE_FUND));

    let convert_coin = public::GetConvertContractCoinParams {
        r#type: Some("1".to_string()),
        inst_id: Some("BTC-USD-SWAP".to_string()),
        sz: Some("1".to_string()),
        px: Some("30000".to_string()),
        unit: Some("coin".to_string()),
    };
    let msg = expect_http_error(
        client
            .get_convert_contract_coin(convert_coin)
            .await
            .unwrap_err(),
    );
    assert!(msg.contains(public::endpoints::CONVERT_CONTRACT_COIN));
}

#[tokio::test]
async fn market_api_offline_covers_all_methods() {
    let client = dummy_client();

    let tickers = market::GetTickersParams {
        inst_type: "SPOT".to_string(),
        uly: None,
        inst_family: None,
    };
    let msg = expect_http_error(client.get_tickers(tickers).await.unwrap_err());
    assert!(msg.contains(market::endpoints::TICKERS));

    let msg = expect_http_error(client.get_ticker("BTC-USDT").await.unwrap_err());
    assert!(msg.contains(market::endpoints::TICKER));

    let msg = expect_http_error(client.get_orderbook("BTC-USDT", Some(1)).await.unwrap_err());
    assert!(msg.contains(market::endpoints::BOOKS));

    let candles = market::GetCandlesParams {
        inst_id: "BTC-USDT".to_string(),
        bar: Some("1m".to_string()),
        after: None,
        before: None,
        limit: Some("2".to_string()),
    };
    let msg = expect_http_error(client.get_candles(candles).await.unwrap_err());
    assert!(msg.contains(market::endpoints::CANDLES));

    let history_candles = market::GetCandlesParams {
        inst_id: "BTC-USDT".to_string(),
        bar: Some("1m".to_string()),
        after: Some("1".to_string()),
        before: None,
        limit: Some("2".to_string()),
    };
    let msg = expect_http_error(
        client
            .get_history_candles(history_candles)
            .await
            .unwrap_err(),
    );
    assert!(msg.contains(market::endpoints::CANDLES_HISTORY));

    let index_candles = market::GetIndexCandlesParams {
        inst_id: "BTC-USD".to_string(),
        bar: Some("1m".to_string()),
        after: None,
        before: None,
        limit: Some("2".to_string()),
    };
    let msg = expect_http_error(client.get_index_candles(index_candles).await.unwrap_err());
    assert!(msg.contains(market::endpoints::INDEX_CANDLES));

    let mark_price_candles = market::GetMarkPriceCandlesParams {
        inst_id: "BTC-USD-SWAP".to_string(),
        bar: Some("1m".to_string()),
        after: None,
        before: None,
        limit: Some("2".to_string()),
    };
    let msg = expect_http_error(
        client
            .get_mark_price_candles(mark_price_candles)
            .await
            .unwrap_err(),
    );
    assert!(msg.contains(market::endpoints::MARK_PRICE_CANDLES));

    let msg = expect_http_error(
        MarketApi::get_trades(&client, "BTC-USDT", Some(1))
            .await
            .unwrap_err(),
    );
    assert!(msg.contains(market::endpoints::TRADES));

    let history_trades = market::GetHistoryTradesParams {
        inst_id: "BTC-USDT".to_string(),
        after: Some("1".to_string()),
        before: None,
        limit: Some("1".to_string()),
        r#type: Some("1".to_string()),
    };
    let msg = expect_http_error(client.get_history_trades(history_trades).await.unwrap_err());
    assert!(msg.contains(market::endpoints::TRADES_HISTORY));

    let msg = expect_http_error(client.get_platform_24_volume().await.unwrap_err());
    assert!(msg.contains(market::endpoints::PLATFORM_24_VOLUME));

    let index_components = market::GetIndexComponentsParams {
        index: "BTC-USD".to_string(),
    };
    let msg = expect_http_error(
        client
            .get_index_components(index_components)
            .await
            .unwrap_err(),
    );
    assert!(msg.contains(market::endpoints::INDEX_COMPONENTS));

    let msg = expect_http_error(client.get_exchange_rate().await.unwrap_err());
    assert!(msg.contains(market::endpoints::EXCHANGE_RATE));

    let index_tickers = market::GetIndexTickersParams {
        quote_ccy: Some("USD".to_string()),
        inst_id: Some("BTC-USD".to_string()),
    };
    let msg = expect_http_error(client.get_index_tickers(index_tickers).await.unwrap_err());
    assert!(msg.contains(market::endpoints::INDEX_TICKERS));

    let msg = expect_http_error(client.get_orderbook_lite("BTC-USDT").await.unwrap_err());
    assert!(msg.contains(market::endpoints::BOOKS_LITE));

    let msg = expect_http_error(client.get_block_ticker("BTC-USDT").await.unwrap_err());
    assert!(msg.contains(market::endpoints::BLOCK_TICKER));

    let block_tickers = market::GetBlockTickersParams {
        inst_type: "SPOT".to_string(),
        uly: None,
        inst_family: None,
    };
    let msg = expect_http_error(client.get_block_tickers(block_tickers).await.unwrap_err());
    assert!(msg.contains(market::endpoints::BLOCK_TICKERS));

    let msg = expect_http_error(client.get_block_trades("BTC-USDT").await.unwrap_err());
    assert!(msg.contains(market::endpoints::BLOCK_TRADES));

    let msg = expect_http_error(
        client
            .get_option_family_trades("BTC-USD")
            .await
            .unwrap_err(),
    );
    assert!(msg.contains(market::endpoints::OPTION_FAMILY_TRADES));
}

#[tokio::test]
async fn block_rfq_api_offline_covers_all_methods() {
    let client = dummy_client();

    let msg = expect_http_error(client.get_counterparties().await.unwrap_err());
    assert!(msg.contains(block_rfq::endpoints::COUNTERPARTIES));

    let create_rfq = json!({"instId": "BTC-USDT"});
    let msg = expect_http_error(client.create_rfq(create_rfq).await.unwrap_err());
    assert!(msg.contains(block_rfq::endpoints::CREATE_RFQ));

    let cancel_rfq = json!({"rfqId": "1"});
    let msg = expect_http_error(client.cancel_rfq(cancel_rfq).await.unwrap_err());
    assert!(msg.contains(block_rfq::endpoints::CANCEL_RFQ));

    let cancel_batch = json!({"rfqIds": ["1", "2"]});
    let msg = expect_http_error(client.cancel_batch_rfqs(cancel_batch).await.unwrap_err());
    assert!(msg.contains(block_rfq::endpoints::CANCEL_BATCH_RFQS));

    let cancel_all = json!({"instType": "SPOT"});
    let msg = expect_http_error(client.cancel_all_rfqs(cancel_all).await.unwrap_err());
    assert!(msg.contains(block_rfq::endpoints::CANCEL_ALL_RFQS));

    let exec = json!({"quoteId": "1"});
    let msg = expect_http_error(client.execute_quote(exec).await.unwrap_err());
    assert!(msg.contains(block_rfq::endpoints::EXECUTE_QUOTE));

    let create_quote = json!({ "rfqId": "1", "quoteSide": "buy" });
    let msg = expect_http_error(client.create_quote(create_quote).await.unwrap_err());
    assert!(msg.contains(block_rfq::endpoints::CREATE_QUOTE));

    let cancel_quote = json!({ "quoteId": "1" });
    let msg = expect_http_error(client.cancel_quote(cancel_quote).await.unwrap_err());
    assert!(msg.contains(block_rfq::endpoints::CANCEL_QUOTE));

    let cancel_batch_quotes = json!({"quoteIds": ["1", "2"]});
    let msg = expect_http_error(
        client
            .cancel_batch_quotes(cancel_batch_quotes)
            .await
            .unwrap_err(),
    );
    assert!(msg.contains(block_rfq::endpoints::CANCEL_BATCH_QUOTES));

    let cancel_all_quotes = json!({"instType": "SPOT"});
    let msg = expect_http_error(
        client
            .cancel_all_quotes(cancel_all_quotes)
            .await
            .unwrap_err(),
    );
    assert!(msg.contains(block_rfq::endpoints::CANCEL_ALL_QUOTES));

    let msg = expect_http_error(
        client
            .get_rfqs(Some(json!({"state":"filled"})))
            .await
            .unwrap_err(),
    );
    assert!(msg.contains(block_rfq::endpoints::GET_RFQS));

    let msg = expect_http_error(
        client
            .get_quotes(Some(json!({"rfqId":"1"})))
            .await
            .unwrap_err(),
    );
    assert!(msg.contains(block_rfq::endpoints::GET_QUOTES));

    let msg = expect_http_error(
        BlockRfqApi::get_trades(&client, Some(json!({"instId":"BTC-USDT"})))
            .await
            .unwrap_err(),
    );
    assert!(msg.contains(block_rfq::endpoints::GET_TRADES));

    let msg = expect_http_error(
        client
            .get_public_trades(Some(json!({"instId":"BTC-USDT"})))
            .await
            .unwrap_err(),
    );
    assert!(msg.contains(block_rfq::endpoints::GET_PUBLIC_TRADES));

    let reset = json!({"timeInterval": "1000"});
    let msg = expect_http_error(client.reset_mmp(reset).await.unwrap_err());
    assert!(msg.contains(block_rfq::endpoints::RESET_MMP));

    let set_mmp = json!({ "timeInterval": "1000", "frozenInterval": "1000", "qtyLimit": "1" });
    let msg = expect_http_error(client.set_mmp_config(set_mmp).await.unwrap_err());
    assert!(msg.contains(block_rfq::endpoints::SET_MMP));

    let msg = expect_http_error(client.get_mmp_config().await.unwrap_err());
    assert!(msg.contains(block_rfq::endpoints::GET_MMP_CONFIG));

    let set_marker = json!({"instFamily": "BTC-USD"});
    let msg = expect_http_error(client.set_marker_instrument(set_marker).await.unwrap_err());
    assert!(msg.contains(block_rfq::endpoints::SET_MARKER_INSTRUMENT));

    let msg = expect_http_error(
        client
            .get_quote_products(Some(json!({"instType":"SPOT"})))
            .await
            .unwrap_err(),
    );
    assert!(msg.contains(block_rfq::endpoints::GET_QUOTE_PRODUCTS));
}

#[tokio::test]
async fn spread_api_offline_covers_all_methods() {
    let client = dummy_client();

    let place = json!({"sprdId": "spread1","side": "buy","ordType": "limit","px": "1","sz": "1"});
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

    let msg = expect_http_error(
        client
            .spread_get_active_orders(Some(json!({"instId":"BTC-USDT-SPRD"})))
            .await
            .unwrap_err(),
    );
    assert!(msg.contains(spread::endpoints::ACTIVE_ORDERS));

    let msg = expect_http_error(
        client
            .spread_get_orders(Some(json!({"instId":"BTC-USDT-SPRD"})))
            .await
            .unwrap_err(),
    );
    assert!(msg.contains(spread::endpoints::ORDERS));

    let msg = expect_http_error(
        client
            .spread_get_trades(Some(json!({"instId":"BTC-USDT-SPRD"})))
            .await
            .unwrap_err(),
    );
    assert!(msg.contains(spread::endpoints::TRADES));

    let msg = expect_http_error(
        client
            .spread_get_spreads(Some(json!({"sprdId":"spread1"})))
            .await
            .unwrap_err(),
    );
    assert!(msg.contains(spread::endpoints::SPREADS));

    let msg = expect_http_error(
        client
            .spread_get_order_book(json!({"sprdId":"spread1"}))
            .await
            .unwrap_err(),
    );
    assert!(msg.contains(spread::endpoints::ORDER_BOOK));

    let msg = expect_http_error(
        client
            .spread_get_ticker(json!({"sprdId":"spread1"}))
            .await
            .unwrap_err(),
    );
    assert!(msg.contains(spread::endpoints::TICKER));

    let msg = expect_http_error(
        client
            .spread_get_public_trades(json!({"sprdId":"spread1"}))
            .await
            .unwrap_err(),
    );
    assert!(msg.contains(spread::endpoints::PUBLIC_TRADES));
}

#[tokio::test]
async fn convert_api_offline_covers_remaining_methods() {
    let client = dummy_client();

    let msg = expect_http_error(client.get_convert_currencies().await.unwrap_err());
    assert!(msg.contains(convert::endpoints::CONVERT_CURRENCIES));

    let msg = expect_http_error(
        client
            .get_convert_currency_pair("BTC", "USDT")
            .await
            .unwrap_err(),
    );
    assert!(msg.contains(convert::endpoints::CONVERT_CURRENCY_PAIR));

    let msg = expect_http_error(client.get_convert_history(None).await.unwrap_err());
    assert!(msg.contains(convert::endpoints::CONVERT_HISTORY));

    let msg = expect_http_error(client.get_easy_convert_currency_list().await.unwrap_err());
    assert!(msg.contains(convert::endpoints::EASY_CONVERT_CURRENCY_LIST));

    let req = okx_core::types::EasyConvertRequest {
        from_ccy: vec!["BTC".to_string()],
        to_ccy: "USDT".to_string(),
    };
    let msg = expect_http_error(client.easy_convert(req).await.unwrap_err());
    assert!(msg.contains(convert::endpoints::EASY_CONVERT));

    let msg = expect_http_error(
        client
            .get_one_click_repay_currency_list()
            .await
            .unwrap_err(),
    );
    assert!(msg.contains(convert::endpoints::ONE_CLICK_REPAY_CURRENCY_LIST));

    let repay = okx_core::types::OneClickRepayRequest {
        debt_ccy: vec!["BTC".to_string()],
        repay_ccy: "USDT".to_string(),
    };
    let msg = expect_http_error(client.one_click_repay(repay).await.unwrap_err());
    assert!(msg.contains(convert::endpoints::ONE_CLICK_REPAY));
}

#[tokio::test]
async fn copy_trading_api_offline_covers_remaining_methods() {
    let client = dummy_client();

    let msg = expect_http_error(
        client
            .get_existing_lead_positions(Some(json!({"instId": "BTC-USDT"})))
            .await
            .unwrap_err(),
    );
    assert!(msg.contains(copy_trading::endpoints::EXISTING_LEAD_POSITIONS));

    let msg = expect_http_error(
        client
            .get_lead_position_history(Some(json!({"after": "1"})))
            .await
            .unwrap_err(),
    );
    assert!(msg.contains(copy_trading::endpoints::LEAD_POSITION_HISTORY));

    let stop_order = json!({"subPosId": "1", "tpTriggerPx": "20000"});
    let msg = expect_http_error(client.place_lead_stop_order(stop_order).await.unwrap_err());
    assert!(msg.contains(copy_trading::endpoints::PLACE_LEAD_STOP_ORDER));

    let close_pos = json!({"subPosId": "1"});
    let msg = expect_http_error(client.close_lead_position(close_pos).await.unwrap_err());
    assert!(msg.contains(copy_trading::endpoints::CLOSE_LEAD_POSITION));

    let msg = expect_http_error(
        client
            .get_leading_instruments(Some(json!({"instType": "SWAP"})))
            .await
            .unwrap_err(),
    );
    assert!(msg.contains(copy_trading::endpoints::LEADING_INSTRUMENTS));

    let msg = expect_http_error(
        client
            .amend_leading_instruments(json!({"instId":"BTC-USDT","enabled":true}))
            .await
            .unwrap_err(),
    );
    assert!(msg.contains(copy_trading::endpoints::AMEND_LEADING_INSTRUMENTS));

    let msg = expect_http_error(
        client
            .get_profit_sharing_details(Some(json!({"after":"1"})))
            .await
            .unwrap_err(),
    );
    assert!(msg.contains(copy_trading::endpoints::PROFIT_SHARING_DETAILS));

    let msg = expect_http_error(client.get_total_profit_sharing().await.unwrap_err());
    assert!(msg.contains(copy_trading::endpoints::TOTAL_PROFIT_SHARING));

    let msg = expect_http_error(
        client
            .get_unrealized_profit_sharing_details(Some(json!({"after":"1"})))
            .await
            .unwrap_err(),
    );
    assert!(msg.contains(copy_trading::endpoints::UNREALIZED_PROFIT_SHARING_DETAILS));
}

#[tokio::test]
async fn subaccount_api_offline_covers_remaining_methods() {
    let client = dummy_client();

    let msg = expect_http_error(client.get_subaccount_balance("sub1").await.unwrap_err());
    assert!(msg.contains(subaccount::endpoints::BALANCE));

    let bills = subaccount::SubaccountBillsParams {
        ccy: Some("BTC".to_string()),
        bill_type: None,
        sub_acct: Some("sub1".to_string()),
        after: Some("1".to_string()),
        before: None,
        limit: Some(1),
    };
    let msg = expect_http_error(client.get_subaccount_bills(Some(bills)).await.unwrap_err());
    assert!(msg.contains(subaccount::endpoints::BILLS));

    let reset = subaccount::ResetSubaccountApikeyRequest {
        sub_acct: "sub1".to_string(),
        api_key: "key123".to_string(),
        label: "label".to_string(),
        perm: "read_only".to_string(),
        ip: Some("127.0.0.1".to_string()),
    };
    let msg = expect_http_error(client.reset_subaccount_apikey(reset).await.unwrap_err());
    assert!(msg.contains(subaccount::endpoints::RESET_APIKEY));

    let msg = expect_http_error(client.get_subaccount_list(None).await.unwrap_err());
    assert!(msg.contains(subaccount::endpoints::LIST));

    let transfer = subaccount::SubaccountTransferRequest {
        ccy: "USDT".into(),
        amt: "10".into(),
        froms: "6".into(),
        to: "18".into(),
        from_sub_account: "sub1".into(),
        to_sub_account: "sub2".into(),
        loan_trans: Some(false),
        omit_pos_risk: Some(false),
    };
    let msg = expect_http_error(client.subaccount_transfer(transfer).await.unwrap_err());
    assert!(msg.contains(subaccount::endpoints::TRANSFER));

    let msg = expect_http_error(client.get_entrust_subaccount_list(None).await.unwrap_err());
    assert!(msg.contains(subaccount::endpoints::ENTRUST_LIST));

    let set_transfer_out = subaccount::SetTransferOutRequest {
        sub_acct: "sub1".to_string(),
        can_trans_out: true,
    };
    let msg = expect_http_error(
        client
            .set_permission_transfer_out(set_transfer_out)
            .await
            .unwrap_err(),
    );
    assert!(msg.contains(subaccount::endpoints::SET_TRANSFER_OUT));

    let msg = expect_http_error(client.get_funding_balance("sub1", None).await.unwrap_err());
    assert!(msg.contains(subaccount::endpoints::FUNDING_BALANCE));

    let msg = expect_http_error(
        client
            .get_affiliate_rebate_info("key123")
            .await
            .unwrap_err(),
    );
    assert!(msg.contains(subaccount::endpoints::AFFILIATE_REBATE));

    let vip_loan = subaccount::SetVipLoanRequest {
        enable: true,
        alloc: json!([{"ccy":"BTC","ratio":"1"}]),
    };
    let msg = expect_http_error(
        client
            .set_sub_accounts_vip_loan(vip_loan)
            .await
            .unwrap_err(),
    );
    assert!(msg.contains(subaccount::endpoints::SET_VIP_LOAN));

    let interest = subaccount::SubaccountInterestParams {
        sub_acct: Some("sub1".to_string()),
        ccy: Some("BTC".to_string()),
    };
    let msg = expect_http_error(
        client
            .get_sub_account_borrow_interest_and_limit(Some(interest))
            .await
            .unwrap_err(),
    );
    assert!(msg.contains(subaccount::endpoints::BORROW_INTEREST_LIMIT));
}

#[tokio::test]
async fn finance_api_offline_covers_remaining_methods() {
    let client = dummy_client();

    // Savings: 出借利率历史（公共）
    let msg = expect_http_error(
        client
            .saving_lending_rate_history(Some(json!({"ccy":"USDT"})))
            .await
            .unwrap_err(),
    );
    assert!(msg.contains(finance::endpoints::SAVING_LENDING_RATE_HISTORY));

    // Flexible Loan
    let msg = expect_http_error(client.flexible_loan_borrow_currencies().await.unwrap_err());
    assert!(msg.contains(finance::endpoints::FLEXIBLE_LOAN_BORROW_CURRENCIES));

    let msg = expect_http_error(
        client
            .flexible_loan_collateral_assets(Some(json!({"ccy":"BTC"})))
            .await
            .unwrap_err(),
    );
    assert!(msg.contains(finance::endpoints::FLEXIBLE_LOAN_COLLATERAL_ASSETS));

    let msg = expect_http_error(
        client
            .flexible_loan_max_loan(json!({"ccy":"USDT","amt":"10"}))
            .await
            .unwrap_err(),
    );
    assert!(msg.contains(finance::endpoints::FLEXIBLE_LOAN_MAX_LOAN));

    let msg = expect_http_error(
        client
            .flexible_loan_max_collateral_redeem_amount(Some(json!({"ccy":"BTC"})))
            .await
            .unwrap_err(),
    );
    assert!(msg.contains(finance::endpoints::FLEXIBLE_LOAN_MAX_REDEEM_AMOUNT));

    let msg = expect_http_error(
        client
            .flexible_loan_adjust_collateral(json!({"ccy":"BTC","amt":"1","type":"add"}))
            .await
            .unwrap_err(),
    );
    assert!(msg.contains(finance::endpoints::FLEXIBLE_LOAN_ADJUST_COLLATERAL));

    let msg = expect_http_error(client.flexible_loan_loan_info().await.unwrap_err());
    assert!(msg.contains(finance::endpoints::FLEXIBLE_LOAN_LOAN_INFO));

    let msg = expect_http_error(
        client
            .flexible_loan_loan_history(Some(json!({"ccy":"BTC"})))
            .await
            .unwrap_err(),
    );
    assert!(msg.contains(finance::endpoints::FLEXIBLE_LOAN_LOAN_HISTORY));

    let msg = expect_http_error(
        client
            .flexible_loan_interest_accrued(Some(json!({"ccy":"BTC"})))
            .await
            .unwrap_err(),
    );
    assert!(msg.contains(finance::endpoints::FLEXIBLE_LOAN_INTEREST_ACCRUED));

    // ETH Staking-Defi
    let msg = expect_http_error(client.staking_defi_eth_product_info().await.unwrap_err());
    assert!(msg.contains(finance::endpoints::STAKING_DEFI_ETH_PRODUCT_INFO));

    let msg = expect_http_error(
        client
            .staking_defi_eth_purchase(json!({"ccy":"ETH","amt":"1"}))
            .await
            .unwrap_err(),
    );
    assert!(msg.contains(finance::endpoints::STAKING_DEFI_ETH_PURCHASE));

    let msg = expect_http_error(
        client
            .staking_defi_eth_redeem(json!({"ordId":"1"}))
            .await
            .unwrap_err(),
    );
    assert!(msg.contains(finance::endpoints::STAKING_DEFI_ETH_REDEEM));

    let msg = expect_http_error(client.staking_defi_eth_balance().await.unwrap_err());
    assert!(msg.contains(finance::endpoints::STAKING_DEFI_ETH_BALANCE));

    let msg = expect_http_error(
        client
            .staking_defi_eth_purchase_redeem_history(Some(json!({"after":"1"})))
            .await
            .unwrap_err(),
    );
    assert!(msg.contains(finance::endpoints::STAKING_DEFI_ETH_PURCHASE_REDEEM_HISTORY));

    let msg = expect_http_error(
        client
            .staking_defi_eth_apy_history(json!({"ccy":"ETH"}))
            .await
            .unwrap_err(),
    );
    assert!(msg.contains(finance::endpoints::STAKING_DEFI_ETH_APY_HISTORY));

    // SOL Staking-Defi
    let msg = expect_http_error(client.staking_defi_sol_product_info().await.unwrap_err());
    assert!(msg.contains(finance::endpoints::STAKING_DEFI_SOL_PRODUCT_INFO));

    let msg = expect_http_error(
        client
            .staking_defi_sol_purchase(json!({"ccy":"SOL","amt":"1"}))
            .await
            .unwrap_err(),
    );
    assert!(msg.contains(finance::endpoints::STAKING_DEFI_SOL_PURCHASE));

    let msg = expect_http_error(
        client
            .staking_defi_sol_redeem(json!({"ordId":"1"}))
            .await
            .unwrap_err(),
    );
    assert!(msg.contains(finance::endpoints::STAKING_DEFI_SOL_REDEEM));

    let msg = expect_http_error(client.staking_defi_sol_balance().await.unwrap_err());
    assert!(msg.contains(finance::endpoints::STAKING_DEFI_SOL_BALANCE));

    let msg = expect_http_error(
        client
            .staking_defi_sol_purchase_redeem_history(Some(json!({"after":"1"})))
            .await
            .unwrap_err(),
    );
    assert!(msg.contains(finance::endpoints::STAKING_DEFI_SOL_PURCHASE_REDEEM_HISTORY));

    let msg = expect_http_error(
        client
            .staking_defi_sol_apy_history(json!({"ccy":"SOL"}))
            .await
            .unwrap_err(),
    );
    assert!(msg.contains(finance::endpoints::STAKING_DEFI_SOL_APY_HISTORY));
}
