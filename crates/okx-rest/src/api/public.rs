//! Public Data API endpoints.
//!
//! Source: OKX API v5 Public Data REST API
//! - <https://www.okx.com/docs-v5/en/#public-data-rest-api>

use serde::{Deserialize, Serialize};
use serde_json::Value;

use okx_core::{
    types::{FundingRate, Instrument, MarkPrice},
    OkxError, Result,
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
    /// Get option tick bands
    pub const INSTRUMENT_TICK_BANDS: &str = "/api/v5/public/instrument-tick-bands";
    /// Get option trades
    pub const OPTION_TRADES: &str = "/api/v5/public/option-trades";
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
    /// 产品类型，`FUTURES`（交割合约）或 `OPTION`（期权）
    pub inst_type: String,
    /// 标的指数，如 BTC-USD
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uly: Option<String>,
    /// 交易品种，如 BTC-USD（适用于期权）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inst_family: Option<String>,
    /// 分页参数，查询此时间戳之后的记录
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// 分页参数，查询此时间戳之前的记录
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    /// 返回记录数量限制
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,
}

/// Query parameters for open interest.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetOpenInterestParams {
    /// 产品类型，`SWAP`（永续合约）或 `FUTURES`（交割合约）或 `OPTION`（期权）
    pub inst_type: String,
    /// 标的指数，如 BTC-USD
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uly: Option<String>,
    /// 产品 ID，如 BTC-USDT-SWAP
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inst_id: Option<String>,
    /// 交易品种，如 BTC-USD（适用于期权）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inst_family: Option<String>,
}

/// Query parameters for position tiers.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPositionTiersParams {
    /// 产品类型，`MARGIN`、`SWAP`、`FUTURES`、`OPTION`
    pub inst_type: String,
    /// 交易模式，`cross`（全仓）或 `isolated`（逐仓）
    pub td_mode: String,
    /// 标的指数，如 BTC-USD
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uly: Option<String>,
    /// 产品 ID，如 BTC-USDT
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inst_id: Option<String>,
    /// 币种，如 BTC
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ccy: Option<String>,
    /// 档位，如 `1`、`2`、`3`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
    /// 交易品种，如 BTC-USD（适用于期权）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inst_family: Option<String>,
}

/// Query parameters for price limit.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPriceLimitParams {
    /// 产品 ID，如 BTC-USDT-SWAP
    #[serde(rename = "instId")]
    pub inst_id: String,
}

/// Query parameters for option summary.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetOptSummaryParams {
    /// 标的指数，如 BTC-USD
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uly: Option<String>,
    /// 到期时间，格式：YYMMDD，如 `250628`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exp_time: Option<String>,
    /// 交易品种，如 BTC-USD
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inst_family: Option<String>,
}

/// Query parameters for estimated price.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetEstimatedPriceParams {
    /// 产品 ID，如 BTC-USDT-SWAP
    #[serde(rename = "instId")]
    pub inst_id: String,
}

/// Query parameters for discount rate and interest-free quota.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetDiscountQuotaParams {
    /// 币种，如 BTC
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ccy: Option<String>,
}

/// Query parameters for underlying.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetUnderlyingParams {
    /// 产品类型，`SWAP`、`FUTURES`、`OPTION`
    #[serde(rename = "instType", skip_serializing_if = "Option::is_none")]
    pub inst_type: Option<String>,
}

/// Query parameters for insurance fund.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetInsuranceFundParams {
    /// 产品类型，`MARGIN`、`SWAP`、`FUTURES`、`OPTION`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inst_type: Option<String>,
    /// 风险准备金类型，`all`（全部）、`liquidation_balance_deposit`（强平注入）、`bankruptcy_loss`（穿仓亏损）、`platform_revenue`（平台收入）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// 标的指数，如 BTC-USD
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uly: Option<String>,
    /// 币种，如 BTC
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ccy: Option<String>,
    /// 分页参数，查询此时间戳之前的记录
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    /// 分页参数，查询此时间戳之后的记录
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// 返回记录数量限制
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,
    /// 交易品种，如 BTC-USD（适用于期权）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inst_family: Option<String>,
}

/// Query parameters for convert contract coin.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetConvertContractCoinParams {
    /// 转换类型，`1`（币转张）或 `2`（张转币）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// 产品 ID，如 BTC-USD-SWAP
    #[serde(rename = "instId", skip_serializing_if = "Option::is_none")]
    pub inst_id: Option<String>,
    /// 数量
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sz: Option<String>,
    /// 委托价格
    #[serde(skip_serializing_if = "Option::is_none")]
    pub px: Option<String>,
    /// 币种单位，`coin`（币）或 `usds`（U本位合约面值）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
}

/// System time response.
#[derive(Debug, Clone, Deserialize)]
pub struct SystemTime {
    /// System time (Unix timestamp in milliseconds)
    pub ts: String,
}

/// Query parameters for option tick bands.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetInstrumentTickBandsParams {
    /// 产品类型，当前仅支持 `OPTION`
    pub inst_type: String,
    /// 交易品种，如 BTC-USD
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inst_family: Option<String>,
}

/// Query parameters for option trades (public).
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetOptionTradesParams {
    /// 产品 ID，如 BTC-USD-221230-40000-C
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inst_id: Option<String>,
    /// 交易品种，如 BTC-USD
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inst_family: Option<String>,
    /// 期权类型，`C`（看涨期权）或 `P`（看跌期权）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opt_type: Option<String>,
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

    /// Get option tick bands.
    fn get_instrument_tick_bands(
        &self,
        params: GetInstrumentTickBandsParams,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;

    /// Get option trades (public).
    ///
    /// Either `inst_id` or `inst_family` must be provided. If both are provided, `inst_id` is used.
    fn get_option_trades(
        &self,
        params: GetOptionTradesParams,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;

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

    async fn get_instrument_tick_bands(
        &self,
        params: GetInstrumentTickBandsParams,
    ) -> Result<Vec<Value>> {
        self.get_public(endpoints::INSTRUMENT_TICK_BANDS, Some(&params))
            .await
    }

    async fn get_option_trades(&self, mut params: GetOptionTradesParams) -> Result<Vec<Value>> {
        if params.inst_id.is_none() && params.inst_family.is_none() {
            return Err(OkxError::Other(
                "Either inst_id or inst_family must be provided".to_string(),
            ));
        }

        if params.inst_id.is_some() {
            params.inst_family = None;
        }

        self.get_public(endpoints::OPTION_TRADES, Some(&params)).await
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
