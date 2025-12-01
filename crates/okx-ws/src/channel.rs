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

    /// Balance and position channel - combined updates
    #[serde(rename = "balance_and_position")]
    BalanceAndPosition,
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
            Self::BalanceAndPosition => "balance_and_position",
        }
    }
}
