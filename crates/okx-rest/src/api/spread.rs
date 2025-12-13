//! 价差交易（Spread Trading）相关接口。
//!
//! 对应官方 `/api/v5/sprd/*` 路径。

use okx_core::Result;
use serde_json::Value;

use crate::OkxRestClient;

/// 端点常量。
pub mod endpoints {
    /// 下单
    pub const PLACE_ORDER: &str = "/api/v5/sprd/order";
    /// 撤单
    pub const CANCEL_ORDER: &str = "/api/v5/sprd/cancel-order";
    /// 撤销所有订单
    pub const CANCEL_ALL_ORDERS: &str = "/api/v5/sprd/mass-cancel";
    /// 获取订单详情
    pub const ORDER_DETAILS: &str = "/api/v5/sprd/order";
    /// 获取未完成订单
    pub const ACTIVE_ORDERS: &str = "/api/v5/sprd/orders-pending";
    /// 获取历史订单
    pub const ORDERS: &str = "/api/v5/sprd/orders-history";
    /// 获取成交记录
    pub const TRADES: &str = "/api/v5/sprd/trades";
    /// 获取可交易价差
    pub const SPREADS: &str = "/api/v5/sprd/spreads";
    /// 获取价差深度
    pub const ORDER_BOOK: &str = "/api/v5/sprd/books";
    /// 获取价差行情
    pub const TICKER: &str = "/api/v5/sprd/ticker";
    /// 获取公共成交记录
    pub const PUBLIC_TRADES: &str = "/api/v5/sprd/public-trades";
}

/// Spread Trading API。
pub trait SpreadApi {
    /// 下单
    fn spread_place_order(
        &self,
        request: Value,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;

    /// 撤单
    fn spread_cancel_order(
        &self,
        request: Value,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;

    /// 全量撤单
    fn spread_cancel_all_orders(
        &self,
        request: Value,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;

    /// 订单详情
    fn spread_get_order_details(
        &self,
        params: Value,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;

    /// 活跃订单
    fn spread_get_active_orders(
        &self,
        params: Option<Value>,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;

    /// 订单历史
    fn spread_get_orders(
        &self,
        params: Option<Value>,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;

    /// 成交
    fn spread_get_trades(
        &self,
        params: Option<Value>,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;

    /// 价差信息（公共）
    fn spread_get_spreads(
        &self,
        params: Option<Value>,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;

    /// 订单簿（公共）
    fn spread_get_order_book(
        &self,
        params: Value,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;

    /// 行情（公共）
    fn spread_get_ticker(
        &self,
        params: Value,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;

    /// 公共成交（公共）
    fn spread_get_public_trades(
        &self,
        params: Value,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;
}

impl SpreadApi for OkxRestClient {
    async fn spread_place_order(&self, request: Value) -> Result<Vec<Value>> {
        self.post(endpoints::PLACE_ORDER, &request).await
    }

    async fn spread_cancel_order(&self, request: Value) -> Result<Vec<Value>> {
        self.post(endpoints::CANCEL_ORDER, &request).await
    }

    async fn spread_cancel_all_orders(&self, request: Value) -> Result<Vec<Value>> {
        self.post(endpoints::CANCEL_ALL_ORDERS, &request).await
    }

    async fn spread_get_order_details(&self, params: Value) -> Result<Vec<Value>> {
        self.get(endpoints::ORDER_DETAILS, Some(&params)).await
    }

    async fn spread_get_active_orders(&self, params: Option<Value>) -> Result<Vec<Value>> {
        self.get(endpoints::ACTIVE_ORDERS, params.as_ref()).await
    }

    async fn spread_get_orders(&self, params: Option<Value>) -> Result<Vec<Value>> {
        self.get(endpoints::ORDERS, params.as_ref()).await
    }

    async fn spread_get_trades(&self, params: Option<Value>) -> Result<Vec<Value>> {
        self.get(endpoints::TRADES, params.as_ref()).await
    }

    async fn spread_get_spreads(&self, params: Option<Value>) -> Result<Vec<Value>> {
        self.get_public(endpoints::SPREADS, params.as_ref()).await
    }

    async fn spread_get_order_book(&self, params: Value) -> Result<Vec<Value>> {
        self.get_public(endpoints::ORDER_BOOK, Some(&params)).await
    }

    async fn spread_get_ticker(&self, params: Value) -> Result<Vec<Value>> {
        self.get_public(endpoints::TICKER, Some(&params)).await
    }

    async fn spread_get_public_trades(&self, params: Value) -> Result<Vec<Value>> {
        self.get_public(endpoints::PUBLIC_TRADES, Some(&params))
            .await
    }
}
