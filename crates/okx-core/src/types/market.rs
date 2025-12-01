//! Market data types.
//!
//! Source: OKX API v5 Market Data REST API
//! - GET /api/v5/market/ticker
//! - GET /api/v5/market/books
//! - GET /api/v5/market/candles
//! - GET /api/v5/market/trades

use serde::{Deserialize, Serialize};

/// Ticker information.
///
/// Source: GET /api/v5/market/ticker response
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ticker {
    /// Instrument type
    pub inst_type: String,
    /// Instrument ID
    pub inst_id: String,
    /// Last traded price
    pub last: String,
    /// Last traded size
    #[serde(default)]
    pub last_sz: String,
    /// Best ask price
    #[serde(default)]
    pub ask_px: String,
    /// Best ask size
    #[serde(default)]
    pub ask_sz: String,
    /// Best bid price
    #[serde(default)]
    pub bid_px: String,
    /// Best bid size
    #[serde(default)]
    pub bid_sz: String,
    /// Open price in the past 24 hours
    #[serde(default)]
    pub open_24h: String,
    /// Highest price in the past 24 hours
    #[serde(default)]
    pub high_24h: String,
    /// Lowest price in the past 24 hours
    #[serde(default)]
    pub low_24h: String,
    /// 24h trading volume in base currency
    #[serde(default)]
    pub vol_ccy_24h: String,
    /// 24h trading volume in contracts (for derivatives)
    #[serde(default)]
    pub vol_24h: String,
    /// Open interest (for derivatives)
    #[serde(default)]
    pub sod_utc_0: String,
    /// Open interest (for derivatives)
    #[serde(default)]
    pub sod_utc_8: String,
    /// Timestamp of the data (Unix timestamp in milliseconds)
    pub ts: String,
}

/// Order book entry (price level).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BookLevel {
    /// Price
    pub price: String,
    /// Quantity
    pub size: String,
    /// Deprecated: Number of liquidated orders
    pub liquidated_orders: String,
    /// Number of orders at this price level
    pub order_count: String,
}

/// Order book data.
///
/// Source: GET /api/v5/market/books response
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderBook {
    /// Ask side (sell orders), sorted by price ascending
    pub asks: Vec<Vec<String>>,
    /// Bid side (buy orders), sorted by price descending
    pub bids: Vec<Vec<String>>,
    /// Timestamp (Unix timestamp in milliseconds)
    pub ts: String,
}

impl OrderBook {
    /// Parse asks into structured book levels.
    #[must_use]
    pub fn parsed_asks(&self) -> Vec<BookLevel> {
        self.asks
            .iter()
            .filter_map(|level| {
                if level.len() >= 4 {
                    Some(BookLevel {
                        price: level[0].clone(),
                        size: level[1].clone(),
                        liquidated_orders: level[2].clone(),
                        order_count: level[3].clone(),
                    })
                } else {
                    None
                }
            })
            .collect()
    }

    /// Parse bids into structured book levels.
    #[must_use]
    pub fn parsed_bids(&self) -> Vec<BookLevel> {
        self.bids
            .iter()
            .filter_map(|level| {
                if level.len() >= 4 {
                    Some(BookLevel {
                        price: level[0].clone(),
                        size: level[1].clone(),
                        liquidated_orders: level[2].clone(),
                        order_count: level[3].clone(),
                    })
                } else {
                    None
                }
            })
            .collect()
    }
}

/// Candlestick/K-line data.
///
/// Source: GET /api/v5/market/candles response
/// Response format: [ts, o, h, l, c, vol, volCcy, volCcyQuote, confirm]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Candle {
    /// Timestamp (Unix timestamp in milliseconds)
    pub ts: String,
    /// Open price
    pub open: String,
    /// Highest price
    pub high: String,
    /// Lowest price
    pub low: String,
    /// Close price
    pub close: String,
    /// Trading volume in contracts (for derivatives) or base currency (for spot)
    pub vol: String,
    /// Trading volume in currency
    pub vol_ccy: String,
    /// Trading volume in quote currency
    pub vol_ccy_quote: String,
    /// Candlestick state: 0 = incomplete, 1 = complete
    pub confirm: String,
}

impl Candle {
    /// Parse a candle from the raw API response array.
    ///
    /// API returns: [ts, o, h, l, c, vol, volCcy, volCcyQuote, confirm]
    #[must_use]
    pub fn from_array(arr: &[String]) -> Option<Self> {
        if arr.len() >= 9 {
            Some(Self {
                ts: arr[0].clone(),
                open: arr[1].clone(),
                high: arr[2].clone(),
                low: arr[3].clone(),
                close: arr[4].clone(),
                vol: arr[5].clone(),
                vol_ccy: arr[6].clone(),
                vol_ccy_quote: arr[7].clone(),
                confirm: arr[8].clone(),
            })
        } else {
            None
        }
    }

    /// Check if the candle is complete.
    #[must_use]
    pub fn is_confirmed(&self) -> bool {
        self.confirm == "1"
    }
}

/// Public trade data.
///
/// Source: GET /api/v5/market/trades response
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Trade {
    /// Instrument ID
    pub inst_id: String,
    /// Trade ID
    pub trade_id: String,
    /// Trade price
    pub px: String,
    /// Trade quantity
    pub sz: String,
    /// Trade side: buy, sell
    pub side: String,
    /// Trade time (Unix timestamp in milliseconds)
    pub ts: String,
    /// Number of fills bundled in this trade
    #[serde(default)]
    pub count: String,
}

/// Instrument information.
///
/// Source: GET /api/v5/public/instruments response
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Instrument {
    /// Instrument type
    pub inst_type: String,
    /// Instrument ID
    pub inst_id: String,
    /// Underlying (for derivatives)
    #[serde(default)]
    pub uly: String,
    /// Instrument family (for options)
    #[serde(default)]
    pub inst_family: String,
    /// Category
    #[serde(default)]
    pub category: String,
    /// Base currency
    #[serde(default)]
    pub base_ccy: String,
    /// Quote currency
    #[serde(default)]
    pub quote_ccy: String,
    /// Settlement currency
    #[serde(default)]
    pub settle_ccy: String,
    /// Contract value (for derivatives)
    #[serde(default)]
    pub ct_val: String,
    /// Contract multiplier (for derivatives)
    #[serde(default)]
    pub ct_mult: String,
    /// Contract value currency (for derivatives)
    #[serde(default)]
    pub ct_val_ccy: String,
    /// Option type: C (call), P (put)
    #[serde(default)]
    pub opt_type: String,
    /// Strike price (for options)
    #[serde(default)]
    pub stk: String,
    /// Listing time (Unix timestamp in milliseconds)
    #[serde(default)]
    pub list_time: String,
    /// Expiry time (Unix timestamp in milliseconds)
    #[serde(default)]
    pub exp_time: String,
    /// Leverage (for derivatives)
    #[serde(default)]
    pub lever: String,
    /// Tick size (minimum price increment)
    pub tick_sz: String,
    /// Lot size (minimum order size)
    pub lot_sz: String,
    /// Minimum order size
    pub min_sz: String,
    /// Contract type: linear, inverse
    #[serde(default)]
    pub ct_type: String,
    /// Alias (for futures): this_week, next_week, quarter, next_quarter
    #[serde(default)]
    pub alias: String,
    /// Instrument status: live, suspend, preopen, settlement
    pub state: String,
    /// Maximum leverage
    #[serde(default)]
    pub max_lmt_sz: String,
    /// Maximum market order size
    #[serde(default)]
    pub max_mkt_sz: String,
    /// Maximum TWAP size
    #[serde(default)]
    pub max_twap_sz: String,
    /// Maximum iceberg size
    #[serde(default)]
    pub max_iceberg_sz: String,
    /// Maximum trigger size
    #[serde(default)]
    pub max_trigger_sz: String,
    /// Maximum stop size
    #[serde(default)]
    pub max_stop_sz: String,
}

/// Mark price data.
///
/// Source: GET /api/v5/public/mark-price response
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarkPrice {
    /// Instrument type
    pub inst_type: String,
    /// Instrument ID
    pub inst_id: String,
    /// Mark price
    pub mark_px: String,
    /// Timestamp (Unix timestamp in milliseconds)
    pub ts: String,
}

/// Funding rate data.
///
/// Source: GET /api/v5/public/funding-rate response
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FundingRate {
    /// Instrument type
    pub inst_type: String,
    /// Instrument ID
    pub inst_id: String,
    /// Current funding rate
    pub funding_rate: String,
    /// Next funding rate
    #[serde(default)]
    pub next_funding_rate: String,
    /// Funding time (Unix timestamp in milliseconds)
    pub funding_time: String,
    /// Next funding time (Unix timestamp in milliseconds)
    #[serde(default)]
    pub next_funding_time: String,
    /// Minimum funding rate
    #[serde(default)]
    pub min_funding_rate: String,
    /// Maximum funding rate
    #[serde(default)]
    pub max_funding_rate: String,
    /// Settlement funding rate
    #[serde(default)]
    pub settle_funding_rate: String,
    /// Premium between index and mark price
    #[serde(default)]
    pub premium: String,
    /// Settlement state
    #[serde(default)]
    pub settle_state: String,
    /// Method
    #[serde(default)]
    pub method: String,
}

/// Index ticker data.
///
/// Source: GET /api/v5/market/index-tickers response
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndexTicker {
    /// Index
    pub inst_id: String,
    /// Index price
    pub idx_px: String,
    /// Highest price in the past 24 hours
    #[serde(default)]
    pub high_24h: String,
    /// Lowest price in the past 24 hours
    #[serde(default)]
    pub low_24h: String,
    /// Open price in the past 24 hours (UTC 0)
    #[serde(default)]
    pub sod_utc_0: String,
    /// Open price in the past 24 hours (UTC 8)
    #[serde(default)]
    pub sod_utc_8: String,
    /// Timestamp (Unix timestamp in milliseconds)
    pub ts: String,
}
