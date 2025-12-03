//! 系统状态接口。
//!
//! 对应 `/api/v5/system/status`，用于查询交易所当前状态。

use serde::Serialize;
use serde_json::Value;

use okx_core::Result;

use crate::OkxRestClient;

/// 端点常量。
pub mod endpoints {
    /// 系统状态查询
    pub const SYSTEM_STATUS: &str = "/api/v5/system/status";
}

/// 系统状态响应。
pub type SystemStatus = Value;

/// 系统状态查询参数。
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSystemStatusParams {
    /// 状态过滤：0 正常，1 系统维护
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

/// Status API trait。
pub trait StatusApi {
    /// 获取系统状态。
    fn get_system_status(
        &self,
        state: Option<&str>,
    ) -> impl std::future::Future<Output = Result<Vec<SystemStatus>>> + Send;
}

impl StatusApi for OkxRestClient {
    async fn get_system_status(&self, state: Option<&str>) -> Result<Vec<SystemStatus>> {
        let params = state.map(|v| GetSystemStatusParams {
            state: Some(v.to_string()),
        });
        self.get(endpoints::SYSTEM_STATUS, params.as_ref()).await
    }
}
