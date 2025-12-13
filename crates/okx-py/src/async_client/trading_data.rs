//! Trading Data (Rubik) API #[pymethods] 块（异步）

use pyo3::prelude::*;

use crate::trading_data as trading_data_impl;

use super::PyAsyncOkxClient;

#[pymethods]
impl PyAsyncOkxClient {
    /// 获取支持的币种列表（异步）。
    fn get_support_coin<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        trading_data_impl::async_api::get_support_coin(self, py)
    }

    /// 获取主动买入/卖出情况（异步）。
    #[pyo3(signature = (params_json=None))]
    fn get_taker_volume<'py>(
        &self,
        py: Python<'py>,
        params_json: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        trading_data_impl::async_api::get_taker_volume(self, py, params_json)
    }

    /// 获取杠杆多空比（异步）。
    #[pyo3(signature = (params_json=None))]
    fn get_margin_lending_ratio<'py>(
        &self,
        py: Python<'py>,
        params_json: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        trading_data_impl::async_api::get_margin_lending_ratio(self, py, params_json)
    }

    /// 获取多空持仓人数比（异步）。
    #[pyo3(signature = (params_json=None))]
    fn get_long_short_ratio<'py>(
        &self,
        py: Python<'py>,
        params_json: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        trading_data_impl::async_api::get_long_short_ratio(self, py, params_json)
    }

    /// 获取合约持仓量及交易量（异步）。
    #[pyo3(signature = (params_json=None))]
    fn get_contracts_open_interest_volume<'py>(
        &self,
        py: Python<'py>,
        params_json: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        trading_data_impl::async_api::get_contracts_open_interest_volume(self, py, params_json)
    }

    /// 获取期权持仓量及交易量（异步）。
    #[pyo3(signature = (params_json=None))]
    fn get_options_open_interest_volume<'py>(
        &self,
        py: Python<'py>,
        params_json: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        trading_data_impl::async_api::get_options_open_interest_volume(self, py, params_json)
    }

    /// 获取看涨/看跌期权合约持仓量比值（异步）。
    #[pyo3(signature = (params_json=None))]
    fn get_put_call_ratio<'py>(
        &self,
        py: Python<'py>,
        params_json: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        trading_data_impl::async_api::get_put_call_ratio(self, py, params_json)
    }

    /// 获取期权持仓量及交易量（按到期日）（异步）。
    #[pyo3(signature = (params_json=None))]
    fn get_open_interest_volume_expiry<'py>(
        &self,
        py: Python<'py>,
        params_json: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        trading_data_impl::async_api::get_open_interest_volume_expiry(self, py, params_json)
    }

    /// 获取期权持仓量及交易量（按执行价）（异步）。
    #[pyo3(signature = (params_json=None))]
    fn get_interest_volume_strike<'py>(
        &self,
        py: Python<'py>,
        params_json: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        trading_data_impl::async_api::get_interest_volume_strike(self, py, params_json)
    }

    /// 获取期权主动买入/卖出情况（异步）。
    #[pyo3(signature = (params_json=None))]
    fn get_taker_flow<'py>(
        &self,
        py: Python<'py>,
        params_json: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        trading_data_impl::async_api::get_taker_flow(self, py, params_json)
    }
}
