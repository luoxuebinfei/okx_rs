//! 闪兑与一键还债相关类型。

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 闪兑币种列表条目（字段随官方返回，保持灵活）。
pub type ConvertCurrency = Value;

/// 闪兑币对信息。
pub type ConvertCurrencyPair = Value;

/// 闪兑报价参数。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EstimateQuoteParams {
    /// 基础币
    #[serde(rename = "baseCcy")]
    pub base_ccy: String,
    /// 报价币
    #[serde(rename = "quoteCcy")]
    pub quote_ccy: String,
    /// 方向（buy/sell）
    pub side: String,
    /// 报价数量
    #[serde(rename = "rfqSz")]
    pub rfq_sz: String,
    /// 报价数量币种（baseCcy 或 quoteCcy）
    #[serde(rename = "rfqSzCcy")]
    pub rfq_sz_ccy: String,
    /// 客户端请求 ID
    #[serde(rename = "clQReqId", skip_serializing_if = "Option::is_none")]
    pub cl_q_req_id: Option<String>,
    /// 业务标签
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

/// 闪兑报价响应（官方字段较多，保持 Value 以兼容）。
pub type EstimateQuoteResponse = Value;

/// 闪兑成交请求。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConvertTradeRequest {
    /// 报价 ID
    #[serde(rename = "quoteId")]
    pub quote_id: String,
    /// 基础币
    #[serde(rename = "baseCcy")]
    pub base_ccy: String,
    /// 报价币
    #[serde(rename = "quoteCcy")]
    pub quote_ccy: String,
    /// 方向（buy/sell）
    pub side: String,
    /// 成交数量
    pub sz: String,
    /// 数量币种
    #[serde(rename = "szCcy")]
    pub sz_ccy: String,
    /// 客户端请求 ID
    #[serde(rename = "clTReqId", skip_serializing_if = "Option::is_none")]
    pub cl_t_req_id: Option<String>,
    /// 业务标签
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

/// 闪兑历史查询参数。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ConvertHistoryParams {
    /// 游标：下一页
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// 游标：上一页
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    /// 条数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// 标签过滤
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

/// 闪兑历史记录。
pub type ConvertHistoryRecord = Value;

/// Easy Convert 请求。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EasyConvertRequest {
    /// 需兑换的币种列表
    #[serde(rename = "fromCcy")]
    pub from_ccy: Vec<String>,
    /// 目标币种
    #[serde(rename = "toCcy")]
    pub to_ccy: String,
}

/// 一键还债请求。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OneClickRepayRequest {
    /// 欠款币种列表
    #[serde(rename = "debtCcy")]
    pub debt_ccy: Vec<String>,
    /// 还款币种
    #[serde(rename = "repayCcy")]
    pub repay_ccy: String,
}

/// Easy Convert 与一键还债返回结构（保持灵活）。
pub type ConvertGenericResponse = Value;
