//! OKX WebSocket 性能基准测试

#![allow(clippy::semicolon_if_nothing_returned)]
#![allow(missing_docs)]

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use okx_ws::{Channel, WsMessage};

fn bench_message_parsing(c: &mut Criterion) {
    // 测试 Ticker 消息解析
    let ticker_msg = r#"{
        "arg": {
            "channel": "tickers",
            "instId": "BTC-USDT"
        },
        "data": [{
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
        }]
    }"#;

    c.bench_function("parse_ticker_message", |b| {
        b.iter(|| {
            let _ = WsMessage::parse(black_box(ticker_msg));
        })
    });

    // 测试 Orderbook 消息解析
    let book_msg = r#"{
        "arg": {
            "channel": "books",
            "instId": "BTC-USDT"
        },
        "action": "snapshot",
        "data": [{
            "asks": [
                ["50000", "1.5", "0", "3"],
                ["50001", "2.0", "0", "5"]
            ],
            "bids": [
                ["49999", "1.0", "0", "2"],
                ["49998", "1.5", "0", "4"]
            ],
            "ts": "1640000000000",
            "checksum": 123456789
        }]
    }"#;

    c.bench_function("parse_orderbook_message", |b| {
        b.iter(|| {
            let _ = WsMessage::parse(black_box(book_msg));
        })
    });

    // 测试 Trade 消息解析
    let trade_msg = r#"{
        "arg": {
            "channel": "trades",
            "instId": "BTC-USDT"
        },
        "data": [{
            "instId": "BTC-USDT",
            "tradeId": "12345",
            "px": "50000",
            "sz": "0.1",
            "side": "buy",
            "ts": "1640000000000"
        }]
    }"#;

    c.bench_function("parse_trade_message", |b| {
        b.iter(|| {
            let _ = WsMessage::parse(black_box(trade_msg));
        })
    });

    // 测试 Event 消息解析
    let event_msg = r#"{
        "event": "subscribe",
        "arg": {
            "channel": "tickers",
            "instId": "BTC-USDT"
        }
    }"#;

    c.bench_function("parse_event_message", |b| {
        b.iter(|| {
            let _ = WsMessage::parse(black_box(event_msg));
        })
    });

    // 测试 Error 消息解析
    let error_msg = r#"{
        "event": "error",
        "code": "50001",
        "msg": "Invalid parameter"
    }"#;

    c.bench_function("parse_error_message", |b| {
        b.iter(|| {
            let _ = WsMessage::parse(black_box(error_msg));
        })
    });
}

fn bench_channel_serialization(c: &mut Criterion) {
    let channel = Channel::Tickers {
        inst_id: "BTC-USDT".to_string(),
    };

    c.bench_function("serialize_ticker_channel", |b| {
        b.iter(|| {
            let _ = serde_json::to_string(black_box(&channel)).unwrap();
        })
    });

    let channel = Channel::Books {
        inst_id: "BTC-USDT".to_string(),
    };

    c.bench_function("serialize_books_channel", |b| {
        b.iter(|| {
            let _ = serde_json::to_string(black_box(&channel)).unwrap();
        })
    });

    let channel = Channel::Positions {
        inst_type: "SWAP".to_string(),
        inst_family: None,
        inst_id: None,
    };

    c.bench_function("serialize_positions_channel", |b| {
        b.iter(|| {
            let _ = serde_json::to_string(black_box(&channel)).unwrap();
        })
    });

    let channel = Channel::Orders {
        inst_type: "SPOT".to_string(),
        inst_family: None,
        inst_id: Some("BTC-USDT".to_string()),
    };

    c.bench_function("serialize_orders_channel", |b| {
        b.iter(|| {
            let _ = serde_json::to_string(black_box(&channel)).unwrap();
        })
    });
}

fn bench_channel_operations(c: &mut Criterion) {
    let channels = vec![
        Channel::Tickers {
            inst_id: "BTC-USDT".to_string(),
        },
        Channel::Books {
            inst_id: "ETH-USDT".to_string(),
        },
        Channel::Trades {
            inst_id: "SOL-USDT".to_string(),
        },
    ];

    c.bench_function("check_channel_privacy", |b| {
        b.iter(|| {
            for channel in &channels {
                let _ = black_box(channel.is_private());
            }
        })
    });

    c.bench_function("get_channel_names", |b| {
        b.iter(|| {
            for channel in &channels {
                let _ = black_box(channel.name());
            }
        })
    });
}

criterion_group!(
    benches,
    bench_message_parsing,
    bench_channel_serialization,
    bench_channel_operations
);
criterion_main!(benches);
