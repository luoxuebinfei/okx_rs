use okx_rest::api::{
    account, block_rfq, broker, convert, copy_trading, finance, funding, grid, market, public,
    spread, subaccount, trade, trading_data,
};

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
        (account::endpoints::MAX_LOAN, "/api/v5/account/max-loan"),
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
            account::endpoints::INTEREST_RATE,
            "/api/v5/account/interest-rate",
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
        (
            account::endpoints::ADJUSTMENT_MARGIN,
            "/api/v5/account/position/margin-balance",
        ),
        (
            account::endpoints::SET_RISK_OFFSET_TYPE,
            "/api/v5/account/set-riskOffset-type",
        ),
        (
            account::endpoints::SET_AUTO_LOAN,
            "/api/v5/account/set-auto-loan",
        ),
        (
            account::endpoints::INSTRUMENTS,
            "/api/v5/account/instruments",
        ),
        (account::endpoints::RISK_STATE, "/api/v5/account/risk-state"),
    ];

    for (actual, expected) in cases {
        assert_eq!(actual, expected);
    }
}

#[test]
fn broker_endpoints_match_official_sdk() {
    let cases = [
        (
            broker::endpoints::FD_REBATE_PER_ORDERS,
            "/api/v5/broker/fd/rebate-per-orders",
        ),
        (
            broker::endpoints::FD_GET_REBATE_PER_ORDERS,
            "/api/v5/broker/fd/rebate-per-orders",
        ),
    ];

    for (actual, expected) in cases {
        assert_eq!(actual, expected);
    }
}

#[test]
fn finance_endpoints_match_official_sdk() {
    let cases = [
        (
            finance::endpoints::DEFI_OFFERS,
            "/api/v5/finance/staking-defi/offers",
        ),
        (
            finance::endpoints::DEFI_PURCHASE,
            "/api/v5/finance/staking-defi/purchase",
        ),
        (
            finance::endpoints::DEFI_REDEEM,
            "/api/v5/finance/staking-defi/redeem",
        ),
        (
            finance::endpoints::DEFI_CANCEL,
            "/api/v5/finance/staking-defi/cancel",
        ),
        (
            finance::endpoints::DEFI_ORDERS_ACTIVE,
            "/api/v5/finance/staking-defi/orders-active",
        ),
        (
            finance::endpoints::DEFI_ORDERS_HISTORY,
            "/api/v5/finance/staking-defi/orders-history",
        ),
        (
            finance::endpoints::SAVING_BALANCE,
            "/api/v5/finance/savings/balance",
        ),
        (
            finance::endpoints::SAVING_PURCHASE_REDEMPTION,
            "/api/v5/finance/savings/purchase-redempt",
        ),
        (
            finance::endpoints::SAVING_SET_LENDING_RATE,
            "/api/v5/finance/savings/set-lending-rate",
        ),
        (
            finance::endpoints::SAVING_LENDING_HISTORY,
            "/api/v5/finance/savings/lending-history",
        ),
        (
            finance::endpoints::SAVING_PUBLIC_LENDING_RATE,
            "/api/v5/finance/savings/lending-rate-summary",
        ),
        (
            finance::endpoints::SAVING_LENDING_RATE_HISTORY,
            "/api/v5/finance/savings/lending-rate-history",
        ),
        (
            finance::endpoints::FLEXIBLE_LOAN_BORROW_CURRENCIES,
            "/api/v5/finance/flexible-loan/borrow-currencies",
        ),
        (
            finance::endpoints::FLEXIBLE_LOAN_COLLATERAL_ASSETS,
            "/api/v5/finance/flexible-loan/collateral-assets",
        ),
        (
            finance::endpoints::FLEXIBLE_LOAN_MAX_LOAN,
            "/api/v5/finance/flexible-loan/max-loan",
        ),
        (
            finance::endpoints::FLEXIBLE_LOAN_LOAN_INFO,
            "/api/v5/finance/flexible-loan/loan-info",
        ),
        (
            finance::endpoints::FLEXIBLE_LOAN_LOAN_HISTORY,
            "/api/v5/finance/flexible-loan/loan-history",
        ),
        (
            finance::endpoints::FLEXIBLE_LOAN_INTEREST_ACCRUED,
            "/api/v5/finance/flexible-loan/interest-accrued",
        ),
        (
            finance::endpoints::FLEXIBLE_LOAN_MAX_REDEEM_AMOUNT,
            "/api/v5/finance/flexible-loan/max-collateral-redeem-amount",
        ),
        (
            finance::endpoints::FLEXIBLE_LOAN_ADJUST_COLLATERAL,
            "/api/v5/finance/flexible-loan/adjust-collateral",
        ),
        (
            finance::endpoints::STAKING_DEFI_ETH_PRODUCT_INFO,
            "/api/v5/finance/staking-defi/eth/product-info",
        ),
        (
            finance::endpoints::STAKING_DEFI_ETH_BALANCE,
            "/api/v5/finance/staking-defi/eth/balance",
        ),
        (
            finance::endpoints::STAKING_DEFI_ETH_APY_HISTORY,
            "/api/v5/finance/staking-defi/eth/apy-history",
        ),
        (
            finance::endpoints::STAKING_DEFI_ETH_PURCHASE,
            "/api/v5/finance/staking-defi/eth/purchase",
        ),
        (
            finance::endpoints::STAKING_DEFI_ETH_REDEEM,
            "/api/v5/finance/staking-defi/eth/redeem",
        ),
        (
            finance::endpoints::STAKING_DEFI_ETH_PURCHASE_REDEEM_HISTORY,
            "/api/v5/finance/staking-defi/eth/purchase-redeem-history",
        ),
        (
            finance::endpoints::STAKING_DEFI_SOL_PRODUCT_INFO,
            "/api/v5/finance/staking-defi/sol/product-info",
        ),
        (
            finance::endpoints::STAKING_DEFI_SOL_BALANCE,
            "/api/v5/finance/staking-defi/sol/balance",
        ),
        (
            finance::endpoints::STAKING_DEFI_SOL_APY_HISTORY,
            "/api/v5/finance/staking-defi/sol/apy-history",
        ),
        (
            finance::endpoints::STAKING_DEFI_SOL_PURCHASE,
            "/api/v5/finance/staking-defi/sol/purchase",
        ),
        (
            finance::endpoints::STAKING_DEFI_SOL_REDEEM,
            "/api/v5/finance/staking-defi/sol/redeem",
        ),
        (
            finance::endpoints::STAKING_DEFI_SOL_PURCHASE_REDEEM_HISTORY,
            "/api/v5/finance/staking-defi/sol/purchase-redeem-history",
        ),
        (
            finance::endpoints::SIMPLE_EARN_LENDING_OFFERS,
            "/api/v5/finance/fixed-loan/lending-offers",
        ),
        (
            finance::endpoints::SIMPLE_EARN_LENDING_APY_HISTORY,
            "/api/v5/finance/fixed-loan/lending-apy-history",
        ),
        (
            finance::endpoints::SIMPLE_EARN_PENDING_LENDING_VOLUME,
            "/api/v5/finance/fixed-loan/pending-lending-volume",
        ),
        (
            finance::endpoints::SIMPLE_EARN_LENDING_ORDER,
            "/api/v5/finance/fixed-loan/lending-order",
        ),
        (
            finance::endpoints::SIMPLE_EARN_AMEND_LENDING_ORDER,
            "/api/v5/finance/fixed-loan/amend-lending-order",
        ),
        (
            finance::endpoints::SIMPLE_EARN_LENDING_ORDERS_LIST,
            "/api/v5/finance/fixed-loan/lending-orders-list",
        ),
        (
            finance::endpoints::SIMPLE_EARN_LENDING_SUB_ORDERS,
            "/api/v5/finance/fixed-loan/lending-sub-orders",
        ),
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
        (trade::endpoints::MASS_CANCEL, "/api/v5/trade/mass-cancel"),
        (
            trade::endpoints::CANCEL_ALL_AFTER,
            "/api/v5/trade/cancel-all-after",
        ),
        (
            trade::endpoints::ORDER_PRECHECK,
            "/api/v5/trade/order-precheck",
        ),
        (
            trade::endpoints::ONE_CLICK_REPAY_CURRENCY_LIST_V2,
            "/api/v5/trade/one-click-repay-currency-list-v2",
        ),
        (
            trade::endpoints::ONE_CLICK_REPAY_V2,
            "/api/v5/trade/one-click-repay-v2",
        ),
        (
            trade::endpoints::ONE_CLICK_REPAY_HISTORY_V2,
            "/api/v5/trade/one-click-repay-history-v2",
        ),
    ];

    for (actual, expected) in cases {
        assert_eq!(actual, expected);
    }
}

#[test]
fn trading_data_endpoints_match_official_sdk() {
    let cases = [
        (
            trading_data::endpoints::SUPPORT_COIN,
            "/api/v5/rubik/stat/trading-data/support-coin",
        ),
        (
            trading_data::endpoints::TAKER_VOLUME,
            "/api/v5/rubik/stat/taker-volume",
        ),
        (
            trading_data::endpoints::MARGIN_LENDING_RATIO,
            "/api/v5/rubik/stat/margin/loan-ratio",
        ),
        (
            trading_data::endpoints::LONG_SHORT_RATIO,
            "/api/v5/rubik/stat/contracts/long-short-account-ratio",
        ),
        (
            trading_data::endpoints::CONTRACTS_INTEREST_VOLUME,
            "/api/v5/rubik/stat/contracts/open-interest-volume",
        ),
        (
            trading_data::endpoints::OPTIONS_INTEREST_VOLUME,
            "/api/v5/rubik/stat/option/open-interest-volume",
        ),
        (
            trading_data::endpoints::PUT_CALL_RATIO,
            "/api/v5/rubik/stat/option/open-interest-volume-ratio",
        ),
        (
            trading_data::endpoints::OPEN_INTEREST_VOLUME_EXPIRY,
            "/api/v5/rubik/stat/option/open-interest-volume-expiry",
        ),
        (
            trading_data::endpoints::INTEREST_VOLUME_STRIKE,
            "/api/v5/rubik/stat/option/open-interest-volume-strike",
        ),
        (
            trading_data::endpoints::TAKER_FLOW,
            "/api/v5/rubik/stat/option/taker-block-volume",
        ),
    ];

    for (actual, expected) in cases {
        assert_eq!(actual, expected);
    }
}

#[test]
fn spread_endpoints_match_official_sdk() {
    let cases = [
        (spread::endpoints::PLACE_ORDER, "/api/v5/sprd/order"),
        (spread::endpoints::CANCEL_ORDER, "/api/v5/sprd/cancel-order"),
        (
            spread::endpoints::CANCEL_ALL_ORDERS,
            "/api/v5/sprd/mass-cancel",
        ),
        (spread::endpoints::ORDER_DETAILS, "/api/v5/sprd/order"),
        (
            spread::endpoints::ACTIVE_ORDERS,
            "/api/v5/sprd/orders-pending",
        ),
        (spread::endpoints::ORDERS, "/api/v5/sprd/orders-history"),
        (spread::endpoints::TRADES, "/api/v5/sprd/trades"),
        (spread::endpoints::SPREADS, "/api/v5/sprd/spreads"),
        (spread::endpoints::ORDER_BOOK, "/api/v5/sprd/books"),
        (spread::endpoints::TICKER, "/api/v5/sprd/ticker"),
        (
            spread::endpoints::PUBLIC_TRADES,
            "/api/v5/sprd/public-trades",
        ),
    ];

    for (actual, expected) in cases {
        assert_eq!(actual, expected);
    }
}

#[test]
fn block_rfq_endpoints_match_official_sdk() {
    let cases = [
        (
            block_rfq::endpoints::COUNTERPARTIES,
            "/api/v5/rfq/counterparties",
        ),
        (block_rfq::endpoints::CREATE_RFQ, "/api/v5/rfq/create-rfq"),
        (block_rfq::endpoints::CANCEL_RFQ, "/api/v5/rfq/cancel-rfq"),
        (
            block_rfq::endpoints::CANCEL_BATCH_RFQS,
            "/api/v5/rfq/cancel-batch-rfqs",
        ),
        (
            block_rfq::endpoints::CANCEL_ALL_RFQS,
            "/api/v5/rfq/cancel-all-rfqs",
        ),
        (
            block_rfq::endpoints::EXECUTE_QUOTE,
            "/api/v5/rfq/execute-quote",
        ),
        (
            block_rfq::endpoints::CREATE_QUOTE,
            "/api/v5/rfq/create-quote",
        ),
        (
            block_rfq::endpoints::CANCEL_QUOTE,
            "/api/v5/rfq/cancel-quote",
        ),
        (
            block_rfq::endpoints::CANCEL_BATCH_QUOTES,
            "/api/v5/rfq/cancel-batch-quotes",
        ),
        (
            block_rfq::endpoints::CANCEL_ALL_QUOTES,
            "/api/v5/rfq/cancel-all-quotes",
        ),
        (block_rfq::endpoints::GET_RFQS, "/api/v5/rfq/rfqs"),
        (block_rfq::endpoints::GET_QUOTES, "/api/v5/rfq/quotes"),
        (block_rfq::endpoints::GET_TRADES, "/api/v5/rfq/trades"),
        (
            block_rfq::endpoints::GET_PUBLIC_TRADES,
            "/api/v5/rfq/public-trades",
        ),
        (block_rfq::endpoints::RESET_MMP, "/api/v5/rfq/mmp-reset"),
        (block_rfq::endpoints::SET_MMP, "/api/v5/rfq/mmp-config"),
        (
            block_rfq::endpoints::GET_MMP_CONFIG,
            "/api/v5/rfq/mmp-config",
        ),
        (
            block_rfq::endpoints::SET_MARKER_INSTRUMENT,
            "/api/v5/rfq/maker-instrument-settings",
        ),
        (
            block_rfq::endpoints::GET_QUOTE_PRODUCTS,
            "/api/v5/rfq/quote-products",
        ),
    ];

    for (actual, expected) in cases {
        assert_eq!(actual, expected);
    }
}

#[test]
fn copy_trading_endpoints_match_official_sdk() {
    let cases = [
        (
            copy_trading::endpoints::EXISTING_LEAD_POSITIONS,
            "/api/v5/copytrading/current-subpositions",
        ),
        (
            copy_trading::endpoints::LEAD_POSITION_HISTORY,
            "/api/v5/copytrading/subpositions-history",
        ),
        (
            copy_trading::endpoints::PLACE_LEAD_STOP_ORDER,
            "/api/v5/copytrading/algo-order",
        ),
        (
            copy_trading::endpoints::CLOSE_LEAD_POSITION,
            "/api/v5/copytrading/close-subposition",
        ),
        (
            copy_trading::endpoints::LEADING_INSTRUMENTS,
            "/api/v5/copytrading/instruments",
        ),
        (
            copy_trading::endpoints::AMEND_LEADING_INSTRUMENTS,
            "/api/v5/copytrading/set-instruments",
        ),
        (
            copy_trading::endpoints::PROFIT_SHARING_DETAILS,
            "/api/v5/copytrading/profit-sharing-details",
        ),
        (
            copy_trading::endpoints::TOTAL_PROFIT_SHARING,
            "/api/v5/copytrading/total-profit-sharing",
        ),
        (
            copy_trading::endpoints::UNREALIZED_PROFIT_SHARING_DETAILS,
            "/api/v5/copytrading/unrealized-profit-sharing-details",
        ),
    ];

    for (actual, expected) in cases {
        assert_eq!(actual, expected);
    }
}

#[test]
fn grid_endpoints_match_official_sdk() {
    let cases = [
        (
            grid::endpoints::GRID_ORDER_ALGO,
            "/api/v5/tradingBot/grid/order-algo",
        ),
        (
            grid::endpoints::GRID_AMEND_ORDER_ALGO,
            "/api/v5/tradingBot/grid/amend-order-algo",
        ),
        (
            grid::endpoints::GRID_STOP_ORDER_ALGO,
            "/api/v5/tradingBot/grid/stop-order-algo",
        ),
        (
            grid::endpoints::GRID_ORDERS_ALGO_PENDING,
            "/api/v5/tradingBot/grid/orders-algo-pending",
        ),
        (
            grid::endpoints::GRID_ORDERS_ALGO_HISTORY,
            "/api/v5/tradingBot/grid/orders-algo-history",
        ),
        (
            grid::endpoints::GRID_ORDERS_ALGO_DETAILS,
            "/api/v5/tradingBot/grid/orders-algo-details",
        ),
        (
            grid::endpoints::GRID_SUB_ORDERS,
            "/api/v5/tradingBot/grid/sub-orders",
        ),
        (
            grid::endpoints::GRID_POSITIONS,
            "/api/v5/tradingBot/grid/positions",
        ),
        (
            grid::endpoints::GRID_WITHDRAW_INCOME,
            "/api/v5/tradingBot/grid/withdraw-income",
        ),
        (
            grid::endpoints::GRID_COMPUTE_MARGIN_BALANCE,
            "/api/v5/tradingBot/grid/compute-margin-balance",
        ),
        (
            grid::endpoints::GRID_MARGIN_BALANCE,
            "/api/v5/tradingBot/grid/margin-balance",
        ),
        (
            grid::endpoints::GRID_AI_PARAM,
            "/api/v5/tradingBot/grid/ai-param",
        ),
        (
            grid::endpoints::PLACE_RECURRING_BUY_ORDER,
            "/api/v5/tradingBot/recurring/order-algo",
        ),
        (
            grid::endpoints::AMEND_RECURRING_BUY_ORDER,
            "/api/v5/tradingBot/recurring/amend-order-algo",
        ),
        (
            grid::endpoints::STOP_RECURRING_BUY_ORDER,
            "/api/v5/tradingBot/recurring/stop-order-algo",
        ),
        (
            grid::endpoints::GET_RECURRING_BUY_ORDER_LIST,
            "/api/v5/tradingBot/recurring/orders-algo-pending",
        ),
        (
            grid::endpoints::GET_RECURRING_BUY_ORDER_HISTORY,
            "/api/v5/tradingBot/recurring/orders-algo-history",
        ),
        (
            grid::endpoints::GET_RECURRING_BUY_ORDER_DETAILS,
            "/api/v5/tradingBot/recurring/orders-algo-details",
        ),
        (
            grid::endpoints::GET_RECURRING_BUY_SUB_ORDERS,
            "/api/v5/tradingBot/recurring/sub-orders",
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
        (
            market::endpoints::PLATFORM_24_VOLUME,
            "/api/v5/market/platform-24-volume",
        ),
        (
            market::endpoints::INDEX_COMPONENTS,
            "/api/v5/market/index-components",
        ),
        (
            market::endpoints::EXCHANGE_RATE,
            "/api/v5/market/exchange-rate",
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
        (
            public::endpoints::CONVERT_CONTRACT_COIN,
            "/api/v5/public/convert-contract-coin",
        ),
        (
            public::endpoints::VIP_INTEREST_RATE_LOAN_QUOTA,
            "/api/v5/public/vip-interest-rate-loan-quota",
        ),
        (
            public::endpoints::INSTRUMENT_TICK_BANDS,
            "/api/v5/public/instrument-tick-bands",
        ),
        (
            public::endpoints::OPTION_TRADES,
            "/api/v5/public/option-trades",
        ),
    ];

    for (actual, expected) in cases {
        assert_eq!(actual, expected);
    }
}

#[test]
fn convert_endpoints_match_official_sdk() {
    let cases = [
        (
            convert::endpoints::CONVERT_CURRENCIES,
            "/api/v5/asset/convert/currencies",
        ),
        (
            convert::endpoints::CONVERT_CURRENCY_PAIR,
            "/api/v5/asset/convert/currency-pair",
        ),
        (
            convert::endpoints::CONVERT_ESTIMATE_QUOTE,
            "/api/v5/asset/convert/estimate-quote",
        ),
        (
            convert::endpoints::CONVERT_TRADE,
            "/api/v5/asset/convert/trade",
        ),
        (
            convert::endpoints::CONVERT_HISTORY,
            "/api/v5/asset/convert/history",
        ),
        (
            convert::endpoints::EASY_CONVERT_CURRENCY_LIST,
            "/api/v5/trade/easy-convert-currency-list",
        ),
        (
            convert::endpoints::EASY_CONVERT,
            "/api/v5/trade/easy-convert",
        ),
        (
            convert::endpoints::EASY_CONVERT_HISTORY,
            "/api/v5/trade/easy-convert-history",
        ),
        (
            convert::endpoints::ONE_CLICK_REPAY_CURRENCY_LIST,
            "/api/v5/trade/one-click-repay-currency-list",
        ),
        (
            convert::endpoints::ONE_CLICK_REPAY,
            "/api/v5/trade/one-click-repay",
        ),
        (
            convert::endpoints::ONE_CLICK_REPAY_HISTORY,
            "/api/v5/trade/one-click-repay-history",
        ),
    ];

    for (actual, expected) in cases {
        assert_eq!(actual, expected);
    }
}

#[test]
fn subaccount_endpoints_match_official_sdk() {
    let cases = [
        (
            subaccount::endpoints::BALANCE,
            "/api/v5/account/subaccount/balances",
        ),
        (
            subaccount::endpoints::BILLS,
            "/api/v5/asset/subaccount/bills",
        ),
        (
            subaccount::endpoints::RESET_APIKEY,
            "/api/v5/users/subaccount/modify-apikey",
        ),
        (subaccount::endpoints::LIST, "/api/v5/users/subaccount/list"),
        (
            subaccount::endpoints::TRANSFER,
            "/api/v5/asset/subaccount/transfer",
        ),
        (
            subaccount::endpoints::ENTRUST_LIST,
            "/api/v5/users/entrust-subaccount-list",
        ),
        (
            subaccount::endpoints::SET_TRANSFER_OUT,
            "/api/v5/users/subaccount/set-transfer-out",
        ),
        (
            subaccount::endpoints::FUNDING_BALANCE,
            "/api/v5/asset/subaccount/balances",
        ),
        (
            subaccount::endpoints::AFFILIATE_REBATE,
            "/api/v5/users/partner/if-rebate",
        ),
        (
            subaccount::endpoints::SET_VIP_LOAN,
            "/api/v5/account/subaccount/set-loan-allocation",
        ),
        (
            subaccount::endpoints::BORROW_INTEREST_LIMIT,
            "/api/v5/account/subaccount/interest-limits",
        ),
    ];

    for (actual, expected) in cases {
        assert_eq!(actual, expected);
    }
}
