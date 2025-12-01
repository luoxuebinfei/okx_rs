use okx_rest::api::{account, funding, market, public, trade};

#[test]
fn account_endpoints_match_official_sdk() {
    let cases = [
        (
            account::endpoints::POSITIONS_HISTORY,
            "/api/v5/account/positions-history",
        ),
        (
            account::endpoints::MAX_WITHDRAWAL,
            "/api/v5/account/max-withdrawal",
        ),
        (account::endpoints::BILLS, "/api/v5/account/bills"),
        (
            account::endpoints::BILLS_ARCHIVE,
            "/api/v5/account/bills-archive",
        ),
        (
            account::endpoints::POSITION_BUILDER,
            "/api/v5/account/position-builder",
        ),
        (account::endpoints::SET_GREEKS, "/api/v5/account/set-greeks"),
        (
            account::endpoints::SET_ISOLATED_MODE,
            "/api/v5/account/set-isolated-mode",
        ),
        (
            account::endpoints::SET_ACCOUNT_LEVEL,
            "/api/v5/account/set-account-level",
        ),
        (
            account::endpoints::BORROW_REPAY,
            "/api/v5/account/borrow-repay",
        ),
        (
            account::endpoints::BORROW_REPAY_HISTORY,
            "/api/v5/account/borrow-repay-history",
        ),
        (
            account::endpoints::SPOT_MANUAL_BORROW_REPAY,
            "/api/v5/account/spot-manual-borrow-repay",
        ),
        (
            account::endpoints::SPOT_BORROW_REPAY_HISTORY,
            "/api/v5/account/spot-borrow-repay-history",
        ),
        (
            account::endpoints::INTEREST_ACCRUED,
            "/api/v5/account/interest-accrued",
        ),
        (
            account::endpoints::VIP_INTEREST_ACCRUED,
            "/api/v5/account/vip-interest-accrued",
        ),
        (
            account::endpoints::VIP_INTEREST_DEDUCTED,
            "/api/v5/account/vip-interest-deducted",
        ),
        (
            account::endpoints::SIMULATED_MARGIN,
            "/api/v5/account/simulated_margin",
        ),
        (
            account::endpoints::ACCOUNT_POSITION_TIERS,
            "/api/v5/account/position-tiers",
        ),
        (account::endpoints::GREEKS, "/api/v5/account/greeks"),
    ];

    for (actual, expected) in cases {
        assert_eq!(actual, expected);
    }
}

#[test]
fn trade_endpoints_match_official_sdk() {
    let cases = [
        (
            trade::endpoints::ORDERS_HISTORY_ARCHIVE,
            "/api/v5/trade/orders-history-archive",
        ),
        (
            trade::endpoints::FILLS_HISTORY,
            "/api/v5/trade/fills-history",
        ),
        (
            trade::endpoints::AMEND_ALGO_ORDER,
            "/api/v5/trade/amend-algos",
        ),
        (
            trade::endpoints::ALGO_ORDER_DETAILS,
            "/api/v5/trade/order-algo",
        ),
    ];

    for (actual, expected) in cases {
        assert_eq!(actual, expected);
    }
}

#[test]
fn funding_endpoints_match_official_sdk() {
    let cases = [
        (
            funding::endpoints::NON_TRADABLE_ASSETS,
            "/api/v5/asset/non-tradable-assets",
        ),
        (
            funding::endpoints::TRANSFER_STATE,
            "/api/v5/asset/transfer-state",
        ),
        (
            funding::endpoints::PURCHASE_REDEMPT,
            "/api/v5/asset/purchase_redempt",
        ),
        (funding::endpoints::BILLS, "/api/v5/asset/bills"),
        (
            funding::endpoints::DEPOSIT_LIGHTNING,
            "/api/v5/asset/deposit-lightning",
        ),
        (
            funding::endpoints::WITHDRAWAL_LIGHTNING,
            "/api/v5/asset/withdrawal-lightning",
        ),
        (
            funding::endpoints::CANCEL_WITHDRAWAL,
            "/api/v5/asset/cancel-withdrawal",
        ),
        (
            funding::endpoints::DEPOSIT_WITHDRAW_STATUS,
            "/api/v5/asset/deposit-withdraw-status",
        ),
        (
            funding::endpoints::SET_LENDING_RATE,
            "/api/v5/asset/set-lending-rate",
        ),
        (
            funding::endpoints::LENDING_HISTORY,
            "/api/v5/asset/lending-history",
        ),
        (
            funding::endpoints::LENDING_RATE_HISTORY,
            "/api/v5/asset/lending-rate-history",
        ),
        (
            funding::endpoints::LENDING_RATE_SUMMARY,
            "/api/v5/asset/lending-rate-summary",
        ),
        (
            funding::endpoints::CONVERT_DUST_ASSETS,
            "/api/v5/asset/convert-dust-assets",
        ),
        (
            funding::endpoints::ASSET_VALUATION,
            "/api/v5/asset/asset-valuation",
        ),
        (
            funding::endpoints::SAVING_BALANCE,
            "/api/v5/asset/saving-balance",
        ),
    ];

    for (actual, expected) in cases {
        assert_eq!(actual, expected);
    }
}

#[test]
fn market_endpoints_match_official_sdk() {
    let cases = [
        (
            market::endpoints::CANDLES_HISTORY,
            "/api/v5/market/history-candles",
        ),
        (
            market::endpoints::INDEX_CANDLES,
            "/api/v5/market/index-candles",
        ),
        (
            market::endpoints::MARK_PRICE_CANDLES,
            "/api/v5/market/mark-price-candles",
        ),
        (
            market::endpoints::TRADES_HISTORY,
            "/api/v5/market/history-trades",
        ),
        (market::endpoints::BOOKS_LITE, "/api/v5/market/books-lite"),
        (
            market::endpoints::BLOCK_TICKER,
            "/api/v5/market/block-ticker",
        ),
        (
            market::endpoints::BLOCK_TICKERS,
            "/api/v5/market/block-tickers",
        ),
        (
            market::endpoints::BLOCK_TRADES,
            "/api/v5/market/block-trades",
        ),
        (
            market::endpoints::OPTION_FAMILY_TRADES,
            "/api/v5/market/option/instrument-family-trades",
        ),
    ];

    for (actual, expected) in cases {
        assert_eq!(actual, expected);
    }
}

#[test]
fn public_endpoints_match_official_sdk() {
    let cases = [
        (
            public::endpoints::DELIVERY_EXERCISE_HISTORY,
            "/api/v5/public/delivery-exercise-history",
        ),
        (
            public::endpoints::OPEN_INTEREST,
            "/api/v5/public/open-interest",
        ),
        (
            public::endpoints::POSITION_TIERS,
            "/api/v5/public/position-tiers",
        ),
    ];

    for (actual, expected) in cases {
        assert_eq!(actual, expected);
    }
}
