//! 大宗交易 / RFQ 相关接口。
//!
//! 对应 `/api/v5/rfq/*` 端点，包含 RFQ、Quote、成交、MMP 配置等。

use okx_core::Result;
use serde_json::Value;

use crate::OkxRestClient;

/// 端点常量。
pub mod endpoints {
    pub const COUNTERPARTIES: &str = "/api/v5/rfq/counterparties";
    pub const CREATE_RFQ: &str = "/api/v5/rfq/create-rfq";
    pub const CANCEL_RFQ: &str = "/api/v5/rfq/cancel-rfq";
    pub const CANCEL_BATCH_RFQS: &str = "/api/v5/rfq/cancel-batch-rfqs";
    pub const CANCEL_ALL_RFQS: &str = "/api/v5/rfq/cancel-all-rfqs";
    pub const EXECUTE_QUOTE: &str = "/api/v5/rfq/execute-quote";
    pub const CREATE_QUOTE: &str = "/api/v5/rfq/create-quote";
    pub const CANCEL_QUOTE: &str = "/api/v5/rfq/cancel-quote";
    pub const CANCEL_BATCH_QUOTES: &str = "/api/v5/rfq/cancel-batch-quotes";
    pub const CANCEL_ALL_QUOTES: &str = "/api/v5/rfq/cancel-all-quotes";
    pub const GET_RFQS: &str = "/api/v5/rfq/rfqs";
    pub const GET_QUOTES: &str = "/api/v5/rfq/quotes";
    pub const GET_TRADES: &str = "/api/v5/rfq/trades";
    pub const GET_PUBLIC_TRADES: &str = "/api/v5/rfq/public-trades";
    pub const RESET_MMP: &str = "/api/v5/rfq/mmp-reset";
    pub const SET_MMP: &str = "/api/v5/rfq/mmp-config";
    pub const GET_MMP_CONFIG: &str = "/api/v5/rfq/mmp-config";
    pub const SET_MARKER_INSTRUMENT: &str = "/api/v5/rfq/maker-instrument-settings";
    pub const GET_QUOTE_PRODUCTS: &str = "/api/v5/rfq/quote-products";
}

/// RFQ / Block Trading API。
pub trait BlockRfqApi {
    fn get_counterparties(&self) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;

    fn create_rfq(
        &self,
        request: Value,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;

    fn cancel_rfq(
        &self,
        request: Value,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;

    fn cancel_batch_rfqs(
        &self,
        request: Value,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;

    fn cancel_all_rfqs(
        &self,
        request: Value,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;

    fn execute_quote(
        &self,
        request: Value,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;

    fn create_quote(
        &self,
        request: Value,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;

    fn cancel_quote(
        &self,
        request: Value,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;

    fn cancel_batch_quotes(
        &self,
        request: Value,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;

    fn cancel_all_quotes(
        &self,
        request: Value,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;

    fn get_rfqs(
        &self,
        params: Option<Value>,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;

    fn get_quotes(
        &self,
        params: Option<Value>,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;

    fn get_trades(
        &self,
        params: Option<Value>,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;

    fn get_public_trades(
        &self,
        params: Option<Value>,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;

    fn reset_mmp(
        &self,
        request: Value,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;

    fn set_mmp_config(
        &self,
        request: Value,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;

    fn get_mmp_config(&self) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;

    fn set_marker_instrument(
        &self,
        request: Value,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;

    fn get_quote_products(
        &self,
        params: Option<Value>,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;
}

impl BlockRfqApi for OkxRestClient {
    async fn get_counterparties(&self) -> Result<Vec<Value>> {
        self.get(endpoints::COUNTERPARTIES, None::<&()>).await
    }

    async fn create_rfq(&self, request: Value) -> Result<Vec<Value>> {
        self.post(endpoints::CREATE_RFQ, &request).await
    }

    async fn cancel_rfq(&self, request: Value) -> Result<Vec<Value>> {
        self.post(endpoints::CANCEL_RFQ, &request).await
    }

    async fn cancel_batch_rfqs(&self, request: Value) -> Result<Vec<Value>> {
        self.post(endpoints::CANCEL_BATCH_RFQS, &request).await
    }

    async fn cancel_all_rfqs(&self, request: Value) -> Result<Vec<Value>> {
        self.post(endpoints::CANCEL_ALL_RFQS, &request).await
    }

    async fn execute_quote(&self, request: Value) -> Result<Vec<Value>> {
        self.post(endpoints::EXECUTE_QUOTE, &request).await
    }

    async fn create_quote(&self, request: Value) -> Result<Vec<Value>> {
        self.post(endpoints::CREATE_QUOTE, &request).await
    }

    async fn cancel_quote(&self, request: Value) -> Result<Vec<Value>> {
        self.post(endpoints::CANCEL_QUOTE, &request).await
    }

    async fn cancel_batch_quotes(&self, request: Value) -> Result<Vec<Value>> {
        self.post(endpoints::CANCEL_BATCH_QUOTES, &request).await
    }

    async fn cancel_all_quotes(&self, request: Value) -> Result<Vec<Value>> {
        self.post(endpoints::CANCEL_ALL_QUOTES, &request).await
    }

    async fn get_rfqs(&self, params: Option<Value>) -> Result<Vec<Value>> {
        self.get(endpoints::GET_RFQS, params.as_ref()).await
    }

    async fn get_quotes(&self, params: Option<Value>) -> Result<Vec<Value>> {
        self.get(endpoints::GET_QUOTES, params.as_ref()).await
    }

    async fn get_trades(&self, params: Option<Value>) -> Result<Vec<Value>> {
        self.get(endpoints::GET_TRADES, params.as_ref()).await
    }

    async fn get_public_trades(&self, params: Option<Value>) -> Result<Vec<Value>> {
        self.get_public(endpoints::GET_PUBLIC_TRADES, params.as_ref())
            .await
    }

    async fn reset_mmp(&self, request: Value) -> Result<Vec<Value>> {
        self.post(endpoints::RESET_MMP, &request).await
    }

    async fn set_mmp_config(&self, request: Value) -> Result<Vec<Value>> {
        self.post(endpoints::SET_MMP, &request).await
    }

    async fn get_mmp_config(&self) -> Result<Vec<Value>> {
        self.get(endpoints::GET_MMP_CONFIG, None::<&()>).await
    }

    async fn set_marker_instrument(&self, request: Value) -> Result<Vec<Value>> {
        self.post(endpoints::SET_MARKER_INSTRUMENT, &request).await
    }

    async fn get_quote_products(&self, params: Option<Value>) -> Result<Vec<Value>> {
        self.get(endpoints::GET_QUOTE_PRODUCTS, params.as_ref())
            .await
    }
}
