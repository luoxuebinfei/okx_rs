//! Trade API endpoints.
//!
//! Source: OKX API v5 Order Book Trading REST API
//! - <https://www.okx.com/docs-v5/en/#order-book-trading-trade>

use serde::Serialize;
use serde_json::Value;

use okx_core::{
    types::{
        AlgoOrder, AmendOrderRequest, AmendOrderResponse, CancelAlgoOrderRequest,
        CancelAlgoOrderResponse, CancelOrderRequest, CancelOrderResponse, Fill, Order,
        PlaceAlgoOrderRequest, PlaceAlgoOrderResponse, PlaceOrderRequest, PlaceOrderResponse,
    },
    Result,
};

use crate::OkxRestClient;

/// API endpoints for trade operations.
pub mod endpoints {
    /// Place order
    pub const PLACE_ORDER: &str = "/api/v5/trade/order";
    /// Place multiple orders
    pub const PLACE_BATCH_ORDERS: &str = "/api/v5/trade/batch-orders";
    /// Cancel order
    pub const CANCEL_ORDER: &str = "/api/v5/trade/cancel-order";
    /// Cancel multiple orders
    pub const CANCEL_BATCH_ORDERS: &str = "/api/v5/trade/cancel-batch-orders";
    /// Amend order
    pub const AMEND_ORDER: &str = "/api/v5/trade/amend-order";
    /// Amend multiple orders
    pub const AMEND_BATCH_ORDERS: &str = "/api/v5/trade/amend-batch-orders";
    /// Get order details
    pub const GET_ORDER: &str = "/api/v5/trade/order";
    /// Get pending orders
    pub const ORDERS_PENDING: &str = "/api/v5/trade/orders-pending";
    /// Get order history (last 7 days)
    pub const ORDERS_HISTORY: &str = "/api/v5/trade/orders-history";
    /// Get order history archive (last 3 months)
    pub const ORDERS_HISTORY_ARCHIVE: &str = "/api/v5/trade/orders-history-archive";
    /// Get fills (last 3 days)
    pub const FILLS: &str = "/api/v5/trade/fills";
    /// Get fills history (last 3 months)
    pub const FILLS_HISTORY: &str = "/api/v5/trade/fills-history";
    /// Place algo order
    pub const PLACE_ALGO_ORDER: &str = "/api/v5/trade/order-algo";
    /// Cancel algo orders
    pub const CANCEL_ALGO_ORDERS: &str = "/api/v5/trade/cancel-algos";
    /// Amend algo order
    pub const AMEND_ALGO_ORDER: &str = "/api/v5/trade/amend-algos";
    /// Get pending algo orders
    pub const ALGO_ORDERS_PENDING: &str = "/api/v5/trade/orders-algo-pending";
    /// Get algo order history
    pub const ALGO_ORDERS_HISTORY: &str = "/api/v5/trade/orders-algo-history";
    /// Close position
    pub const CLOSE_POSITION: &str = "/api/v5/trade/close-position";
    /// Get algo order details
    pub const ALGO_ORDER_DETAILS: &str = "/api/v5/trade/order-algo";
    /// Mass cancel
    pub const MASS_CANCEL: &str = "/api/v5/trade/mass-cancel";
    /// Cancel all after
    pub const CANCEL_ALL_AFTER: &str = "/api/v5/trade/cancel-all-after";
    /// Order precheck
    pub const ORDER_PRECHECK: &str = "/api/v5/trade/order-precheck";
    /// Get one-click repay currency list v2
    pub const ONE_CLICK_REPAY_CURRENCY_LIST_V2: &str =
        "/api/v5/trade/one-click-repay-currency-list-v2";
    /// One-click repay v2
    pub const ONE_CLICK_REPAY_V2: &str = "/api/v5/trade/one-click-repay-v2";
    /// Get one-click repay history v2
    pub const ONE_CLICK_REPAY_HISTORY_V2: &str = "/api/v5/trade/one-click-repay-history-v2";
}

/// Query parameters for get_order.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetOrderParams {
    /// Instrument ID
    pub inst_id: String,
    /// Order ID (either ordId or clOrdId required)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ord_id: Option<String>,
    /// Client-supplied order ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cl_ord_id: Option<String>,
}

/// Query parameters for get_orders_pending.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetOrdersPendingParams {
    /// Instrument type: SPOT, MARGIN, SWAP, FUTURES, OPTION
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inst_type: Option<String>,
    /// Underlying
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uly: Option<String>,
    /// Instrument family
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inst_family: Option<String>,
    /// Instrument ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inst_id: Option<String>,
    /// Order type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ord_type: Option<String>,
    /// Order state
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// Pagination: order ID to start from
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// Pagination: order ID to end at
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    /// Number of results (max 100, default 100)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,
}

/// Query parameters for get_orders_history.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetOrdersHistoryParams {
    /// Instrument type: SPOT, MARGIN, SWAP, FUTURES, OPTION
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
    /// Order type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ord_type: Option<String>,
    /// Order state: canceled, filled
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// Category: twap, adl, full_liquidation, partial_liquidation, delivery, ddh
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    /// Pagination: order ID to start from
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// Pagination: order ID to end at
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    /// Start timestamp (Unix ms)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub begin: Option<String>,
    /// End timestamp (Unix ms)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<String>,
    /// Number of results (max 100, default 100)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,
}

/// Query parameters for get_fills.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetFillsParams {
    /// Instrument type: SPOT, MARGIN, SWAP, FUTURES, OPTION
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inst_type: Option<String>,
    /// Underlying
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uly: Option<String>,
    /// Instrument family
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inst_family: Option<String>,
    /// Instrument ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inst_id: Option<String>,
    /// Order ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ord_id: Option<String>,
    /// Pagination: bill ID to start from
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// Pagination: bill ID to end at
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    /// Start timestamp (Unix ms)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub begin: Option<String>,
    /// End timestamp (Unix ms)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<String>,
    /// Number of results (max 100, default 100)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,
}

/// Query parameters for get_fills_history.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetFillsHistoryParams {
    /// Instrument type: SPOT, MARGIN, SWAP, FUTURES, OPTION
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
    /// Order ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ord_id: Option<String>,
    /// Pagination: bill ID to start from
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// Pagination: bill ID to end at
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    /// Number of results (max 100, default 100)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,
}

/// Query parameters for get_orders_history_archive.
pub type GetOrdersHistoryArchiveParams = GetOrdersHistoryParams;

/// Request body for amend_algo_order.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AmendAlgoOrderRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inst_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub algo_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub algo_cl_ord_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cxl_on_fail: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub req_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_sz: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_tp_trigger_px: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_tp_ord_px: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_sl_trigger_px: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_sl_ord_px: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_tp_trigger_px_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_sl_trigger_px_type: Option<String>,
}

/// Query parameters for algo order details.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAlgoOrderDetailsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub algo_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub algo_cl_ord_id: Option<String>,
}

/// Trade API trait for OKX REST client.
///
/// Provides methods for order management.
pub trait TradeApi {
    /// Place a new order.
    ///
    /// ## API Details
    ///
    /// - Endpoint: POST /api/v5/trade/order
    /// - Rate limit: 60 requests per 2 seconds
    /// - Permission: Trade
    fn place_order(
        &self,
        request: PlaceOrderRequest,
    ) -> impl std::future::Future<Output = Result<Vec<PlaceOrderResponse>>> + Send;

    /// Place multiple orders (up to 20).
    ///
    /// ## API Details
    ///
    /// - Endpoint: POST /api/v5/trade/batch-orders
    /// - Rate limit: 300 requests per 2 seconds
    /// - Permission: Trade
    fn place_batch_orders(
        &self,
        requests: Vec<PlaceOrderRequest>,
    ) -> impl std::future::Future<Output = Result<Vec<PlaceOrderResponse>>> + Send;

    /// Cancel an order.
    ///
    /// ## API Details
    ///
    /// - Endpoint: POST /api/v5/trade/cancel-order
    /// - Rate limit: 60 requests per 2 seconds
    /// - Permission: Trade
    fn cancel_order(
        &self,
        request: CancelOrderRequest,
    ) -> impl std::future::Future<Output = Result<Vec<CancelOrderResponse>>> + Send;

    /// Cancel multiple orders (up to 20).
    ///
    /// ## API Details
    ///
    /// - Endpoint: POST /api/v5/trade/cancel-batch-orders
    /// - Rate limit: 300 requests per 2 seconds
    /// - Permission: Trade
    fn cancel_batch_orders(
        &self,
        requests: Vec<CancelOrderRequest>,
    ) -> impl std::future::Future<Output = Result<Vec<CancelOrderResponse>>> + Send;

    /// Amend an order.
    ///
    /// ## API Details
    ///
    /// - Endpoint: POST /api/v5/trade/amend-order
    /// - Rate limit: 60 requests per 2 seconds
    /// - Permission: Trade
    fn amend_order(
        &self,
        request: AmendOrderRequest,
    ) -> impl std::future::Future<Output = Result<Vec<AmendOrderResponse>>> + Send;

    /// Amend multiple orders in a single request.
    ///
    /// ## API Details
    ///
    /// - Endpoint: POST /api/v5/trade/amend-batch-orders
    /// - Rate limit: 60 requests per 2 seconds
    /// - Permission: Trade
    fn amend_batch_orders(
        &self,
        requests: Vec<AmendOrderRequest>,
    ) -> impl std::future::Future<Output = Result<Vec<AmendOrderResponse>>> + Send;

    /// Get order details.
    ///
    /// ## API Details
    ///
    /// - Endpoint: GET /api/v5/trade/order
    /// - Rate limit: 60 requests per 2 seconds
    /// - Permission: Read
    fn get_order(
        &self,
        params: GetOrderParams,
    ) -> impl std::future::Future<Output = Result<Vec<Order>>> + Send;

    /// Get pending orders.
    ///
    /// ## API Details
    ///
    /// - Endpoint: GET /api/v5/trade/orders-pending
    /// - Rate limit: 60 requests per 2 seconds
    /// - Permission: Read
    fn get_orders_pending(
        &self,
        params: Option<GetOrdersPendingParams>,
    ) -> impl std::future::Future<Output = Result<Vec<Order>>> + Send;

    /// Get order history (last 7 days).
    ///
    /// ## API Details
    ///
    /// - Endpoint: GET /api/v5/trade/orders-history
    /// - Rate limit: 40 requests per 2 seconds
    /// - Permission: Read
    fn get_orders_history(
        &self,
        params: GetOrdersHistoryParams,
    ) -> impl std::future::Future<Output = Result<Vec<Order>>> + Send;

    /// Get order history archive (last 3 months).
    fn get_orders_history_archive(
        &self,
        params: GetOrdersHistoryArchiveParams,
    ) -> impl std::future::Future<Output = Result<Vec<Order>>> + Send;

    /// Get fills (last 3 days).
    ///
    /// ## API Details
    ///
    /// - Endpoint: GET /api/v5/trade/fills
    /// - Rate limit: 60 requests per 2 seconds
    /// - Permission: Read
    fn get_fills(
        &self,
        params: Option<GetFillsParams>,
    ) -> impl std::future::Future<Output = Result<Vec<Fill>>> + Send;

    /// Get fills history (last 3 months).
    fn get_fills_history(
        &self,
        params: GetFillsHistoryParams,
    ) -> impl std::future::Future<Output = Result<Vec<Fill>>> + Send;

    /// Place an algo order.
    ///
    /// ## API Details
    ///
    /// - Endpoint: POST /api/v5/trade/order-algo
    /// - Rate limit: 20 requests per 2 seconds
    /// - Permission: Trade
    fn place_algo_order(
        &self,
        request: PlaceAlgoOrderRequest,
    ) -> impl std::future::Future<Output = Result<Vec<PlaceAlgoOrderResponse>>> + Send;

    /// Cancel algo orders.
    ///
    /// ## API Details
    ///
    /// - Endpoint: POST /api/v5/trade/cancel-algos
    /// - Rate limit: 20 requests per 2 seconds
    /// - Permission: Trade
    fn cancel_algo_orders(
        &self,
        requests: Vec<CancelAlgoOrderRequest>,
    ) -> impl std::future::Future<Output = Result<Vec<CancelAlgoOrderResponse>>> + Send;

    /// Amend an existing algo order.
    fn amend_algo_order(
        &self,
        request: AmendAlgoOrderRequest,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;

    /// Get pending algo orders.
    ///
    /// ## API Details
    ///
    /// - Endpoint: GET /api/v5/trade/orders-algo-pending
    /// - Rate limit: 20 requests per 2 seconds
    /// - Permission: Read
    fn get_algo_orders_pending(
        &self,
        params: GetAlgoOrdersParams,
    ) -> impl std::future::Future<Output = Result<Vec<AlgoOrder>>> + Send;

    /// Get algo order history.
    ///
    /// ## API Details
    ///
    /// - Endpoint: GET /api/v5/trade/orders-algo-history
    /// - Rate limit: 20 requests per 2 seconds
    /// - Permission: Read
    fn get_algo_orders_history(
        &self,
        params: GetAlgoOrdersHistoryParams,
    ) -> impl std::future::Future<Output = Result<Vec<AlgoOrder>>> + Send;

    /// Get algo order details.
    fn get_algo_order_details(
        &self,
        params: GetAlgoOrderDetailsParams,
    ) -> impl std::future::Future<Output = Result<Vec<AlgoOrder>>> + Send;

    /// Close position.
    ///
    /// ## API Details
    ///
    /// - Endpoint: POST /api/v5/trade/close-position
    /// - Rate limit: 20 requests per 2 seconds
    /// - Permission: Trade
    fn close_position(
        &self,
        request: ClosePositionRequest,
    ) -> impl std::future::Future<Output = Result<Vec<ClosePositionResponse>>> + Send;

    /// Mass cancel orders (全量撤单)。
    fn mass_cancel(
        &self,
        request: serde_json::Value,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;

    /// Cancel all after（定时全撤）。
    fn cancel_all_after(
        &self,
        request: serde_json::Value,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;

    /// Order 预检查。
    fn order_precheck(
        &self,
        request: serde_json::Value,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;

    /// Get one-click repay currency list v2.
    fn get_one_click_repay_currency_list_v2(
        &self,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;

    /// One-click repay v2.
    fn one_click_repay_v2(
        &self,
        request: OneClickRepayV2Request,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;

    /// Get one-click repay history v2.
    fn get_one_click_repay_history_v2(
        &self,
        params: Option<OneClickRepayHistoryV2Params>,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;
}

/// Request for one-click repay v2.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OneClickRepayV2Request {
    pub debt_ccy: String,
    pub repay_ccy_list: Vec<String>,
}

/// Query parameters for one-click repay history v2.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OneClickRepayHistoryV2Params {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,
}

/// Query parameters for get_algo_orders_pending.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAlgoOrdersParams {
    /// Order type: conditional, oco, trigger, move_order_stop, iceberg, twap
    pub ord_type: String,
    /// Algo order ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub algo_id: Option<String>,
    /// Instrument type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inst_type: Option<String>,
    /// Instrument ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inst_id: Option<String>,
    /// Pagination: algo ID to start from
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// Pagination: algo ID to end at
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    /// Number of results (max 100, default 100)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,
}

/// Query parameters for get_algo_orders_history.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAlgoOrdersHistoryParams {
    /// Order type: conditional, oco, trigger, move_order_stop, iceberg, twap
    pub ord_type: String,
    /// State: effective, canceled, order_failed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// Algo order ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub algo_id: Option<String>,
    /// Instrument type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inst_type: Option<String>,
    /// Instrument ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inst_id: Option<String>,
    /// Pagination: algo ID to start from
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// Pagination: algo ID to end at
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    /// Number of results (max 100, default 100)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,
}

/// Request for close_position.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ClosePositionRequest {
    /// Instrument ID
    pub inst_id: String,
    /// Margin mode: cross, isolated
    pub mgn_mode: String,
    /// Position side: long, short, net
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pos_side: Option<String>,
    /// Currency
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ccy: Option<String>,
    /// Auto cancel
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_cancel: Option<bool>,
    /// Client order ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cl_ord_id: Option<String>,
    /// Tag
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

/// Response for close_position.
#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClosePositionResponse {
    /// Instrument ID
    pub inst_id: String,
    /// Position side
    #[serde(default)]
    pub pos_side: String,
    /// Client order ID
    #[serde(default)]
    pub cl_ord_id: String,
    /// Tag
    #[serde(default)]
    pub tag: String,
}

impl TradeApi for OkxRestClient {
    async fn place_order(&self, request: PlaceOrderRequest) -> Result<Vec<PlaceOrderResponse>> {
        self.post(endpoints::PLACE_ORDER, &request).await
    }

    async fn place_batch_orders(
        &self,
        requests: Vec<PlaceOrderRequest>,
    ) -> Result<Vec<PlaceOrderResponse>> {
        self.post(endpoints::PLACE_BATCH_ORDERS, &requests).await
    }

    async fn cancel_order(&self, request: CancelOrderRequest) -> Result<Vec<CancelOrderResponse>> {
        self.post(endpoints::CANCEL_ORDER, &request).await
    }

    async fn cancel_batch_orders(
        &self,
        requests: Vec<CancelOrderRequest>,
    ) -> Result<Vec<CancelOrderResponse>> {
        self.post(endpoints::CANCEL_BATCH_ORDERS, &requests).await
    }

    async fn amend_order(&self, request: AmendOrderRequest) -> Result<Vec<AmendOrderResponse>> {
        self.post(endpoints::AMEND_ORDER, &request).await
    }

    async fn amend_batch_orders(
        &self,
        requests: Vec<AmendOrderRequest>,
    ) -> Result<Vec<AmendOrderResponse>> {
        self.post(endpoints::AMEND_BATCH_ORDERS, &requests).await
    }

    async fn get_order(&self, params: GetOrderParams) -> Result<Vec<Order>> {
        self.get(endpoints::GET_ORDER, Some(&params)).await
    }

    async fn get_orders_pending(
        &self,
        params: Option<GetOrdersPendingParams>,
    ) -> Result<Vec<Order>> {
        self.get(endpoints::ORDERS_PENDING, params.as_ref()).await
    }

    async fn get_orders_history(&self, params: GetOrdersHistoryParams) -> Result<Vec<Order>> {
        self.get(endpoints::ORDERS_HISTORY, Some(&params)).await
    }

    async fn get_orders_history_archive(
        &self,
        params: GetOrdersHistoryArchiveParams,
    ) -> Result<Vec<Order>> {
        self.get(endpoints::ORDERS_HISTORY_ARCHIVE, Some(&params))
            .await
    }

    async fn get_fills(&self, params: Option<GetFillsParams>) -> Result<Vec<Fill>> {
        self.get(endpoints::FILLS, params.as_ref()).await
    }

    async fn get_fills_history(&self, params: GetFillsHistoryParams) -> Result<Vec<Fill>> {
        self.get(endpoints::FILLS_HISTORY, Some(&params)).await
    }

    async fn place_algo_order(
        &self,
        request: PlaceAlgoOrderRequest,
    ) -> Result<Vec<PlaceAlgoOrderResponse>> {
        self.post(endpoints::PLACE_ALGO_ORDER, &request).await
    }

    async fn cancel_algo_orders(
        &self,
        requests: Vec<CancelAlgoOrderRequest>,
    ) -> Result<Vec<CancelAlgoOrderResponse>> {
        self.post(endpoints::CANCEL_ALGO_ORDERS, &requests).await
    }

    async fn amend_algo_order(&self, request: AmendAlgoOrderRequest) -> Result<Vec<Value>> {
        self.post(endpoints::AMEND_ALGO_ORDER, &request).await
    }

    async fn get_algo_orders_pending(&self, params: GetAlgoOrdersParams) -> Result<Vec<AlgoOrder>> {
        self.get(endpoints::ALGO_ORDERS_PENDING, Some(&params))
            .await
    }

    async fn get_algo_orders_history(
        &self,
        params: GetAlgoOrdersHistoryParams,
    ) -> Result<Vec<AlgoOrder>> {
        self.get(endpoints::ALGO_ORDERS_HISTORY, Some(&params))
            .await
    }

    async fn get_algo_order_details(
        &self,
        params: GetAlgoOrderDetailsParams,
    ) -> Result<Vec<AlgoOrder>> {
        self.get(endpoints::ALGO_ORDER_DETAILS, Some(&params)).await
    }

    async fn close_position(
        &self,
        request: ClosePositionRequest,
    ) -> Result<Vec<ClosePositionResponse>> {
        self.post(endpoints::CLOSE_POSITION, &request).await
    }

    async fn mass_cancel(&self, request: serde_json::Value) -> Result<Vec<Value>> {
        self.post(endpoints::MASS_CANCEL, &request).await
    }

    async fn cancel_all_after(&self, request: serde_json::Value) -> Result<Vec<Value>> {
        self.post(endpoints::CANCEL_ALL_AFTER, &request).await
    }

    async fn order_precheck(&self, request: serde_json::Value) -> Result<Vec<Value>> {
        self.post(endpoints::ORDER_PRECHECK, &request).await
    }

    async fn get_one_click_repay_currency_list_v2(&self) -> Result<Vec<Value>> {
        self.get(endpoints::ONE_CLICK_REPAY_CURRENCY_LIST_V2, None::<&()>)
            .await
    }

    async fn one_click_repay_v2(&self, request: OneClickRepayV2Request) -> Result<Vec<Value>> {
        self.post(endpoints::ONE_CLICK_REPAY_V2, &request).await
    }

    async fn get_one_click_repay_history_v2(
        &self,
        params: Option<OneClickRepayHistoryV2Params>,
    ) -> Result<Vec<Value>> {
        self.get(endpoints::ONE_CLICK_REPAY_HISTORY_V2, params.as_ref())
            .await
    }
}
