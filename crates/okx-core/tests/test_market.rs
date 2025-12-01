use okx_core::types::{BookLevel, Candle, OrderBook};

#[test]
fn test_order_book_parsing_filters_and_maps() {
    let book = OrderBook {
        asks: vec![
            vec!["100".into(), "1".into(), "0".into(), "2".into()],
            vec!["101".into(), "1.5".into()], // 应被过滤
        ],
        bids: vec![vec!["99".into(), "2".into(), "0".into(), "3".into()]],
        ts: "123".into(),
    };

    let asks: Vec<BookLevel> = book.parsed_asks();
    assert_eq!(asks.len(), 1);
    assert_eq!(asks[0].price, "100");
    assert_eq!(asks[0].size, "1");
    assert_eq!(asks[0].liquidated_orders, "0");
    assert_eq!(asks[0].order_count, "2");

    let bids: Vec<BookLevel> = book.parsed_bids();
    assert_eq!(bids.len(), 1);
    assert_eq!(bids[0].price, "99");
    assert_eq!(bids[0].size, "2");
    assert_eq!(bids[0].order_count, "3");
}

#[test]
fn test_candle_from_array_and_confirm_flag() {
    let raw = vec![
        "1700000000000".into(),
        "10".into(),
        "12".into(),
        "9".into(),
        "11".into(),
        "100".into(),
        "50".into(),
        "500".into(),
        "1".into(),
    ];

    let candle = Candle::from_array(&raw).expect("应解析成功");
    assert!(candle.is_confirmed());
    assert_eq!(candle.open, "10");
    assert_eq!(candle.close, "11");
    assert_eq!(candle.vol_ccy_quote, "500");

    let incomplete = vec!["1".into(), "2".into()];
    assert!(Candle::from_array(&incomplete).is_none());
}

#[test]
fn test_order_book_parsed_bids_filters_incomplete_levels() {
    let book = OrderBook {
        asks: vec![],
        bids: vec![
            vec!["100".into(), "1".into(), "0".into(), "2".into()],
            vec!["bad".into(), "skip".into()], // 长度不足应被过滤
        ],
        ts: "123".into(),
    };

    let bids = book.parsed_bids();
    assert_eq!(bids.len(), 1);
    assert_eq!(bids[0].price, "100");
}
