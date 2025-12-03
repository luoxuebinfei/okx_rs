//! 复制交易（Copy Trading）相关接口。
//!
//! 对应 `/api/v5/copytrading/*` 端点。

use okx_core::Result;
use serde_json::Value;

use crate::OkxRestClient;

/// 端点常量。
pub mod endpoints {
    pub const EXISTING_LEAD_POSITIONS: &str = "/api/v5/copytrading/current-subpositions";
    pub const LEAD_POSITION_HISTORY: &str = "/api/v5/copytrading/subpositions-history";
    pub const PLACE_LEAD_STOP_ORDER: &str = "/api/v5/copytrading/algo-order";
    pub const CLOSE_LEAD_POSITION: &str = "/api/v5/copytrading/close-subposition";
    pub const LEADING_INSTRUMENTS: &str = "/api/v5/copytrading/instruments";
    pub const AMEND_LEADING_INSTRUMENTS: &str = "/api/v5/copytrading/set-instruments";
    pub const PROFIT_SHARING_DETAILS: &str = "/api/v5/copytrading/profit-sharing-details";
    pub const TOTAL_PROFIT_SHARING: &str = "/api/v5/copytrading/total-profit-sharing";
    pub const UNREALIZED_PROFIT_SHARING_DETAILS: &str =
        "/api/v5/copytrading/unrealized-profit-sharing-details";
}

/// Copy Trading API。
pub trait CopyTradingApi {
    fn get_existing_lead_positions(
        &self,
        params: Option<Value>,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;

    fn get_lead_position_history(
        &self,
        params: Option<Value>,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;

    fn place_lead_stop_order(
        &self,
        request: Value,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;

    fn close_lead_position(
        &self,
        request: Value,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;

    fn get_leading_instruments(
        &self,
        params: Option<Value>,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;

    fn amend_leading_instruments(
        &self,
        request: Value,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;

    fn get_profit_sharing_details(
        &self,
        params: Option<Value>,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;

    fn get_total_profit_sharing(
        &self,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;

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
