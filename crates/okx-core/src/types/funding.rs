//! Funding-related data types.
//!
//! Source: OKX API v5 Funding REST API
//! - GET /api/v5/asset/balances
//! - POST /api/v5/asset/transfer
//! - POST /api/v5/asset/withdrawal
//! - GET /api/v5/asset/deposit-address
//! - GET /api/v5/asset/deposit-history
//! - GET /api/v5/asset/withdrawal-history

use serde::{Deserialize, Serialize};

/// Asset balance information.
///
/// Source: GET /api/v5/asset/balances response
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AssetBalance {
    /// Currency
    pub ccy: String,
    /// Balance
    #[serde(default)]
    pub bal: String,
    /// Frozen balance
    #[serde(default)]
    pub frozen_bal: String,
    /// Available balance
    #[serde(default)]
    pub avail_bal: String,
}

/// Deposit address information.
///
/// Source: GET /api/v5/asset/deposit-address response
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DepositAddress {
    /// Currency
    pub ccy: String,
    /// Chain name
    pub chain: String,
    /// Deposit address
    pub addr: String,
    /// Deposit tag/memo (for some chains like XRP, EOS)
    #[serde(default)]
    pub tag: String,
    /// Payment ID (for Monero)
    #[serde(default)]
    pub pmt_id: String,
    /// Deposit memo
    #[serde(default)]
    pub memo: String,
    /// Contract address (for tokens)
    #[serde(default)]
    pub ct_addr: String,
    /// Selected or not
    #[serde(default)]
    pub selected: bool,
}

/// Deposit history record.
///
/// Source: GET /api/v5/asset/deposit-history response
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DepositRecord {
    /// Currency
    pub ccy: String,
    /// Chain name
    pub chain: String,
    /// Deposit amount
    pub amt: String,
    /// Deposit address
    #[serde(default)]
    pub to: String,
    /// Transaction ID
    #[serde(default)]
    pub tx_id: String,
    /// Deposit ID
    #[serde(default)]
    pub dep_id: String,
    /// From address
    #[serde(default)]
    pub from: String,
    /// State: 0=waiting, 1=credited, 2=successful, 8=pending, 11=match, 12=confirmed, 13=blocked
    pub state: String,
    /// Deposit time (Unix timestamp in milliseconds)
    pub ts: String,
    /// Actual deposit amount after fee
    #[serde(default)]
    pub actual_dep_blk_confirm: String,
}

/// Withdrawal history record.
///
/// Source: GET /api/v5/asset/withdrawal-history response
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WithdrawalRecord {
    /// Currency
    pub ccy: String,
    /// Chain name
    pub chain: String,
    /// Withdrawal amount
    pub amt: String,
    /// Withdrawal address
    #[serde(default)]
    pub to: String,
    /// Transaction ID
    #[serde(default)]
    pub tx_id: String,
    /// Withdrawal ID
    #[serde(default)]
    pub wd_id: String,
    /// Client ID
    #[serde(default)]
    pub client_id: String,
    /// Fee
    #[serde(default)]
    pub fee: String,
    /// State: -3=canceling, -2=canceled, -1=failed, 0=pending, 1=sending, 2=sent, 3=awaiting, 4=processing, 5=success
    pub state: String,
    /// Withdrawal time (Unix timestamp in milliseconds)
    pub ts: String,
}

/// Funds transfer request.
///
/// Source: POST /api/v5/asset/transfer request body
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FundsTransferRequest {
    /// Currency
    pub ccy: String,
    /// Amount to transfer
    pub amt: String,
    /// Source account type
    /// 6: Funding account, 18: Trading account
    pub from: String,
    /// Destination account type
    pub to: String,
    /// Transfer type: 0=within account, 1=master to sub, 2=sub to master, 3=sub to master (managed), 4=sub to sub
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// Sub-account name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_acct: Option<String>,
    /// Instrument ID (for isolated margin)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inst_id: Option<String>,
    /// Destination instrument ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_inst_id: Option<String>,
    /// Loan transfer
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loan_trans: Option<bool>,
}

/// Funds transfer response.
///
/// Source: POST /api/v5/asset/transfer response
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FundsTransferResponse {
    /// Transfer ID
    pub trans_id: String,
    /// Currency
    #[serde(default)]
    pub ccy: String,
    /// Client ID
    #[serde(default)]
    pub client_id: String,
    /// Source account
    #[serde(default)]
    pub from: String,
    /// Amount
    #[serde(default)]
    pub amt: String,
    /// Destination account
    #[serde(default)]
    pub to: String,
}

/// Withdrawal request.
///
/// Source: POST /api/v5/asset/withdrawal request body
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WithdrawalRequest {
    /// Currency
    pub ccy: String,
    /// Withdrawal amount
    pub amt: String,
    /// Withdrawal destination: 3=on-chain, 4=internal transfer
    pub dest: String,
    /// Withdrawal address or account
    pub to_addr: String,
    /// Chain (e.g., "USDT-ERC20")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chain: Option<String>,
    /// Area code (for phone number)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub area_code: Option<String>,
    /// Client ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    /// Fee
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee: Option<String>,
}

/// Withdrawal response.
///
/// Source: POST /api/v5/asset/withdrawal response
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WithdrawalResponse {
    /// Withdrawal ID
    pub wd_id: String,
    /// Currency
    #[serde(default)]
    pub ccy: String,
    /// Chain
    #[serde(default)]
    pub chain: String,
    /// Amount
    #[serde(default)]
    pub amt: String,
    /// Client ID
    #[serde(default)]
    pub client_id: String,
}

/// Currency information.
///
/// Source: GET /api/v5/asset/currencies response
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CurrencyInfo {
    /// Currency
    pub ccy: String,
    /// Currency name
    #[serde(default)]
    pub name: String,
    /// Logo link
    #[serde(default)]
    pub logo_link: String,
    /// Chain name
    pub chain: String,
    /// Whether deposit is allowed
    #[serde(default)]
    pub can_dep: bool,
    /// Whether withdrawal is allowed
    #[serde(default)]
    pub can_wd: bool,
    /// Whether internal transfer is allowed
    #[serde(default)]
    pub can_internal: bool,
    /// Minimum withdrawal amount
    #[serde(default)]
    pub min_wd: String,
    /// Minimum withdrawal fee
    #[serde(default)]
    pub min_fee: String,
    /// Maximum withdrawal fee
    #[serde(default)]
    pub max_fee: String,
    /// Minimum deposit amount
    #[serde(default)]
    pub min_dep: String,
    /// Minimum number of confirmations for deposit
    #[serde(default)]
    pub min_dep_arrive_confirm: String,
    /// Withdrawal precision
    #[serde(default)]
    pub wd_tick_sz: String,
    /// Withdrawal quota
    #[serde(default)]
    pub wd_quota: String,
    /// Used withdrawal quota
    #[serde(default)]
    pub used_wd_quota: String,
    /// Main network
    #[serde(default)]
    pub main_net: bool,
}
