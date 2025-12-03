//! WebSocket channel definitions.
//!
//! Source: OKX API v5 WebSocket API
//! - <https://www.okx.com/docs-v5/en/#websocket-api>

use serde::{Deserialize, Serialize};

/// WebSocket channel subscription.
///
/// Represents a channel to subscribe to on the OKX WebSocket API.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "channel", rename_all = "kebab-case")]
pub enum Channel {
    // ==================== Public Channels ====================
    /// Ticker channel - real-time price updates
    #[serde(rename = "tickers")]
    Tickers {
        /// Instrument ID (e.g., "BTC-USDT")
        #[serde(rename = "instId")]
        inst_id: String,
    },

    /// Order book channel - depth updates
    #[serde(rename = "books")]
    Books {
        /// Instrument ID
        #[serde(rename = "instId")]
        inst_id: String,
    },

    /// Order book channel - 5 levels
    #[serde(rename = "books5")]
    Books5 {
        /// Instrument ID
        #[serde(rename = "instId")]
        inst_id: String,
    },

    /// Order book channel - 50 levels, 10ms push
    #[serde(rename = "books50-l2-tbt")]
    Books50L2Tbt {
        /// Instrument ID
        #[serde(rename = "instId")]
        inst_id: String,
    },

    /// Order book channel - full depth, 10ms push
    #[serde(rename = "books-l2-tbt")]
    BooksL2Tbt {
        /// Instrument ID
        #[serde(rename = "instId")]
        inst_id: String,
    },

    /// Trades channel - real-time trades
    #[serde(rename = "trades")]
    Trades {
        /// Instrument ID
        #[serde(rename = "instId")]
        inst_id: String,
    },

    /// Candlesticks channel
    #[serde(rename = "candle1m")]
    Candle1m {
        /// Instrument ID
        #[serde(rename = "instId")]
        inst_id: String,
    },

    /// Candlesticks channel - 5 minutes
    #[serde(rename = "candle5m")]
    Candle5m {
        /// Instrument ID
        #[serde(rename = "instId")]
        inst_id: String,
    },

    /// Candlesticks channel - 15 minutes
    #[serde(rename = "candle15m")]
    Candle15m {
        /// Instrument ID
        #[serde(rename = "instId")]
        inst_id: String,
    },

    /// Candlesticks channel - 1 hour
    #[serde(rename = "candle1H")]
    Candle1H {
        /// Instrument ID
        #[serde(rename = "instId")]
        inst_id: String,
    },

    /// Candlesticks channel - 4 hours
    #[serde(rename = "candle4H")]
    Candle4H {
        /// Instrument ID
        #[serde(rename = "instId")]
        inst_id: String,
    },

    /// Candlesticks channel - 1 day
    #[serde(rename = "candle1D")]
    Candle1D {
        /// Instrument ID
        #[serde(rename = "instId")]
        inst_id: String,
    },

    /// Mark price channel
    #[serde(rename = "mark-price")]
    MarkPrice {
        /// Instrument ID
        #[serde(rename = "instId")]
        inst_id: String,
    },

    /// Index tickers channel
    #[serde(rename = "index-tickers")]
    IndexTickers {
        /// Instrument ID
        #[serde(rename = "instId")]
        inst_id: String,
    },

    /// Funding rate channel
    #[serde(rename = "funding-rate")]
    FundingRate {
        /// Instrument ID
        #[serde(rename = "instId")]
        inst_id: String,
    },

    // ==================== Private Channels ====================
    /// Account channel - balance updates
    #[serde(rename = "account")]
    Account {
        /// Currency (optional)
        #[serde(skip_serializing_if = "Option::is_none")]
        ccy: Option<String>,
    },

    /// Positions channel - position updates
    #[serde(rename = "positions")]
    Positions {
        /// Instrument type: MARGIN, SWAP, FUTURES, OPTION
        #[serde(rename = "instType")]
        inst_type: String,
        /// Instrument family (optional)
        #[serde(rename = "instFamily", skip_serializing_if = "Option::is_none")]
        inst_family: Option<String>,
        /// Instrument ID (optional)
        #[serde(rename = "instId", skip_serializing_if = "Option::is_none")]
        inst_id: Option<String>,
    },

    /// Orders channel - order updates
    #[serde(rename = "orders")]
    Orders {
        /// Instrument type: SPOT, MARGIN, SWAP, FUTURES, OPTION
        #[serde(rename = "instType")]
        inst_type: String,
        /// Instrument family (optional)
        #[serde(rename = "instFamily", skip_serializing_if = "Option::is_none")]
        inst_family: Option<String>,
        /// Instrument ID (optional)
        #[serde(rename = "instId", skip_serializing_if = "Option::is_none")]
        inst_id: Option<String>,
    },

    /// Algo orders channel
    #[serde(rename = "orders-algo")]
    OrdersAlgo {
        /// Instrument type
        #[serde(rename = "instType")]
        inst_type: String,
        /// Instrument family (optional)
        #[serde(rename = "instFamily", skip_serializing_if = "Option::is_none")]
        inst_family: Option<String>,
        /// Instrument ID (optional)
        #[serde(rename = "instId", skip_serializing_if = "Option::is_none")]
        inst_id: Option<String>,
    },

    /// Fills channel - execution updates
    #[serde(rename = "fills")]
    Fills {
        /// Instrument type: SPOT, MARGIN, SWAP, FUTURES, OPTION
        #[serde(rename = "instType")]
        inst_type: String,
        /// Instrument family (optional)
        #[serde(rename = "instFamily", skip_serializing_if = "Option::is_none")]
        inst_family: Option<String>,
        /// Instrument ID (optional)
        #[serde(rename = "instId", skip_serializing_if = "Option::is_none")]
        inst_id: Option<String>,
    },

    /// Balance and position channel - combined updates
    #[serde(rename = "balance_and_position")]
    BalanceAndPosition,

    /// Grid algo orders channel
    #[serde(rename = "grid-orders")]
    GridOrders {
        #[serde(rename = "algoId", skip_serializing_if = "Option::is_none")]
        algo_id: Option<String>,
        #[serde(rename = "instType", skip_serializing_if = "Option::is_none")]
        inst_type: Option<String>,
        #[serde(rename = "instId", skip_serializing_if = "Option::is_none")]
        inst_id: Option<String>,
    },

    /// Copy trading lead notifications channel
    #[serde(rename = "copytrading-lead-notify")]
    CopyTradingLeadNotify {
        #[serde(rename = "instId", skip_serializing_if = "Option::is_none")]
        inst_id: Option<String>,
    },

    /// Recurring buy orders channel
    #[serde(rename = "recurring-orders")]
    RecurringOrders {
        #[serde(rename = "algoId", skip_serializing_if = "Option::is_none")]
        algo_id: Option<String>,
    },

    /// Block trading RFQs（business私有）
    #[serde(rename = "rfqs")]
    Rfqs {
        #[serde(rename = "instFamily", skip_serializing_if = "Option::is_none")]
        inst_family: Option<String>,
    },

    /// Block trading quotes（business私有）
    #[serde(rename = "quotes")]
    Quotes {
        #[serde(rename = "instFamily", skip_serializing_if = "Option::is_none")]
        inst_family: Option<String>,
    },

    /// 结构化大宗成交（私有）
    #[serde(rename = "struc-block-trades")]
    StrucBlockTrades {
        #[serde(rename = "instFamily", skip_serializing_if = "Option::is_none")]
        inst_family: Option<String>,
    },

    /// 公开结构化大宗成交（公共）
    #[serde(rename = "public-struc-block-trades")]
    PublicStrucBlockTrades {
        #[serde(rename = "instFamily", skip_serializing_if = "Option::is_none")]
        inst_family: Option<String>,
    },

    /// 公开大宗成交（公共）
    #[serde(rename = "public-block-trades")]
    PublicBlockTrades {
        #[serde(rename = "instFamily", skip_serializing_if = "Option::is_none")]
        inst_family: Option<String>,
    },

    /// Block Tickers（公共）
    #[serde(rename = "block-tickers")]
    BlockTickers {
        #[serde(rename = "instFamily", skip_serializing_if = "Option::is_none")]
        inst_family: Option<String>,
    },

    /// 高级算法频道（私有）
    #[serde(rename = "algo-advance")]
    AlgoAdvance {
        #[serde(rename = "instType", skip_serializing_if = "Option::is_none")]
        inst_type: Option<String>,
        #[serde(rename = "instFamily", skip_serializing_if = "Option::is_none")]
        inst_family: Option<String>,
        #[serde(rename = "instId", skip_serializing_if = "Option::is_none")]
        inst_id: Option<String>,
    },

    /// 现货网格订单（私有）
    #[serde(rename = "grid-orders-spot")]
    GridOrdersSpot {
        #[serde(rename = "algoId", skip_serializing_if = "Option::is_none")]
        algo_id: Option<String>,
        #[serde(rename = "instId", skip_serializing_if = "Option::is_none")]
        inst_id: Option<String>,
    },

    /// 合约网格订单（私有）
    #[serde(rename = "grid-orders-contract")]
    GridOrdersContract {
        #[serde(rename = "algoId", skip_serializing_if = "Option::is_none")]
        algo_id: Option<String>,
        #[serde(rename = "instId", skip_serializing_if = "Option::is_none")]
        inst_id: Option<String>,
    },

    /// 月亮网格订单（私有）
    #[serde(rename = "grid-orders-moon")]
    GridOrdersMoon {
        #[serde(rename = "algoId", skip_serializing_if = "Option::is_none")]
        algo_id: Option<String>,
        #[serde(rename = "instId", skip_serializing_if = "Option::is_none")]
        inst_id: Option<String>,
    },

    /// 网格仓位（私有）
    #[serde(rename = "grid-positions")]
    GridPositions {
        #[serde(rename = "algoId", skip_serializing_if = "Option::is_none")]
        algo_id: Option<String>,
        #[serde(rename = "instType", skip_serializing_if = "Option::is_none")]
        inst_type: Option<String>,
        #[serde(rename = "instId", skip_serializing_if = "Option::is_none")]
        inst_id: Option<String>,
    },

    /// 网格子订单（私有）
    #[serde(rename = "grid-sub-orders")]
    GridSubOrders {
        #[serde(rename = "algoId", skip_serializing_if = "Option::is_none")]
        algo_id: Option<String>,
        #[serde(rename = "instId", skip_serializing_if = "Option::is_none")]
        inst_id: Option<String>,
    },

    /// 定投订单（私有）
    #[serde(rename = "algo-recurring-buy")]
    AlgoRecurringBuy {
        #[serde(rename = "algoId", skip_serializing_if = "Option::is_none")]
        algo_id: Option<String>,
    },
}

impl Channel {
    /// Check if this is a private channel (requires authentication).
    #[must_use]
    pub fn is_private(&self) -> bool {
        matches!(
            self,
            Self::Account { .. }
                | Self::Positions { .. }
                | Self::Orders { .. }
                | Self::OrdersAlgo { .. }
                | Self::AlgoAdvance { .. }
                | Self::Fills { .. }
                | Self::GridOrders { .. }
                | Self::GridOrdersSpot { .. }
                | Self::GridOrdersContract { .. }
                | Self::GridOrdersMoon { .. }
                | Self::GridPositions { .. }
                | Self::GridSubOrders { .. }
                | Self::CopyTradingLeadNotify { .. }
                | Self::RecurringOrders { .. }
                | Self::AlgoRecurringBuy { .. }
                | Self::Rfqs { .. }
                | Self::Quotes { .. }
                | Self::StrucBlockTrades { .. }
                | Self::BalanceAndPosition
        )
    }

    /// Get the channel name.
    #[must_use]
    pub fn name(&self) -> &'static str {
        match self {
            Self::Tickers { .. } => "tickers",
            Self::Books { .. } => "books",
            Self::Books5 { .. } => "books5",
            Self::Books50L2Tbt { .. } => "books50-l2-tbt",
            Self::BooksL2Tbt { .. } => "books-l2-tbt",
            Self::Trades { .. } => "trades",
            Self::Candle1m { .. } => "candle1m",
            Self::Candle5m { .. } => "candle5m",
            Self::Candle15m { .. } => "candle15m",
            Self::Candle1H { .. } => "candle1H",
            Self::Candle4H { .. } => "candle4H",
            Self::Candle1D { .. } => "candle1D",
            Self::MarkPrice { .. } => "mark-price",
            Self::IndexTickers { .. } => "index-tickers",
            Self::FundingRate { .. } => "funding-rate",
            Self::Account { .. } => "account",
            Self::Positions { .. } => "positions",
            Self::Orders { .. } => "orders",
            Self::OrdersAlgo { .. } => "orders-algo",
            Self::AlgoAdvance { .. } => "algo-advance",
            Self::Fills { .. } => "fills",
            Self::BalanceAndPosition => "balance_and_position",
            Self::GridOrders { .. } => "grid-orders",
            Self::GridOrdersSpot { .. } => "grid-orders-spot",
            Self::GridOrdersContract { .. } => "grid-orders-contract",
            Self::GridOrdersMoon { .. } => "grid-orders-moon",
            Self::GridPositions { .. } => "grid-positions",
            Self::GridSubOrders { .. } => "grid-sub-orders",
            Self::CopyTradingLeadNotify { .. } => "copytrading-lead-notify",
            Self::RecurringOrders { .. } => "recurring-orders",
            Self::AlgoRecurringBuy { .. } => "algo-recurring-buy",
            Self::Rfqs { .. } => "rfqs",
            Self::Quotes { .. } => "quotes",
            Self::StrucBlockTrades { .. } => "struc-block-trades",
            Self::PublicStrucBlockTrades { .. } => "public-struc-block-trades",
            Self::PublicBlockTrades { .. } => "public-block-trades",
            Self::BlockTickers { .. } => "block-tickers",
        }
    }
}
