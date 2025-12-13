//! Public/Market API #[pymethods] 块

use pyo3::prelude::*;

use okx_rest::api::market::{GetCandlesParams, GetHistoryTradesParams, GetMarkPriceCandlesParams};

use crate::public as public_impl;
use crate::types::{
    PyCandle, PyFundingRate, PyIndexTicker, PyMarkPrice, PyOrderBook, PyPublicTrade, PyTicker,
};

use super::PyOkxClient;

#[pymethods]
impl PyOkxClient {
    // ==================== Market API ====================

    /// 获取单个产品行情。
    fn get_ticker(&self, inst_id: &str) -> PyResult<Option<PyTicker>> {
        public_impl::sync::get_ticker(self, inst_id)
    }

    /// 获取所有产品行情信息。
    #[pyo3(signature = (inst_type, uly=None, inst_family=None))]
    fn get_tickers(
        &self,
        inst_type: &str,
        uly: Option<&str>,
        inst_family: Option<&str>,
    ) -> PyResult<Vec<PyTicker>> {
        public_impl::sync::get_tickers(self, inst_type, uly, inst_family)
    }

    /// 获取深度数据。
    #[pyo3(signature = (inst_id, depth=None))]
    fn get_orderbook(&self, inst_id: &str, depth: Option<u32>) -> PyResult<Option<PyOrderBook>> {
        public_impl::sync::get_orderbook(self, inst_id, depth)
    }

    /// 获取 K 线数据。
    #[pyo3(signature = (inst_id, bar=None, after=None, before=None, limit=None))]
    fn get_candles(
        &self,
        inst_id: &str,
        bar: Option<&str>,
        after: Option<&str>,
        before: Option<&str>,
        limit: Option<u32>,
    ) -> PyResult<Vec<PyCandle>> {
        let params = GetCandlesParams {
            inst_id: inst_id.to_string(),
            bar: bar.map(String::from),
            after: after.map(String::from),
            before: before.map(String::from),
            limit: limit.map(|v| v.to_string()),
        };
        public_impl::sync::get_candles(self, params)
    }

    /// 获取历史 K 线数据。
    #[pyo3(signature = (inst_id, bar=None, after=None, before=None, limit=None))]
    fn get_history_candles(
        &self,
        inst_id: &str,
        bar: Option<&str>,
        after: Option<&str>,
        before: Option<&str>,
        limit: Option<u32>,
    ) -> PyResult<Vec<PyCandle>> {
        let params = GetCandlesParams {
            inst_id: inst_id.to_string(),
            bar: bar.map(String::from),
            after: after.map(String::from),
            before: before.map(String::from),
            limit: limit.map(|v| v.to_string()),
        };
        public_impl::sync::get_history_candles(self, params)
    }

    /// 获取指数 K 线数据。
    #[pyo3(signature = (inst_id, bar=None, after=None, before=None, limit=None))]
    fn get_index_candles(
        &self,
        inst_id: &str,
        bar: Option<&str>,
        after: Option<&str>,
        before: Option<&str>,
        limit: Option<u32>,
    ) -> PyResult<Vec<PyCandle>> {
        let params = GetMarkPriceCandlesParams {
            inst_id: inst_id.to_string(),
            bar: bar.map(String::from),
            after: after.map(String::from),
            before: before.map(String::from),
            limit: limit.map(|v| v.to_string()),
        };
        public_impl::sync::get_index_candles(self, params)
    }

    /// 获取标记价格 K 线数据。
    #[pyo3(signature = (inst_id, bar=None, after=None, before=None, limit=None))]
    fn get_mark_price_candles(
        &self,
        inst_id: &str,
        bar: Option<&str>,
        after: Option<&str>,
        before: Option<&str>,
        limit: Option<u32>,
    ) -> PyResult<Vec<PyCandle>> {
        let params = GetMarkPriceCandlesParams {
            inst_id: inst_id.to_string(),
            bar: bar.map(String::from),
            after: after.map(String::from),
            before: before.map(String::from),
            limit: limit.map(|v| v.to_string()),
        };
        public_impl::sync::get_mark_price_candles(self, params)
    }

    /// 获取最近成交记录。
    #[pyo3(signature = (inst_id, limit=None))]
    fn get_trades(&self, inst_id: &str, limit: Option<u32>) -> PyResult<Vec<PyPublicTrade>> {
        public_impl::sync::get_trades(self, inst_id, limit)
    }

    /// 获取历史成交记录。
    #[pyo3(signature = (inst_id, trade_type=None, after=None, before=None, limit=None))]
    fn get_history_trades(
        &self,
        inst_id: &str,
        trade_type: Option<&str>,
        after: Option<&str>,
        before: Option<&str>,
        limit: Option<u32>,
    ) -> PyResult<Vec<PyPublicTrade>> {
        let params = GetHistoryTradesParams {
            inst_id: inst_id.to_string(),
            r#type: trade_type.map(String::from),
            after: after.map(String::from),
            before: before.map(String::from),
            limit: limit.map(|v| v.to_string()),
        };
        public_impl::sync::get_history_trades(self, params)
    }

    /// 获取轻量级深度数据。
    fn get_orderbook_lite(&self, inst_id: &str) -> PyResult<Option<PyOrderBook>> {
        public_impl::sync::get_orderbook_lite(self, inst_id)
    }

    /// 获取大宗交易单个产品行情。
    fn get_block_ticker(&self, inst_id: &str) -> PyResult<Vec<Py<PyAny>>> {
        public_impl::sync::get_block_ticker(self, inst_id)
    }

    /// 获取大宗交易成交记录。
    fn get_block_trades(&self, inst_id: &str) -> PyResult<Vec<Py<PyAny>>> {
        public_impl::sync::get_block_trades(self, inst_id)
    }

    /// 获取大宗交易行情。
    #[pyo3(signature = (inst_type, uly=None, inst_family=None))]
    fn get_block_tickers(
        &self,
        inst_type: &str,
        uly: Option<&str>,
        inst_family: Option<&str>,
    ) -> PyResult<Vec<Py<PyAny>>> {
        public_impl::sync::get_block_tickers(self, inst_type, uly, inst_family)
    }

    /// 获取期权家族成交记录。
    fn get_option_family_trades(&self, inst_family: &str) -> PyResult<Vec<Py<PyAny>>> {
        public_impl::sync::get_option_family_trades(self, inst_family)
    }

    /// 获取指数行情。
    #[pyo3(signature = (quote_ccy=None, inst_id=None))]
    fn get_index_tickers(
        &self,
        quote_ccy: Option<&str>,
        inst_id: Option<&str>,
    ) -> PyResult<Vec<PyIndexTicker>> {
        public_impl::sync::get_index_tickers(self, quote_ccy, inst_id)
    }

    /// 获取平台 24 小时总成交量。
    fn get_platform_24_volume(&self) -> PyResult<Vec<Py<PyAny>>> {
        public_impl::sync::get_platform_24_volume(self)
    }

    /// 获取指数成分数据。
    fn get_index_components(&self, index: &str) -> PyResult<Vec<Py<PyAny>>> {
        public_impl::sync::get_index_components(self, index)
    }

    /// 获取汇率。
    fn get_exchange_rate(&self) -> PyResult<Vec<Py<PyAny>>> {
        public_impl::sync::get_exchange_rate(self)
    }

    // ==================== Public API ====================

    /// 获取交易产品基础信息。
    #[pyo3(signature = (inst_type, inst_id=None))]
    fn get_instruments(&self, inst_type: &str, inst_id: Option<&str>) -> PyResult<Vec<Py<PyAny>>> {
        public_impl::sync::get_instruments(self, inst_type, inst_id)
    }

    /// 获取期权产品报价最小变动价位（Tick Bands）。
    #[pyo3(signature = (inst_type, inst_family=None))]
    fn get_instrument_tick_bands(
        &self,
        inst_type: &str,
        inst_family: Option<&str>,
    ) -> PyResult<Vec<Py<PyAny>>> {
        public_impl::sync::get_instrument_tick_bands(self, inst_type, inst_family)
    }

    /// 获取公共期权成交数据。
    ///
    /// `inst_id` 与 `inst_family` 至少提供一个；如两者都提供，则优先使用 `inst_id`（底层与 Rust 实现一致）。
    #[pyo3(signature = (inst_id=None, inst_family=None, opt_type=None))]
    fn get_option_trades(
        &self,
        inst_id: Option<&str>,
        inst_family: Option<&str>,
        opt_type: Option<&str>,
    ) -> PyResult<Vec<Py<PyAny>>> {
        public_impl::sync::get_option_trades(self, inst_id, inst_family, opt_type)
    }

    /// 获取交割/行权历史。
    #[pyo3(signature = (inst_type, uly=None, inst_family=None, after=None, before=None, limit=None))]
    fn get_delivery_exercise_history(
        &self,
        inst_type: &str,
        uly: Option<&str>,
        inst_family: Option<&str>,
        after: Option<&str>,
        before: Option<&str>,
        limit: Option<&str>,
    ) -> PyResult<Vec<Py<PyAny>>> {
        public_impl::sync::get_delivery_exercise_history(
            self,
            inst_type,
            uly,
            inst_family,
            after,
            before,
            limit,
        )
    }

    /// 获取持仓总量。
    #[pyo3(signature = (inst_type, uly=None, inst_id=None, inst_family=None))]
    fn get_open_interest(
        &self,
        inst_type: &str,
        uly: Option<&str>,
        inst_id: Option<&str>,
        inst_family: Option<&str>,
    ) -> PyResult<Vec<Py<PyAny>>> {
        public_impl::sync::get_open_interest(self, inst_type, uly, inst_id, inst_family)
    }

    /// 获取仓位档位。
    #[pyo3(signature = (inst_type, td_mode, uly=None, inst_id=None, ccy=None, tier=None, inst_family=None))]
    fn get_position_tiers(
        &self,
        inst_type: &str,
        td_mode: &str,
        uly: Option<&str>,
        inst_id: Option<&str>,
        ccy: Option<&str>,
        tier: Option<&str>,
        inst_family: Option<&str>,
    ) -> PyResult<Vec<Py<PyAny>>> {
        public_impl::sync::get_position_tiers(
            self,
            inst_type,
            td_mode,
            uly,
            inst_id,
            ccy,
            tier,
            inst_family,
        )
    }

    /// 获取当前资金费率。
    fn get_funding_rate(&self, inst_id: &str) -> PyResult<Option<PyFundingRate>> {
        public_impl::sync::get_funding_rate(self, inst_id)
    }

    /// 获取资金费率历史。
    #[pyo3(signature = (inst_id, after=None, before=None, limit=None))]
    fn get_funding_rate_history(
        &self,
        inst_id: &str,
        after: Option<&str>,
        before: Option<&str>,
        limit: Option<&str>,
    ) -> PyResult<Vec<PyFundingRate>> {
        public_impl::sync::get_funding_rate_history(self, inst_id, after, before, limit)
    }

    /// 获取标记价格。
    #[pyo3(signature = (inst_type, uly=None, inst_family=None, inst_id=None))]
    fn get_mark_price(
        &self,
        inst_type: &str,
        uly: Option<&str>,
        inst_family: Option<&str>,
        inst_id: Option<&str>,
    ) -> PyResult<Vec<PyMarkPrice>> {
        public_impl::sync::get_mark_price(self, inst_type, uly, inst_family, inst_id)
    }

    /// 获取限价。
    fn get_price_limit(&self, inst_id: &str) -> PyResult<Vec<Py<PyAny>>> {
        public_impl::sync::get_price_limit(self, inst_id)
    }

    /// 获取期权定价。
    #[pyo3(signature = (uly=None, exp_time=None, inst_family=None))]
    fn get_opt_summary(
        &self,
        uly: Option<&str>,
        exp_time: Option<&str>,
        inst_family: Option<&str>,
    ) -> PyResult<Vec<Py<PyAny>>> {
        public_impl::sync::get_opt_summary(self, uly, exp_time, inst_family)
    }

    /// 获取预估交割/行权价格。
    fn get_estimated_price(&self, inst_id: &str) -> PyResult<Vec<Py<PyAny>>> {
        public_impl::sync::get_estimated_price(self, inst_id)
    }

    /// 获取免息额度和币种折算率。
    #[pyo3(signature = (ccy=None))]
    fn get_discount_interest_free_quota(&self, ccy: Option<&str>) -> PyResult<Vec<Py<PyAny>>> {
        public_impl::sync::get_discount_interest_free_quota(self, ccy)
    }

    /// 获取市场借币杠杆利率和借币限额。
    fn get_interest_rate_loan_quota(&self) -> PyResult<Vec<Py<PyAny>>> {
        public_impl::sync::get_interest_rate_loan_quota(self)
    }

    /// 获取 VIP 借币杠杆利率和借币限额。
    fn get_vip_interest_rate_loan_quota(&self) -> PyResult<Vec<Py<PyAny>>> {
        public_impl::sync::get_vip_interest_rate_loan_quota(self)
    }

    /// 获取标的指数。
    #[pyo3(signature = (inst_type=None))]
    fn get_underlying(&self, inst_type: Option<&str>) -> PyResult<Vec<Py<PyAny>>> {
        public_impl::sync::get_underlying(self, inst_type)
    }

    /// 获取风险准备金余额。
    #[pyo3(signature = (inst_type=None, type_=None, uly=None, ccy=None, before=None, after=None, limit=None, inst_family=None))]
    fn get_insurance_fund(
        &self,
        inst_type: Option<&str>,
        type_: Option<&str>,
        uly: Option<&str>,
        ccy: Option<&str>,
        before: Option<&str>,
        after: Option<&str>,
        limit: Option<&str>,
        inst_family: Option<&str>,
    ) -> PyResult<Vec<Py<PyAny>>> {
        public_impl::sync::get_insurance_fund(
            self,
            inst_type,
            type_,
            uly,
            ccy,
            before,
            after,
            limit,
            inst_family,
        )
    }

    /// 张币转换。
    #[pyo3(signature = (inst_id, sz, px=None, unit=None, op_type=None))]
    fn get_convert_contract_coin(
        &self,
        inst_id: &str,
        sz: &str,
        px: Option<&str>,
        unit: Option<&str>,
        op_type: Option<&str>,
    ) -> PyResult<Vec<Py<PyAny>>> {
        public_impl::sync::get_convert_contract_coin(
            self,
            op_type,
            Some(inst_id),
            Some(sz),
            px,
            unit,
        )
    }

    // ==================== Status API ====================

    /// 获取系统时间。
    fn get_system_time(&self) -> PyResult<String> {
        public_impl::sync::get_system_time(self)
    }

    /// 获取系统状态。
    #[pyo3(signature = (state=None))]
    fn get_system_status(&self, state: Option<&str>) -> PyResult<Vec<Py<PyAny>>> {
        public_impl::sync::get_system_status(self, state)
    }
}
