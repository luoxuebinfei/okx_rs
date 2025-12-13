//! 经纪商（Broker）相关接口。
//!
//! 包含 FD Broker 返佣等功能，对应 `/api/v5/broker/fd/*`。

use okx_core::Result;
use serde_json::Value;

use crate::OkxRestClient;

/// 端点常量。
pub mod endpoints {
    /// 生成返佣明细下载链接端点。
    pub const FD_REBATE_PER_ORDERS: &str = "/api/v5/broker/fd/rebate-per-orders";
    /// 获取返佣明细下载链接端点。
    pub const FD_GET_REBATE_PER_ORDERS: &str = "/api/v5/broker/fd/rebate-per-orders";
}

/// Broker API。
pub trait BrokerApi {
    /// 生成返佣明细下载链接。
    fn fd_rebate_per_orders(
        &self,
        params: Value,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;

    /// 获取返佣明细下载链接。
    fn fd_get_rebate_per_orders(
        &self,
        params: Value,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;
}

impl BrokerApi for OkxRestClient {
    async fn fd_rebate_per_orders(&self, params: Value) -> Result<Vec<Value>> {
        self.get(endpoints::FD_REBATE_PER_ORDERS, Some(&params))
            .await
    }

    async fn fd_get_rebate_per_orders(&self, params: Value) -> Result<Vec<Value>> {
        self.get(endpoints::FD_GET_REBATE_PER_ORDERS, Some(&params))
            .await
    }
}
