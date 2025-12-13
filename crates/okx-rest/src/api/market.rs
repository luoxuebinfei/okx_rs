//! Market Data API endpoints.
//!
//! Source: OKX API v5 Market Data REST API
//! - <https://www.okx.com/docs-v5/en/#order-book-trading-market-data>

use serde::Serialize;
use serde_json::Value;

use okx_core::{
    types::{IndexTicker, OrderBook, Ticker, Trade},
    Result,
};

use crate::OkxRestClient;

/// API endpoints for market data.
pub mod endpoints {
    /// Get tickers
    pub const TICKERS: &str = "/api/v5/market/tickers";
    /// Get single ticker
    pub const TICKER: &str = "/api/v5/market/ticker";
    /// Get order book
    pub const BOOKS: &str = "/api/v5/market/books";
    /// Get candlesticks
    pub const CANDLES: &str = "/api/v5/market/candles";
    /// Get candlesticks history
    pub const CANDLES_HISTORY: &str = "/api/v5/market/history-candles";
    /// Get index candlesticks
    pub const INDEX_CANDLES: &str = "/api/v5/market/index-candles";
    /// Get mark price candlesticks
    pub const MARK_PRICE_CANDLES: &str = "/api/v5/market/mark-price-candles";
    /// Get trades
    pub const TRADES: &str = "/api/v5/market/trades";
    /// Get trades history
    pub const TRADES_HISTORY: &str = "/api/v5/market/history-trades";
    /// Get platform 24H total volume
    pub const PLATFORM_24_VOLUME: &str = "/api/v5/market/platform-24-volume";
    /// Get index components
    pub const INDEX_COMPONENTS: &str = "/api/v5/market/index-components";
    /// Get exchange rate
    pub const EXCHANGE_RATE: &str = "/api/v5/market/exchange-rate";
    /// Get index tickers
    pub const INDEX_TICKERS: &str = "/api/v5/market/index-tickers";
    /// Get lite order book
    pub const BOOKS_LITE: &str = "/api/v5/market/books-lite";
    /// Get block ticker
    pub const BLOCK_TICKER: &str = "/api/v5/market/block-ticker";
    /// Get block tickers
    pub const BLOCK_TICKERS: &str = "/api/v5/market/block-tickers";
    /// Get block trades
    pub const BLOCK_TRADES: &str = "/api/v5/market/block-trades";
    /// Get option family trades
    pub const OPTION_FAMILY_TRADES: &str = "/api/v5/market/option/instrument-family-trades";
}

/// Query parameters for get_tickers.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetTickersParams {
    /// Instrument type: SPOT, SWAP, FUTURES, OPTION
    pub inst_type: String,
    /// Underlying (for FUTURES/SWAP/OPTION)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uly: Option<String>,
    /// Instrument family (for FUTURES/SWAP/OPTION)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inst_family: Option<String>,
}

/// Query parameters for get_ticker.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetTickerParams {
    /// Instrument ID, e.g., "BTC-USDT"
    pub inst_id: String,
}

/// Query parameters for get_orderbook.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetOrderBookParams {
    /// Instrument ID
    pub inst_id: String,
    /// Order book depth (max 400, default 1)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sz: Option<String>,
}

/// Query parameters for get_candles.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetCandlesParams {
    /// Instrument ID
    pub inst_id: String,
    /// Bar size: 1m, 3m, 5m, 15m, 30m, 1H, 2H, 4H, 6H, 12H, 1D, 1W, 1M, 3M
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bar: Option<String>,
    /// Pagination: timestamp to start from
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// Pagination: timestamp to end at
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    /// Number of results (max 300, default 100)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,
}

/// Query parameters for get_trades.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetTradesParams {
    /// Instrument ID
    pub inst_id: String,
    /// Number of results (max 500, default 100)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,
}

/// Query parameters for history trades.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetHistoryTradesParams {
    /// Instrument ID
    pub inst_id: String,
    /// Pagination: query ID after this
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// Pagination: query ID before this
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    /// Number of results (max 100)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,
    /// Trade type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

/// Query parameters for get_index_tickers.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetIndexTickersParams {
    /// Quote currency (e.g., "USD", "USDT")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_ccy: Option<String>,
    /// Index (e.g., "BTC-USD")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inst_id: Option<String>,
}

/// Query parameters for block tickers.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetBlockTickersParams {
    pub inst_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uly: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inst_family: Option<String>,
}

/// Alias for index candles params.
pub type GetIndexCandlesParams = GetCandlesParams;
/// Alias for mark price candles params.
pub type GetMarkPriceCandlesParams = GetCandlesParams;

/// Query parameters for option family trades.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetOptionFamilyTradesParams {
    pub inst_family: String,
}

/// Query parameters for index components.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetIndexComponentsParams {
    pub index: String,
}

/// Market Data API trait for OKX REST client.
///
/// Provides methods for retrieving market data.
/// All endpoints are public and do not require authentication.
pub trait MarketApi {
    /// Get tickers for all instruments of a type.
    ///
    /// ## API Details
    ///
    /// - Endpoint: GET /api/v5/market/tickers
    /// - Rate limit: 20 requests per 2 seconds
    /// - Permission: Public (no auth required)
    fn get_tickers(
        &self,
        params: GetTickersParams,
    ) -> impl std::future::Future<Output = Result<Vec<Ticker>>> + Send;

    /// Get ticker for a single instrument.
    ///
    /// ## API Details
    ///
    /// - Endpoint: GET /api/v5/market/ticker
    /// - Rate limit: 20 requests per 2 seconds
    /// - Permission: Public (no auth required)
    fn get_ticker(
        &self,
        inst_id: &str,
    ) -> impl std::future::Future<Output = Result<Vec<Ticker>>> + Send;

    /// Get order book.
    ///
    /// ## API Details
    ///
    /// - Endpoint: GET /api/v5/market/books
    /// - Rate limit: 40 requests per 2 seconds
    /// - Permission: Public (no auth required)
    ///
    /// ## Arguments
    ///
    /// * `inst_id` - Instrument ID
    /// * `depth` - Order book depth (max 400, default 1)
    fn get_orderbook(
        &self,
        inst_id: &str,
        depth: Option<u32>,
    ) -> impl std::future::Future<Output = Result<Vec<OrderBook>>> + Send;

    /// Get candlesticks (K-line data).
    ///
    /// ## API Details
    ///
    /// - Endpoint: GET /api/v5/market/candles
    /// - Rate limit: 40 requests per 2 seconds
    /// - Permission: Public (no auth required)
    ///
    /// ## Returns
    ///
    /// Returns raw candle data as arrays. Use `Candle::from_array` to parse.
    fn get_candles(
        &self,
        params: GetCandlesParams,
    ) -> impl std::future::Future<Output = Result<Vec<Vec<String>>>> + Send;

    /// Get recent trades.
    ///
    /// ## API Details
    ///
    /// - Endpoint: GET /api/v5/market/trades
    /// - Rate limit: 100 requests per 2 seconds
    /// - Permission: Public (no auth required)
    fn get_trades(
        &self,
        inst_id: &str,
        limit: Option<u32>,
    ) -> impl std::future::Future<Output = Result<Vec<Trade>>> + Send;

    /// Get platform 24H total volume.
    fn get_platform_24_volume(&self)
        -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;

    /// Get index components.
    fn get_index_components(
        &self,
        params: GetIndexComponentsParams,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;

    /// Get exchange rate.
    fn get_exchange_rate(&self) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;

    /// Get index tickers.
    ///
    /// ## API Details
    ///
    /// - Endpoint: GET /api/v5/market/index-tickers
    /// - Rate limit: 20 requests per 2 seconds
    /// - Permission: Public (no auth required)
    fn get_index_tickers(
        &self,
        params: GetIndexTickersParams,
    ) -> impl std::future::Future<Output = Result<Vec<IndexTicker>>> + Send;

    /// Get historical candles.
    fn get_history_candles(
        &self,
        params: GetCandlesParams,
    ) -> impl std::future::Future<Output = Result<Vec<Vec<String>>>> + Send;

    /// Get index candles.
    fn get_index_candles(
        &self,
        params: GetIndexCandlesParams,
    ) -> impl std::future::Future<Output = Result<Vec<Vec<String>>>> + Send;

    /// Get mark price candles.
    fn get_mark_price_candles(
        &self,
        params: GetMarkPriceCandlesParams,
    ) -> impl std::future::Future<Output = Result<Vec<Vec<String>>>> + Send;

    /// Get history trades.
    fn get_history_trades(
        &self,
        params: GetHistoryTradesParams,
    ) -> impl std::future::Future<Output = Result<Vec<Trade>>> + Send;

    /// Get lite order book.
    fn get_orderbook_lite(
        &self,
        inst_id: &str,
    ) -> impl std::future::Future<Output = Result<Vec<OrderBook>>> + Send;

    /// Get block ticker for an instrument.
    fn get_block_ticker(
        &self,
        inst_id: &str,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;

    /// Get block tickers by type.
    fn get_block_tickers(
        &self,
        params: GetBlockTickersParams,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;

    /// Get block trades.
    fn get_block_trades(
        &self,
        inst_id: &str,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;

    /// Get option family trades.
    fn get_option_family_trades(
        &self,
        inst_family: &str,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;
}

impl MarketApi for OkxRestClient {
    async fn get_tickers(&self, params: GetTickersParams) -> Result<Vec<Ticker>> {
        self.get_public(endpoints::TICKERS, Some(&params)).await
    }

    async fn get_ticker(&self, inst_id: &str) -> Result<Vec<Ticker>> {
        let params = GetTickerParams {
            inst_id: inst_id.to_string(),
        };
        self.get_public(endpoints::TICKER, Some(&params)).await
    }

    async fn get_orderbook(&self, inst_id: &str, depth: Option<u32>) -> Result<Vec<OrderBook>> {
        let params = GetOrderBookParams {
            inst_id: inst_id.to_string(),
            sz: depth.map(|d| d.to_string()),
        };
        self.get_public(endpoints::BOOKS, Some(&params)).await
    }

    async fn get_candles(&self, params: GetCandlesParams) -> Result<Vec<Vec<String>>> {
        self.get_public(endpoints::CANDLES, Some(&params)).await
    }

    async fn get_trades(&self, inst_id: &str, limit: Option<u32>) -> Result<Vec<Trade>> {
        let params = GetTradesParams {
            inst_id: inst_id.to_string(),
            limit: limit.map(|l| l.to_string()),
        };
        self.get_public(endpoints::TRADES, Some(&params)).await
    }

    async fn get_platform_24_volume(&self) -> Result<Vec<Value>> {
        self.get_public(endpoints::PLATFORM_24_VOLUME, None::<&()>)
            .await
    }

    async fn get_index_components(&self, params: GetIndexComponentsParams) -> Result<Vec<Value>> {
        self.get_public(endpoints::INDEX_COMPONENTS, Some(&params))
            .await
    }

    async fn get_exchange_rate(&self) -> Result<Vec<Value>> {
        self.get_public(endpoints::EXCHANGE_RATE, None::<&()>).await
    }

    async fn get_index_tickers(&self, params: GetIndexTickersParams) -> Result<Vec<IndexTicker>> {
        self.get_public(endpoints::INDEX_TICKERS, Some(&params))
            .await
    }

    async fn get_history_candles(&self, params: GetCandlesParams) -> Result<Vec<Vec<String>>> {
        self.get_public(endpoints::CANDLES_HISTORY, Some(&params))
            .await
    }

    async fn get_index_candles(&self, params: GetIndexCandlesParams) -> Result<Vec<Vec<String>>> {
        self.get_public(endpoints::INDEX_CANDLES, Some(&params))
            .await
    }

    async fn get_mark_price_candles(
        &self,
        params: GetMarkPriceCandlesParams,
    ) -> Result<Vec<Vec<String>>> {
        self.get_public(endpoints::MARK_PRICE_CANDLES, Some(&params))
            .await
    }

    async fn get_history_trades(&self, params: GetHistoryTradesParams) -> Result<Vec<Trade>> {
        self.get_public(endpoints::TRADES_HISTORY, Some(&params))
            .await
    }

    async fn get_orderbook_lite(&self, inst_id: &str) -> Result<Vec<OrderBook>> {
        let params = GetOrderBookParams {
            inst_id: inst_id.to_string(),
            sz: None,
        };
        self.get_public(endpoints::BOOKS_LITE, Some(&params)).await
    }

    async fn get_block_ticker(&self, inst_id: &str) -> Result<Vec<Value>> {
        let params = GetTickerParams {
            inst_id: inst_id.to_string(),
        };
        self.get_public(endpoints::BLOCK_TICKER, Some(&params))
            .await
    }

    async fn get_block_tickers(&self, params: GetBlockTickersParams) -> Result<Vec<Value>> {
        self.get_public(endpoints::BLOCK_TICKERS, Some(&params))
            .await
    }

    async fn get_block_trades(&self, inst_id: &str) -> Result<Vec<Value>> {
        let params = GetTickerParams {
            inst_id: inst_id.to_string(),
        };
        self.get_public(endpoints::BLOCK_TRADES, Some(&params))
            .await
    }

    async fn get_option_family_trades(&self, inst_family: &str) -> Result<Vec<Value>> {
        let params = GetOptionFamilyTradesParams {
            inst_family: inst_family.to_string(),
        };
        self.get_public(endpoints::OPTION_FAMILY_TRADES, Some(&params))
            .await
    }
}
