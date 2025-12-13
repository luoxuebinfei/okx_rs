//! Account API endpoints.
//!
//! Source: OKX API v5 Trading Account REST API
//! - <https://www.okx.com/docs-v5/en/#trading-account-rest-api>

use serde::Serialize;
use serde_json::Value;

use okx_core::{
    types::{AccountConfig, Balance, Position},
    Result,
};

use crate::OkxRestClient;

/// API endpoints for account operations.
pub mod endpoints {
    /// Get account balance
    pub const BALANCE: &str = "/api/v5/account/balance";
    /// Get positions
    pub const POSITIONS: &str = "/api/v5/account/positions";
    /// Get positions history
    pub const POSITIONS_HISTORY: &str = "/api/v5/account/positions-history";
    /// Get account configuration
    pub const CONFIG: &str = "/api/v5/account/config";
    /// Set leverage
    pub const SET_LEVERAGE: &str = "/api/v5/account/set-leverage";
    /// Get leverage info
    pub const LEVERAGE_INFO: &str = "/api/v5/account/leverage-info";
    /// Get maximum available tradable amount
    pub const MAX_SIZE: &str = "/api/v5/account/max-size";
    /// Get maximum available margin
    pub const MAX_AVAIL_SIZE: &str = "/api/v5/account/max-avail-size";
    /// Get maximum loan
    pub const MAX_LOAN: &str = "/api/v5/account/max-loan";
    /// Get maximum withdrawal
    pub const MAX_WITHDRAWAL: &str = "/api/v5/account/max-withdrawal";
    /// Get fee rates
    pub const TRADE_FEE: &str = "/api/v5/account/trade-fee";
    /// Get interest accrued
    pub const INTEREST_ACCRUED: &str = "/api/v5/account/interest-accrued";
    /// Get interest rate
    pub const INTEREST_RATE: &str = "/api/v5/account/interest-rate";
    /// Set position mode
    pub const SET_POSITION_MODE: &str = "/api/v5/account/set-position-mode";
    /// Get account position risk
    pub const POSITION_RISK: &str = "/api/v5/account/account-position-risk";
    /// Get available instruments info of current account
    pub const INSTRUMENTS: &str = "/api/v5/account/instruments";
    /// Get account risk state
    pub const RISK_STATE: &str = "/api/v5/account/risk-state";
    /// Get bills
    pub const BILLS: &str = "/api/v5/account/bills";
    /// Get bills archive
    pub const BILLS_ARCHIVE: &str = "/api/v5/account/bills-archive";
    /// Build simulated position
    pub const POSITION_BUILDER: &str = "/api/v5/account/position-builder";
    /// Set greeks display type
    pub const SET_GREEKS: &str = "/api/v5/account/set-greeks";
    /// Set isolated mode
    pub const SET_ISOLATED_MODE: &str = "/api/v5/account/set-isolated-mode";
    /// Set account level
    pub const SET_ACCOUNT_LEVEL: &str = "/api/v5/account/set-account-level";
    /// Borrow or repay
    pub const BORROW_REPAY: &str = "/api/v5/account/borrow-repay";
    /// Borrow or repay history
    pub const BORROW_REPAY_HISTORY: &str = "/api/v5/account/borrow-repay-history";
    /// Spot manual borrow or repay
    pub const SPOT_MANUAL_BORROW_REPAY: &str = "/api/v5/account/spot-manual-borrow-repay";
    /// Spot borrow repay history
    pub const SPOT_BORROW_REPAY_HISTORY: &str = "/api/v5/account/spot-borrow-repay-history";
    /// 调整持仓保证金
    pub const ADJUSTMENT_MARGIN: &str = "/api/v5/account/position/margin-balance";
    /// 设置风险对冲类型
    pub const SET_RISK_OFFSET_TYPE: &str = "/api/v5/account/set-riskOffset-type";
    /// 设置自动借币
    pub const SET_AUTO_LOAN: &str = "/api/v5/account/set-auto-loan";
    /// VIP interest accrued
    pub const VIP_INTEREST_ACCRUED: &str = "/api/v5/account/vip-interest-accrued";
    /// VIP interest deducted
    pub const VIP_INTEREST_DEDUCTED: &str = "/api/v5/account/vip-interest-deducted";
    /// Simulated margin
    pub const SIMULATED_MARGIN: &str = "/api/v5/account/simulated_margin";
    /// Account position tiers
    pub const ACCOUNT_POSITION_TIERS: &str = "/api/v5/account/position-tiers";
    /// Greeks
    pub const GREEKS: &str = "/api/v5/account/greeks";
    /// Activate option trading
    pub const ACTIVATE_OPTION: &str = "/api/v5/account/activate-option";
    /// Set auto repay
    pub const SET_AUTO_REPAY: &str = "/api/v5/account/set-auto-repay";
    /// Interest limits
    pub const INTEREST_LIMITS: &str = "/api/v5/account/interest-limits";
    /// VIP loan order list
    pub const VIP_LOAN_ORDER_LIST: &str = "/api/v5/account/vip-loan-order-list";
    /// VIP loan order detail
    pub const VIP_LOAN_ORDER_DETAIL: &str = "/api/v5/account/vip-loan-order-detail";
    /// Fixed loan borrowing limit
    pub const FIX_LOAN_BORROWING_LIMIT: &str = "/api/v5/account/fixed-loan/borrowing-limit";
    /// Fixed loan borrowing quote
    pub const FIX_LOAN_BORROWING_QUOTE: &str = "/api/v5/account/fixed-loan/borrowing-quote";
    /// Fixed loan borrowing order
    pub const FIX_LOAN_BORROWING_ORDER: &str = "/api/v5/account/fixed-loan/borrowing-order";
    /// Fixed loan amend borrowing order
    pub const FIX_LOAN_AMEND_BORROWING_ORDER: &str =
        "/api/v5/account/fixed-loan/amend-borrowing-order";
    /// Fixed loan manual reborrow
    pub const FIX_LOAN_MANUAL_REBORROW: &str = "/api/v5/account/fixed-loan/manual-reborrow";
    /// Fixed loan repay borrowing order
    pub const FIX_LOAN_REPAY_BORROWING_ORDER: &str =
        "/api/v5/account/fixed-loan/repay-borrowing-order";
    /// Fixed loan borrowing orders list
    pub const FIX_LOAN_BORROWING_ORDERS_LIST: &str =
        "/api/v5/account/fixed-loan/borrowing-orders-list";
}

/// Query parameters for get_account_instruments.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAccountInstrumentsParams {
    /// Instrument type, e.g. SPOT/MARGIN/SWAP/FUTURES/OPTION
    pub inst_type: String,
    /// Instrument family, required when inst_type is OPTION
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inst_family: Option<String>,
    /// Instrument ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inst_id: Option<String>,
}

/// Query parameters for get_balance.
#[derive(Debug, Default, Serialize)]
pub struct GetBalanceParams {
    /// Currency, e.g., "BTC" or "BTC,ETH" (up to 20)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ccy: Option<String>,
}

/// Query parameters for get_positions.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPositionsParams {
    /// Instrument type: MARGIN, SWAP, FUTURES, OPTION
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inst_type: Option<String>,
    /// Instrument ID, e.g., "BTC-USDT-SWAP"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inst_id: Option<String>,
    /// Position ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pos_id: Option<String>,
}

/// Request body for set_leverage.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetLeverageRequest {
    /// Instrument ID (required for isolated margin)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inst_id: Option<String>,
    /// Currency (required for cross margin)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ccy: Option<String>,
    /// Leverage
    pub lever: String,
    /// Margin mode: cross, isolated
    pub mgn_mode: String,
    /// Position side: long, short (only for isolated in long/short mode)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pos_side: Option<String>,
}

/// Response for set_leverage.
#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetLeverageResponse {
    /// Leverage
    pub lever: String,
    /// Margin mode
    pub mgn_mode: String,
    /// Instrument ID
    #[serde(default)]
    pub inst_id: String,
    /// Position side
    #[serde(default)]
    pub pos_side: String,
}

/// Query parameters for get_leverage_info.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetLeverageInfoParams {
    /// Margin mode: cross, isolated
    pub mgn_mode: String,
    /// Currency (for cross margin)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ccy: Option<String>,
    /// Instrument ID (for isolated margin)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inst_id: Option<String>,
}

/// Leverage info response.
#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LeverageInfo {
    /// Instrument ID
    #[serde(default)]
    pub inst_id: String,
    /// Margin mode
    pub mgn_mode: String,
    /// Position side
    #[serde(default)]
    pub pos_side: String,
    /// Leverage
    pub lever: String,
}

/// Query parameters for get_max_size.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetMaxSizeParams {
    /// Instrument ID
    pub inst_id: String,
    /// Trade mode: cash, cross, isolated
    pub td_mode: String,
    /// Currency
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ccy: Option<String>,
    /// Price
    #[serde(skip_serializing_if = "Option::is_none")]
    pub px: Option<String>,
    /// Leverage
    #[serde(skip_serializing_if = "Option::is_none")]
    pub leverage: Option<String>,
}

/// Max size response.
#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MaxSize {
    /// Instrument ID
    pub inst_id: String,
    /// Currency
    #[serde(default)]
    pub ccy: String,
    /// Maximum buy amount
    #[serde(default)]
    pub max_buy: String,
    /// Maximum sell amount
    #[serde(default)]
    pub max_sell: String,
}

/// Query parameters for get_max_avail_size.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetMaxAvailSizeParams {
    /// Instrument ID
    pub inst_id: String,
    /// Trade mode: cash, cross, isolated
    pub td_mode: String,
    /// Currency
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ccy: Option<String>,
    /// Reduce only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reduce_only: Option<bool>,
    /// Quick margin type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quick_mgn_type: Option<String>,
}

/// Max available size response.
#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MaxAvailSize {
    /// Instrument ID
    pub inst_id: String,
    /// Available buy amount
    #[serde(default)]
    pub avail_buy: String,
    /// Available sell amount
    #[serde(default)]
    pub avail_sell: String,
}

/// Query parameters for get_max_loan.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetMaxLoanParams {
    /// Instrument ID
    #[serde(rename = "instId")]
    pub inst_id: String,
    /// Margin mode
    #[serde(rename = "mgnMode")]
    pub mgn_mode: String,
    /// Margin currency
    #[serde(rename = "mgnCcy", skip_serializing_if = "Option::is_none")]
    pub mgn_ccy: Option<String>,
}

/// Query parameters for get_fee_rates.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetFeeRatesParams {
    /// Instrument type: SPOT, MARGIN, SWAP, FUTURES, OPTION
    pub inst_type: String,
    /// Instrument ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inst_id: Option<String>,
    /// Underlying
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uly: Option<String>,
    /// Instrument family
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inst_family: Option<String>,
}

/// Fee rates response.
#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FeeRates {
    /// Instrument type
    pub inst_type: String,
    /// Maker fee rate
    #[serde(default)]
    pub maker: String,
    /// Taker fee rate
    #[serde(default)]
    pub taker: String,
    /// Maker fee rate for USDT margined contracts
    #[serde(default)]
    pub maker_u: String,
    /// Taker fee rate for USDT margined contracts
    #[serde(default)]
    pub taker_u: String,
    /// Maker fee rate for USDC margined contracts
    #[serde(default)]
    pub maker_usdc: String,
    /// Taker fee rate for USDC margined contracts
    #[serde(default)]
    pub taker_usdc: String,
    /// Fee level
    #[serde(default)]
    pub level: String,
    /// Timestamp
    #[serde(default)]
    pub ts: String,
}

/// Request for set_position_mode.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetPositionModeRequest {
    /// Position mode: long_short_mode, net_mode
    pub pos_mode: String,
}

/// Response for set_position_mode.
#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetPositionModeResponse {
    /// Position mode
    pub pos_mode: String,
}

/// Account position risk response.
#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountPositionRisk {
    /// Adjusted equity
    #[serde(default)]
    pub adj_eq: String,
    /// Balance data
    #[serde(default)]
    pub bal_data: Vec<BalanceRiskData>,
    /// Position data
    #[serde(default)]
    pub pos_data: Vec<PositionRiskData>,
    /// Timestamp
    #[serde(default)]
    pub ts: String,
}

/// Balance risk data.
#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BalanceRiskData {
    /// Currency
    pub ccy: String,
    /// Equity
    #[serde(default)]
    pub eq: String,
    /// Discount equity
    #[serde(default)]
    pub dis_eq: String,
}

/// Position risk data.
#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PositionRiskData {
    /// Instrument ID
    pub inst_id: String,
    /// Instrument type
    #[serde(default)]
    pub inst_type: String,
    /// Margin mode
    #[serde(default)]
    pub mgn_mode: String,
    /// Position side
    #[serde(default)]
    pub pos_side: String,
    /// Position
    #[serde(default)]
    pub pos: String,
    /// Base currency balance
    #[serde(default)]
    pub base_bal: String,
    /// Quote currency balance
    #[serde(default)]
    pub quote_bal: String,
    /// Position currency
    #[serde(default)]
    pub pos_ccy: String,
    /// Currency
    #[serde(default)]
    pub ccy: String,
    /// Notional currency
    #[serde(default)]
    pub notional_ccy: String,
    /// Notional USD
    #[serde(default)]
    pub notional_usd: String,
}

/// Query parameters for get_positions_history.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPositionsHistoryParams {
    /// Instrument type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inst_type: Option<String>,
    /// Instrument ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inst_id: Option<String>,
    /// Margin mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mgn_mode: Option<String>,
    /// Position type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// Position ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pos_id: Option<String>,
    /// Pagination: query after this ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// Pagination: query before this ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    /// Page size (max 100)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,
}

/// Query parameters for recent bills (7 days).
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetBillsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inst_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ccy: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mgn_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ct_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,
}

/// Query parameters for historical bills (3 months).
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetBillsArchiveParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inst_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ccy: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mgn_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ct_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub begin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<String>,
}

/// Query parameters for interest accrued.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetInterestAccruedParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inst_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ccy: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mgn_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,
}

/// Query parameters for interest rate.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetInterestRateParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ccy: Option<String>,
}

/// Query parameters for VIP interest records.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetVipInterestParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ccy: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ord_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,
}

/// Query parameters for simulated margin.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSimulatedMarginParams {
    /// Instrument type
    pub inst_type: String,
    /// Include real positions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub incl_real_pos: Option<bool>,
    /// Spot offset type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spot_offset_type: Option<String>,
    /// Simulated positions payload
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sim_pos: Option<Vec<Value>>,
}

/// Query parameters for account position tiers.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAccountPositionTiersParams {
    pub inst_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uly: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inst_family: Option<String>,
}

/// Query parameters for greeks.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetGreeksParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ccy: Option<String>,
}

/// Query parameters for max withdrawal.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetMaxWithdrawalParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ccy: Option<String>,
}

/// Request body for position_builder.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PositionBuilderRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acct_lv: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub incl_real_pos_and_eq: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lever: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub greeks_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sim_pos: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sim_asset: Option<Value>,
}

/// Request body for set_greeks.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetGreeksRequest {
    pub greeks_type: String,
}

/// Request body for set_isolated_mode.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetIsolatedModeRequest {
    pub iso_mode: String,
    pub r#type: String,
}

/// Request body for set_account_level.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetAccountLevelRequest {
    pub acct_lv: String,
}

/// Request body for borrow_repay.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BorrowRepayRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ccy: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub side: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amt: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ord_id: Option<String>,
}

/// Query parameters for borrow_repay history.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BorrowRepayHistoryParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ccy: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,
}

/// Request body for spot manual borrow or repay.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SpotManualBorrowRepayRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ccy: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub side: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amt: Option<String>,
}

/// Query parameters for spot borrow repay history.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SpotBorrowRepayHistoryParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ccy: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,
}

/// 调整持仓保证金请求。
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AdjustmentMarginRequest {
    /// 交易对 ID
    pub inst_id: String,
    /// 持仓方向
    pub pos_side: String,
    /// add / reduce
    pub r#type: String,
    /// 调整数量
    pub amt: String,
    /// 是否借贷划转
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loan_trans: Option<bool>,
}

/// 设置风险对冲类型请求。
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetRiskOffsetTypeRequest {
    /// 风险对冲类型
    pub r#type: String,
}

/// 设置自动借币请求。
#[derive(Debug, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetAutoLoanRequest {
    /// 是否开启自动借币
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_loan: Option<String>,
}

/// 设置自动还币请求。
#[derive(Debug, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetAutoRepayRequest {
    /// 是否开启自动还币
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_repay: Option<bool>,
}

/// 借币利息抵扣额度查询参数。
#[derive(Debug, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetInterestLimitsParams {
    /// 类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// 币种
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ccy: Option<String>,
}

/// 尊享借币订单列表查询参数。
#[derive(Debug, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetVipLoanOrderListParams {
    /// 订单 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ord_id: Option<String>,
    /// 订单状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// 币种
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ccy: Option<String>,
    /// 分页游标
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// 分页游标
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    /// 分页数量
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,
}

/// 尊享借币订单详情查询参数。
#[derive(Debug, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetVipLoanOrderDetailParams {
    /// 币种
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ccy: Option<String>,
    /// 订单 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ord_id: Option<String>,
    /// 分页游标
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// 分页游标
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    /// 分页数量
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,
}

/// 定期借币报价查询参数。
#[derive(Debug, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetFixLoanBorrowingQuoteParams {
    /// 类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// 币种
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ccy: Option<String>,
    /// 借币数量
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amt: Option<String>,
    /// 最高利率
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_rate: Option<String>,
    /// 借币期限
    #[serde(skip_serializing_if = "Option::is_none")]
    pub term: Option<String>,
    /// 订单 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ord_id: Option<String>,
}

/// 定期借币下单请求。
#[derive(Debug, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct FixLoanBorrowingOrderRequest {
    /// 币种
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ccy: Option<String>,
    /// 借币数量
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amt: Option<String>,
    /// 最高利率
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_rate: Option<String>,
    /// 借币期限
    #[serde(skip_serializing_if = "Option::is_none")]
    pub term: Option<String>,
    /// 是否自动续借
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reborrow: Option<bool>,
    /// 自动续借最高利率
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reborrow_rate: Option<String>,
}

/// 修改定期借币订单请求。
#[derive(Debug, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AmendFixLoanBorrowingOrderRequest {
    /// 订单 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ord_id: Option<String>,
    /// 是否自动续借
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reborrow: Option<bool>,
    /// 续借最高利率
    #[serde(skip_serializing_if = "Option::is_none")]
    pub renew_max_rate: Option<String>,
}

/// 手动续借请求。
#[derive(Debug, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct FixLoanManualReborrowRequest {
    /// 订单 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ord_id: Option<String>,
    /// 最高利率
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_rate: Option<String>,
}

/// 定期借币还币请求。
#[derive(Debug, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RepayFixLoanBorrowingOrderRequest {
    /// 订单 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ord_id: Option<String>,
}

/// 定期借币订单列表查询参数。
#[derive(Debug, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetFixLoanBorrowingOrdersListParams {
    /// 订单 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ord_id: Option<String>,
    /// 币种
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ccy: Option<String>,
    /// 状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// 分页游标
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// 分页游标
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    /// 分页数量
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,
}

/// Raw account payload used for untyped endpoints.
pub type AccountRaw = Value;

/// Account API trait for OKX REST client.
///
/// Provides methods for account-related operations.
pub trait AccountApi {
    /// Get account balance.
    ///
    /// Retrieves assets with non-zero balance in the trading account.
    ///
    /// ## API Details
    ///
    /// - Endpoint: GET /api/v5/account/balance
    /// - Rate limit: 10 requests per 2 seconds
    /// - Permission: Read
    ///
    /// ## Arguments
    ///
    /// * `ccy` - Optional currency filter (e.g., "BTC" or "BTC,ETH")
    ///
    /// ## Example
    ///
    /// ```rust,no_run
    /// # use okx_rest::{OkxRestClient, AccountApi};
    /// # use okx_core::{Config, Credentials};
    /// # async fn example() -> okx_core::Result<()> {
    /// let client = OkxRestClient::new(Config::new(Credentials::new("", "", "")));
    /// let balances = client.get_balance(Some("BTC,ETH")).await?;
    /// # Ok(())
    /// # }
    /// ```
    fn get_balance(
        &self,
        ccy: Option<&str>,
    ) -> impl std::future::Future<Output = Result<Vec<Balance>>> + Send;

    /// Get positions.
    ///
    /// Retrieves position information. Empty data if no positions.
    ///
    /// ## API Details
    ///
    /// - Endpoint: GET /api/v5/account/positions
    /// - Rate limit: 10 requests per 2 seconds
    /// - Permission: Read
    ///
    /// ## Arguments
    ///
    /// * `params` - Optional filter parameters
    fn get_positions(
        &self,
        params: Option<GetPositionsParams>,
    ) -> impl std::future::Future<Output = Result<Vec<Position>>> + Send;

    /// Get available instruments info of current account.
    ///
    /// ## API Details
    ///
    /// - Endpoint: GET /api/v5/account/instruments
    /// - Permission: Read
    fn get_account_instruments(
        &self,
        params: GetAccountInstrumentsParams,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;

    /// Get account configuration.
    ///
    /// Retrieves current account configuration.
    ///
    /// ## API Details
    ///
    /// - Endpoint: GET /api/v5/account/config
    /// - Rate limit: 5 requests per 2 seconds
    /// - Permission: Read
    fn get_account_config(
        &self,
    ) -> impl std::future::Future<Output = Result<Vec<AccountConfig>>> + Send;

    /// Set leverage.
    ///
    /// Sets leverage for an instrument.
    ///
    /// ## API Details
    ///
    /// - Endpoint: POST /api/v5/account/set-leverage
    /// - Rate limit: 20 requests per 2 seconds
    /// - Permission: Trade
    fn set_leverage(
        &self,
        request: SetLeverageRequest,
    ) -> impl std::future::Future<Output = Result<Vec<SetLeverageResponse>>> + Send;

    /// Get leverage info.
    ///
    /// ## API Details
    ///
    /// - Endpoint: GET /api/v5/account/leverage-info
    /// - Rate limit: 20 requests per 2 seconds
    /// - Permission: Read
    fn get_leverage_info(
        &self,
        params: GetLeverageInfoParams,
    ) -> impl std::future::Future<Output = Result<Vec<LeverageInfo>>> + Send;

    /// Get maximum tradable amount.
    ///
    /// ## API Details
    ///
    /// - Endpoint: GET /api/v5/account/max-size
    /// - Rate limit: 20 requests per 2 seconds
    /// - Permission: Read
    fn get_max_size(
        &self,
        params: GetMaxSizeParams,
    ) -> impl std::future::Future<Output = Result<Vec<MaxSize>>> + Send;

    /// Get maximum available tradable amount.
    ///
    /// ## API Details
    ///
    /// - Endpoint: GET /api/v5/account/max-avail-size
    /// - Rate limit: 20 requests per 2 seconds
    /// - Permission: Read
    fn get_max_avail_size(
        &self,
        params: GetMaxAvailSizeParams,
    ) -> impl std::future::Future<Output = Result<Vec<MaxAvailSize>>> + Send;

    /// Get maximum loan for instrument.
    ///
    /// ## API Details
    ///
    /// - Endpoint: GET /api/v5/account/max-loan
    /// - Permission: Read
    fn get_max_loan(
        &self,
        params: GetMaxLoanParams,
    ) -> impl std::future::Future<Output = Result<Vec<AccountRaw>>> + Send;

    /// Get fee rates.
    ///
    /// ## API Details
    ///
    /// - Endpoint: GET /api/v5/account/trade-fee
    /// - Rate limit: 5 requests per 2 seconds
    /// - Permission: Read
    fn get_fee_rates(
        &self,
        params: GetFeeRatesParams,
    ) -> impl std::future::Future<Output = Result<Vec<FeeRates>>> + Send;

    /// Set position mode.
    ///
    /// ## API Details
    ///
    /// - Endpoint: POST /api/v5/account/set-position-mode
    /// - Rate limit: 5 requests per 2 seconds
    /// - Permission: Trade
    fn set_position_mode(
        &self,
        pos_mode: &str,
    ) -> impl std::future::Future<Output = Result<Vec<SetPositionModeResponse>>> + Send;

    /// Get account position risk.
    ///
    /// ## API Details
    ///
    /// - Endpoint: GET /api/v5/account/account-position-risk
    /// - Rate limit: 10 requests per 2 seconds
    /// - Permission: Read
    fn get_account_position_risk(
        &self,
    ) -> impl std::future::Future<Output = Result<Vec<AccountPositionRisk>>> + Send;

    /// Get account risk state.
    ///
    /// ## API Details
    ///
    /// - Endpoint: GET /api/v5/account/risk-state
    /// - Permission: Read
    fn get_account_risk_state(&self) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;

    /// Get historical positions (up to 3 months).
    fn get_positions_history(
        &self,
        params: Option<GetPositionsHistoryParams>,
    ) -> impl std::future::Future<Output = Result<Vec<AccountRaw>>> + Send;

    /// Get maximum withdrawal amount.
    fn get_max_withdrawal(
        &self,
        ccy: Option<&str>,
    ) -> impl std::future::Future<Output = Result<Vec<AccountRaw>>> + Send;

    /// Get recent account bills (7 days).
    fn get_account_bills(
        &self,
        params: Option<GetBillsParams>,
    ) -> impl std::future::Future<Output = Result<Vec<AccountRaw>>> + Send;

    /// Get archived account bills (3 months).
    fn get_account_bills_archive(
        &self,
        params: GetBillsArchiveParams,
    ) -> impl std::future::Future<Output = Result<Vec<AccountRaw>>> + Send;

    /// Set greeks display preference.
    fn set_greeks(
        &self,
        request: SetGreeksRequest,
    ) -> impl std::future::Future<Output = Result<Vec<AccountRaw>>> + Send;

    /// Set isolated margin mode.
    fn set_isolated_mode(
        &self,
        request: SetIsolatedModeRequest,
    ) -> impl std::future::Future<Output = Result<Vec<AccountRaw>>> + Send;

    /// Set account level.
    fn set_account_level(
        &self,
        request: SetAccountLevelRequest,
    ) -> impl std::future::Future<Output = Result<Vec<AccountRaw>>> + Send;

    /// Borrow or repay.
    fn borrow_repay(
        &self,
        request: BorrowRepayRequest,
    ) -> impl std::future::Future<Output = Result<Vec<AccountRaw>>> + Send;

    /// Borrow/repay history.
    fn get_borrow_repay_history(
        &self,
        params: Option<BorrowRepayHistoryParams>,
    ) -> impl std::future::Future<Output = Result<Vec<AccountRaw>>> + Send;

    /// Spot manual borrow/repay.
    fn spot_manual_borrow_repay(
        &self,
        request: SpotManualBorrowRepayRequest,
    ) -> impl std::future::Future<Output = Result<Vec<AccountRaw>>> + Send;

    /// Spot borrow/repay history.
    fn spot_borrow_repay_history(
        &self,
        params: Option<SpotBorrowRepayHistoryParams>,
    ) -> impl std::future::Future<Output = Result<Vec<AccountRaw>>> + Send;

    /// 调整持仓保证金。
    fn adjustment_margin(
        &self,
        request: AdjustmentMarginRequest,
    ) -> impl std::future::Future<Output = Result<Vec<AccountRaw>>> + Send;

    /// 设置风险对冲类型。
    fn set_risk_offset_type(
        &self,
        request: SetRiskOffsetTypeRequest,
    ) -> impl std::future::Future<Output = Result<Vec<AccountRaw>>> + Send;

    /// 设置自动借币。
    fn set_auto_loan(
        &self,
        request: SetAutoLoanRequest,
    ) -> impl std::future::Future<Output = Result<Vec<AccountRaw>>> + Send;

    /// 激活期权交易。
    fn activate_option(&self) -> impl std::future::Future<Output = Result<Vec<AccountRaw>>> + Send;

    /// 设置自动还币。
    fn set_auto_repay(
        &self,
        request: SetAutoRepayRequest,
    ) -> impl std::future::Future<Output = Result<Vec<AccountRaw>>> + Send;

    /// 借币利息抵扣额度及利率。
    fn get_interest_limits(
        &self,
        params: Option<GetInterestLimitsParams>,
    ) -> impl std::future::Future<Output = Result<Vec<AccountRaw>>> + Send;

    /// 尊享借币订单列表。
    fn get_vip_loan_order_list(
        &self,
        params: Option<GetVipLoanOrderListParams>,
    ) -> impl std::future::Future<Output = Result<Vec<AccountRaw>>> + Send;

    /// 尊享借币订单详情。
    fn get_vip_loan_order_detail(
        &self,
        params: Option<GetVipLoanOrderDetailParams>,
    ) -> impl std::future::Future<Output = Result<Vec<AccountRaw>>> + Send;

    /// 定期借币限额。
    fn get_fix_loan_borrowing_limit(
        &self,
    ) -> impl std::future::Future<Output = Result<Vec<AccountRaw>>> + Send;

    /// 定期借币报价。
    fn get_fix_loan_borrowing_quote(
        &self,
        params: Option<GetFixLoanBorrowingQuoteParams>,
    ) -> impl std::future::Future<Output = Result<Vec<AccountRaw>>> + Send;

    /// 定期借币下单。
    fn place_fix_loan_borrowing_order(
        &self,
        request: FixLoanBorrowingOrderRequest,
    ) -> impl std::future::Future<Output = Result<Vec<AccountRaw>>> + Send;

    /// 修改定期借币订单。
    fn amend_fix_loan_borrowing_order(
        &self,
        request: AmendFixLoanBorrowingOrderRequest,
    ) -> impl std::future::Future<Output = Result<Vec<AccountRaw>>> + Send;

    /// 定期借币手动续借。
    fn fix_loan_manual_reborrow(
        &self,
        request: FixLoanManualReborrowRequest,
    ) -> impl std::future::Future<Output = Result<Vec<AccountRaw>>> + Send;

    /// 定期借币还币。
    fn repay_fix_loan_borrowing_order(
        &self,
        request: RepayFixLoanBorrowingOrderRequest,
    ) -> impl std::future::Future<Output = Result<Vec<AccountRaw>>> + Send;

    /// 定期借币订单列表。
    fn get_fix_loan_borrowing_orders_list(
        &self,
        params: Option<GetFixLoanBorrowingOrdersListParams>,
    ) -> impl std::future::Future<Output = Result<Vec<AccountRaw>>> + Send;

    /// Interest accrued records.
    fn get_interest_accrued(
        &self,
        params: GetInterestAccruedParams,
    ) -> impl std::future::Future<Output = Result<Vec<AccountRaw>>> + Send;

    /// Get interest rate.
    ///
    /// ## API Details
    ///
    /// - Endpoint: GET /api/v5/account/interest-rate
    /// - Permission: Read
    fn get_interest_rate(
        &self,
        ccy: Option<&str>,
    ) -> impl std::future::Future<Output = Result<Vec<AccountRaw>>> + Send;

    /// VIP interest accrued records.
    fn get_vip_interest_accrued(
        &self,
        params: GetVipInterestParams,
    ) -> impl std::future::Future<Output = Result<Vec<AccountRaw>>> + Send;

    /// VIP interest deducted records.
    fn get_vip_interest_deducted(
        &self,
        params: GetVipInterestParams,
    ) -> impl std::future::Future<Output = Result<Vec<AccountRaw>>> + Send;

    /// Simulated margin preview.
    fn get_simulated_margin(
        &self,
        params: GetSimulatedMarginParams,
    ) -> impl std::future::Future<Output = Result<Vec<AccountRaw>>> + Send;

    /// Account position tiers.
    fn get_account_position_tiers(
        &self,
        params: GetAccountPositionTiersParams,
    ) -> impl std::future::Future<Output = Result<Vec<AccountRaw>>> + Send;

    /// Account greeks.
    fn get_greeks(
        &self,
        params: GetGreeksParams,
    ) -> impl std::future::Future<Output = Result<Vec<AccountRaw>>> + Send;

    /// Build simulated positions.
    fn position_builder(
        &self,
        request: PositionBuilderRequest,
    ) -> impl std::future::Future<Output = Result<Vec<AccountRaw>>> + Send;
}

impl AccountApi for OkxRestClient {
    async fn get_balance(&self, ccy: Option<&str>) -> Result<Vec<Balance>> {
        let params = ccy.map(|c| GetBalanceParams {
            ccy: Some(c.to_string()),
        });
        self.get(endpoints::BALANCE, params.as_ref()).await
    }

    async fn get_positions(&self, params: Option<GetPositionsParams>) -> Result<Vec<Position>> {
        self.get(endpoints::POSITIONS, params.as_ref()).await
    }

    async fn get_account_instruments(
        &self,
        params: GetAccountInstrumentsParams,
    ) -> Result<Vec<Value>> {
        self.get(endpoints::INSTRUMENTS, Some(&params)).await
    }

    async fn get_account_config(&self) -> Result<Vec<AccountConfig>> {
        self.get::<AccountConfig, ()>(endpoints::CONFIG, None).await
    }

    async fn set_leverage(&self, request: SetLeverageRequest) -> Result<Vec<SetLeverageResponse>> {
        self.post(endpoints::SET_LEVERAGE, &request).await
    }

    async fn get_leverage_info(&self, params: GetLeverageInfoParams) -> Result<Vec<LeverageInfo>> {
        self.get(endpoints::LEVERAGE_INFO, Some(&params)).await
    }

    async fn get_max_size(&self, params: GetMaxSizeParams) -> Result<Vec<MaxSize>> {
        self.get(endpoints::MAX_SIZE, Some(&params)).await
    }

    async fn get_max_avail_size(&self, params: GetMaxAvailSizeParams) -> Result<Vec<MaxAvailSize>> {
        self.get(endpoints::MAX_AVAIL_SIZE, Some(&params)).await
    }

    async fn get_max_loan(&self, params: GetMaxLoanParams) -> Result<Vec<AccountRaw>> {
        self.get(endpoints::MAX_LOAN, Some(&params)).await
    }

    async fn get_fee_rates(&self, params: GetFeeRatesParams) -> Result<Vec<FeeRates>> {
        self.get(endpoints::TRADE_FEE, Some(&params)).await
    }

    async fn set_position_mode(&self, pos_mode: &str) -> Result<Vec<SetPositionModeResponse>> {
        let request = SetPositionModeRequest {
            pos_mode: pos_mode.to_string(),
        };
        self.post(endpoints::SET_POSITION_MODE, &request).await
    }

    async fn get_account_position_risk(&self) -> Result<Vec<AccountPositionRisk>> {
        self.get::<AccountPositionRisk, ()>(endpoints::POSITION_RISK, None)
            .await
    }

    async fn get_account_risk_state(&self) -> Result<Vec<Value>> {
        self.get(endpoints::RISK_STATE, None::<&()>).await
    }

    async fn get_positions_history(
        &self,
        params: Option<GetPositionsHistoryParams>,
    ) -> Result<Vec<AccountRaw>> {
        self.get(endpoints::POSITIONS_HISTORY, params.as_ref())
            .await
    }

    async fn get_max_withdrawal(&self, ccy: Option<&str>) -> Result<Vec<AccountRaw>> {
        let params = GetMaxWithdrawalParams {
            ccy: ccy.map(|v| v.to_string()),
        };
        self.get(endpoints::MAX_WITHDRAWAL, Some(&params)).await
    }

    async fn get_account_bills(&self, params: Option<GetBillsParams>) -> Result<Vec<AccountRaw>> {
        self.get(endpoints::BILLS, params.as_ref()).await
    }

    async fn get_account_bills_archive(
        &self,
        params: GetBillsArchiveParams,
    ) -> Result<Vec<AccountRaw>> {
        self.get(endpoints::BILLS_ARCHIVE, Some(&params)).await
    }

    async fn set_greeks(&self, request: SetGreeksRequest) -> Result<Vec<AccountRaw>> {
        self.post(endpoints::SET_GREEKS, &request).await
    }

    async fn set_isolated_mode(&self, request: SetIsolatedModeRequest) -> Result<Vec<AccountRaw>> {
        self.post(endpoints::SET_ISOLATED_MODE, &request).await
    }

    async fn set_account_level(&self, request: SetAccountLevelRequest) -> Result<Vec<AccountRaw>> {
        self.post(endpoints::SET_ACCOUNT_LEVEL, &request).await
    }

    async fn borrow_repay(&self, request: BorrowRepayRequest) -> Result<Vec<AccountRaw>> {
        self.post(endpoints::BORROW_REPAY, &request).await
    }

    async fn get_borrow_repay_history(
        &self,
        params: Option<BorrowRepayHistoryParams>,
    ) -> Result<Vec<AccountRaw>> {
        self.get(endpoints::BORROW_REPAY_HISTORY, params.as_ref())
            .await
    }

    async fn spot_manual_borrow_repay(
        &self,
        request: SpotManualBorrowRepayRequest,
    ) -> Result<Vec<AccountRaw>> {
        self.post(endpoints::SPOT_MANUAL_BORROW_REPAY, &request)
            .await
    }

    async fn spot_borrow_repay_history(
        &self,
        params: Option<SpotBorrowRepayHistoryParams>,
    ) -> Result<Vec<AccountRaw>> {
        self.get(endpoints::SPOT_BORROW_REPAY_HISTORY, params.as_ref())
            .await
    }

    async fn adjustment_margin(&self, request: AdjustmentMarginRequest) -> Result<Vec<AccountRaw>> {
        self.post(endpoints::ADJUSTMENT_MARGIN, &request).await
    }

    async fn set_risk_offset_type(
        &self,
        request: SetRiskOffsetTypeRequest,
    ) -> Result<Vec<AccountRaw>> {
        self.post(endpoints::SET_RISK_OFFSET_TYPE, &request).await
    }

    async fn set_auto_loan(&self, request: SetAutoLoanRequest) -> Result<Vec<AccountRaw>> {
        self.post(endpoints::SET_AUTO_LOAN, &request).await
    }

    async fn activate_option(&self) -> Result<Vec<AccountRaw>> {
        self.post(endpoints::ACTIVATE_OPTION, &serde_json::json!({}))
            .await
    }

    async fn set_auto_repay(&self, request: SetAutoRepayRequest) -> Result<Vec<AccountRaw>> {
        self.post(endpoints::SET_AUTO_REPAY, &request).await
    }

    async fn get_interest_limits(
        &self,
        params: Option<GetInterestLimitsParams>,
    ) -> Result<Vec<AccountRaw>> {
        self.get(endpoints::INTEREST_LIMITS, params.as_ref()).await
    }

    async fn get_vip_loan_order_list(
        &self,
        params: Option<GetVipLoanOrderListParams>,
    ) -> Result<Vec<AccountRaw>> {
        self.get(endpoints::VIP_LOAN_ORDER_LIST, params.as_ref())
            .await
    }

    async fn get_vip_loan_order_detail(
        &self,
        params: Option<GetVipLoanOrderDetailParams>,
    ) -> Result<Vec<AccountRaw>> {
        self.get(endpoints::VIP_LOAN_ORDER_DETAIL, params.as_ref())
            .await
    }

    async fn get_fix_loan_borrowing_limit(&self) -> Result<Vec<AccountRaw>> {
        self.get(endpoints::FIX_LOAN_BORROWING_LIMIT, None::<&()>)
            .await
    }

    async fn get_fix_loan_borrowing_quote(
        &self,
        params: Option<GetFixLoanBorrowingQuoteParams>,
    ) -> Result<Vec<AccountRaw>> {
        self.get(endpoints::FIX_LOAN_BORROWING_QUOTE, params.as_ref())
            .await
    }

    async fn place_fix_loan_borrowing_order(
        &self,
        request: FixLoanBorrowingOrderRequest,
    ) -> Result<Vec<AccountRaw>> {
        self.post(endpoints::FIX_LOAN_BORROWING_ORDER, &request)
            .await
    }

    async fn amend_fix_loan_borrowing_order(
        &self,
        request: AmendFixLoanBorrowingOrderRequest,
    ) -> Result<Vec<AccountRaw>> {
        self.post(endpoints::FIX_LOAN_AMEND_BORROWING_ORDER, &request)
            .await
    }

    async fn fix_loan_manual_reborrow(
        &self,
        request: FixLoanManualReborrowRequest,
    ) -> Result<Vec<AccountRaw>> {
        self.post(endpoints::FIX_LOAN_MANUAL_REBORROW, &request)
            .await
    }

    async fn repay_fix_loan_borrowing_order(
        &self,
        request: RepayFixLoanBorrowingOrderRequest,
    ) -> Result<Vec<AccountRaw>> {
        self.post(endpoints::FIX_LOAN_REPAY_BORROWING_ORDER, &request)
            .await
    }

    async fn get_fix_loan_borrowing_orders_list(
        &self,
        params: Option<GetFixLoanBorrowingOrdersListParams>,
    ) -> Result<Vec<AccountRaw>> {
        self.get(endpoints::FIX_LOAN_BORROWING_ORDERS_LIST, params.as_ref())
            .await
    }

    async fn get_interest_accrued(
        &self,
        params: GetInterestAccruedParams,
    ) -> Result<Vec<AccountRaw>> {
        self.get(endpoints::INTEREST_ACCRUED, Some(&params)).await
    }

    async fn get_interest_rate(&self, ccy: Option<&str>) -> Result<Vec<AccountRaw>> {
        let params = GetInterestRateParams {
            ccy: ccy.map(|v| v.to_string()),
        };
        self.get(endpoints::INTEREST_RATE, Some(&params)).await
    }

    async fn get_vip_interest_accrued(
        &self,
        params: GetVipInterestParams,
    ) -> Result<Vec<AccountRaw>> {
        self.get(endpoints::VIP_INTEREST_ACCRUED, Some(&params))
            .await
    }

    async fn get_vip_interest_deducted(
        &self,
        params: GetVipInterestParams,
    ) -> Result<Vec<AccountRaw>> {
        self.get(endpoints::VIP_INTEREST_DEDUCTED, Some(&params))
            .await
    }

    async fn get_simulated_margin(
        &self,
        params: GetSimulatedMarginParams,
    ) -> Result<Vec<AccountRaw>> {
        self.get(endpoints::SIMULATED_MARGIN, Some(&params)).await
    }

    async fn get_account_position_tiers(
        &self,
        params: GetAccountPositionTiersParams,
    ) -> Result<Vec<AccountRaw>> {
        self.get(endpoints::ACCOUNT_POSITION_TIERS, Some(&params))
            .await
    }

    async fn get_greeks(&self, params: GetGreeksParams) -> Result<Vec<AccountRaw>> {
        self.get(endpoints::GREEKS, Some(&params)).await
    }

    async fn position_builder(&self, request: PositionBuilderRequest) -> Result<Vec<AccountRaw>> {
        self.post(endpoints::POSITION_BUILDER, &request).await
    }
}
