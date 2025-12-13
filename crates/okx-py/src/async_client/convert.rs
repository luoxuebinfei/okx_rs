//! Convert API #[pymethods] 块（异步）

use pyo3::prelude::*;

use okx_core::types::{
    ConvertHistoryParams, ConvertTradeRequest, EasyConvertRequest, EstimateQuoteParams,
    OneClickRepayRequest,
};

use crate::convert as convert_impl;

use super::PyAsyncOkxClient;

#[pymethods]
impl PyAsyncOkxClient {
    /// 获取可兑换币种列表（异步）。
    fn get_convert_currencies<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        convert_impl::async_api::get_convert_currencies(self, py)
    }

    /// 获取币对信息（异步）。
    fn get_convert_currency_pair<'py>(
        &self,
        py: Python<'py>,
        from_ccy: String,
        to_ccy: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        convert_impl::async_api::get_convert_currency_pair(self, py, from_ccy, to_ccy)
    }

    /// 闪兑预估询价（异步）。
    #[pyo3(signature = (base_ccy, quote_ccy, side, rfq_sz, rfq_sz_ccy=None, tag=None))]
    fn estimate_convert_quote<'py>(
        &self,
        py: Python<'py>,
        base_ccy: String,
        quote_ccy: String,
        side: String,
        rfq_sz: String,
        rfq_sz_ccy: Option<String>,
        tag: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let params = EstimateQuoteParams {
            base_ccy: base_ccy.clone(),
            quote_ccy,
            side,
            rfq_sz,
            rfq_sz_ccy: rfq_sz_ccy.unwrap_or(base_ccy),
            tag,
            cl_q_req_id: None,
        };
        convert_impl::async_api::estimate_convert_quote(self, py, params)
    }

    /// 闪兑交易（异步）。
    #[pyo3(signature = (quote_id, base_ccy, quote_ccy, side, sz, sz_ccy, tag=None))]
    fn convert_trade<'py>(
        &self,
        py: Python<'py>,
        quote_id: String,
        base_ccy: String,
        quote_ccy: String,
        side: String,
        sz: String,
        sz_ccy: String,
        tag: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let request = ConvertTradeRequest {
            quote_id,
            base_ccy,
            quote_ccy,
            side,
            sz,
            sz_ccy,
            tag,
            cl_t_req_id: None,
        };
        convert_impl::async_api::convert_trade(self, py, request)
    }

    /// 获取闪兑历史（异步）。
    #[pyo3(signature = (after=None, before=None, limit=None, tag=None))]
    fn get_convert_history<'py>(
        &self,
        py: Python<'py>,
        after: Option<String>,
        before: Option<String>,
        limit: Option<u32>,
        tag: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let params = if after.is_some() || before.is_some() || limit.is_some() || tag.is_some() {
            Some(ConvertHistoryParams {
                after,
                before,
                limit,
                tag,
            })
        } else {
            None
        };
        convert_impl::async_api::get_convert_history(self, py, params)
    }

    /// 获取小额兑换币种列表（异步）。
    fn get_easy_convert_currency_list<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        convert_impl::async_api::get_easy_convert_currency_list(self, py)
    }

    /// 小额兑换（异步）。
    fn easy_convert<'py>(
        &self,
        py: Python<'py>,
        from_ccy: Vec<String>,
        to_ccy: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let request = EasyConvertRequest { from_ccy, to_ccy };
        convert_impl::async_api::easy_convert(self, py, request)
    }

    /// 获取小额兑换历史（异步）。
    #[pyo3(signature = (after=None, before=None, limit=None))]
    fn get_easy_convert_history<'py>(
        &self,
        py: Python<'py>,
        after: Option<String>,
        before: Option<String>,
        limit: Option<u32>,
    ) -> PyResult<Bound<'py, PyAny>> {
        convert_impl::async_api::get_easy_convert_history(self, py, after, before, limit)
    }

    /// 获取一键还债币种列表（异步）。
    fn get_one_click_repay_currency_list<'py>(
        &self,
        py: Python<'py>,
    ) -> PyResult<Bound<'py, PyAny>> {
        convert_impl::async_api::get_one_click_repay_currency_list(self, py)
    }

    /// 一键还债交易（异步）。
    fn one_click_repay<'py>(
        &self,
        py: Python<'py>,
        debt_ccy: Vec<String>,
        repay_ccy: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let request = OneClickRepayRequest {
            debt_ccy,
            repay_ccy,
        };
        convert_impl::async_api::one_click_repay(self, py, request)
    }

    /// 获取一键还债历史（异步）。
    #[pyo3(signature = (after=None, before=None, limit=None))]
    fn get_one_click_repay_history<'py>(
        &self,
        py: Python<'py>,
        after: Option<String>,
        before: Option<String>,
        limit: Option<u32>,
    ) -> PyResult<Bound<'py, PyAny>> {
        convert_impl::async_api::get_one_click_repay_history(self, py, after, before, limit)
    }
}
