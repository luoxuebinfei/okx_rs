//! 交易数据（Rubik）相关公共接口。
//!
//! 对应官方 `/api/v5/rubik/stat/*` 路径，全部为公共端点。

use okx_core::Result;
use serde_json::Value;

use crate::OkxRestClient;

/// 端点常量。
pub mod endpoints {
    /// 支持币种
    pub const SUPPORT_COIN: &str = "/api/v5/rubik/stat/trading-data/support-coin";
    /// Taker 主动交易量
    pub const TAKER_VOLUME: &str = "/api/v5/rubik/stat/taker-volume";
    /// 保证金借贷比
    pub const MARGIN_LENDING_RATIO: &str = "/api/v5/rubik/stat/margin/loan-ratio";
    /// 合约多空账户数比
    pub const LONG_SHORT_RATIO: &str = "/api/v5/rubik/stat/contracts/long-short-account-ratio";
    /// 合约持仓量与交易量
    pub const CONTRACTS_INTEREST_VOLUME: &str = "/api/v5/rubik/stat/contracts/open-interest-volume";
    /// 期权持仓量与交易量
    pub const OPTIONS_INTEREST_VOLUME: &str = "/api/v5/rubik/stat/option/open-interest-volume";
    /// 期权看涨/看跌比
    pub const PUT_CALL_RATIO: &str = "/api/v5/rubik/stat/option/open-interest-volume-ratio";
    /// 期权持仓量-到期日分布
    pub const OPEN_INTEREST_VOLUME_EXPIRY: &str =
        "/api/v5/rubik/stat/option/open-interest-volume-expiry";
    /// 期权持仓量-执行价分布
    pub const INTEREST_VOLUME_STRIKE: &str =
        "/api/v5/rubik/stat/option/open-interest-volume-strike";
    /// 期权 Taker 流
    pub const TAKER_FLOW: &str = "/api/v5/rubik/stat/option/taker-block-volume";
}

/// 交易数据 API（公共，无需鉴权）。
pub trait TradingDataApi {
    /// 支持币种列表。
    fn get_support_coin(&self) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;

    /// Taker 主动交易量。
    fn get_taker_volume(
        &self,
        params: Option<Value>,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;

    /// 保证金借贷比。
    fn get_margin_lending_ratio(
        &self,
        params: Option<Value>,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;

    /// 合约多空账户数比。
    fn get_long_short_ratio(
        &self,
        params: Option<Value>,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;

    /// 合约持仓量与交易量。
    fn get_contracts_open_interest_volume(
        &self,
        params: Option<Value>,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;

    /// 期权持仓量与交易量。
    fn get_options_open_interest_volume(
        &self,
        params: Option<Value>,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;

    /// 期权看涨/看跌比。
    fn get_put_call_ratio(
        &self,
        params: Option<Value>,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;

    /// 期权持仓量-到期日分布。
    fn get_open_interest_volume_expiry(
        &self,
        params: Option<Value>,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;

    /// 期权持仓量-执行价分布。
    fn get_interest_volume_strike(
        &self,
        params: Option<Value>,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;

    /// 期权 Taker 流。
    fn get_taker_flow(
        &self,
        params: Option<Value>,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;
}

impl TradingDataApi for OkxRestClient {
    async fn get_support_coin(&self) -> Result<Vec<Value>> {
        self.get_public(endpoints::SUPPORT_COIN, None::<&()>).await
    }

    async fn get_taker_volume(&self, params: Option<Value>) -> Result<Vec<Value>> {
        self.get_public(endpoints::TAKER_VOLUME, params.as_ref())
            .await
    }

    async fn get_margin_lending_ratio(&self, params: Option<Value>) -> Result<Vec<Value>> {
        self.get_public(endpoints::MARGIN_LENDING_RATIO, params.as_ref())
            .await
    }

    async fn get_long_short_ratio(&self, params: Option<Value>) -> Result<Vec<Value>> {
        self.get_public(endpoints::LONG_SHORT_RATIO, params.as_ref())
            .await
    }

    async fn get_contracts_open_interest_volume(
        &self,
        params: Option<Value>,
    ) -> Result<Vec<Value>> {
        self.get_public(endpoints::CONTRACTS_INTEREST_VOLUME, params.as_ref())
            .await
    }

    async fn get_options_open_interest_volume(&self, params: Option<Value>) -> Result<Vec<Value>> {
        self.get_public(endpoints::OPTIONS_INTEREST_VOLUME, params.as_ref())
            .await
    }

    async fn get_put_call_ratio(&self, params: Option<Value>) -> Result<Vec<Value>> {
        self.get_public(endpoints::PUT_CALL_RATIO, params.as_ref())
            .await
    }

    async fn get_open_interest_volume_expiry(&self, params: Option<Value>) -> Result<Vec<Value>> {
        self.get_public(endpoints::OPEN_INTEREST_VOLUME_EXPIRY, params.as_ref())
            .await
    }

    async fn get_interest_volume_strike(&self, params: Option<Value>) -> Result<Vec<Value>> {
        self.get_public(endpoints::INTEREST_VOLUME_STRIKE, params.as_ref())
            .await
    }

    async fn get_taker_flow(&self, params: Option<Value>) -> Result<Vec<Value>> {
        self.get_public(endpoints::TAKER_FLOW, params.as_ref())
            .await
    }
}
