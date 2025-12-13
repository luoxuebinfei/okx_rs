//! Trading Data (Rubik) API #[pymethods] 块

use pyo3::prelude::*;

use crate::trading_data as trading_data_impl;

use super::PyOkxClient;

#[pymethods]
impl PyOkxClient {
    // ==================== Trading Data API ====================

    /// 获取支持的币种列表。
    fn get_support_coin(&self) -> PyResult<Vec<Py<PyAny>>> {
        trading_data_impl::sync::get_support_coin(self)
    }

    /// 获取主动买入/卖出情况。
    #[pyo3(signature = (params_json=None))]
    fn get_taker_volume(&self, params_json: Option<&str>) -> PyResult<Vec<Py<PyAny>>> {
        trading_data_impl::sync::get_taker_volume(self, params_json)
    }

    /// 获取杠杆多空比。
    #[pyo3(signature = (params_json=None))]
    fn get_margin_lending_ratio(&self, params_json: Option<&str>) -> PyResult<Vec<Py<PyAny>>> {
        trading_data_impl::sync::get_margin_lending_ratio(self, params_json)
    }

    /// 获取多空持仓人数比。
    #[pyo3(signature = (params_json=None))]
    fn get_long_short_ratio(&self, params_json: Option<&str>) -> PyResult<Vec<Py<PyAny>>> {
        trading_data_impl::sync::get_long_short_ratio(self, params_json)
    }

    /// 获取合约持仓量及交易量。
    #[pyo3(signature = (params_json=None))]
    fn get_contracts_open_interest_volume(
        &self,
        params_json: Option<&str>,
    ) -> PyResult<Vec<Py<PyAny>>> {
        trading_data_impl::sync::get_contracts_open_interest_volume(self, params_json)
    }

    /// 获取期权持仓量及交易量。
    #[pyo3(signature = (params_json=None))]
    fn get_options_open_interest_volume(
        &self,
        params_json: Option<&str>,
    ) -> PyResult<Vec<Py<PyAny>>> {
        trading_data_impl::sync::get_options_open_interest_volume(self, params_json)
    }

    /// 获取看涨/看跌期权合约持仓量比值。
    #[pyo3(signature = (params_json=None))]
    fn get_put_call_ratio(&self, params_json: Option<&str>) -> PyResult<Vec<Py<PyAny>>> {
        trading_data_impl::sync::get_put_call_ratio(self, params_json)
    }

    /// 获取期权持仓量及交易量（按到期日）。
    #[pyo3(signature = (params_json=None))]
    fn get_open_interest_volume_expiry(
        &self,
        params_json: Option<&str>,
    ) -> PyResult<Vec<Py<PyAny>>> {
        trading_data_impl::sync::get_open_interest_volume_expiry(self, params_json)
    }

    /// 获取期权持仓量及交易量（按执行价）。
    #[pyo3(signature = (params_json=None))]
    fn get_interest_volume_strike(&self, params_json: Option<&str>) -> PyResult<Vec<Py<PyAny>>> {
        trading_data_impl::sync::get_interest_volume_strike(self, params_json)
    }

    /// 获取期权主动买入/卖出情况。
    #[pyo3(signature = (params_json=None))]
    fn get_taker_flow(&self, params_json: Option<&str>) -> PyResult<Vec<Py<PyAny>>> {
        trading_data_impl::sync::get_taker_flow(self, params_json)
    }
}
