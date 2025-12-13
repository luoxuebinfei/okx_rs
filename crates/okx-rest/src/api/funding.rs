//! Funding API endpoints.
//!
//! Source: OKX API v5 Funding REST API
//! - <https://www.okx.com/docs-v5/en/#funding-account-rest-api>

use serde::Serialize;
use serde_json::Value;

use okx_core::{
    types::{
        AssetBalance, CurrencyInfo, DepositAddress, DepositRecord, FundsTransferRequest,
        FundsTransferResponse, WithdrawalRecord, WithdrawalRequest, WithdrawalResponse,
    },
    Result,
};

use crate::OkxRestClient;

/// API endpoints for funding operations.
pub mod endpoints {
    /// Get asset balances
    pub const BALANCES: &str = "/api/v5/asset/balances";
    /// Get non-tradable assets
    pub const NON_TRADABLE_ASSETS: &str = "/api/v5/asset/non-tradable-assets";
    /// Get asset valuation
    pub const ASSET_VALUATION: &str = "/api/v5/asset/asset-valuation";
    /// Funds transfer
    pub const TRANSFER: &str = "/api/v5/asset/transfer";
    /// Get transfer state
    pub const TRANSFER_STATE: &str = "/api/v5/asset/transfer-state";
    /// Get bills
    pub const BILLS: &str = "/api/v5/asset/bills";
    /// Purchase or redempt saving
    pub const PURCHASE_REDEMPT: &str = "/api/v5/asset/purchase_redempt";
    /// Get deposit address
    pub const DEPOSIT_ADDRESS: &str = "/api/v5/asset/deposit-address";
    /// Get lightning deposit info
    pub const DEPOSIT_LIGHTNING: &str = "/api/v5/asset/deposit-lightning";
    /// Get deposit history
    pub const DEPOSIT_HISTORY: &str = "/api/v5/asset/deposit-history";
    /// Withdrawal
    pub const WITHDRAWAL: &str = "/api/v5/asset/withdrawal";
    /// Lightning withdrawal
    pub const WITHDRAWAL_LIGHTNING: &str = "/api/v5/asset/withdrawal-lightning";
    /// Cancel withdrawal
    pub const CANCEL_WITHDRAWAL: &str = "/api/v5/asset/cancel-withdrawal";
    /// Get withdrawal history
    pub const WITHDRAWAL_HISTORY: &str = "/api/v5/asset/withdrawal-history";
    /// Withdrawal/deposit status
    pub const DEPOSIT_WITHDRAW_STATUS: &str = "/api/v5/asset/deposit-withdraw-status";
    /// Get currencies
    pub const CURRENCIES: &str = "/api/v5/asset/currencies";
    /// Set lending rate
    pub const SET_LENDING_RATE: &str = "/api/v5/asset/set-lending-rate";
    /// Lending history
    pub const LENDING_HISTORY: &str = "/api/v5/asset/lending-history";
    /// Lending rate history
    pub const LENDING_RATE_HISTORY: &str = "/api/v5/asset/lending-rate-history";
    /// Lending rate summary
    pub const LENDING_RATE_SUMMARY: &str = "/api/v5/asset/lending-rate-summary";
    /// Convert dust assets
    pub const CONVERT_DUST_ASSETS: &str = "/api/v5/asset/convert-dust-assets";
    /// Saving balance
    pub const SAVING_BALANCE: &str = "/api/v5/asset/saving-balance";
}

/// Query parameters for get_asset_balances.
#[derive(Debug, Default, Serialize)]
pub struct GetAssetBalancesParams {
    /// Currency, e.g., "BTC" (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ccy: Option<String>,
}

/// Query parameters for get_deposit_address.
#[derive(Debug, Serialize)]
pub struct GetDepositAddressParams {
    /// Currency
    pub ccy: String,
}

/// Query parameters for get_deposit_history.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetDepositHistoryParams {
    /// Currency
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ccy: Option<String>,
    /// Deposit ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dep_id: Option<String>,
    /// Transaction ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx_id: Option<String>,
    /// Deposit type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// State: 0=waiting, 1=credited, 2=successful
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// Pagination: deposit ID to start from
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// Pagination: deposit ID to end at
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    /// Number of results (max 100, default 100)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,
}

/// Query parameters for get_withdrawal_history.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetWithdrawalHistoryParams {
    /// Currency
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ccy: Option<String>,
    /// Withdrawal ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wd_id: Option<String>,
    /// Client ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    /// Transaction ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx_id: Option<String>,
    /// Withdrawal type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// State
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// Pagination: withdrawal ID to start from
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// Pagination: withdrawal ID to end at
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    /// Number of results (max 100, default 100)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,
}

/// Query parameters for get_currencies.
#[derive(Debug, Default, Serialize)]
pub struct GetCurrenciesParams {
    /// Currency
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ccy: Option<String>,
}

/// Query parameters for transfer state.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetTransferStateParams {
    /// Transfer ID
    pub trans_id: String,
    /// Transfer type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

/// Request body for purchase_redempt.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PurchaseRedemptRequest {
    /// 币种，如 USDT
    pub ccy: String,
    /// 申购/赎回数量
    pub amt: String,
    /// 操作类型，`purchase`（申购）或 `redempt`（赎回）
    pub side: String,
    /// 利率，申购时可选
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate: Option<String>,
}

/// Query parameters for funding bills.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetFundingBillsParams {
    /// 币种，如 BTC
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ccy: Option<String>,
    /// 账单类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// 分页参数，查询此 ID 之后的记录
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// 分页参数，查询此 ID 之前的记录
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    /// 返回记录数量限制
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,
}

/// Query parameters for lightning deposit.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetDepositLightningParams {
    /// 币种，如 BTC
    pub ccy: String,
    /// 充值数量
    pub amt: String,
    /// 收款账户类型，`6`（资金账户）或 `18`（交易账户）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<String>,
}

/// Request body for lightning withdrawal.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WithdrawalLightningRequest {
    /// 币种，如 BTC
    pub ccy: String,
    /// 闪电网络 invoice
    pub invoice: String,
    /// 备注
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memo: Option<String>,
}

/// Request body for setting lending rate.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetLendingRateRequest {
    /// 币种，如 USDT
    pub ccy: String,
    /// 利率，年化利率（0-365%）
    pub rate: String,
}

/// Query parameters for lending history.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetLendingHistoryParams {
    /// 币种，如 USDT
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ccy: Option<String>,
    /// 分页参数，查询此时间戳之前的记录
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    /// 分页参数，查询此时间戳之后的记录
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// 返回记录数量限制
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,
}

/// Query parameters for lending rate history.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetLendingRateHistoryParams {
    /// 币种，如 USDT
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ccy: Option<String>,
    /// 分页参数，查询此时间戳之后的记录
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// 分页参数，查询此时间戳之前的记录
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    /// 返回记录数量限制
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,
}

/// Query parameters for lending rate summary.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetLendingRateSummaryParams {
    /// 币种，如 USDT
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ccy: Option<String>,
}

/// Query parameters for asset valuation.
#[derive(Debug, Default, Serialize)]
pub struct GetAssetValuationParams {
    /// 币种，如 BTC，多个币种用逗号分隔
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ccy: Option<String>,
}

/// Query parameters for cancel withdrawal.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelWithdrawalParams {
    /// 提币申请 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wd_id: Option<String>,
}

/// Request body for convert dust assets.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConvertDustAssetsRequest {
    /// 币种列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ccy: Option<Vec<String>>,
}

/// Query parameters for saving balance.
#[derive(Debug, Default, Serialize)]
pub struct GetSavingBalanceParams {
    /// 币种，如 USDT
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ccy: Option<String>,
}

/// Query parameters for deposit/withdraw status.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetDepositWithdrawStatusParams {
    /// 提币申请 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wd_id: Option<String>,
    /// 区块转账哈希记录 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx_id: Option<String>,
    /// 币种，如 BTC
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ccy: Option<String>,
    /// 充值地址
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<String>,
    /// 币种链信息，如 `BTC-Bitcoin`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chain: Option<String>,
}

/// Raw funding payload for untyped endpoints.
pub type FundingRaw = Value;

/// Funding API trait for OKX REST client.
///
/// Provides methods for funding account operations.
pub trait FundingApi {
    /// Get asset balances.
    ///
    /// Retrieves balances in the funding account.
    ///
    /// ## API Details
    ///
    /// - Endpoint: GET /api/v5/asset/balances
    /// - Rate limit: 6 requests per second
    /// - Permission: Read
    fn get_asset_balances(
        &self,
        ccy: Option<&str>,
    ) -> impl std::future::Future<Output = Result<Vec<AssetBalance>>> + Send;

    /// Get deposit address.
    ///
    /// Retrieves deposit addresses for a currency.
    ///
    /// ## API Details
    ///
    /// - Endpoint: GET /api/v5/asset/deposit-address
    /// - Rate limit: 6 requests per second
    /// - Permission: Read
    fn get_deposit_address(
        &self,
        ccy: &str,
    ) -> impl std::future::Future<Output = Result<Vec<DepositAddress>>> + Send;

    /// Get deposit history.
    ///
    /// Retrieves deposit history records.
    ///
    /// ## API Details
    ///
    /// - Endpoint: GET /api/v5/asset/deposit-history
    /// - Rate limit: 6 requests per second
    /// - Permission: Read
    fn get_deposit_history(
        &self,
        params: Option<GetDepositHistoryParams>,
    ) -> impl std::future::Future<Output = Result<Vec<DepositRecord>>> + Send;

    /// Get withdrawal history.
    ///
    /// Retrieves withdrawal history records.
    ///
    /// ## API Details
    ///
    /// - Endpoint: GET /api/v5/asset/withdrawal-history
    /// - Rate limit: 6 requests per second
    /// - Permission: Read
    fn get_withdrawal_history(
        &self,
        params: Option<GetWithdrawalHistoryParams>,
    ) -> impl std::future::Future<Output = Result<Vec<WithdrawalRecord>>> + Send;

    /// Transfer funds between accounts.
    ///
    /// ## API Details
    ///
    /// - Endpoint: POST /api/v5/asset/transfer
    /// - Rate limit: 1 request per second
    /// - Permission: Trade
    ///
    /// ## Account Types
    ///
    /// - 6: Funding account
    /// - 18: Trading account
    fn funds_transfer(
        &self,
        request: FundsTransferRequest,
    ) -> impl std::future::Future<Output = Result<Vec<FundsTransferResponse>>> + Send;

    /// Withdraw funds.
    ///
    /// ## API Details
    ///
    /// - Endpoint: POST /api/v5/asset/withdrawal
    /// - Rate limit: 6 requests per second
    /// - Permission: Withdraw
    ///
    /// ## Destination Types
    ///
    /// - 3: On-chain withdrawal
    /// - 4: Internal transfer
    fn withdrawal(
        &self,
        request: WithdrawalRequest,
    ) -> impl std::future::Future<Output = Result<Vec<WithdrawalResponse>>> + Send;

    /// Get currencies.
    ///
    /// Retrieves list of currencies with their chain information.
    ///
    /// ## API Details
    ///
    /// - Endpoint: GET /api/v5/asset/currencies
    /// - Rate limit: 6 requests per second
    /// - Permission: Read
    fn get_currencies(
        &self,
        ccy: Option<&str>,
    ) -> impl std::future::Future<Output = Result<Vec<CurrencyInfo>>> + Send;

    /// Get non-tradable assets.
    fn get_non_tradable_assets(
        &self,
        ccy: Option<&str>,
    ) -> impl std::future::Future<Output = Result<Vec<FundingRaw>>> + Send;

    /// Get asset valuation.
    fn get_asset_valuation(
        &self,
        params: Option<GetAssetValuationParams>,
    ) -> impl std::future::Future<Output = Result<Vec<FundingRaw>>> + Send;

    /// Get transfer state.
    fn get_transfer_state(
        &self,
        params: GetTransferStateParams,
    ) -> impl std::future::Future<Output = Result<Vec<FundingRaw>>> + Send;

    /// Get funding bills.
    fn get_funding_bills(
        &self,
        params: Option<GetFundingBillsParams>,
    ) -> impl std::future::Future<Output = Result<Vec<FundingRaw>>> + Send;

    /// Purchase or redeem saving products.
    fn purchase_redempt(
        &self,
        request: PurchaseRedemptRequest,
    ) -> impl std::future::Future<Output = Result<Vec<FundingRaw>>> + Send;

    /// Lightning deposit info.
    fn get_deposit_lightning(
        &self,
        params: GetDepositLightningParams,
    ) -> impl std::future::Future<Output = Result<Vec<FundingRaw>>> + Send;

    /// Lightning withdrawal.
    fn withdrawal_lightning(
        &self,
        request: WithdrawalLightningRequest,
    ) -> impl std::future::Future<Output = Result<Vec<FundingRaw>>> + Send;

    /// Cancel withdrawal.
    fn cancel_withdrawal(
        &self,
        params: CancelWithdrawalParams,
    ) -> impl std::future::Future<Output = Result<Vec<FundingRaw>>> + Send;

    /// Deposit/withdraw status.
    fn get_deposit_withdraw_status(
        &self,
        params: GetDepositWithdrawStatusParams,
    ) -> impl std::future::Future<Output = Result<Vec<FundingRaw>>> + Send;

    /// Set lending rate.
    fn set_lending_rate(
        &self,
        request: SetLendingRateRequest,
    ) -> impl std::future::Future<Output = Result<Vec<FundingRaw>>> + Send;

    /// Lending history.
    fn get_lending_history(
        &self,
        params: Option<GetLendingHistoryParams>,
    ) -> impl std::future::Future<Output = Result<Vec<FundingRaw>>> + Send;

    /// Lending rate history.
    fn get_lending_rate_history(
        &self,
        params: Option<GetLendingRateHistoryParams>,
    ) -> impl std::future::Future<Output = Result<Vec<FundingRaw>>> + Send;

    /// Lending rate summary.
    fn get_lending_rate_summary(
        &self,
        params: Option<GetLendingRateSummaryParams>,
    ) -> impl std::future::Future<Output = Result<Vec<FundingRaw>>> + Send;

    /// Convert dust assets.
    fn convert_dust_assets(
        &self,
        request: ConvertDustAssetsRequest,
    ) -> impl std::future::Future<Output = Result<Vec<FundingRaw>>> + Send;

    /// Saving balance.
    fn get_saving_balance(
        &self,
        params: Option<GetSavingBalanceParams>,
    ) -> impl std::future::Future<Output = Result<Vec<FundingRaw>>> + Send;
}

impl FundingApi for OkxRestClient {
    async fn get_asset_balances(&self, ccy: Option<&str>) -> Result<Vec<AssetBalance>> {
        let params = ccy.map(|c| GetAssetBalancesParams {
            ccy: Some(c.to_string()),
        });
        self.get(endpoints::BALANCES, params.as_ref()).await
    }

    async fn get_deposit_address(&self, ccy: &str) -> Result<Vec<DepositAddress>> {
        let params = GetDepositAddressParams {
            ccy: ccy.to_string(),
        };
        self.get(endpoints::DEPOSIT_ADDRESS, Some(&params)).await
    }

    async fn get_deposit_history(
        &self,
        params: Option<GetDepositHistoryParams>,
    ) -> Result<Vec<DepositRecord>> {
        self.get(endpoints::DEPOSIT_HISTORY, params.as_ref()).await
    }

    async fn get_withdrawal_history(
        &self,
        params: Option<GetWithdrawalHistoryParams>,
    ) -> Result<Vec<WithdrawalRecord>> {
        self.get(endpoints::WITHDRAWAL_HISTORY, params.as_ref())
            .await
    }

    async fn funds_transfer(
        &self,
        request: FundsTransferRequest,
    ) -> Result<Vec<FundsTransferResponse>> {
        self.post(endpoints::TRANSFER, &request).await
    }

    async fn withdrawal(&self, request: WithdrawalRequest) -> Result<Vec<WithdrawalResponse>> {
        self.post(endpoints::WITHDRAWAL, &request).await
    }

    async fn get_currencies(&self, ccy: Option<&str>) -> Result<Vec<CurrencyInfo>> {
        let params = ccy.map(|c| GetCurrenciesParams {
            ccy: Some(c.to_string()),
        });
        self.get(endpoints::CURRENCIES, params.as_ref()).await
    }

    async fn get_non_tradable_assets(&self, ccy: Option<&str>) -> Result<Vec<FundingRaw>> {
        let params = ccy.map(|c| GetAssetBalancesParams {
            ccy: Some(c.to_string()),
        });
        self.get(endpoints::NON_TRADABLE_ASSETS, params.as_ref())
            .await
    }

    async fn get_asset_valuation(
        &self,
        params: Option<GetAssetValuationParams>,
    ) -> Result<Vec<FundingRaw>> {
        self.get(endpoints::ASSET_VALUATION, params.as_ref()).await
    }

    async fn get_transfer_state(&self, params: GetTransferStateParams) -> Result<Vec<FundingRaw>> {
        self.get(endpoints::TRANSFER_STATE, Some(&params)).await
    }

    async fn get_funding_bills(
        &self,
        params: Option<GetFundingBillsParams>,
    ) -> Result<Vec<FundingRaw>> {
        self.get(endpoints::BILLS, params.as_ref()).await
    }

    async fn purchase_redempt(&self, request: PurchaseRedemptRequest) -> Result<Vec<FundingRaw>> {
        self.post(endpoints::PURCHASE_REDEMPT, &request).await
    }

    async fn get_deposit_lightning(
        &self,
        params: GetDepositLightningParams,
    ) -> Result<Vec<FundingRaw>> {
        self.get(endpoints::DEPOSIT_LIGHTNING, Some(&params)).await
    }

    async fn withdrawal_lightning(
        &self,
        request: WithdrawalLightningRequest,
    ) -> Result<Vec<FundingRaw>> {
        self.post(endpoints::WITHDRAWAL_LIGHTNING, &request).await
    }

    async fn cancel_withdrawal(&self, params: CancelWithdrawalParams) -> Result<Vec<FundingRaw>> {
        self.post(endpoints::CANCEL_WITHDRAWAL, &params).await
    }

    async fn get_deposit_withdraw_status(
        &self,
        params: GetDepositWithdrawStatusParams,
    ) -> Result<Vec<FundingRaw>> {
        self.get(endpoints::DEPOSIT_WITHDRAW_STATUS, Some(&params))
            .await
    }

    async fn set_lending_rate(&self, request: SetLendingRateRequest) -> Result<Vec<FundingRaw>> {
        self.post(endpoints::SET_LENDING_RATE, &request).await
    }

    async fn get_lending_history(
        &self,
        params: Option<GetLendingHistoryParams>,
    ) -> Result<Vec<FundingRaw>> {
        self.get(endpoints::LENDING_HISTORY, params.as_ref()).await
    }

    async fn get_lending_rate_history(
        &self,
        params: Option<GetLendingRateHistoryParams>,
    ) -> Result<Vec<FundingRaw>> {
        self.get(endpoints::LENDING_RATE_HISTORY, params.as_ref())
            .await
    }

    async fn get_lending_rate_summary(
        &self,
        params: Option<GetLendingRateSummaryParams>,
    ) -> Result<Vec<FundingRaw>> {
        self.get(endpoints::LENDING_RATE_SUMMARY, params.as_ref())
            .await
    }

    async fn convert_dust_assets(
        &self,
        request: ConvertDustAssetsRequest,
    ) -> Result<Vec<FundingRaw>> {
        self.post(endpoints::CONVERT_DUST_ASSETS, &request).await
    }

    async fn get_saving_balance(
        &self,
        params: Option<GetSavingBalanceParams>,
    ) -> Result<Vec<FundingRaw>> {
        self.get(endpoints::SAVING_BALANCE, params.as_ref()).await
    }
}
