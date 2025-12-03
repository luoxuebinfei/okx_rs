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
    /// Get VIP interest rate and loan quota
    pub const VIP_INTEREST_RATE_LOAN_QUOTA: &str = "/api/v5/public/vip-interest-rate-loan-quota";
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

/// Query parameters for price limit.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPriceLimitParams {
    #[serde(rename = "instId")]
    pub inst_id: String,
}

/// Query parameters for option summary.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetOptSummaryParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uly: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exp_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inst_family: Option<String>,
}

/// Query parameters for estimated price.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetEstimatedPriceParams {
    #[serde(rename = "instId")]
    pub inst_id: String,
}

/// Query parameters for discount rate and interest-free quota.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetDiscountQuotaParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ccy: Option<String>,
}

/// Query parameters for underlying.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetUnderlyingParams {
    #[serde(rename = "instType", skip_serializing_if = "Option::is_none")]
    pub inst_type: Option<String>,
}

/// Query parameters for insurance fund.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetInsuranceFundParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inst_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uly: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ccy: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inst_family: Option<String>,
}

/// Query parameters for convert contract coin.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetConvertContractCoinParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "instId", skip_serializing_if = "Option::is_none")]
    pub inst_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sz: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub px: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
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

    /// Get price limit.
    fn get_price_limit(
        &self,
        params: GetPriceLimitParams,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;

    /// Get option summary.
    fn get_opt_summary(
        &self,
        params: GetOptSummaryParams,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;

    /// Get estimated delivery/exercise price.
    fn get_estimated_price(
        &self,
        params: GetEstimatedPriceParams,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;

    /// Get discount rate and interest-free quota.
    fn get_discount_interest_free_quota(
        &self,
        params: GetDiscountQuotaParams,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;

    /// Get interest rate and loan quota.
    fn get_interest_rate_loan_quota(
        &self,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;

    /// Get VIP interest rate and loan quota.
    fn get_vip_interest_rate_loan_quota(
        &self,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;

    /// Get underlying list.
    fn get_underlying(
        &self,
        params: GetUnderlyingParams,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;

    /// Get insurance fund.
    fn get_insurance_fund(
        &self,
        params: GetInsuranceFundParams,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;

    /// Convert contract coin units.
    fn get_convert_contract_coin(
        &self,
        params: GetConvertContractCoinParams,
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

    async fn get_price_limit(&self, params: GetPriceLimitParams) -> Result<Vec<Value>> {
        self.get_public(endpoints::PRICE_LIMIT, Some(&params)).await
    }

    async fn get_opt_summary(&self, params: GetOptSummaryParams) -> Result<Vec<Value>> {
        self.get_public(endpoints::OPT_SUMMARY, Some(&params)).await
    }

    async fn get_estimated_price(&self, params: GetEstimatedPriceParams) -> Result<Vec<Value>> {
        self.get_public(endpoints::ESTIMATED_PRICE, Some(&params))
            .await
    }

    async fn get_discount_interest_free_quota(
        &self,
        params: GetDiscountQuotaParams,
    ) -> Result<Vec<Value>> {
        self.get_public(endpoints::DISCOUNT_RATE_INTEREST_FREE_QUOTA, Some(&params))
            .await
    }

    async fn get_interest_rate_loan_quota(&self) -> Result<Vec<Value>> {
        self.get_public::<Value, ()>(endpoints::INTEREST_RATE_LOAN_QUOTA, None)
            .await
    }

    async fn get_vip_interest_rate_loan_quota(&self) -> Result<Vec<Value>> {
        self.get_public::<Value, ()>(endpoints::VIP_INTEREST_RATE_LOAN_QUOTA, None)
            .await
    }

    async fn get_underlying(&self, params: GetUnderlyingParams) -> Result<Vec<Value>> {
        self.get_public(endpoints::UNDERLYING, Some(&params)).await
    }

    async fn get_insurance_fund(&self, params: GetInsuranceFundParams) -> Result<Vec<Value>> {
        self.get_public(endpoints::INSURANCE_FUND, Some(&params))
            .await
    }

    async fn get_convert_contract_coin(
        &self,
        params: GetConvertContractCoinParams,
    ) -> Result<Vec<Value>> {
        self.get_public(endpoints::CONVERT_CONTRACT_COIN, Some(&params))
            .await
    }
}
