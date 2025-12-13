//! Trade-related data types.
//!
//! Source: OKX API v5 Order Book Trading REST API
//! - POST /api/v5/trade/order
//! - GET /api/v5/trade/order
//! - GET /api/v5/trade/orders-pending
//! - GET /api/v5/trade/fills

use serde::{Deserialize, Serialize};

/// Order information.
///
/// Source: GET /api/v5/trade/order response
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Order {
    /// Instrument type
    pub inst_type: String,
    /// Instrument ID
    pub inst_id: String,
    /// Trade currency (for SPOT/MARGIN `tgtCcy=quote_ccy`)
    #[serde(default)]
    pub tgt_ccy: String,
    /// Margin currency
    #[serde(default)]
    pub ccy: String,
    /// Order ID
    pub ord_id: String,
    /// Client-supplied order ID
    #[serde(default)]
    pub cl_ord_id: String,
    /// Order tag
    #[serde(default)]
    pub tag: String,
    /// Price
    #[serde(default)]
    pub px: String,
    /// Quantity to buy or sell
    pub sz: String,
    /// Profit and loss
    #[serde(default)]
    pub pnl: String,
    /// Order type (market, limit, `post_only`, fok, ioc, etc.)
    pub ord_type: String,
    /// Order side (buy, sell)
    pub side: String,
    /// Position side (long, short, net)
    #[serde(default)]
    pub pos_side: String,
    /// Trade mode (cash, cross, isolated)
    pub td_mode: String,
    /// Accumulated fill quantity
    #[serde(default)]
    pub acc_fill_sz: String,
    /// Last filled price
    #[serde(default)]
    pub fill_px: String,
    /// Last trade ID
    #[serde(default)]
    pub trade_id: String,
    /// Last filled quantity
    #[serde(default)]
    pub fill_sz: String,
    /// Last filled time
    #[serde(default)]
    pub fill_time: String,
    /// Average filled price
    #[serde(default)]
    pub avg_px: String,
    /// Order state (created, live, `partially_filled`, canceled, filled)
    pub state: String,
    /// Leverage
    #[serde(default)]
    pub lever: String,
    /// Attached TPSL order details
    #[serde(default)]
    pub attach_algo_ords: Vec<AttachAlgoOrd>,
    /// Take-profit trigger price
    #[serde(default)]
    pub tp_trigger_px: String,
    /// Take-profit trigger price type
    #[serde(default)]
    pub tp_trigger_px_type: String,
    /// Take-profit order price
    #[serde(default)]
    pub tp_ord_px: String,
    /// Stop-loss trigger price
    #[serde(default)]
    pub sl_trigger_px: String,
    /// Stop-loss trigger price type
    #[serde(default)]
    pub sl_trigger_px_type: String,
    /// Stop-loss order price
    #[serde(default)]
    pub sl_ord_px: String,
    /// Self trade prevention ID
    #[serde(default)]
    pub stp_id: String,
    /// Self trade prevention mode
    #[serde(default)]
    pub stp_mode: String,
    /// Fee currency
    #[serde(default)]
    pub fee_ccy: String,
    /// Fee
    #[serde(default)]
    pub fee: String,
    /// Rebate currency
    #[serde(default)]
    pub rebate_ccy: String,
    /// Rebate
    #[serde(default)]
    pub rebate: String,
    /// Liquidity taker or maker (T: taker, M: maker)
    #[serde(default)]
    pub exec_type: String,
    /// Category
    #[serde(default)]
    pub category: String,
    /// Whether to reduce position only
    #[serde(default)]
    pub reduce_only: String,
    /// Quick margin type
    #[serde(default)]
    pub quick_mgn_type: String,
    /// Client algo order ID
    #[serde(default)]
    pub algo_cl_ord_id: String,
    /// Algo order ID
    #[serde(default)]
    pub algo_id: String,
    /// Cancel source
    #[serde(default)]
    pub cancel_source: String,
    /// Cancel source reason
    #[serde(default)]
    pub cancel_source_reason: String,
    /// Amend source
    #[serde(default)]
    pub amend_source: String,
    /// Request ID
    #[serde(default)]
    pub req_id: String,
    /// Amendment result
    #[serde(default)]
    pub amend_result: String,
    /// Whether it is close position order
    #[serde(default)]
    pub is_tp_limit: String,
    /// Update time
    #[serde(default)]
    pub u_time: String,
    /// Creation time
    #[serde(default)]
    pub c_time: String,
}

/// Attached algo order (TP/SL) information.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AttachAlgoOrd {
    /// Attached algo order ID
    #[serde(default)]
    pub attach_algo_id: String,
    /// Client-supplied algo ID
    #[serde(default)]
    pub attach_algo_cl_ord_id: String,
    /// Take-profit trigger price
    #[serde(default)]
    pub tp_trigger_px: String,
    /// Take-profit trigger price type
    #[serde(default)]
    pub tp_trigger_px_type: String,
    /// Take-profit order price
    #[serde(default)]
    pub tp_ord_px: String,
    /// Stop-loss trigger price
    #[serde(default)]
    pub sl_trigger_px: String,
    /// Stop-loss trigger price type
    #[serde(default)]
    pub sl_trigger_px_type: String,
    /// Stop-loss order price
    #[serde(default)]
    pub sl_ord_px: String,
    /// Order size
    #[serde(default)]
    pub sz: String,
    /// Whether it is a profit-taking order
    #[serde(default)]
    pub amend_px_on_trigger_type: String,
}

/// Trade/Fill information.
///
/// Source: GET /api/v5/trade/fills response
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Fill {
    /// Instrument type
    pub inst_type: String,
    /// Instrument ID
    pub inst_id: String,
    /// Trade ID
    pub trade_id: String,
    /// Order ID
    pub ord_id: String,
    /// Client-supplied order ID
    #[serde(default)]
    pub cl_ord_id: String,
    /// Bill ID
    #[serde(default)]
    pub bill_id: String,
    /// Order tag
    #[serde(default)]
    pub tag: String,
    /// Filled price
    pub fill_px: String,
    /// Filled quantity
    pub fill_sz: String,
    /// Filled P&L
    #[serde(default)]
    pub fill_pnl: String,
    /// Order side (buy, sell)
    pub side: String,
    /// Position side (long, short, net)
    #[serde(default)]
    pub pos_side: String,
    /// Liquidity taker or maker (T: taker, M: maker)
    #[serde(default)]
    pub exec_type: String,
    /// Fee currency
    #[serde(default)]
    pub fee_ccy: String,
    /// Fee
    #[serde(default)]
    pub fee: String,
    /// Data generation time (Unix timestamp in milliseconds)
    pub ts: String,
    /// Index price at fill time
    #[serde(default)]
    pub fill_idx_px: String,
    /// Mark price at fill time
    #[serde(default)]
    pub fill_mark_px: String,
    /// P&L at fill time
    #[serde(default)]
    pub fill_time: String,
    /// Implied volatility (for options)
    #[serde(default)]
    pub fill_mark_vol: String,
    /// Forward price at fill time (for options)
    #[serde(default)]
    pub fill_fwd_px: String,
}

/// Place order request parameters.
///
/// Source: POST /api/v5/trade/order request body
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaceOrderRequest {
    /// Instrument ID (e.g., "BTC-USDT")
    pub inst_id: String,
    /// Trade mode: cash, cross, isolated
    pub td_mode: String,
    /// Order side: buy, sell
    pub side: String,
    /// Order type: market, limit, `post_only`, fok, ioc
    pub ord_type: String,
    /// Quantity to buy or sell
    pub sz: String,
    /// Currency (for SPOT/MARGIN orders in `tgtCcy=quote_ccy` mode)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ccy: Option<String>,
    /// Client-supplied order ID (max 32 characters)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cl_ord_id: Option<String>,
    /// Order tag (max 16 characters)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    /// Position side: long, short, net
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pos_side: Option<String>,
    /// Order price (required for limit orders)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub px: Option<String>,
    /// Whether to reduce position only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reduce_only: Option<bool>,
    /// Target currency: `base_ccy`, `quote_ccy`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tgt_ccy: Option<String>,
    /// Take-profit trigger price
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tp_trigger_px: Option<String>,
    /// Take-profit order price
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tp_ord_px: Option<String>,
    /// Stop-loss trigger price
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sl_trigger_px: Option<String>,
    /// Stop-loss order price
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sl_ord_px: Option<String>,
    /// Take-profit trigger price type: last, index, mark
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tp_trigger_px_type: Option<String>,
    /// Stop-loss trigger price type: last, index, mark
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sl_trigger_px_type: Option<String>,
    /// Quick margin type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quick_mgn_type: Option<String>,
    /// Self trade prevention ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stp_id: Option<String>,
    /// Self trade prevention mode: `cancel_maker`, `cancel_taker`, `cancel_both`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stp_mode: Option<String>,
    /// Attached algo orders (TP/SL orders attached to this order)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attach_algo_ords: Option<Vec<AttachAlgoOrdRequest>>,
}

/// Attached algo order request for placing orders with TP/SL.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AttachAlgoOrdRequest {
    /// Client-supplied algo ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attach_algo_cl_ord_id: Option<String>,
    /// Take-profit trigger price
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tp_trigger_px: Option<String>,
    /// Take-profit trigger price type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tp_trigger_px_type: Option<String>,
    /// Take-profit order price
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tp_ord_px: Option<String>,
    /// Stop-loss trigger price
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sl_trigger_px: Option<String>,
    /// Stop-loss trigger price type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sl_trigger_px_type: Option<String>,
    /// Stop-loss order price
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sl_ord_px: Option<String>,
    /// Size
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sz: Option<String>,
}

/// Place order response.
///
/// Source: POST /api/v5/trade/order response data
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaceOrderResponse {
    /// Order ID
    pub ord_id: String,
    /// Client-supplied order ID
    #[serde(default)]
    pub cl_ord_id: String,
    /// Order tag
    #[serde(default)]
    pub tag: String,
    /// Response code for this order
    #[serde(default)]
    pub s_code: String,
    /// Response message for this order
    #[serde(default)]
    pub s_msg: String,
}

/// Cancel order request parameters.
///
/// Source: POST /api/v5/trade/cancel-order request body
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelOrderRequest {
    /// Instrument ID
    pub inst_id: String,
    /// Order ID (either ordId or clOrdId required)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ord_id: Option<String>,
    /// Client-supplied order ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cl_ord_id: Option<String>,
}

/// Cancel order response.
///
/// Source: POST /api/v5/trade/cancel-order response data
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelOrderResponse {
    /// Order ID
    pub ord_id: String,
    /// Client-supplied order ID
    #[serde(default)]
    pub cl_ord_id: String,
    /// Response code for this order
    #[serde(default)]
    pub s_code: String,
    /// Response message for this order
    #[serde(default)]
    pub s_msg: String,
}

/// Amend order request parameters.
///
/// Source: POST /api/v5/trade/amend-order request body
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AmendOrderRequest {
    /// Instrument ID
    pub inst_id: String,
    /// Order ID (either ordId or clOrdId required)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ord_id: Option<String>,
    /// Client-supplied order ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cl_ord_id: Option<String>,
    /// Request ID for the amendment
    #[serde(skip_serializing_if = "Option::is_none")]
    pub req_id: Option<String>,
    /// New quantity
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_sz: Option<String>,
    /// New price
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_px: Option<String>,
    /// New take-profit trigger price
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_tp_trigger_px: Option<String>,
    /// New take-profit order price
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_tp_ord_px: Option<String>,
    /// New stop-loss trigger price
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_sl_trigger_px: Option<String>,
    /// New stop-loss order price
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_sl_ord_px: Option<String>,
    /// New take-profit trigger price type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_tp_trigger_px_type: Option<String>,
    /// New stop-loss trigger price type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_sl_trigger_px_type: Option<String>,
}

/// Amend order response.
///
/// Source: POST /api/v5/trade/amend-order response data
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AmendOrderResponse {
    /// Order ID
    pub ord_id: String,
    /// Client-supplied order ID
    #[serde(default)]
    pub cl_ord_id: String,
    /// Request ID
    #[serde(default)]
    pub req_id: String,
    /// Response code for this order
    #[serde(default)]
    pub s_code: String,
    /// Response message for this order
    #[serde(default)]
    pub s_msg: String,
}

/// Place algo order request.
///
/// Source: POST /api/v5/trade/order-algo request body
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaceAlgoOrderRequest {
    /// Instrument ID
    pub inst_id: String,
    /// Trade mode: cash, cross, isolated
    pub td_mode: String,
    /// Order side: buy, sell
    pub side: String,
    /// Order type: conditional, oco, trigger, `move_order_stop`, iceberg, twap
    pub ord_type: String,
    /// Quantity
    pub sz: String,
    /// Currency
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ccy: Option<String>,
    /// Position side: long, short, net
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pos_side: Option<String>,
    /// Reduce only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reduce_only: Option<bool>,
    /// Target currency
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tgt_ccy: Option<String>,
    /// Client algo order ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub algo_cl_ord_id: Option<String>,
    /// Trigger price
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_px: Option<String>,
    /// Order price (-1 for market)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_px: Option<String>,
    /// Trigger price type: last, index, mark
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_px_type: Option<String>,
    /// Take profit trigger price
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tp_trigger_px: Option<String>,
    /// Take profit order price
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tp_ord_px: Option<String>,
    /// Take profit trigger price type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tp_trigger_px_type: Option<String>,
    /// Stop loss trigger price
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sl_trigger_px: Option<String>,
    /// Stop loss order price
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sl_ord_px: Option<String>,
    /// Stop loss trigger price type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sl_trigger_px_type: Option<String>,
    /// Trailing stop callback ratio
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_ratio: Option<String>,
    /// Trailing stop callback spread
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_spread: Option<String>,
    /// Trailing stop active price
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_px: Option<String>,
}

/// Place algo order response.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaceAlgoOrderResponse {
    /// Algo order ID
    pub algo_id: String,
    /// Client algo order ID
    #[serde(default)]
    pub algo_cl_ord_id: String,
    /// Response code
    #[serde(default)]
    pub s_code: String,
    /// Response message
    #[serde(default)]
    pub s_msg: String,
}

/// Cancel algo order request.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelAlgoOrderRequest {
    /// Instrument ID
    pub inst_id: String,
    /// Algo order ID
    pub algo_id: String,
}

/// Cancel algo order response.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelAlgoOrderResponse {
    /// Algo order ID
    pub algo_id: String,
    /// Response code
    #[serde(default)]
    pub s_code: String,
    /// Response message
    #[serde(default)]
    pub s_msg: String,
}

/// Algo order information.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlgoOrder {
    /// Instrument type
    pub inst_type: String,
    /// Instrument ID
    pub inst_id: String,
    /// Algo order ID
    pub algo_id: String,
    /// Client algo order ID
    #[serde(default)]
    pub algo_cl_ord_id: String,
    /// Order type
    pub ord_type: String,
    /// Order side
    pub side: String,
    /// Position side
    #[serde(default)]
    pub pos_side: String,
    /// Trade mode
    pub td_mode: String,
    /// Size
    pub sz: String,
    /// Currency
    #[serde(default)]
    pub ccy: String,
    /// State: live, pause, effective, canceled, `order_failed`
    pub state: String,
    /// Trigger price
    #[serde(default)]
    pub trigger_px: String,
    /// Order price
    #[serde(default)]
    pub order_px: String,
    /// Actual order price
    #[serde(default)]
    pub actual_px: String,
    /// Actual size
    #[serde(default)]
    pub actual_sz: String,
    /// Actual side
    #[serde(default)]
    pub actual_side: String,
    /// Trigger price type
    #[serde(default)]
    pub trigger_px_type: String,
    /// Take profit trigger price
    #[serde(default)]
    pub tp_trigger_px: String,
    /// Take profit order price
    #[serde(default)]
    pub tp_ord_px: String,
    /// Stop loss trigger price
    #[serde(default)]
    pub sl_trigger_px: String,
    /// Stop loss order price
    #[serde(default)]
    pub sl_ord_px: String,
    /// Trigger time
    #[serde(default)]
    pub trigger_time: String,
    /// Creation time
    #[serde(default)]
    pub c_time: String,
}
