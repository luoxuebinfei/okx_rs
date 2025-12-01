use okx_core::types::{ApiResponse, Bar, InstType, OrdType, PosSide, Side, TdMode};

#[test]
fn test_api_response_success_flag() {
    let resp = ApiResponse {
        code: "0".into(),
        msg: "".into(),
        data: vec![1, 2, 3],
    };
    assert!(resp.is_success());

    let fail = ApiResponse::<u8> {
        code: "500".into(),
        msg: "error".into(),
        data: vec![],
    };
    assert!(!fail.is_success());
}

#[test]
fn test_enum_as_str_and_serde_roundtrip() {
    assert_eq!(InstType::Swap.as_str(), "SWAP");
    assert_eq!(TdMode::Isolated.as_str(), "isolated");
    assert_eq!(Side::Sell.as_str(), "sell");
    assert_eq!(PosSide::Long.as_str(), "long");
    assert_eq!(OrdType::PostOnly.as_str(), "post_only");
    assert_eq!(OrdType::OptimalLimitIoc.as_str(), "optimal_limit_ioc");

    let ord_json = serde_json::to_string(&OrdType::OptimalLimitIoc).unwrap();
    assert_eq!(ord_json, "\"optimal_limit_ioc\"");

    let inst: InstType = serde_json::from_str("\"FUTURES\"").unwrap();
    assert_eq!(inst, InstType::Futures);
}

#[test]
fn test_enum_as_str_full_coverage() {
    let ord_cases = vec![
        (OrdType::Market, "market"),
        (OrdType::Limit, "limit"),
        (OrdType::PostOnly, "post_only"),
        (OrdType::Fok, "fok"),
        (OrdType::Ioc, "ioc"),
        (OrdType::OptimalLimitIoc, "optimal_limit_ioc"),
        (OrdType::Mmp, "mmp"),
        (OrdType::MmpAndTarget, "mmp_and_target"),
    ];

    for (ord, expected) in ord_cases {
        assert_eq!(ord.as_str(), expected);
    }

    assert_eq!(PosSide::Short.as_str(), "short");
    assert_eq!(PosSide::Net.as_str(), "net");
}

#[test]
fn test_bar_as_str() {
    let cases = vec![
        (Bar::M1, "1m"),
        (Bar::M3, "3m"),
        (Bar::M5, "5m"),
        (Bar::M15, "15m"),
        (Bar::M30, "30m"),
        (Bar::H1, "1H"),
        (Bar::H2, "2H"),
        (Bar::H4, "4H"),
        (Bar::H6, "6H"),
        (Bar::H12, "12H"),
        (Bar::D1, "1D"),
        (Bar::W1, "1W"),
        (Bar::Mo1, "1M"),
        (Bar::Mo3, "3M"),
    ];

    for (bar, expected) in cases {
        assert_eq!(bar.as_str(), expected);
    }
}
