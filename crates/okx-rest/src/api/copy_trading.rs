//! 复制交易（Copy Trading）相关接口。
//!
//! 对应 `/api/v5/copytrading/*` 端点。

use okx_core::Result;
use serde_json::Value;

use crate::OkxRestClient;

/// 端点常量。
pub mod endpoints {
    /// 获取当前带单仓位
    pub const EXISTING_LEAD_POSITIONS: &str = "/api/v5/copytrading/current-subpositions";
    /// 获取历史带单仓位
    pub const LEAD_POSITION_HISTORY: &str = "/api/v5/copytrading/subpositions-history";
    /// 带单止盈止损委托
    pub const PLACE_LEAD_STOP_ORDER: &str = "/api/v5/copytrading/algo-order";
    /// 平带单仓位
    pub const CLOSE_LEAD_POSITION: &str = "/api/v5/copytrading/close-subposition";
    /// 获取带单产品
    pub const LEADING_INSTRUMENTS: &str = "/api/v5/copytrading/instruments";
    /// 修改带单产品
    pub const AMEND_LEADING_INSTRUMENTS: &str = "/api/v5/copytrading/set-instruments";
    /// 获取利润分成明细
    pub const PROFIT_SHARING_DETAILS: &str = "/api/v5/copytrading/profit-sharing-details";
    /// 获取利润分成总额
    pub const TOTAL_PROFIT_SHARING: &str = "/api/v5/copytrading/total-profit-sharing";
    /// 获取未实现利润分成明细
    pub const UNREALIZED_PROFIT_SHARING_DETAILS: &str =
        "/api/v5/copytrading/unrealized-profit-sharing-details";
}

/// Copy Trading API。
pub trait CopyTradingApi {
    /// 获取当前带单仓位
    fn get_existing_lead_positions(
        &self,
        params: Option<Value>,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;

    /// 获取历史带单仓位
    fn get_lead_position_history(
        &self,
        params: Option<Value>,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;

    /// 带单止盈止损委托
    fn place_lead_stop_order(
        &self,
        request: Value,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;

    /// 平带单仓位
    fn close_lead_position(
        &self,
        request: Value,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;

    /// 获取带单产品
    fn get_leading_instruments(
        &self,
        params: Option<Value>,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;

    /// 修改带单产品
    fn amend_leading_instruments(
        &self,
        request: Value,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;

    /// 获取利润分成明细
    fn get_profit_sharing_details(
        &self,
        params: Option<Value>,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;

    /// 获取利润分成总额
    fn get_total_profit_sharing(
        &self,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;

    /// 获取未实现利润分成明细
    fn get_unrealized_profit_sharing_details(
        &self,
        params: Option<Value>,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;
}

impl CopyTradingApi for OkxRestClient {
    async fn get_existing_lead_positions(&self, params: Option<Value>) -> Result<Vec<Value>> {
        self.get(endpoints::EXISTING_LEAD_POSITIONS, params.as_ref())
            .await
    }

    async fn get_lead_position_history(&self, params: Option<Value>) -> Result<Vec<Value>> {
        self.get(endpoints::LEAD_POSITION_HISTORY, params.as_ref())
            .await
    }

    async fn place_lead_stop_order(&self, request: Value) -> Result<Vec<Value>> {
        self.post(endpoints::PLACE_LEAD_STOP_ORDER, &request).await
    }

    async fn close_lead_position(&self, request: Value) -> Result<Vec<Value>> {
        self.post(endpoints::CLOSE_LEAD_POSITION, &request).await
    }

    async fn get_leading_instruments(&self, params: Option<Value>) -> Result<Vec<Value>> {
        self.get(endpoints::LEADING_INSTRUMENTS, params.as_ref())
            .await
    }

    async fn amend_leading_instruments(&self, request: Value) -> Result<Vec<Value>> {
        self.post(endpoints::AMEND_LEADING_INSTRUMENTS, &request)
            .await
    }

    async fn get_profit_sharing_details(&self, params: Option<Value>) -> Result<Vec<Value>> {
        self.get(endpoints::PROFIT_SHARING_DETAILS, params.as_ref())
            .await
    }

    async fn get_total_profit_sharing(&self) -> Result<Vec<Value>> {
        self.get(endpoints::TOTAL_PROFIT_SHARING, None::<&()>).await
    }

    async fn get_unrealized_profit_sharing_details(
        &self,
        params: Option<Value>,
    ) -> Result<Vec<Value>> {
        self.get(
            endpoints::UNREALIZED_PROFIT_SHARING_DETAILS,
            params.as_ref(),
        )
        .await
    }
}
