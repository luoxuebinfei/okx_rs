//! Account-related data types.
//!
//! Source: OKX API v5 Trading Account REST API
//! - GET /api/v5/account/balance
//! - GET /api/v5/account/positions
//! - GET /api/v5/account/config

use serde::{Deserialize, Serialize};

/// Account balance information.
///
/// Source: GET /api/v5/account/balance response
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Balance {
    /// Total equity in USD
    #[serde(default)]
    pub total_eq: String,
    /// Isolated margin equity in USD
    #[serde(default)]
    pub iso_eq: String,
    /// Adjusted equity (for portfolio margin)
    #[serde(default)]
    pub adj_eq: String,
    /// Order frozen amount
    #[serde(default)]
    pub ord_froz: String,
    /// Initial margin requirement
    #[serde(default)]
    pub imr: String,
    /// Maintenance margin requirement
    #[serde(default)]
    pub mmr: String,
    /// Borrowed frozen amount
    #[serde(default)]
    pub borrow_froz: String,
    /// Margin ratio
    #[serde(default)]
    pub mgn_ratio: String,
    /// Notional value in USD
    #[serde(default)]
    pub notional_usd: String,
    /// Currency-specific balance details
    #[serde(default)]
    pub details: Vec<BalanceDetail>,
    /// Update time (Unix timestamp in milliseconds)
    #[serde(default)]
    pub u_time: String,
}

/// Currency-specific balance detail.
///
/// Source: GET /api/v5/account/balance response - details array
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BalanceDetail {
    /// Currency code (e.g., "BTC", "USDT")
    pub ccy: String,
    /// Equity of the currency
    #[serde(default)]
    pub eq: String,
    /// Cash balance
    #[serde(default)]
    pub cash_bal: String,
    /// Update time (Unix timestamp in milliseconds)
    #[serde(default)]
    pub u_time: String,
    /// Isolated margin equity
    #[serde(default)]
    pub iso_eq: String,
    /// Available equity
    #[serde(default)]
    pub avail_eq: String,
    /// Discount equity
    #[serde(default)]
    pub dis_eq: String,
    /// Fixed balance
    #[serde(default)]
    pub fixed_bal: String,
    /// Available balance
    #[serde(default)]
    pub avail_bal: String,
    /// Frozen balance
    #[serde(default)]
    pub frozen_bal: String,
    /// Order frozen amount
    #[serde(default)]
    pub ord_frozen: String,
    /// Liability
    #[serde(default)]
    pub liab: String,
    /// Unrealized profit and loss
    #[serde(default)]
    pub upl: String,
    /// Unrealized profit and loss liability
    #[serde(default)]
    pub upl_liab: String,
    /// Cross liability
    #[serde(default)]
    pub cross_liab: String,
    /// Isolated liability
    #[serde(default)]
    pub iso_liab: String,
    /// Margin ratio
    #[serde(default)]
    pub mgn_ratio: String,
    /// Interest
    #[serde(default)]
    pub interest: String,
    /// TWAP value
    #[serde(default)]
    pub twap: String,
    /// Maximum loan
    #[serde(default)]
    pub max_loan: String,
    /// Equity in USD
    #[serde(default)]
    pub eq_usd: String,
    /// Borrowed frozen amount
    #[serde(default)]
    pub borrow_froz: String,
    /// Notional leverage
    #[serde(default)]
    pub notional_lever: String,
    /// Strategy equity
    #[serde(default)]
    pub stgy_eq: String,
    /// Isolated unrealized profit and loss
    #[serde(default)]
    pub iso_upl: String,
    /// Spot in use amount
    #[serde(default)]
    pub spot_in_use_amt: String,
    /// Spot balance
    #[serde(default)]
    pub spot_bal: String,
    /// Open average price
    #[serde(default)]
    pub open_avg_px: String,
    /// Accumulated average price
    #[serde(default)]
    pub acc_avg_px: String,
    /// Spot unrealized profit and loss
    #[serde(default)]
    pub spot_upl: String,
    /// Spot unrealized profit and loss ratio
    #[serde(default)]
    pub spot_upl_ratio: String,
    /// Total profit and loss
    #[serde(default)]
    pub total_pnl: String,
    /// Total profit and loss ratio
    #[serde(default)]
    pub total_pnl_ratio: String,
}

/// Position information.
///
/// Source: GET /api/v5/account/positions response
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Position {
    /// Instrument type
    pub inst_type: String,
    /// Margin mode (cross/isolated)
    #[serde(default)]
    pub mgn_mode: String,
    /// Position ID
    pub pos_id: String,
    /// Position side (long/short/net)
    #[serde(default)]
    pub pos_side: String,
    /// Position quantity
    pub pos: String,
    /// Base currency balance (for MARGIN positions)
    #[serde(default)]
    pub base_bal: String,
    /// Quote currency balance (for MARGIN positions)
    #[serde(default)]
    pub quote_bal: String,
    /// Base currency borrowed (for MARGIN positions)
    #[serde(default)]
    pub base_borrowed: String,
    /// Base currency interest (for MARGIN positions)
    #[serde(default)]
    pub base_interest: String,
    /// Quote currency borrowed (for MARGIN positions)
    #[serde(default)]
    pub quote_borrowed: String,
    /// Quote currency interest (for MARGIN positions)
    #[serde(default)]
    pub quote_interest: String,
    /// Position currency (for MARGIN positions)
    #[serde(default)]
    pub pos_ccy: String,
    /// Available position quantity
    #[serde(default)]
    pub avail_pos: String,
    /// Average open price
    #[serde(default)]
    pub avg_px: String,
    /// Unrealized profit and loss
    #[serde(default)]
    pub upl: String,
    /// Unrealized profit and loss ratio
    #[serde(default)]
    pub upl_ratio: String,
    /// Unrealized profit and loss in the last price
    #[serde(default)]
    pub upl_last_px: String,
    /// Unrealized profit and loss ratio in the last price
    #[serde(default)]
    pub upl_ratio_last_px: String,
    /// Instrument ID
    pub inst_id: String,
    /// Leverage
    #[serde(default)]
    pub lever: String,
    /// Liquidation price
    #[serde(default)]
    pub liq_px: String,
    /// Mark price
    #[serde(default)]
    pub mark_px: String,
    /// Initial margin requirement
    #[serde(default)]
    pub imr: String,
    /// Margin
    #[serde(default)]
    pub margin: String,
    /// Margin ratio
    #[serde(default)]
    pub mgn_ratio: String,
    /// Maintenance margin requirement
    #[serde(default)]
    pub mmr: String,
    /// Liability
    #[serde(default)]
    pub liab: String,
    /// Liability currency
    #[serde(default)]
    pub liab_ccy: String,
    /// Interest
    #[serde(default)]
    pub interest: String,
    /// Last trade ID
    #[serde(default)]
    pub trade_id: String,
    /// Option value (for options)
    #[serde(default)]
    pub opt_val: String,
    /// Pending close order algo count
    #[serde(default)]
    pub pending_close_ord_liab_val: String,
    /// Notional value in USD
    #[serde(default)]
    pub notional_usd: String,
    /// Auto-deleveraging indicator
    #[serde(default)]
    pub adl: String,
    /// Currency
    #[serde(default)]
    pub ccy: String,
    /// Last price
    #[serde(default)]
    pub last: String,
    /// Index price
    #[serde(default)]
    pub idx_px: String,
    /// USD price
    #[serde(default)]
    pub usd_px: String,
    /// Break-even price
    #[serde(default)]
    pub be_px: String,
    /// Delta (for options)
    #[serde(default)]
    pub delta_bs: String,
    /// Delta (for portfolio account)
    #[serde(default)]
    pub delta_pa: String,
    /// Gamma (for options)
    #[serde(default)]
    pub gamma_bs: String,
    /// Gamma (for portfolio account)
    #[serde(default)]
    pub gamma_pa: String,
    /// Theta (for options)
    #[serde(default)]
    pub theta_bs: String,
    /// Theta (for portfolio account)
    #[serde(default)]
    pub theta_pa: String,
    /// Vega (for options)
    #[serde(default)]
    pub vega_bs: String,
    /// Vega (for portfolio account)
    #[serde(default)]
    pub vega_pa: String,
    /// Spot in use amount
    #[serde(default)]
    pub spot_in_use_amt: String,
    /// Spot in use currency
    #[serde(default)]
    pub spot_in_use_ccy: String,
    /// Realized profit and loss
    #[serde(default)]
    pub realized_pnl: String,
    /// Position quantity in base currency
    #[serde(default)]
    pub pnl: String,
    /// Fee
    #[serde(default)]
    pub fee: String,
    /// Funding fee
    #[serde(default)]
    pub funding_fee: String,
    /// Liquidation fee
    #[serde(default)]
    pub liq_penalty: String,
    /// Close order algo details
    #[serde(default)]
    pub close_order_algo: Vec<CloseOrderAlgo>,
    /// Creation time
    #[serde(default)]
    pub c_time: String,
    /// Update time
    #[serde(default)]
    pub u_time: String,
    /// Bid price
    #[serde(default)]
    pub bid_px: String,
    /// Ask price
    #[serde(default)]
    pub ask_px: String,
}

/// Close order algo information attached to a position.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CloseOrderAlgo {
    /// Algo ID
    #[serde(default)]
    pub algo_id: String,
    /// Stop-loss trigger price
    #[serde(default)]
    pub sl_trigger_px: String,
    /// Stop-loss trigger price type
    #[serde(default)]
    pub sl_trigger_px_type: String,
    /// Take-profit trigger price
    #[serde(default)]
    pub tp_trigger_px: String,
    /// Take-profit trigger price type
    #[serde(default)]
    pub tp_trigger_px_type: String,
    /// Close fraction
    #[serde(default)]
    pub close_fraction: String,
}

/// Account configuration.
///
/// Source: GET /api/v5/account/config response
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountConfig {
    /// Account ID
    pub uid: String,
    /// Main account ID
    #[serde(default)]
    pub main_uid: String,
    /// Account level
    /// 1: Simple, 2: Single-currency margin, 3: Multi-currency margin, 4: Portfolio margin
    pub acct_lv: String,
    /// Position mode
    /// `long_short_mode`: Dual-position mode
    /// `net_mode`: One-way position mode
    pub pos_mode: String,
    /// Auto borrow flag
    #[serde(default)]
    pub auto_loan: bool,
    /// Greeks display type (PA: Portfolio account, BS: Black-Scholes)
    #[serde(default)]
    pub greeks_type: String,
    /// Current account level
    #[serde(default)]
    pub level: String,
    /// Temporary level
    #[serde(default)]
    pub level_tmp: String,
    /// Contract isolated margin trading settings
    #[serde(default)]
    pub ct_iso_mode: String,
    /// Margin isolated margin trading settings
    #[serde(default)]
    pub mgn_iso_mode: String,
    /// Risk offset type
    #[serde(default)]
    pub risk_offset_type: String,
    /// Whether spot trading is enabled
    #[serde(default)]
    pub spot_offset_type: String,
    /// Account label
    #[serde(default)]
    pub label: String,
    /// Role type
    #[serde(default)]
    pub role_type: String,
    /// Trailing commission rate
    #[serde(default)]
    pub spot_role_type: String,
    /// Option trade mode
    #[serde(default)]
    pub opauth: String,
    /// KYC level
    #[serde(default)]
    pub kyc_lv: String,
    /// Whether the user has enabled the feature of copy trading
    #[serde(default)]
    pub enable_spot_borrow: bool,
    /// Spot borrow auto repay
    #[serde(default)]
    pub spot_borrow_auto_repay: bool,
}
