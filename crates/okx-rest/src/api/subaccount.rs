//! 子账户相关 REST 接口。
//!
//! 对应 python-okx SubAccountAPI 与官方文档的子账户管理、划转、权限等接口。

use okx_core::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::OkxRestClient;

/// 端点常量。
pub mod endpoints {
    /// 查询子账户交易账户余额
    pub const BALANCE: &str = "/api/v5/account/subaccount/balances";
    /// 查询子账户资金账户流水
    pub const BILLS: &str = "/api/v5/asset/subaccount/bills";
    /// 重置子账户 API Key
    pub const RESET_APIKEY: &str = "/api/v5/users/subaccount/modify-apikey";
    /// 查询子账户列表
    pub const LIST: &str = "/api/v5/users/subaccount/list";
    /// 母子账户间资产划转
    pub const TRANSFER: &str = "/api/v5/asset/subaccount/transfer";
    /// 查询托管子账户列表
    pub const ENTRUST_LIST: &str = "/api/v5/users/entrust-subaccount-list";
    /// 设置子账户主动转出权限
    pub const SET_TRANSFER_OUT: &str = "/api/v5/users/subaccount/set-transfer-out";
    /// 查询子账户资金账户余额
    pub const FUNDING_BALANCE: &str = "/api/v5/asset/subaccount/balances";
    /// 查询返佣信息
    pub const AFFILIATE_REBATE: &str = "/api/v5/users/partner/if-rebate";
    /// 设置子账户借币利息分配
    pub const SET_VIP_LOAN: &str = "/api/v5/account/subaccount/set-loan-allocation";
    /// 获取子账户借币利息及额度
    pub const BORROW_INTEREST_LIMIT: &str = "/api/v5/account/subaccount/interest-limits";
}

/// 子账户账单参数。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SubaccountBillsParams {
    /// 币种，如 BTC
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ccy: Option<String>,
    /// 账单类型
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub bill_type: Option<String>,
    /// 子账户名称
    #[serde(rename = "subAcct", skip_serializing_if = "Option::is_none")]
    pub sub_acct: Option<String>,
    /// 分页参数，查询此 ID 之后的记录
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// 分页参数，查询此 ID 之前的记录
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    /// 返回记录数量限制
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

/// 重置子账户 APIKey 请求。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResetSubaccountApikeyRequest {
    /// 子账户名称
    #[serde(rename = "subAcct")]
    pub sub_acct: String,
    /// API Key
    #[serde(rename = "apiKey")]
    pub api_key: String,
    /// API Key 标签
    pub label: String,
    /// API Key 权限，如 `read_only`、`trade`、`withdraw`
    pub perm: String,
    /// IP 白名单，多个 IP 用逗号分隔
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
}

/// 子账户列表查询参数。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SubaccountListParams {
    /// 是否启用，`true` 或 `false`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable: Option<bool>,
    /// 子账户名称
    #[serde(rename = "subAcct", skip_serializing_if = "Option::is_none")]
    pub sub_acct: Option<String>,
    /// 分页参数，查询此时间戳之后的记录
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// 分页参数，查询此时间戳之前的记录
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    /// 返回记录数量限制
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

/// 子账户划转请求。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubaccountTransferRequest {
    /// 币种，如 BTC
    pub ccy: String,
    /// 划转数量
    pub amt: String,
    /// 转出账户类型，`6`（资金账户）、`18`（交易账户）
    #[serde(rename = "from")]
    pub froms: String,
    /// 转入账户类型，`6`（资金账户）、`18`（交易账户）
    #[serde(rename = "to")]
    pub to: String,
    /// 转出子账户名称
    #[serde(rename = "fromSubAccount")]
    pub from_sub_account: String,
    /// 转入子账户名称
    #[serde(rename = "toSubAccount")]
    pub to_sub_account: String,
    /// 是否借贷划转
    #[serde(skip_serializing_if = "Option::is_none", rename = "loanTrans")]
    pub loan_trans: Option<bool>,
    /// 是否忽略仓位风险
    #[serde(skip_serializing_if = "Option::is_none", rename = "omitPosRisk")]
    pub omit_pos_risk: Option<bool>,
}

/// 设置转出权限请求。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetTransferOutRequest {
    /// 子账户名称
    #[serde(rename = "subAcct")]
    pub sub_acct: String,
    /// 是否允许主动转出，`true` 或 `false`
    #[serde(rename = "canTransOut")]
    pub can_trans_out: bool,
}

/// 设置子账户 VIP 借贷分配请求。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetVipLoanRequest {
    /// 是否启用 VIP 借贷分配
    pub enable: bool,
    /// 分配比例列表，官方为数组字符串，保持自由形式
    pub alloc: serde_json::Value,
}

/// 子账户利息与限额查询参数。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SubaccountInterestParams {
    /// 子账户名称
    #[serde(rename = "subAcct", skip_serializing_if = "Option::is_none")]
    pub sub_acct: Option<String>,
    /// 币种，如 BTC
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ccy: Option<String>,
}

/// 子账户 API。
pub trait SubaccountApi {
    /// 查询子账户交易账户余额
    fn get_subaccount_balance(
        &self,
        sub_acct: &str,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;

    /// 查询子账户资金账户流水
    fn get_subaccount_bills(
        &self,
        params: Option<SubaccountBillsParams>,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;

    /// 重置子账户 API Key
    fn reset_subaccount_apikey(
        &self,
        request: ResetSubaccountApikeyRequest,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;

    /// 查询子账户列表
    fn get_subaccount_list(
        &self,
        params: Option<SubaccountListParams>,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;

    /// 母子账户间资产划转
    fn subaccount_transfer(
        &self,
        request: SubaccountTransferRequest,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;

    /// 查询托管子账户列表
    fn get_entrust_subaccount_list(
        &self,
        sub_acct: Option<&str>,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;

    /// 设置子账户主动转出权限
    fn set_permission_transfer_out(
        &self,
        request: SetTransferOutRequest,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;

    /// 查询子账户资金账户余额
    fn get_funding_balance(
        &self,
        sub_acct: &str,
        ccy: Option<&str>,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;

    /// 查询返佣信息
    fn get_affiliate_rebate_info(
        &self,
        api_key: &str,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;

    /// 设置子账户借币利息分配
    fn set_sub_accounts_vip_loan(
        &self,
        request: SetVipLoanRequest,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;

    /// 获取子账户借币利息及额度
    fn get_sub_account_borrow_interest_and_limit(
        &self,
        params: Option<SubaccountInterestParams>,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;
}

impl SubaccountApi for OkxRestClient {
    async fn get_subaccount_balance(&self, sub_acct: &str) -> Result<Vec<Value>> {
        #[derive(Serialize)]
        struct Params<'a> {
            #[serde(rename = "subAcct")]
            sub_acct: &'a str,
        }
        self.get(endpoints::BALANCE, Some(&Params { sub_acct }))
            .await
    }

    async fn get_subaccount_bills(
        &self,
        params: Option<SubaccountBillsParams>,
    ) -> Result<Vec<Value>> {
        self.get(endpoints::BILLS, params.as_ref()).await
    }

    async fn reset_subaccount_apikey(
        &self,
        request: ResetSubaccountApikeyRequest,
    ) -> Result<Vec<Value>> {
        self.post(endpoints::RESET_APIKEY, &request).await
    }

    async fn get_subaccount_list(
        &self,
        params: Option<SubaccountListParams>,
    ) -> Result<Vec<Value>> {
        self.get(endpoints::LIST, params.as_ref()).await
    }

    async fn subaccount_transfer(&self, request: SubaccountTransferRequest) -> Result<Vec<Value>> {
        self.post(endpoints::TRANSFER, &request).await
    }

    async fn get_entrust_subaccount_list(&self, sub_acct: Option<&str>) -> Result<Vec<Value>> {
        #[derive(Serialize)]
        struct Params<'a> {
            #[serde(rename = "subAcct", skip_serializing_if = "Option::is_none")]
            sub_acct: Option<&'a str>,
        }
        let params = Params { sub_acct };
        self.get(endpoints::ENTRUST_LIST, Some(&params)).await
    }

    async fn set_permission_transfer_out(
        &self,
        request: SetTransferOutRequest,
    ) -> Result<Vec<Value>> {
        self.post(endpoints::SET_TRANSFER_OUT, &request).await
    }

    async fn get_funding_balance(&self, sub_acct: &str, ccy: Option<&str>) -> Result<Vec<Value>> {
        #[derive(Serialize)]
        struct Params<'a> {
            #[serde(rename = "subAcct")]
            sub_acct: &'a str,
            #[serde(skip_serializing_if = "Option::is_none")]
            ccy: Option<&'a str>,
        }
        let params = Params { sub_acct, ccy };
        self.get(endpoints::FUNDING_BALANCE, Some(&params)).await
    }

    async fn get_affiliate_rebate_info(&self, api_key: &str) -> Result<Vec<Value>> {
        #[derive(Serialize)]
        struct Params<'a> {
            #[serde(rename = "apiKey")]
            api_key: &'a str,
        }
        self.get(endpoints::AFFILIATE_REBATE, Some(&Params { api_key }))
            .await
    }

    async fn set_sub_accounts_vip_loan(&self, request: SetVipLoanRequest) -> Result<Vec<Value>> {
        self.post(endpoints::SET_VIP_LOAN, &request).await
    }

    async fn get_sub_account_borrow_interest_and_limit(
        &self,
        params: Option<SubaccountInterestParams>,
    ) -> Result<Vec<Value>> {
        self.get(endpoints::BORROW_INTEREST_LIMIT, params.as_ref())
            .await
    }
}
