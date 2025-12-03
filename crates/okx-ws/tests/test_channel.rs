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

#[test]
fn test_fills_channel_serialization_and_flags() {
    let chan = Channel::Fills {
        inst_type: "SWAP".into(),
        inst_family: Some("BTC-USD".into()),
        inst_id: None,
    };
    assert!(chan.is_private());
    assert_eq!(chan.name(), "fills");

    let value = serde_json::to_value(&chan).expect("序列化应成功");
    assert_eq!(value["channel"], "fills");
    assert_eq!(value["instType"], "SWAP");
    assert_eq!(value["instFamily"], "BTC-USD");
    assert!(value.get("instId").is_none());
}

#[test]
fn test_strategy_channels_serialization() {
    // grid-orders
    let grid = Channel::GridOrders {
        algo_id: Some("123".into()),
        inst_type: Some("SWAP".into()),
        inst_id: Some("BTC-USDT-SWAP".into()),
    };
    assert!(grid.is_private());
    assert_eq!(grid.name(), "grid-orders");
    let v = serde_json::to_value(&grid).unwrap();
    assert_eq!(v["channel"], "grid-orders");
    assert_eq!(v["algoId"], "123");
    assert_eq!(v["instType"], "SWAP");

    // copytrading-lead-notify
    let copy = Channel::CopyTradingLeadNotify {
        inst_id: Some("BTC-USDT".into()),
    };
    assert!(copy.is_private());
    assert_eq!(copy.name(), "copytrading-lead-notify");
    let v = serde_json::to_value(&copy).unwrap();
    assert_eq!(v["channel"], "copytrading-lead-notify");

    // recurring-orders
    let recur = Channel::RecurringOrders {
        algo_id: Some("456".into()),
    };
    assert!(recur.is_private());
    assert_eq!(recur.name(), "recurring-orders");
    let v = serde_json::to_value(&recur).unwrap();
    assert_eq!(v["channel"], "recurring-orders");
    assert_eq!(v["algoId"], "456");
}

#[test]
fn test_block_channels_serialization() {
    let rfqs = Channel::Rfqs {
        inst_family: Some("BTC-USD".into()),
    };
    assert!(rfqs.is_private());
    assert_eq!(rfqs.name(), "rfqs");
    let v = serde_json::to_value(&rfqs).unwrap();
    assert_eq!(v["channel"], "rfqs");
    assert_eq!(v["instFamily"], "BTC-USD");

    let public_block = Channel::PublicBlockTrades { inst_family: None };
    assert!(!public_block.is_private());
    assert_eq!(public_block.name(), "public-block-trades");
    let v = serde_json::to_value(&public_block).unwrap();
    assert_eq!(v["channel"], "public-block-trades");
    assert!(v.get("instFamily").is_none());

    let tickers = Channel::BlockTickers {
        inst_family: Some("BTC-USD".into()),
    };
    assert!(!tickers.is_private());
    assert_eq!(tickers.name(), "block-tickers");
    let v = serde_json::to_value(&tickers).unwrap();
    assert_eq!(v["channel"], "block-tickers");
    assert_eq!(v["instFamily"], "BTC-USD");
}

#[test]
fn test_advanced_algo_channel_names() {
    let advance = Channel::AlgoAdvance {
        inst_type: Some("SWAP".into()),
        inst_family: None,
        inst_id: Some("BTC-USD-SWAP".into()),
    };
    assert!(advance.is_private());
    assert_eq!(advance.name(), "algo-advance");

    let grid_spot = Channel::GridOrdersSpot {
        algo_id: Some("1".into()),
        inst_id: Some("BTC-USDT".into()),
    };
    assert!(grid_spot.is_private());
    assert_eq!(grid_spot.name(), "grid-orders-spot");

    let recurring = Channel::AlgoRecurringBuy {
        algo_id: Some("2".into()),
    };
    assert!(recurring.is_private());
    assert_eq!(recurring.name(), "algo-recurring-buy");
}
