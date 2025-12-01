//! Common types and enums used across OKX API.
//!
//! Source: OKX API v5 documentation

use serde::{Deserialize, Serialize};

/// Standard API response wrapper.
///
/// All OKX API responses follow this format:
/// - `code`: "0" for success, error code otherwise
/// - `msg`: Error message (empty on success)
/// - `data`: Array of response data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    /// Response code ("0" = success)
    pub code: String,
    /// Error message
    pub msg: String,
    /// Response data
    pub data: Vec<T>,
}

impl<T> ApiResponse<T> {
    /// Check if the response indicates success.
    #[must_use]
    pub fn is_success(&self) -> bool {
        self.code == "0"
    }
}

/// Instrument type.
///
/// Source: OKX API v5 - instType parameter
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum InstType {
    /// Spot trading
    Spot,
    /// Margin trading
    Margin,
    /// Perpetual swap
    Swap,
    /// Futures
    Futures,
    /// Options
    Option,
}

impl InstType {
    /// Convert to API string representation.
    #[must_use]
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Spot => "SPOT",
            Self::Margin => "MARGIN",
            Self::Swap => "SWAP",
            Self::Futures => "FUTURES",
            Self::Option => "OPTION",
        }
    }
}

/// Trading mode.
///
/// Source: OKX API v5 - tdMode parameter
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TdMode {
    /// Cash (spot)
    Cash,
    /// Cross margin
    Cross,
    /// Isolated margin
    Isolated,
}

impl TdMode {
    /// Convert to API string representation.
    #[must_use]
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Cash => "cash",
            Self::Cross => "cross",
            Self::Isolated => "isolated",
        }
    }
}

/// Order side.
///
/// Source: OKX API v5 - side parameter
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Side {
    /// Buy order
    Buy,
    /// Sell order
    Sell,
}

impl Side {
    /// Convert to API string representation.
    #[must_use]
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Buy => "buy",
            Self::Sell => "sell",
        }
    }
}

/// Position side for derivatives.
///
/// Source: OKX API v5 - posSide parameter
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PosSide {
    /// Long position
    Long,
    /// Short position
    Short,
    /// Net position (for one-way mode)
    Net,
}

impl PosSide {
    /// Convert to API string representation.
    #[must_use]
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Long => "long",
            Self::Short => "short",
            Self::Net => "net",
        }
    }
}

/// Order type.
///
/// Source: OKX API v5 - ordType parameter
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OrdType {
    /// Market order
    Market,
    /// Limit order
    Limit,
    /// Post-only order
    #[serde(rename = "post_only")]
    PostOnly,
    /// Fill or kill
    Fok,
    /// Immediate or cancel
    Ioc,
    /// Optimal limit order (market order with price protection)
    #[serde(rename = "optimal_limit_ioc")]
    OptimalLimitIoc,
    /// Market-maker protection (MMP)
    Mmp,
    /// Market-maker protection and target
    #[serde(rename = "mmp_and_target")]
    MmpAndTarget,
}

impl OrdType {
    /// Convert to API string representation.
    #[must_use]
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Market => "market",
            Self::Limit => "limit",
            Self::PostOnly => "post_only",
            Self::Fok => "fok",
            Self::Ioc => "ioc",
            Self::OptimalLimitIoc => "optimal_limit_ioc",
            Self::Mmp => "mmp",
            Self::MmpAndTarget => "mmp_and_target",
        }
    }
}

/// Order state.
///
/// Source: OKX API v5 - state parameter in order responses
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OrdState {
    /// Order created but not yet in order book
    Created,
    /// Order is live/open
    Live,
    /// Order partially filled
    #[serde(rename = "partially_filled")]
    PartiallyFilled,
    /// Order canceled
    Canceled,
    /// Order filled completely
    Filled,
    /// Market-maker protection canceled
    #[serde(rename = "mmp_canceled")]
    MmpCanceled,
}

/// Margin mode.
///
/// Source: OKX API v5 - mgnMode parameter
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MgnMode {
    /// Cross margin
    Cross,
    /// Isolated margin
    Isolated,
}

/// Contract type.
///
/// Source: OKX API v5 - ctType parameter
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CtType {
    /// Linear contract (settled in USDT/USDC)
    Linear,
    /// Inverse contract (settled in coin)
    Inverse,
}

/// Candlestick/K-line bar interval.
///
/// Source: OKX API v5 - bar parameter
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Bar {
    /// 1 minute
    #[serde(rename = "1m")]
    M1,
    /// 3 minutes
    #[serde(rename = "3m")]
    M3,
    /// 5 minutes
    #[serde(rename = "5m")]
    M5,
    /// 15 minutes
    #[serde(rename = "15m")]
    M15,
    /// 30 minutes
    #[serde(rename = "30m")]
    M30,
    /// 1 hour
    #[serde(rename = "1H")]
    H1,
    /// 2 hours
    #[serde(rename = "2H")]
    H2,
    /// 4 hours
    #[serde(rename = "4H")]
    H4,
    /// 6 hours
    #[serde(rename = "6H")]
    H6,
    /// 12 hours
    #[serde(rename = "12H")]
    H12,
    /// 1 day
    #[serde(rename = "1D")]
    D1,
    /// 1 week
    #[serde(rename = "1W")]
    W1,
    /// 1 month
    #[serde(rename = "1M")]
    Mo1,
    /// 3 months
    #[serde(rename = "3M")]
    Mo3,
}

impl Bar {
    /// Convert to API string representation.
    #[must_use]
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::M1 => "1m",
            Self::M3 => "3m",
            Self::M5 => "5m",
            Self::M15 => "15m",
            Self::M30 => "30m",
            Self::H1 => "1H",
            Self::H2 => "2H",
            Self::H4 => "4H",
            Self::H6 => "6H",
            Self::H12 => "12H",
            Self::D1 => "1D",
            Self::W1 => "1W",
            Self::Mo1 => "1M",
            Self::Mo3 => "3M",
        }
    }
}
