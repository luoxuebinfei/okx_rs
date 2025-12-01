use okx_ws::Channel;

#[test]
fn test_public_channel_serialization_and_flags() {
    let chan = Channel::Tickers {
        inst_id: "BTC-USDT".into(),
    };

    assert!(!chan.is_private());
    assert_eq!(chan.name(), "tickers");

    let json = serde_json::to_string(&chan).expect("序列化应成功");
    assert!(json.contains("\"channel\":\"tickers\""));
    assert!(json.contains("\"instId\":\"BTC-USDT\""));
}

#[test]
fn test_additional_channel_names() {
    let cases = vec![
        (
            Channel::Books5 {
                inst_id: "BTC-USDT".into(),
            },
            "books5",
        ),
        (
            Channel::Books50L2Tbt {
                inst_id: "BTC-USDT".into(),
            },
            "books50-l2-tbt",
        ),
        (
            Channel::BooksL2Tbt {
                inst_id: "BTC-USDT".into(),
            },
            "books-l2-tbt",
        ),
        (
            Channel::Candle1m {
                inst_id: "BTC-USDT".into(),
            },
            "candle1m",
        ),
        (
            Channel::Candle5m {
                inst_id: "BTC-USDT".into(),
            },
            "candle5m",
        ),
        (
            Channel::Candle15m {
                inst_id: "BTC-USDT".into(),
            },
            "candle15m",
        ),
        (
            Channel::Candle1H {
                inst_id: "BTC-USDT".into(),
            },
            "candle1H",
        ),
        (
            Channel::Candle4H {
                inst_id: "BTC-USDT".into(),
            },
            "candle4H",
        ),
        (
            Channel::Candle1D {
                inst_id: "BTC-USDT".into(),
            },
            "candle1D",
        ),
        (
            Channel::MarkPrice {
                inst_id: "BTC-USDT".into(),
            },
            "mark-price",
        ),
        (
            Channel::IndexTickers {
                inst_id: "BTC-USD".into(),
            },
            "index-tickers",
        ),
        (
            Channel::FundingRate {
                inst_id: "BTC-USD-SWAP".into(),
            },
            "funding-rate",
        ),
        (
            Channel::OrdersAlgo {
                inst_type: "SWAP".into(),
                inst_family: None,
                inst_id: Some("BTC-USD-SWAP".into()),
            },
            "orders-algo",
        ),
    ];

    for (channel, expected) in cases {
        assert_eq!(channel.name(), expected);
    }
}

#[test]
fn test_private_channel_serialization_and_flags() {
    let orders = Channel::Orders {
        inst_type: "SPOT".into(),
        inst_family: None,
        inst_id: Some("BTC-USDT".into()),
    };

    assert!(orders.is_private());
    assert_eq!(orders.name(), "orders");

    let value = serde_json::to_value(&orders).expect("序列化应成功");
    assert_eq!(value["channel"], "orders");
    assert_eq!(value["instType"], "SPOT");
    assert_eq!(value["instId"], "BTC-USDT");
    assert!(!value.get("instFamily").is_some());

    let orders_algo = Channel::OrdersAlgo {
        inst_type: "SWAP".into(),
        inst_family: Some("BTC-USD".into()),
        inst_id: None,
    };
    assert!(orders_algo.is_private());
    assert_eq!(orders_algo.name(), "orders-algo");
}

#[test]
fn test_balance_and_position_minimal_serialization() {
    let chan = Channel::BalanceAndPosition;
    assert!(chan.is_private());
    assert_eq!(chan.name(), "balance_and_position");

    let json = serde_json::to_string(&chan).expect("序列化应成功");
    assert_eq!(json, "{\"channel\":\"balance_and_position\"}");
}
