//! Convert API #[pymethods] 块

use pyo3::prelude::*;

use okx_core::types::{
    ConvertHistoryParams, ConvertTradeRequest, EasyConvertRequest, EstimateQuoteParams,
    OneClickRepayRequest,
};

use crate::convert as convert_impl;

use super::PyOkxClient;

#[pymethods]
impl PyOkxClient {
    // ==================== Convert API ====================

    /// 获取可兑换币种列表。
    fn get_convert_currencies(&self) -> PyResult<Vec<Py<PyAny>>> {
        convert_impl::sync::get_convert_currencies(self)
    }

    /// 获取币对信息。
    fn get_convert_currency_pair(&self, from_ccy: &str, to_ccy: &str) -> PyResult<Vec<Py<PyAny>>> {
        convert_impl::sync::get_convert_currency_pair(self, from_ccy, to_ccy)
    }

    /// 闪兑预估询价。
    #[pyo3(signature = (base_ccy, quote_ccy, side, rfq_sz, rfq_sz_ccy=None, tag=None))]
    fn estimate_convert_quote(
        &self,
        base_ccy: &str,
        quote_ccy: &str,
        side: &str,
        rfq_sz: &str,
        rfq_sz_ccy: Option<&str>,
        tag: Option<&str>,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let params = EstimateQuoteParams {
            base_ccy: base_ccy.to_string(),
            quote_ccy: quote_ccy.to_string(),
            side: side.to_string(),
            rfq_sz: rfq_sz.to_string(),
            rfq_sz_ccy: rfq_sz_ccy.unwrap_or(base_ccy).to_string(),
            tag: tag.map(String::from),
            cl_q_req_id: None,
        };
        convert_impl::sync::estimate_convert_quote(self, params)
    }

    /// 闪兑交易。
    #[pyo3(signature = (quote_id, base_ccy, quote_ccy, side, sz, sz_ccy, tag=None))]
    fn convert_trade(
        &self,
        quote_id: &str,
        base_ccy: &str,
        quote_ccy: &str,
        side: &str,
        sz: &str,
        sz_ccy: &str,
        tag: Option<&str>,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let request = ConvertTradeRequest {
            quote_id: quote_id.to_string(),
            base_ccy: base_ccy.to_string(),
            quote_ccy: quote_ccy.to_string(),
            side: side.to_string(),
            sz: sz.to_string(),
            sz_ccy: sz_ccy.to_string(),
            tag: tag.map(String::from),
            cl_t_req_id: None,
        };
        convert_impl::sync::convert_trade(self, request)
    }

    /// 获取闪兑历史。
    #[pyo3(signature = (after=None, before=None, limit=None, tag=None))]
    fn get_convert_history(
        &self,
        after: Option<&str>,
        before: Option<&str>,
        limit: Option<u32>,
        tag: Option<&str>,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let params = if after.is_some() || before.is_some() || limit.is_some() || tag.is_some() {
            Some(ConvertHistoryParams {
                after: after.map(String::from),
                before: before.map(String::from),
                limit,
                tag: tag.map(String::from),
            })
        } else {
            None
        };
        convert_impl::sync::get_convert_history(self, params)
    }

    // ==================== Easy Convert API ====================

    /// 获取小额兑换币种列表。
    fn get_easy_convert_currency_list(&self) -> PyResult<Vec<Py<PyAny>>> {
        convert_impl::sync::get_easy_convert_currency_list(self)
    }

    /// 小额兑换。
    fn easy_convert(&self, from_ccy: Vec<String>, to_ccy: &str) -> PyResult<Vec<Py<PyAny>>> {
        let request = EasyConvertRequest {
            from_ccy,
            to_ccy: to_ccy.to_string(),
        };
        convert_impl::sync::easy_convert(self, request)
    }

    /// 获取小额兑换历史。
    #[pyo3(signature = (after=None, before=None, limit=None))]
    fn get_easy_convert_history(
        &self,
        after: Option<&str>,
        before: Option<&str>,
        limit: Option<u32>,
    ) -> PyResult<Vec<Py<PyAny>>> {
        convert_impl::sync::get_easy_convert_history(self, after, before, limit)
    }

    // ==================== One-Click Repay API ====================

    /// 获取一键还债币种列表。
    fn get_one_click_repay_currency_list(&self) -> PyResult<Vec<Py<PyAny>>> {
        convert_impl::sync::get_one_click_repay_currency_list(self)
    }

    /// 一键还债交易。
    fn one_click_repay(&self, debt_ccy: Vec<String>, repay_ccy: &str) -> PyResult<Vec<Py<PyAny>>> {
        let request = OneClickRepayRequest {
            debt_ccy,
            repay_ccy: repay_ccy.to_string(),
        };
        convert_impl::sync::one_click_repay(self, request)
    }

    /// 获取一键还债历史。
    #[pyo3(signature = (after=None, before=None, limit=None))]
    fn get_one_click_repay_history(
        &self,
        after: Option<&str>,
        before: Option<&str>,
        limit: Option<u32>,
    ) -> PyResult<Vec<Py<PyAny>>> {
        convert_impl::sync::get_one_click_repay_history(self, after, before, limit)
    }
}
