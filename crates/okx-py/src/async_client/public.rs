//! Public/Market API #[pymethods] 块（异步）

use pyo3::prelude::*;

use okx_rest::api::market::GetCandlesParams;

use crate::public as public_impl;

use super::PyAsyncOkxClient;

#[pymethods]
impl PyAsyncOkxClient {
    /// 获取所有产品行情信息（异步）。
    fn get_ticker<'py>(&self, py: Python<'py>, inst_id: String) -> PyResult<Bound<'py, PyAny>> {
        public_impl::async_api::get_ticker(self, py, inst_id)
    }

    /// 获取所有产品行情信息（异步）。
    #[pyo3(signature = (inst_type, uly=None, inst_family=None))]
    fn get_tickers<'py>(
        &self,
        py: Python<'py>,
        inst_type: String,
        uly: Option<String>,
        inst_family: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        public_impl::async_api::get_tickers(self, py, inst_type, uly, inst_family)
    }

    /// 获取深度数据（异步）。
    #[pyo3(signature = (inst_id, depth=None))]
    fn get_orderbook<'py>(
        &self,
        py: Python<'py>,
        inst_id: String,
        depth: Option<u32>,
    ) -> PyResult<Bound<'py, PyAny>> {
        public_impl::async_api::get_orderbook(self, py, inst_id, depth)
    }

    /// 获取 K 线数据（异步）。
    #[pyo3(signature = (inst_id, bar=None, after=None, before=None, limit=None))]
    fn get_candles<'py>(
        &self,
        py: Python<'py>,
        inst_id: String,
        bar: Option<String>,
        after: Option<String>,
        before: Option<String>,
        limit: Option<u32>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let params = GetCandlesParams {
            inst_id,
            bar,
            after,
            before,
            limit: limit.map(|v| v.to_string()),
        };
        public_impl::async_api::get_candles(self, py, params)
    }

    /// 获取最近成交记录（异步）。
    #[pyo3(signature = (inst_id, limit=None))]
    fn get_trades<'py>(
        &self,
        py: Python<'py>,
        inst_id: String,
        limit: Option<u32>,
    ) -> PyResult<Bound<'py, PyAny>> {
        public_impl::async_api::get_trades(self, py, inst_id, limit)
    }

    /// 获取轻量级深度数据（异步）。
    fn get_orderbook_lite<'py>(
        &self,
        py: Python<'py>,
        inst_id: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        public_impl::async_api::get_orderbook_lite(self, py, inst_id)
    }

    /// 获取大宗交易单个产品行情（异步）。
    fn get_block_ticker<'py>(
        &self,
        py: Python<'py>,
        inst_id: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        public_impl::async_api::get_block_ticker(self, py, inst_id)
    }

    /// 获取期权家族成交记录（异步）。
    fn get_option_family_trades<'py>(
        &self,
        py: Python<'py>,
        inst_family: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        public_impl::async_api::get_option_family_trades(self, py, inst_family)
    }

    /// 获取指数行情（异步）。
    #[pyo3(signature = (quote_ccy=None, inst_id=None))]
    fn get_index_tickers<'py>(
        &self,
        py: Python<'py>,
        quote_ccy: Option<String>,
        inst_id: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        public_impl::async_api::get_index_tickers(self, py, quote_ccy, inst_id)
    }

    /// 获取平台 24 小时总成交量（异步）。
    fn get_platform_24_volume<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        public_impl::async_api::get_platform_24_volume(self, py)
    }

    /// 获取指数成分数据（异步）。
    fn get_index_components<'py>(
        &self,
        py: Python<'py>,
        index: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        public_impl::async_api::get_index_components(self, py, index)
    }

    /// 获取汇率（异步）。
    fn get_exchange_rate<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        public_impl::async_api::get_exchange_rate(self, py)
    }

    /// 获取交易产品基础信息（异步）。
    #[pyo3(signature = (inst_type, inst_id=None))]
    fn get_instruments<'py>(
        &self,
        py: Python<'py>,
        inst_type: String,
        inst_id: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        public_impl::async_api::get_instruments(self, py, inst_type, inst_id)
    }

    /// 获取期权产品报价最小变动价位（Tick Bands，异步）。
    #[pyo3(signature = (inst_type, inst_family=None))]
    fn get_instrument_tick_bands<'py>(
        &self,
        py: Python<'py>,
        inst_type: String,
        inst_family: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        public_impl::async_api::get_instrument_tick_bands(self, py, inst_type, inst_family)
    }

    /// 获取公共期权成交数据（异步）。
    ///
    /// `inst_id` 与 `inst_family` 至少提供一个；如两者都提供，则优先使用 `inst_id`（底层与 Rust 实现一致）。
    #[pyo3(signature = (inst_id=None, inst_family=None, opt_type=None))]
    fn get_option_trades<'py>(
        &self,
        py: Python<'py>,
        inst_id: Option<String>,
        inst_family: Option<String>,
        opt_type: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        public_impl::async_api::get_option_trades(self, py, inst_id, inst_family, opt_type)
    }

    /// 获取当前资金费率（异步）。
    fn get_funding_rate<'py>(
        &self,
        py: Python<'py>,
        inst_id: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        public_impl::async_api::get_funding_rate(self, py, inst_id)
    }

    /// 获取资金费率历史（异步）。
    #[pyo3(signature = (inst_id, after=None, before=None, limit=None))]
    fn get_funding_rate_history<'py>(
        &self,
        py: Python<'py>,
        inst_id: String,
        after: Option<String>,
        before: Option<String>,
        limit: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        public_impl::async_api::get_funding_rate_history(self, py, inst_id, after, before, limit)
    }

    /// 获取标记价格（异步）。
    #[pyo3(signature = (inst_type, uly=None, inst_family=None, inst_id=None))]
    fn get_mark_price<'py>(
        &self,
        py: Python<'py>,
        inst_type: String,
        uly: Option<String>,
        inst_family: Option<String>,
        inst_id: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        public_impl::async_api::get_mark_price(self, py, inst_type, uly, inst_family, inst_id)
    }

    /// 获取系统时间（异步）。
    fn get_system_time<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        public_impl::async_api::get_system_time(self, py)
    }

    /// 获取系统状态（异步）。
    #[pyo3(signature = (state=None))]
    fn get_system_status<'py>(
        &self,
        py: Python<'py>,
        state: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        public_impl::async_api::get_system_status(self, py, state)
    }

    /// 获取大宗交易所有产品行情（异步）。
    #[pyo3(signature = (inst_type, uly=None, inst_family=None))]
    fn get_block_tickers<'py>(
        &self,
        py: Python<'py>,
        inst_type: String,
        uly: Option<String>,
        inst_family: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        public_impl::async_api::get_block_tickers(self, py, inst_type, uly, inst_family)
    }

    /// 获取历史 K 线数据（异步）。
    #[pyo3(signature = (inst_id, bar=None, after=None, before=None, limit=None))]
    fn get_history_candles<'py>(
        &self,
        py: Python<'py>,
        inst_id: String,
        bar: Option<String>,
        after: Option<String>,
        before: Option<String>,
        limit: Option<u32>,
    ) -> PyResult<Bound<'py, PyAny>> {
        use okx_rest::api::market::GetCandlesParams;
        let params = GetCandlesParams {
            inst_id,
            bar,
            after,
            before,
            limit: limit.map(|v| v.to_string()),
        };
        public_impl::async_api::get_history_candles(self, py, params)
    }

    /// 获取指数 K 线数据（异步）。
    #[pyo3(signature = (inst_id, bar=None, after=None, before=None, limit=None))]
    fn get_index_candles<'py>(
        &self,
        py: Python<'py>,
        inst_id: String,
        bar: Option<String>,
        after: Option<String>,
        before: Option<String>,
        limit: Option<u32>,
    ) -> PyResult<Bound<'py, PyAny>> {
        use okx_rest::api::market::GetMarkPriceCandlesParams;
        let params = GetMarkPriceCandlesParams {
            inst_id,
            bar,
            after,
            before,
            limit: limit.map(|v| v.to_string()),
        };
        public_impl::async_api::get_index_candles(self, py, params)
    }

    /// 获取标记价格 K 线数据（异步）。
    #[pyo3(signature = (inst_id, bar=None, after=None, before=None, limit=None))]
    fn get_mark_price_candles<'py>(
        &self,
        py: Python<'py>,
        inst_id: String,
        bar: Option<String>,
        after: Option<String>,
        before: Option<String>,
        limit: Option<u32>,
    ) -> PyResult<Bound<'py, PyAny>> {
        use okx_rest::api::market::GetMarkPriceCandlesParams;
        let params = GetMarkPriceCandlesParams {
            inst_id,
            bar,
            after,
            before,
            limit: limit.map(|v| v.to_string()),
        };
        public_impl::async_api::get_mark_price_candles(self, py, params)
    }

    /// 获取历史成交记录（异步）。
    #[pyo3(signature = (inst_id, r#type=None, after=None, before=None, limit=None))]
    fn get_history_trades<'py>(
        &self,
        py: Python<'py>,
        inst_id: String,
        r#type: Option<String>,
        after: Option<String>,
        before: Option<String>,
        limit: Option<u32>,
    ) -> PyResult<Bound<'py, PyAny>> {
        use okx_rest::api::market::GetHistoryTradesParams;
        let params = GetHistoryTradesParams {
            inst_id,
            r#type,
            after,
            before,
            limit: limit.map(|v| v.to_string()),
        };
        public_impl::async_api::get_history_trades(self, py, params)
    }

    /// 获取大宗交易成交记录（异步）。
    fn get_block_trades<'py>(
        &self,
        py: Python<'py>,
        inst_id: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        public_impl::async_api::get_block_trades(self, py, inst_id)
    }

    /// 获取交割/行权历史（异步）。
    #[pyo3(signature = (inst_type, uly=None, inst_family=None, after=None, before=None, limit=None))]
    fn get_delivery_exercise_history<'py>(
        &self,
        py: Python<'py>,
        inst_type: String,
        uly: Option<String>,
        inst_family: Option<String>,
        after: Option<String>,
        before: Option<String>,
        limit: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        public_impl::async_api::get_delivery_exercise_history(
            self,
            py,
            inst_type,
            uly,
            inst_family,
            after,
            before,
            limit,
        )
    }

    /// 获取持仓总量（异步）。
    #[pyo3(signature = (inst_type, uly=None, inst_id=None, inst_family=None))]
    fn get_open_interest<'py>(
        &self,
        py: Python<'py>,
        inst_type: String,
        uly: Option<String>,
        inst_id: Option<String>,
        inst_family: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        public_impl::async_api::get_open_interest(self, py, inst_type, uly, inst_id, inst_family)
    }

    /// 获取持仓档位（异步）。
    #[pyo3(signature = (inst_type, td_mode, uly=None, inst_id=None, ccy=None, tier=None, inst_family=None))]
    fn get_position_tiers<'py>(
        &self,
        py: Python<'py>,
        inst_type: String,
        td_mode: String,
        uly: Option<String>,
        inst_id: Option<String>,
        ccy: Option<String>,
        tier: Option<String>,
        inst_family: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        public_impl::async_api::get_position_tiers(
            self,
            py,
            inst_type,
            td_mode,
            uly,
            inst_id,
            ccy,
            tier,
            inst_family,
        )
    }

    /// 获取限价（异步）。
    fn get_price_limit<'py>(
        &self,
        py: Python<'py>,
        inst_id: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        public_impl::async_api::get_price_limit(self, py, inst_id)
    }

    /// 获取期权定价（异步）。
    #[pyo3(signature = (uly=None, exp_time=None, inst_family=None))]
    fn get_opt_summary<'py>(
        &self,
        py: Python<'py>,
        uly: Option<String>,
        exp_time: Option<String>,
        inst_family: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        public_impl::async_api::get_opt_summary(self, py, uly, exp_time, inst_family)
    }

    /// 获取预估交割/行权价格（异步）。
    fn get_estimated_price<'py>(
        &self,
        py: Python<'py>,
        inst_id: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        public_impl::async_api::get_estimated_price(self, py, inst_id)
    }

    /// 获取折价率和优惠利率（异步）。
    #[pyo3(signature = (ccy=None))]
    fn get_discount_interest_free_quota<'py>(
        &self,
        py: Python<'py>,
        ccy: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        public_impl::async_api::get_discount_interest_free_quota(self, py, ccy)
    }

    /// 获取借币利率和限额（异步）。
    fn get_interest_rate_loan_quota<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        public_impl::async_api::get_interest_rate_loan_quota(self, py)
    }

    /// 获取 VIP 借币利率和限额（异步）。
    fn get_vip_interest_rate_loan_quota<'py>(
        &self,
        py: Python<'py>,
    ) -> PyResult<Bound<'py, PyAny>> {
        public_impl::async_api::get_vip_interest_rate_loan_quota(self, py)
    }

    /// 获取标的（异步）。
    #[pyo3(signature = (inst_type=None))]
    fn get_underlying<'py>(
        &self,
        py: Python<'py>,
        inst_type: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        public_impl::async_api::get_underlying(self, py, inst_type)
    }

    /// 获取风险准备金余额（异步）。
    #[pyo3(signature = (inst_type=None, type_=None, uly=None, ccy=None, before=None, after=None, limit=None, inst_family=None))]
    fn get_insurance_fund<'py>(
        &self,
        py: Python<'py>,
        inst_type: Option<String>,
        type_: Option<String>,
        uly: Option<String>,
        ccy: Option<String>,
        before: Option<String>,
        after: Option<String>,
        limit: Option<String>,
        inst_family: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        public_impl::async_api::get_insurance_fund(
            self,
            py,
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

    /// 获取合约币种转换（异步）。
    #[pyo3(signature = (type_=None, inst_id=None, sz=None, px=None, unit=None))]
    fn get_convert_contract_coin<'py>(
        &self,
        py: Python<'py>,
        type_: Option<String>,
        inst_id: Option<String>,
        sz: Option<String>,
        px: Option<String>,
        unit: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        public_impl::async_api::get_convert_contract_coin(self, py, type_, inst_id, sz, px, unit)
    }
}
