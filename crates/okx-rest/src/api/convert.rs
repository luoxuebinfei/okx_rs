//! 闪兑与一键还债相关 REST 接口。
//!
//! 参考官方文档与 python-okx consts：
//! - /api/v5/asset/convert/currencies
//! - /api/v5/asset/convert/currency-pair
//! - /api/v5/asset/convert/estimate-quote
//! - /api/v5/asset/convert/trade
//! - /api/v5/asset/convert/history
//! - /api/v5/trade/easy-convert-*、one-click-repay-*

use okx_core::types::{
    ConvertCurrency, ConvertCurrencyPair, ConvertGenericResponse, ConvertHistoryParams,
    ConvertHistoryRecord, ConvertTradeRequest, EasyConvertRequest, EstimateQuoteParams,
    EstimateQuoteResponse, OneClickRepayRequest,
};
use okx_core::Result;

use crate::OkxRestClient;

/// 端点常量。
pub mod endpoints {
    /// 闪兑支持币种
    pub const CONVERT_CURRENCIES: &str = "/api/v5/asset/convert/currencies";
    /// 闪兑币对
    pub const CONVERT_CURRENCY_PAIR: &str = "/api/v5/asset/convert/currency-pair";
    /// 闪兑报价
    pub const CONVERT_ESTIMATE_QUOTE: &str = "/api/v5/asset/convert/estimate-quote";
    /// 闪兑成交
    pub const CONVERT_TRADE: &str = "/api/v5/asset/convert/trade";
    /// 闪兑历史
    pub const CONVERT_HISTORY: &str = "/api/v5/asset/convert/history";

    /// Easy Convert 支持币种
    pub const EASY_CONVERT_CURRENCY_LIST: &str = "/api/v5/trade/easy-convert-currency-list";
    /// Easy Convert 兑换
    pub const EASY_CONVERT: &str = "/api/v5/trade/easy-convert";
    /// Easy Convert 历史
    pub const EASY_CONVERT_HISTORY: &str = "/api/v5/trade/easy-convert-history";

    /// 一键还债支持币种
    pub const ONE_CLICK_REPAY_CURRENCY_LIST: &str = "/api/v5/trade/one-click-repay-currency-list";
    /// 一键还债
    pub const ONE_CLICK_REPAY: &str = "/api/v5/trade/one-click-repay";
    /// 一键还债历史
    pub const ONE_CLICK_REPAY_HISTORY: &str = "/api/v5/trade/one-click-repay-history";
}

/// 闪兑与一键还债 API。
pub trait ConvertApi {
    /// 查询可兑换币种
    fn get_convert_currencies(
        &self,
    ) -> impl std::future::Future<Output = Result<Vec<ConvertCurrency>>> + Send;

    /// 查询币对信息
    fn get_convert_currency_pair(
        &self,
        from_ccy: &str,
        to_ccy: &str,
    ) -> impl std::future::Future<Output = Result<Vec<ConvertCurrencyPair>>> + Send;

    /// 获取报价
    fn estimate_convert_quote(
        &self,
        params: EstimateQuoteParams,
    ) -> impl std::future::Future<Output = Result<Vec<EstimateQuoteResponse>>> + Send;

    /// 闪兑成交
    fn convert_trade(
        &self,
        request: ConvertTradeRequest,
    ) -> impl std::future::Future<Output = Result<Vec<ConvertGenericResponse>>> + Send;

    /// 查询闪兑历史
    fn get_convert_history(
        &self,
        params: Option<ConvertHistoryParams>,
    ) -> impl std::future::Future<Output = Result<Vec<ConvertHistoryRecord>>> + Send;

    /// Easy Convert 支持币种
    fn get_easy_convert_currency_list(
        &self,
    ) -> impl std::future::Future<Output = Result<Vec<ConvertGenericResponse>>> + Send;

    /// Easy Convert 兑换
    fn easy_convert(
        &self,
        request: EasyConvertRequest,
    ) -> impl std::future::Future<Output = Result<Vec<ConvertGenericResponse>>> + Send;

    /// Easy Convert 历史
    fn get_easy_convert_history(
        &self,
        after: Option<&str>,
        before: Option<&str>,
        limit: Option<u32>,
    ) -> impl std::future::Future<Output = Result<Vec<ConvertGenericResponse>>> + Send;

    /// 一键还债支持币种
    fn get_one_click_repay_currency_list(
        &self,
    ) -> impl std::future::Future<Output = Result<Vec<ConvertGenericResponse>>> + Send;

    /// 一键还债
    fn one_click_repay(
        &self,
        request: OneClickRepayRequest,
    ) -> impl std::future::Future<Output = Result<Vec<ConvertGenericResponse>>> + Send;

    /// 一键还债历史
    fn get_one_click_repay_history(
        &self,
        after: Option<&str>,
        before: Option<&str>,
        limit: Option<u32>,
    ) -> impl std::future::Future<Output = Result<Vec<ConvertGenericResponse>>> + Send;
}

impl ConvertApi for OkxRestClient {
    async fn get_convert_currencies(&self) -> Result<Vec<ConvertCurrency>> {
        self.get(endpoints::CONVERT_CURRENCIES, None::<&()>).await
    }

    async fn get_convert_currency_pair(
        &self,
        from_ccy: &str,
        to_ccy: &str,
    ) -> Result<Vec<ConvertCurrencyPair>> {
        #[derive(serde::Serialize)]
        struct Params<'a> {
            #[serde(rename = "fromCcy")]
            from_ccy: &'a str,
            #[serde(rename = "toCcy")]
            to_ccy: &'a str,
        }
        let params = Params { from_ccy, to_ccy };
        self.get(endpoints::CONVERT_CURRENCY_PAIR, Some(&params))
            .await
    }

    async fn estimate_convert_quote(
        &self,
        params: EstimateQuoteParams,
    ) -> Result<Vec<EstimateQuoteResponse>> {
        self.post(endpoints::CONVERT_ESTIMATE_QUOTE, &params).await
    }

    async fn convert_trade(
        &self,
        request: ConvertTradeRequest,
    ) -> Result<Vec<ConvertGenericResponse>> {
        self.post(endpoints::CONVERT_TRADE, &request).await
    }

    async fn get_convert_history(
        &self,
        params: Option<ConvertHistoryParams>,
    ) -> Result<Vec<ConvertHistoryRecord>> {
        self.get(endpoints::CONVERT_HISTORY, params.as_ref()).await
    }

    async fn get_easy_convert_currency_list(&self) -> Result<Vec<ConvertGenericResponse>> {
        self.get(endpoints::EASY_CONVERT_CURRENCY_LIST, None::<&()>)
            .await
    }

    async fn easy_convert(
        &self,
        request: EasyConvertRequest,
    ) -> Result<Vec<ConvertGenericResponse>> {
        self.post(endpoints::EASY_CONVERT, &request).await
    }

    async fn get_easy_convert_history(
        &self,
        after: Option<&str>,
        before: Option<&str>,
        limit: Option<u32>,
    ) -> Result<Vec<ConvertGenericResponse>> {
        #[derive(serde::Serialize)]
        struct Params<'a> {
            #[serde(skip_serializing_if = "Option::is_none")]
            after: Option<&'a str>,
            #[serde(skip_serializing_if = "Option::is_none")]
            before: Option<&'a str>,
            #[serde(skip_serializing_if = "Option::is_none")]
            limit: Option<u32>,
        }
        let params = Params {
            after,
            before,
            limit,
        };
        self.get(endpoints::EASY_CONVERT_HISTORY, Some(&params))
            .await
    }

    async fn get_one_click_repay_currency_list(&self) -> Result<Vec<ConvertGenericResponse>> {
        self.get(endpoints::ONE_CLICK_REPAY_CURRENCY_LIST, None::<&()>)
            .await
    }

    async fn one_click_repay(
        &self,
        request: OneClickRepayRequest,
    ) -> Result<Vec<ConvertGenericResponse>> {
        self.post(endpoints::ONE_CLICK_REPAY, &request).await
    }

    async fn get_one_click_repay_history(
        &self,
        after: Option<&str>,
        before: Option<&str>,
        limit: Option<u32>,
    ) -> Result<Vec<ConvertGenericResponse>> {
        #[derive(serde::Serialize)]
        struct Params<'a> {
            #[serde(skip_serializing_if = "Option::is_none")]
            after: Option<&'a str>,
            #[serde(skip_serializing_if = "Option::is_none")]
            before: Option<&'a str>,
            #[serde(skip_serializing_if = "Option::is_none")]
            limit: Option<u32>,
        }
        let params = Params {
            after,
            before,
            limit,
        };
        self.get(endpoints::ONE_CLICK_REPAY_HISTORY, Some(&params))
            .await
    }
}
