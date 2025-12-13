//! 网格/定投交易机器人相关接口。
//!
//! 对应 `/api/v5/tradingBot/grid/*` 与 `/api/v5/tradingBot/recurring/*`。

use okx_core::Result;
use serde_json::Value;

use crate::OkxRestClient;

/// 端点常量。
pub mod endpoints {
    /// 网格策略委托下单
    pub const GRID_ORDER_ALGO: &str = "/api/v5/tradingBot/grid/order-algo";
    /// 修改网格策略订单
    pub const GRID_AMEND_ORDER_ALGO: &str = "/api/v5/tradingBot/grid/amend-order-algo";
    /// 停止网格策略订单
    pub const GRID_STOP_ORDER_ALGO: &str = "/api/v5/tradingBot/grid/stop-order-algo";
    /// 获取未完成网格策略订单
    pub const GRID_ORDERS_ALGO_PENDING: &str = "/api/v5/tradingBot/grid/orders-algo-pending";
    /// 获取历史网格策略订单
    pub const GRID_ORDERS_ALGO_HISTORY: &str = "/api/v5/tradingBot/grid/orders-algo-history";
    /// 获取网格策略订单详情
    pub const GRID_ORDERS_ALGO_DETAILS: &str = "/api/v5/tradingBot/grid/orders-algo-details";
    /// 获取网格策略子订单信息
    pub const GRID_SUB_ORDERS: &str = "/api/v5/tradingBot/grid/sub-orders";
    /// 获取网格策略持仓
    pub const GRID_POSITIONS: &str = "/api/v5/tradingBot/grid/positions";
    /// 现货网格提取利润
    pub const GRID_WITHDRAW_INCOME: &str = "/api/v5/tradingBot/grid/withdraw-income";
    /// 计算调整保证金
    pub const GRID_COMPUTE_MARGIN_BALANCE: &str = "/api/v5/tradingBot/grid/compute-margin-balance";
    /// 调整保证金
    pub const GRID_MARGIN_BALANCE: &str = "/api/v5/tradingBot/grid/margin-balance";
    /// 获取网格 AI 参数（公共）
    pub const GRID_AI_PARAM: &str = "/api/v5/tradingBot/grid/ai-param";

    // Recurring buy
    /// 定投策略委托下单
    pub const PLACE_RECURRING_BUY_ORDER: &str = "/api/v5/tradingBot/recurring/order-algo";
    /// 修改定投策略订单
    pub const AMEND_RECURRING_BUY_ORDER: &str = "/api/v5/tradingBot/recurring/amend-order-algo";
    /// 停止定投策略订单
    pub const STOP_RECURRING_BUY_ORDER: &str = "/api/v5/tradingBot/recurring/stop-order-algo";
    /// 获取未完成定投策略订单
    pub const GET_RECURRING_BUY_ORDER_LIST: &str =
        "/api/v5/tradingBot/recurring/orders-algo-pending";
    /// 获取历史定投策略订单
    pub const GET_RECURRING_BUY_ORDER_HISTORY: &str =
        "/api/v5/tradingBot/recurring/orders-algo-history";
    /// 获取定投策略订单详情
    pub const GET_RECURRING_BUY_ORDER_DETAILS: &str =
        "/api/v5/tradingBot/recurring/orders-algo-details";
    /// 获取定投策略子订单信息
    pub const GET_RECURRING_BUY_SUB_ORDERS: &str = "/api/v5/tradingBot/recurring/sub-orders";
}

/// 网格/定投 API。
pub trait GridApi {
    /// 网格策略委托下单
    fn grid_order_algo(
        &self,
        request: Value,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;
    /// 修改网格策略订单
    fn grid_amend_order_algo(
        &self,
        request: Value,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;
    /// 停止网格策略订单
    fn grid_stop_order_algo(
        &self,
        request: Value,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;
    /// 获取未完成网格策略订单
    fn grid_orders_algo_pending(
        &self,
        params: Option<Value>,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;
    /// 获取历史网格策略订单
    fn grid_orders_algo_history(
        &self,
        params: Option<Value>,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;
    /// 获取网格策略订单详情
    fn grid_orders_algo_details(
        &self,
        params: Value,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;
    /// 获取网格策略子订单信息
    fn grid_sub_orders(
        &self,
        params: Value,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;
    /// 获取网格策略持仓
    fn grid_positions(
        &self,
        params: Option<Value>,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;
    /// 现货网格提取利润
    fn grid_withdraw_income(
        &self,
        request: Value,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;
    /// 计算调整保证金
    fn grid_compute_margin_balance(
        &self,
        request: Value,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;
    /// 调整保证金
    fn grid_margin_balance(
        &self,
        request: Value,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;
    /// 获取网格 AI 参数（公共）
    fn grid_ai_param(
        &self,
        params: Option<Value>,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;

    // Recurring buy
    /// 定投策略委托下单
    fn place_recurring_buy_order(
        &self,
        request: Value,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;
    /// 修改定投策略订单
    fn amend_recurring_buy_order(
        &self,
        request: Value,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;
    /// 停止定投策略订单
    fn stop_recurring_buy_order(
        &self,
        request: Value,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;
    /// 获取未完成定投策略订单
    fn get_recurring_buy_order_list(
        &self,
        params: Option<Value>,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;
    /// 获取历史定投策略订单
    fn get_recurring_buy_order_history(
        &self,
        params: Option<Value>,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;
    /// 获取定投策略订单详情
    fn get_recurring_buy_order_details(
        &self,
        params: Value,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;
    /// 获取定投策略子订单信息
    fn get_recurring_buy_sub_orders(
        &self,
        params: Value,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;
}

impl GridApi for OkxRestClient {
    async fn grid_order_algo(&self, request: Value) -> Result<Vec<Value>> {
        self.post(endpoints::GRID_ORDER_ALGO, &request).await
    }

    async fn grid_amend_order_algo(&self, request: Value) -> Result<Vec<Value>> {
        self.post(endpoints::GRID_AMEND_ORDER_ALGO, &request).await
    }

    async fn grid_stop_order_algo(&self, request: Value) -> Result<Vec<Value>> {
        self.post(endpoints::GRID_STOP_ORDER_ALGO, &request).await
    }

    async fn grid_orders_algo_pending(&self, params: Option<Value>) -> Result<Vec<Value>> {
        self.get(endpoints::GRID_ORDERS_ALGO_PENDING, params.as_ref())
            .await
    }

    async fn grid_orders_algo_history(&self, params: Option<Value>) -> Result<Vec<Value>> {
        self.get(endpoints::GRID_ORDERS_ALGO_HISTORY, params.as_ref())
            .await
    }

    async fn grid_orders_algo_details(&self, params: Value) -> Result<Vec<Value>> {
        self.get(endpoints::GRID_ORDERS_ALGO_DETAILS, Some(&params))
            .await
    }

    async fn grid_sub_orders(&self, params: Value) -> Result<Vec<Value>> {
        self.get(endpoints::GRID_SUB_ORDERS, Some(&params)).await
    }

    async fn grid_positions(&self, params: Option<Value>) -> Result<Vec<Value>> {
        self.get(endpoints::GRID_POSITIONS, params.as_ref()).await
    }

    async fn grid_withdraw_income(&self, request: Value) -> Result<Vec<Value>> {
        self.post(endpoints::GRID_WITHDRAW_INCOME, &request).await
    }

    async fn grid_compute_margin_balance(&self, request: Value) -> Result<Vec<Value>> {
        self.post(endpoints::GRID_COMPUTE_MARGIN_BALANCE, &request)
            .await
    }

    async fn grid_margin_balance(&self, request: Value) -> Result<Vec<Value>> {
        self.post(endpoints::GRID_MARGIN_BALANCE, &request).await
    }

    async fn grid_ai_param(&self, params: Option<Value>) -> Result<Vec<Value>> {
        self.get(endpoints::GRID_AI_PARAM, params.as_ref()).await
    }

    async fn place_recurring_buy_order(&self, request: Value) -> Result<Vec<Value>> {
        self.post(endpoints::PLACE_RECURRING_BUY_ORDER, &request)
            .await
    }

    async fn amend_recurring_buy_order(&self, request: Value) -> Result<Vec<Value>> {
        self.post(endpoints::AMEND_RECURRING_BUY_ORDER, &request)
            .await
    }

    async fn stop_recurring_buy_order(&self, request: Value) -> Result<Vec<Value>> {
        self.post(endpoints::STOP_RECURRING_BUY_ORDER, &request)
            .await
    }

    async fn get_recurring_buy_order_list(&self, params: Option<Value>) -> Result<Vec<Value>> {
        self.get(endpoints::GET_RECURRING_BUY_ORDER_LIST, params.as_ref())
            .await
    }

    async fn get_recurring_buy_order_history(&self, params: Option<Value>) -> Result<Vec<Value>> {
        self.get(endpoints::GET_RECURRING_BUY_ORDER_HISTORY, params.as_ref())
            .await
    }

    async fn get_recurring_buy_order_details(&self, params: Value) -> Result<Vec<Value>> {
        self.get(endpoints::GET_RECURRING_BUY_ORDER_DETAILS, Some(&params))
            .await
    }

    async fn get_recurring_buy_sub_orders(&self, params: Value) -> Result<Vec<Value>> {
        self.get(endpoints::GET_RECURRING_BUY_SUB_ORDERS, Some(&params))
            .await
    }
}
