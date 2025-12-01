//! Public Data API endpoints.
//!
//! Source: OKX API v5 Public Data REST API
//! - <https://www.okx.com/docs-v5/en/#public-data-rest-api>

use serde::{Deserialize, Serialize};
use serde_json::Value;

use okx_core::{
    types::{FundingRate, Instrument, MarkPrice},
    Result,
};

use crate::OkxRestClient;

/// API endpoints for public data.
pub mod endpoints {
    /// Get instruments
    pub const INSTRUMENTS: &str = "/api/v5/public/instruments";
    /// Get delivery/exercise history
    pub const DELIVERY_EXERCISE_HISTORY: &str = "/api/v5/public/delivery-exercise-history";
    /// Get open interest
    pub const OPEN_INTEREST: &str = "/api/v5/public/open-interest";
    /// Get funding rate
    pub const FUNDING_RATE: &str = "/api/v5/public/funding-rate";
    /// Get funding rate history
    pub const FUNDING_RATE_HISTORY: &str = "/api/v5/public/funding-rate-history";
    /// Get price limit
    pub const PRICE_LIMIT: &str = "/api/v5/public/price-limit";
    /// Get option market data
    pub const OPT_SUMMARY: &str = "/api/v5/public/opt-summary";
    /// Get estimated delivery/exercise price
    pub const ESTIMATED_PRICE: &str = "/api/v5/public/estimated-price";
    /// Get discount rate and interest-free quota
    pub const DISCOUNT_RATE_INTEREST_FREE_QUOTA: &str =
        "/api/v5/public/discount-rate-interest-free-quota";
    /// Get system time
    pub const TIME: &str = "/api/v5/public/time";
    /// Get mark price
    pub const MARK_PRICE: &str = "/api/v5/public/mark-price";
    /// Get position tiers
    pub const POSITION_TIERS: &str = "/api/v5/public/position-tiers";
    /// Get interest rate and loan quota
    pub const INTEREST_RATE_LOAN_QUOTA: &str = "/api/v5/public/interest-rate-loan-quota";
    /// Get underlying
    pub const UNDERLYING: &str = "/api/v5/public/underlying";
    /// Get insurance fund
    pub const INSURANCE_FUND: &str = "/api/v5/public/insurance-fund";
    /// Unit convert
    pub const CONVERT_CONTRACT_COIN: &str = "/api/v5/public/convert-contract-coin";
}

/// Query parameters for get_instruments.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetInstrumentsParams {
    /// Instrument type: SPOT, MARGIN, SWAP, FUTURES, OPTION
    pub inst_type: String,
    /// Underlying (for FUTURES/SWAP/OPTION)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uly: Option<String>,
    /// Instrument family (for FUTURES/SWAP/OPTION)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inst_family: Option<String>,
    /// Instrument ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inst_id: Option<String>,
}

/// Query parameters for get_funding_rate.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetFundingRateParams {
    /// Instrument ID, e.g., "BTC-USDT-SWAP"
    pub inst_id: String,
}

/// Query parameters for get_funding_rate_history.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetFundingRateHistoryParams {
    /// Instrument ID
    pub inst_id: String,
    /// Pagination: funding time to start from
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// Pagination: funding time to end at
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    /// Number of results (max 100, default 100)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,
}

/// Query parameters for get_mark_price.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetMarkPriceParams {
    /// Instrument type: MARGIN, SWAP, FUTURES, OPTION
    pub inst_type: String,
    /// Underlying
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uly: Option<String>,
    /// Instrument family
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inst_family: Option<String>,
    /// Instrument ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inst_id: Option<String>,
}

/// Query parameters for delivery/exercise history.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetDeliveryExerciseHistoryParams {
    pub inst_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uly: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inst_family: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,
}

/// Query parameters for open interest.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetOpenInterestParams {
    pub inst_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uly: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inst_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inst_family: Option<String>,
}

/// Query parameters for position tiers.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPositionTiersParams {
    pub inst_type: String,
    pub td_mode: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uly: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inst_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ccy: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inst_family: Option<String>,
}

/// System time response.
#[derive(Debug, Clone, Deserialize)]
pub struct SystemTime {
    /// System time (Unix timestamp in milliseconds)
    pub ts: String,
}

/// Public Data API trait for OKX REST client.
///
/// Provides methods for retrieving public data.
/// All endpoints are public and do not require authentication.
pub trait PublicApi {
    /// Get instruments.
    ///
    /// Retrieves list of instruments with open contracts.
    ///
    /// ## API Details
    ///
    /// - Endpoint: GET /api/v5/public/instruments
    /// - Rate limit: 20 requests per 2 seconds
    /// - Permission: Public (no auth required)
    fn get_instruments(
        &self,
        params: GetInstrumentsParams,
    ) -> impl std::future::Future<Output = Result<Vec<Instrument>>> + Send;

    /// Get funding rate.
    ///
    /// Retrieves funding rate for a perpetual swap.
    ///
    /// ## API Details
    ///
    /// - Endpoint: GET /api/v5/public/funding-rate
    /// - Rate limit: 20 requests per 2 seconds
    /// - Permission: Public (no auth required)
    fn get_funding_rate(
        &self,
        inst_id: &str,
    ) -> impl std::future::Future<Output = Result<Vec<FundingRate>>> + Send;

    /// Get funding rate history.
    ///
    /// ## API Details
    ///
    /// - Endpoint: GET /api/v5/public/funding-rate-history
    /// - Rate limit: 10 requests per 2 seconds
    /// - Permission: Public (no auth required)
    fn get_funding_rate_history(
        &self,
        params: GetFundingRateHistoryParams,
    ) -> impl std::future::Future<Output = Result<Vec<FundingRate>>> + Send;

    /// Get mark price.
    ///
    /// ## API Details
    ///
    /// - Endpoint: GET /api/v5/public/mark-price
    /// - Rate limit: 10 requests per 2 seconds
    /// - Permission: Public (no auth required)
    fn get_mark_price(
        &self,
        params: GetMarkPriceParams,
    ) -> impl std::future::Future<Output = Result<Vec<MarkPrice>>> + Send;

    /// Get system time.
    ///
    /// ## API Details
    ///
    /// - Endpoint: GET /api/v5/public/time
    /// - Rate limit: 10 requests per 2 seconds
    /// - Permission: Public (no auth required)
    fn get_system_time(&self) -> impl std::future::Future<Output = Result<Vec<SystemTime>>> + Send;

    /// Get delivery/exercise history.
    fn get_delivery_exercise_history(
        &self,
        params: GetDeliveryExerciseHistoryParams,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;

    /// Get open interest.
    fn get_open_interest(
        &self,
        params: GetOpenInterestParams,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;

    /// Get position tiers.
    fn get_position_tiers(
        &self,
        params: GetPositionTiersParams,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;
}

impl PublicApi for OkxRestClient {
    async fn get_instruments(&self, params: GetInstrumentsParams) -> Result<Vec<Instrument>> {
        self.get_public(endpoints::INSTRUMENTS, Some(&params)).await
    }

    async fn get_funding_rate(&self, inst_id: &str) -> Result<Vec<FundingRate>> {
        let params = GetFundingRateParams {
            inst_id: inst_id.to_string(),
        };
        self.get_public(endpoints::FUNDING_RATE, Some(&params))
            .await
    }

    async fn get_funding_rate_history(
        &self,
        params: GetFundingRateHistoryParams,
    ) -> Result<Vec<FundingRate>> {
        self.get_public(endpoints::FUNDING_RATE_HISTORY, Some(&params))
            .await
    }

    async fn get_mark_price(&self, params: GetMarkPriceParams) -> Result<Vec<MarkPrice>> {
        self.get_public(endpoints::MARK_PRICE, Some(&params)).await
    }

    async fn get_system_time(&self) -> Result<Vec<SystemTime>> {
        self.get_public::<SystemTime, ()>(endpoints::TIME, None)
            .await
    }

    async fn get_delivery_exercise_history(
        &self,
        params: GetDeliveryExerciseHistoryParams,
    ) -> Result<Vec<Value>> {
        self.get_public(endpoints::DELIVERY_EXERCISE_HISTORY, Some(&params))
            .await
    }

    async fn get_open_interest(&self, params: GetOpenInterestParams) -> Result<Vec<Value>> {
        self.get_public(endpoints::OPEN_INTEREST, Some(&params))
            .await
    }

    async fn get_position_tiers(&self, params: GetPositionTiersParams) -> Result<Vec<Value>> {
        self.get_public(endpoints::POSITION_TIERS, Some(&params))
            .await
    }
}
