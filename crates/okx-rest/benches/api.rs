//! OKX REST API 性能基准测试

#![allow(clippy::semicolon_if_nothing_returned)]
#![allow(missing_docs)]

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use okx_core::types::{Balance, Instrument, Order, Ticker};
use serde_json;

fn bench_deserialization(c: &mut Criterion) {
    // 测试 Ticker 反序列化
    let ticker_json = r#"{
        "instType": "SPOT",
        "instId": "BTC-USDT",
        "last": "50000.5",
        "lastSz": "0.1",
        "askPx": "50001.0",
        "askSz": "1.5",
        "bidPx": "49999.5",
        "bidSz": "2.0",
        "open24h": "49500.0",
        "high24h": "51000.0",
        "low24h": "49000.0",
        "volCcy24h": "1000000",
        "vol24h": "20",
        "ts": "1640000000000",
        "sodUtc0": "49800.0",
        "sodUtc8": "49900.0"
    }"#;

    c.bench_function("deserialize_ticker", |b| {
        b.iter(|| {
            let _: Ticker = serde_json::from_str(black_box(ticker_json)).unwrap();
        })
    });

    // 测试 Balance 反序列化
    let balance_json = r#"{
        "ccy": "BTC",
        "eq": "1.5",
        "cashBal": "1.0",
        "availBal": "0.8",
        "availEq": "1.2",
        "frozenBal": "0.5",
        "ordFrozen": "0.3",
        "liab": "0",
        "upl": "100.5",
        "uplLiab": "0",
        "crossLiab": "0",
        "isoLiab": "0",
        "mgnRatio": "0",
        "interest": "0",
        "twap": "0",
        "maxLoan": "0",
        "eqUsd": "75000.0",
        "borrowFroz": "0",
        "notionalLever": "0",
        "stgyEq": "0",
        "isoUpl": "0",
        "spotInUseAmt": "0"
    }"#;

    c.bench_function("deserialize_balance", |b| {
        b.iter(|| {
            let _: Balance = serde_json::from_str(black_box(balance_json)).unwrap();
        })
    });

    // 测试 Order 反序列化
    let order_json = r#"{
        "instType": "SPOT",
        "instId": "BTC-USDT",
        "ccy": "",
        "ordId": "123456789",
        "clOrdId": "custom_id_001",
        "tag": "",
        "px": "50000",
        "pxUsd": "",
        "pxVol": "",
        "pxType": "",
        "sz": "0.1",
        "notionalUsd": "",
        "ordType": "limit",
        "side": "buy",
        "posSide": "net",
        "tdMode": "cash",
        "accFillSz": "0",
        "fillPx": "0",
        "tradeId": "",
        "fillSz": "0",
        "fillTime": "",
        "state": "live",
        "avgPx": "0",
        "lever": "1",
        "attachAlgoClOrdId": "",
        "tpTriggerPx": "",
        "tpTriggerPxType": "",
        "tpOrdPx": "",
        "slTriggerPx": "",
        "slTriggerPxType": "",
        "slOrdPx": "",
        "attachAlgoOrds": [],
        "stpId": "",
        "stpMode": "",
        "feeCcy": "",
        "fee": "",
        "rebateCcy": "",
        "rebate": "",
        "tgtCcy": "",
        "category": "",
        "uTime": "1640000000000",
        "cTime": "1640000000000",
        "reqId": "",
        "amendResult": "",
        "reduceOnly": "false",
        "cancelSource": "",
        "cancelSourceReason": "",
        "quickMgnType": "",
        "algoClOrdId": "",
        "algoId": "",
        "lastPx": ""
    }"#;

    c.bench_function("deserialize_order", |b| {
        b.iter(|| {
            let _: Order = serde_json::from_str(black_box(order_json)).unwrap();
        })
    });

    // 测试 Instrument 反序列化
    let instrument_json = r#"{
        "instType": "SPOT",
        "instId": "BTC-USDT",
        "uly": "",
        "instFamily": "",
        "category": "1",
        "baseCcy": "BTC",
        "quoteCcy": "USDT",
        "settleCcy": "",
        "ctVal": "",
        "ctMult": "",
        "ctValCcy": "",
        "optType": "",
        "stk": "",
        "listTime": "1640000000000",
        "expTime": "",
        "lever": "10",
        "tickSz": "0.1",
        "lotSz": "0.00000001",
        "minSz": "0.00001",
        "ctType": "",
        "alias": "",
        "state": "live",
        "maxLmtSz": "10000",
        "maxMktSz": "1000",
        "maxTwapSz": "1000",
        "maxIcebergSz": "10000",
        "maxTriggerSz": "10000",
        "maxStopSz": "10000"
    }"#;

    c.bench_function("deserialize_instrument", |b| {
        b.iter(|| {
            let _: Instrument = serde_json::from_str(black_box(instrument_json)).unwrap();
        })
    });
}

fn bench_serialization(c: &mut Criterion) {
    use okx_rest::api::trade::PlaceOrderParams;

    let order_params = PlaceOrderParams {
        inst_id: "BTC-USDT".to_string(),
        td_mode: "cash".to_string(),
        side: "buy".to_string(),
        ord_type: "limit".to_string(),
        sz: "0.1".to_string(),
        px: Some("50000".to_string()),
        ccy: None,
        cl_ord_id: Some("custom_order_001".to_string()),
        tag: None,
        pos_side: None,
        reduce_only: None,
        tgt_ccy: None,
        ban_amend: None,
        tp_trigger_px: None,
        tp_trigger_px_type: None,
        tp_ord_px: None,
        sl_trigger_px: None,
        sl_trigger_px_type: None,
        sl_ord_px: None,
        stp_id: None,
        stpmode: None,
        quick_mgn_type: None,
        attach_algo_ords: None,
    };

    c.bench_function("serialize_order_params", |b| {
        b.iter(|| {
            let _ = serde_json::to_string(black_box(&order_params)).unwrap();
        })
    });
}

criterion_group!(benches, bench_deserialization, bench_serialization);
criterion_main!(benches);
