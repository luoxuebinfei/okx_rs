//! Public/Market 信息类 REST 绑定的共享实现（同步/异步）。

use pyo3::prelude::*;

use okx_rest::api::market::{
    GetBlockTickersParams, GetCandlesParams, GetHistoryTradesParams, GetIndexTickersParams,
    GetMarkPriceCandlesParams, GetTickersParams,
};
use okx_rest::api::public::{
    GetConvertContractCoinParams, GetDeliveryExerciseHistoryParams, GetDiscountQuotaParams,
    GetEstimatedPriceParams, GetFundingRateHistoryParams, GetInstrumentsParams,
    GetInsuranceFundParams, GetMarkPriceParams, GetOpenInterestParams, GetOptSummaryParams,
    GetPositionTiersParams, GetPriceLimitParams, GetUnderlyingParams,
};
use okx_rest::{MarketApi, PublicApi, StatusApi};

use crate::types::{
    PyCandle, PyFundingRate, PyIndexTicker, PyMarkPrice, PyOrderBook, PyPublicTrade, PyTicker,
};
use crate::{to_py_err, values_to_py_list, PyAsyncOkxClient, PyOkxClient};

pub(crate) mod sync {
    use super::*;
    use okx_core::types::Candle;

    pub(crate) fn get_ticker(client: &PyOkxClient, inst_id: &str) -> PyResult<Option<PyTicker>> {
        client.block_on_allow_threads(async {
            client
                .rest_client()
                .get_ticker(inst_id)
                .await
                .map(|mut v| v.pop().map(PyTicker::from))
        })
    }

    pub(crate) fn get_tickers(
        client: &PyOkxClient,
        inst_type: &str,
        uly: Option<&str>,
        inst_family: Option<&str>,
    ) -> PyResult<Vec<PyTicker>> {
        let params = GetTickersParams {
            inst_type: inst_type.to_string(),
            uly: uly.map(String::from),
            inst_family: inst_family.map(String::from),
        };
        client.block_on_allow_threads(async {
            client
                .rest_client()
                .get_tickers(params)
                .await
                .map(|v| v.into_iter().map(PyTicker::from).collect())
        })
    }

    pub(crate) fn get_orderbook(
        client: &PyOkxClient,
        inst_id: &str,
        depth: Option<u32>,
    ) -> PyResult<Option<PyOrderBook>> {
        client.block_on_allow_threads(async {
            client
                .rest_client()
                .get_orderbook(inst_id, depth)
                .await
                .map(|mut v| v.pop().map(PyOrderBook::from))
        })
    }

    fn map_candles(data: Vec<Vec<String>>) -> Vec<PyCandle> {
        data.into_iter()
            .filter_map(|arr| Candle::from_array(&arr))
            .map(PyCandle::from)
            .collect()
    }

    pub(crate) fn get_candles(
        client: &PyOkxClient,
        params: GetCandlesParams,
    ) -> PyResult<Vec<PyCandle>> {
        client.block_on_allow_threads(async {
            client
                .rest_client()
                .get_candles(params)
                .await
                .map(map_candles)
        })
    }

    pub(crate) fn get_history_candles(
        client: &PyOkxClient,
        params: GetCandlesParams,
    ) -> PyResult<Vec<PyCandle>> {
        client.block_on_allow_threads(async {
            client
                .rest_client()
                .get_history_candles(params)
                .await
                .map(map_candles)
        })
    }

    pub(crate) fn get_index_candles(
        client: &PyOkxClient,
        params: GetMarkPriceCandlesParams,
    ) -> PyResult<Vec<PyCandle>> {
        client.block_on_allow_threads(async {
            client
                .rest_client()
                .get_index_candles(params)
                .await
                .map(map_candles)
        })
    }

    pub(crate) fn get_mark_price_candles(
        client: &PyOkxClient,
        params: GetMarkPriceCandlesParams,
    ) -> PyResult<Vec<PyCandle>> {
        client.block_on_allow_threads(async {
            client
                .rest_client()
                .get_mark_price_candles(params)
                .await
                .map(map_candles)
        })
    }

    pub(crate) fn get_trades(
        client: &PyOkxClient,
        inst_id: &str,
        limit: Option<u32>,
    ) -> PyResult<Vec<PyPublicTrade>> {
        client.block_on_allow_threads(async {
            okx_rest::MarketApi::get_trades(client.rest_client(), inst_id, limit)
                .await
                .map(|v| v.into_iter().map(PyPublicTrade::from).collect())
        })
    }

    pub(crate) fn get_history_trades(
        client: &PyOkxClient,
        params: GetHistoryTradesParams,
    ) -> PyResult<Vec<PyPublicTrade>> {
        client.block_on_allow_threads(async {
            client
                .rest_client()
                .get_history_trades(params)
                .await
                .map(|v| v.into_iter().map(PyPublicTrade::from).collect())
        })
    }

    pub(crate) fn get_block_trades(
        client: &PyOkxClient,
        inst_id: &str,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let res = client.block_on_allow_threads(async {
            client.rest_client().get_block_trades(inst_id).await
        })?;
        values_to_py_list(res)
    }

    pub(crate) fn get_orderbook_lite(
        client: &PyOkxClient,
        inst_id: &str,
    ) -> PyResult<Option<PyOrderBook>> {
        client.block_on_allow_threads(async {
            client
                .rest_client()
                .get_orderbook_lite(inst_id)
                .await
                .map(|mut v| v.pop().map(PyOrderBook::from))
        })
    }

    pub(crate) fn get_block_ticker(
        client: &PyOkxClient,
        inst_id: &str,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let res = client.block_on_allow_threads(async {
            client.rest_client().get_block_ticker(inst_id).await
        })?;
        values_to_py_list(res)
    }

    pub(crate) fn get_block_tickers(
        client: &PyOkxClient,
        inst_type: &str,
        uly: Option<&str>,
        inst_family: Option<&str>,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let params = GetBlockTickersParams {
            inst_type: inst_type.to_string(),
            uly: uly.map(String::from),
            inst_family: inst_family.map(String::from),
        };
        let res = client.block_on_allow_threads(async {
            client.rest_client().get_block_tickers(params).await
        })?;
        values_to_py_list(res)
    }

    pub(crate) fn get_option_family_trades(
        client: &PyOkxClient,
        inst_family: &str,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let res = client.block_on_allow_threads(async {
            client
                .rest_client()
                .get_option_family_trades(inst_family)
                .await
        })?;
        values_to_py_list(res)
    }

    pub(crate) fn get_index_tickers(
        client: &PyOkxClient,
        quote_ccy: Option<&str>,
        inst_id: Option<&str>,
    ) -> PyResult<Vec<PyIndexTicker>> {
        let params = GetIndexTickersParams {
            quote_ccy: quote_ccy.map(String::from),
            inst_id: inst_id.map(String::from),
        };
        client.block_on_allow_threads(async {
            client
                .rest_client()
                .get_index_tickers(params)
                .await
                .map(|v| v.into_iter().map(PyIndexTicker::from).collect())
        })
    }

    pub(crate) fn get_instruments(
        client: &PyOkxClient,
        inst_type: &str,
        inst_id: Option<&str>,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let params = GetInstrumentsParams {
            inst_type: inst_type.to_string(),
            uly: None,
            inst_family: None,
            inst_id: inst_id.map(String::from),
        };
        let instruments = client
            .block_on_allow_threads(async { client.rest_client().get_instruments(params).await })?;
        Python::attach(|py| {
            instruments
                .into_iter()
                .map(|inst| {
                    let dict = pyo3::types::PyDict::new(py);
                    dict.set_item("instId", &inst.inst_id)?;
                    dict.set_item("instType", &inst.inst_type)?;
                    dict.set_item("baseCcy", &inst.base_ccy)?;
                    dict.set_item("quoteCcy", &inst.quote_ccy)?;
                    dict.set_item("tickSz", &inst.tick_sz)?;
                    dict.set_item("lotSz", &inst.lot_sz)?;
                    dict.set_item("minSz", &inst.min_sz)?;
                    dict.set_item("state", &inst.state)?;
                    Ok(dict.unbind().into())
                })
                .collect::<PyResult<Vec<Py<PyAny>>>>()
        })
    }

    pub(crate) fn get_delivery_exercise_history(
        client: &PyOkxClient,
        inst_type: &str,
        uly: Option<&str>,
        inst_family: Option<&str>,
        after: Option<&str>,
        before: Option<&str>,
        limit: Option<&str>,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let params = GetDeliveryExerciseHistoryParams {
            inst_type: inst_type.to_string(),
            uly: uly.map(String::from),
            inst_family: inst_family.map(String::from),
            after: after.map(String::from),
            before: before.map(String::from),
            limit: limit.map(String::from),
        };
        let res = client.block_on_allow_threads(async {
            client
                .rest_client()
                .get_delivery_exercise_history(params)
                .await
        })?;
        values_to_py_list(res)
    }

    pub(crate) fn get_open_interest(
        client: &PyOkxClient,
        inst_type: &str,
        uly: Option<&str>,
        inst_id: Option<&str>,
        inst_family: Option<&str>,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let params = GetOpenInterestParams {
            inst_type: inst_type.to_string(),
            uly: uly.map(String::from),
            inst_id: inst_id.map(String::from),
            inst_family: inst_family.map(String::from),
        };
        let res = client.block_on_allow_threads(async {
            client.rest_client().get_open_interest(params).await
        })?;
        values_to_py_list(res)
    }

    pub(crate) fn get_position_tiers(
        client: &PyOkxClient,
        inst_type: &str,
        td_mode: &str,
        uly: Option<&str>,
        inst_id: Option<&str>,
        ccy: Option<&str>,
        tier: Option<&str>,
        inst_family: Option<&str>,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let params = GetPositionTiersParams {
            inst_type: inst_type.to_string(),
            td_mode: td_mode.to_string(),
            uly: uly.map(String::from),
            inst_id: inst_id.map(String::from),
            ccy: ccy.map(String::from),
            tier: tier.map(String::from),
            inst_family: inst_family.map(String::from),
        };
        let res = client.block_on_allow_threads(async {
            client.rest_client().get_position_tiers(params).await
        })?;
        values_to_py_list(res)
    }

    pub(crate) fn get_funding_rate(
        client: &PyOkxClient,
        inst_id: &str,
    ) -> PyResult<Option<PyFundingRate>> {
        client.block_on_allow_threads(async {
            client
                .rest_client()
                .get_funding_rate(inst_id)
                .await
                .map(|v| v.into_iter().next().map(PyFundingRate::from))
        })
    }

    pub(crate) fn get_funding_rate_history(
        client: &PyOkxClient,
        inst_id: &str,
        after: Option<&str>,
        before: Option<&str>,
        limit: Option<&str>,
    ) -> PyResult<Vec<PyFundingRate>> {
        let params = GetFundingRateHistoryParams {
            inst_id: inst_id.to_string(),
            after: after.map(String::from),
            before: before.map(String::from),
            limit: limit.map(String::from),
        };
        client.block_on_allow_threads(async {
            client
                .rest_client()
                .get_funding_rate_history(params)
                .await
                .map(|v| v.into_iter().map(PyFundingRate::from).collect())
        })
    }

    pub(crate) fn get_mark_price(
        client: &PyOkxClient,
        inst_type: &str,
        uly: Option<&str>,
        inst_family: Option<&str>,
        inst_id: Option<&str>,
    ) -> PyResult<Vec<PyMarkPrice>> {
        let params = GetMarkPriceParams {
            inst_type: inst_type.to_string(),
            uly: uly.map(String::from),
            inst_family: inst_family.map(String::from),
            inst_id: inst_id.map(String::from),
        };
        client.block_on_allow_threads(async {
            client
                .rest_client()
                .get_mark_price(params)
                .await
                .map(|v| v.into_iter().map(PyMarkPrice::from).collect())
        })
    }

    pub(crate) fn get_price_limit(client: &PyOkxClient, inst_id: &str) -> PyResult<Vec<Py<PyAny>>> {
        let params = GetPriceLimitParams {
            inst_id: inst_id.to_string(),
        };
        let res = client
            .block_on_allow_threads(async { client.rest_client().get_price_limit(params).await })?;
        values_to_py_list(res)
    }

    pub(crate) fn get_opt_summary(
        client: &PyOkxClient,
        uly: Option<&str>,
        exp_time: Option<&str>,
        inst_family: Option<&str>,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let params = GetOptSummaryParams {
            uly: uly.map(String::from),
            exp_time: exp_time.map(String::from),
            inst_family: inst_family.map(String::from),
        };
        let res = client
            .block_on_allow_threads(async { client.rest_client().get_opt_summary(params).await })?;
        values_to_py_list(res)
    }

    pub(crate) fn get_estimated_price(
        client: &PyOkxClient,
        inst_id: &str,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let params = GetEstimatedPriceParams {
            inst_id: inst_id.to_string(),
        };
        let res = client.block_on_allow_threads(async {
            client.rest_client().get_estimated_price(params).await
        })?;
        values_to_py_list(res)
    }

    pub(crate) fn get_discount_interest_free_quota(
        client: &PyOkxClient,
        ccy: Option<&str>,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let params = GetDiscountQuotaParams {
            ccy: ccy.map(String::from),
        };
        let res = client.block_on_allow_threads(async {
            client
                .rest_client()
                .get_discount_interest_free_quota(params)
                .await
        })?;
        values_to_py_list(res)
    }

    pub(crate) fn get_interest_rate_loan_quota(client: &PyOkxClient) -> PyResult<Vec<Py<PyAny>>> {
        let res = client.block_on_allow_threads(async {
            client.rest_client().get_interest_rate_loan_quota().await
        })?;
        values_to_py_list(res)
    }

    pub(crate) fn get_vip_interest_rate_loan_quota(
        client: &PyOkxClient,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let res = client.block_on_allow_threads(async {
            client
                .rest_client()
                .get_vip_interest_rate_loan_quota()
                .await
        })?;
        values_to_py_list(res)
    }

    pub(crate) fn get_underlying(
        client: &PyOkxClient,
        inst_type: Option<&str>,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let params = GetUnderlyingParams {
            inst_type: inst_type.map(String::from),
        };
        let res = client
            .block_on_allow_threads(async { client.rest_client().get_underlying(params).await })?;
        values_to_py_list(res)
    }

    pub(crate) fn get_insurance_fund(
        client: &PyOkxClient,
        inst_type: Option<&str>,
        type_: Option<&str>,
        uly: Option<&str>,
        ccy: Option<&str>,
        before: Option<&str>,
        after: Option<&str>,
        limit: Option<&str>,
        inst_family: Option<&str>,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let params = GetInsuranceFundParams {
            inst_type: inst_type.map(String::from),
            r#type: type_.map(String::from),
            uly: uly.map(String::from),
            ccy: ccy.map(String::from),
            before: before.map(String::from),
            after: after.map(String::from),
            limit: limit.map(String::from),
            inst_family: inst_family.map(String::from),
        };
        let res = client.block_on_allow_threads(async {
            client.rest_client().get_insurance_fund(params).await
        })?;
        values_to_py_list(res)
    }

    pub(crate) fn get_convert_contract_coin(
        client: &PyOkxClient,
        type_: Option<&str>,
        inst_id: Option<&str>,
        sz: Option<&str>,
        px: Option<&str>,
        unit: Option<&str>,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let params = GetConvertContractCoinParams {
            r#type: type_.map(String::from),
            inst_id: inst_id.map(String::from),
            sz: sz.map(String::from),
            px: px.map(String::from),
            unit: unit.map(String::from),
        };
        let res = client.block_on_allow_threads(async {
            client.rest_client().get_convert_contract_coin(params).await
        })?;
        values_to_py_list(res)
    }

    pub(crate) fn get_system_time(client: &PyOkxClient) -> PyResult<String> {
        client.block_on_allow_threads(async {
            client
                .rest_client()
                .get_system_time()
                .await
                .map(|v| v.first().map(|t| t.ts.clone()).unwrap_or_default())
        })
    }

    pub(crate) fn get_system_status(
        client: &PyOkxClient,
        state: Option<&str>,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let res = client.block_on_allow_threads(async {
            client.rest_client().get_system_status(state).await
        })?;
        values_to_py_list(res)
    }
}

pub(crate) mod async_api {
    use super::*;

    pub(crate) fn get_ticker<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        inst_id: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            rest.get_ticker(&inst_id)
                .await
                .map(|mut v| v.pop().map(PyTicker::from))
                .map_err(to_py_err)
        })
    }

    pub(crate) fn get_tickers<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        inst_type: String,
        uly: Option<String>,
        inst_family: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        let params = GetTickersParams {
            inst_type,
            uly,
            inst_family,
        };
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            rest.get_tickers(params)
                .await
                .map(|v| v.into_iter().map(PyTicker::from).collect::<Vec<_>>())
                .map_err(to_py_err)
        })
    }

    pub(crate) fn get_orderbook_lite<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        inst_id: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            rest.get_orderbook_lite(&inst_id)
                .await
                .map(|mut v| v.pop().map(PyOrderBook::from))
                .map_err(to_py_err)
        })
    }

    pub(crate) fn get_block_ticker<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        inst_id: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            rest.get_block_ticker(&inst_id)
                .await
                .map_err(to_py_err)
                .and_then(values_to_py_list)
        })
    }

    pub(crate) fn get_block_tickers<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        inst_type: String,
        uly: Option<String>,
        inst_family: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        let params = GetBlockTickersParams {
            inst_type,
            uly,
            inst_family,
        };
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            rest.get_block_tickers(params)
                .await
                .map_err(to_py_err)
                .and_then(values_to_py_list)
        })
    }

    pub(crate) fn get_option_family_trades<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        inst_family: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            rest.get_option_family_trades(&inst_family)
                .await
                .map_err(to_py_err)
                .and_then(values_to_py_list)
        })
    }

    pub(crate) fn get_orderbook<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        inst_id: String,
        depth: Option<u32>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            rest.get_orderbook(&inst_id, depth)
                .await
                .map(|mut v| v.pop().map(PyOrderBook::from))
                .map_err(to_py_err)
        })
    }

    fn map_candles(data: Vec<Vec<String>>) -> Vec<PyCandle> {
        use okx_core::types::Candle;
        data.into_iter()
            .filter_map(|arr| Candle::from_array(&arr))
            .map(PyCandle::from)
            .collect()
    }

    pub(crate) fn get_candles<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        params: GetCandlesParams,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            rest.get_candles(params)
                .await
                .map(map_candles)
                .map_err(to_py_err)
        })
    }

    pub(crate) fn get_history_candles<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        params: GetCandlesParams,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            rest.get_history_candles(params)
                .await
                .map(map_candles)
                .map_err(to_py_err)
        })
    }

    pub(crate) fn get_index_candles<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        params: GetMarkPriceCandlesParams,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            rest.get_index_candles(params)
                .await
                .map(map_candles)
                .map_err(to_py_err)
        })
    }

    pub(crate) fn get_mark_price_candles<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        params: GetMarkPriceCandlesParams,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            rest.get_mark_price_candles(params)
                .await
                .map(map_candles)
                .map_err(to_py_err)
        })
    }

    pub(crate) fn get_trades<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        inst_id: String,
        limit: Option<u32>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            okx_rest::MarketApi::get_trades(&*rest, &inst_id, limit)
                .await
                .map(|v| v.into_iter().map(PyPublicTrade::from).collect::<Vec<_>>())
                .map_err(to_py_err)
        })
    }

    pub(crate) fn get_history_trades<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        params: GetHistoryTradesParams,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            rest.get_history_trades(params)
                .await
                .map(|v| v.into_iter().map(PyPublicTrade::from).collect::<Vec<_>>())
                .map_err(to_py_err)
        })
    }

    #[allow(dead_code)]
    pub(crate) fn get_block_trades<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        inst_id: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            rest.get_block_trades(&inst_id)
                .await
                .map_err(to_py_err)
                .and_then(values_to_py_list)
        })
    }

    pub(crate) fn get_index_tickers<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        quote_ccy: Option<String>,
        inst_id: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        let params = GetIndexTickersParams { quote_ccy, inst_id };
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            rest.get_index_tickers(params)
                .await
                .map(|v| v.into_iter().map(PyIndexTicker::from).collect::<Vec<_>>())
                .map_err(to_py_err)
        })
    }

    pub(crate) fn get_instruments<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        inst_type: String,
        inst_id: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        let params = GetInstrumentsParams {
            inst_type,
            uly: None,
            inst_family: None,
            inst_id,
        };
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let instruments = rest.get_instruments(params).await.map_err(to_py_err)?;
            Python::attach(|py| {
                instruments
                    .into_iter()
                    .map(|inst| {
                        let dict = pyo3::types::PyDict::new(py);
                        dict.set_item("instId", &inst.inst_id).ok();
                        dict.set_item("instType", &inst.inst_type).ok();
                        dict.set_item("baseCcy", &inst.base_ccy).ok();
                        dict.set_item("quoteCcy", &inst.quote_ccy).ok();
                        dict.set_item("tickSz", &inst.tick_sz).ok();
                        dict.set_item("lotSz", &inst.lot_sz).ok();
                        dict.set_item("minSz", &inst.min_sz).ok();
                        dict.set_item("state", &inst.state).ok();
                        Ok(dict.unbind().into())
                    })
                    .collect::<PyResult<Vec<Py<PyAny>>>>()
            })
        })
    }

    pub(crate) fn get_delivery_exercise_history<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        inst_type: String,
        uly: Option<String>,
        inst_family: Option<String>,
        after: Option<String>,
        before: Option<String>,
        limit: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        let params = GetDeliveryExerciseHistoryParams {
            inst_type,
            uly,
            inst_family,
            after,
            before,
            limit,
        };
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            rest.get_delivery_exercise_history(params)
                .await
                .map_err(to_py_err)
                .and_then(values_to_py_list)
        })
    }

    pub(crate) fn get_open_interest<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        inst_type: String,
        uly: Option<String>,
        inst_id: Option<String>,
        inst_family: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        let params = GetOpenInterestParams {
            inst_type,
            uly,
            inst_id,
            inst_family,
        };
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            rest.get_open_interest(params)
                .await
                .map_err(to_py_err)
                .and_then(values_to_py_list)
        })
    }

    pub(crate) fn get_position_tiers<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        inst_type: String,
        td_mode: String,
        uly: Option<String>,
        inst_id: Option<String>,
        ccy: Option<String>,
        tier: Option<String>,
        inst_family: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        let params = GetPositionTiersParams {
            inst_type,
            td_mode,
            uly,
            inst_id,
            ccy,
            tier,
            inst_family,
        };
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            rest.get_position_tiers(params)
                .await
                .map_err(to_py_err)
                .and_then(values_to_py_list)
        })
    }

    pub(crate) fn get_funding_rate<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        inst_id: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            rest.get_funding_rate(&inst_id)
                .await
                .map(|v| v.into_iter().next().map(PyFundingRate::from))
                .map_err(to_py_err)
        })
    }

    pub(crate) fn get_funding_rate_history<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        inst_id: String,
        after: Option<String>,
        before: Option<String>,
        limit: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        let params = GetFundingRateHistoryParams {
            inst_id,
            after,
            before,
            limit,
        };
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            rest.get_funding_rate_history(params)
                .await
                .map(|v| v.into_iter().map(PyFundingRate::from).collect::<Vec<_>>())
                .map_err(to_py_err)
        })
    }

    pub(crate) fn get_mark_price<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        inst_type: String,
        uly: Option<String>,
        inst_family: Option<String>,
        inst_id: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        let params = GetMarkPriceParams {
            inst_type,
            uly,
            inst_family,
            inst_id,
        };
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            rest.get_mark_price(params)
                .await
                .map(|v| v.into_iter().map(PyMarkPrice::from).collect::<Vec<_>>())
                .map_err(to_py_err)
        })
    }

    pub(crate) fn get_price_limit<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        inst_id: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        let params = GetPriceLimitParams { inst_id };
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            rest.get_price_limit(params)
                .await
                .map_err(to_py_err)
                .and_then(values_to_py_list)
        })
    }

    pub(crate) fn get_opt_summary<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        uly: Option<String>,
        exp_time: Option<String>,
        inst_family: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        let params = GetOptSummaryParams {
            uly,
            exp_time,
            inst_family,
        };
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            rest.get_opt_summary(params)
                .await
                .map_err(to_py_err)
                .and_then(values_to_py_list)
        })
    }

    pub(crate) fn get_estimated_price<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        inst_id: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        let params = GetEstimatedPriceParams { inst_id };
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            rest.get_estimated_price(params)
                .await
                .map_err(to_py_err)
                .and_then(values_to_py_list)
        })
    }

    pub(crate) fn get_discount_interest_free_quota<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        ccy: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        let params = GetDiscountQuotaParams { ccy };
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            rest.get_discount_interest_free_quota(params)
                .await
                .map_err(to_py_err)
                .and_then(values_to_py_list)
        })
    }

    pub(crate) fn get_interest_rate_loan_quota<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            rest.get_interest_rate_loan_quota()
                .await
                .map_err(to_py_err)
                .and_then(values_to_py_list)
        })
    }

    pub(crate) fn get_vip_interest_rate_loan_quota<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            rest.get_vip_interest_rate_loan_quota()
                .await
                .map_err(to_py_err)
                .and_then(values_to_py_list)
        })
    }

    pub(crate) fn get_underlying<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        inst_type: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        let params = GetUnderlyingParams { inst_type };
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            rest.get_underlying(params)
                .await
                .map_err(to_py_err)
                .and_then(values_to_py_list)
        })
    }

    pub(crate) fn get_insurance_fund<'py>(
        client: &PyAsyncOkxClient,
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
        let rest = client.rest_client();
        let params = GetInsuranceFundParams {
            inst_type,
            r#type: type_,
            uly,
            ccy,
            before,
            after,
            limit,
            inst_family,
        };
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            rest.get_insurance_fund(params)
                .await
                .map_err(to_py_err)
                .and_then(values_to_py_list)
        })
    }

    pub(crate) fn get_convert_contract_coin<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        type_: Option<String>,
        inst_id: Option<String>,
        sz: Option<String>,
        px: Option<String>,
        unit: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        let params = GetConvertContractCoinParams {
            r#type: type_,
            inst_id,
            sz,
            px,
            unit,
        };
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            rest.get_convert_contract_coin(params)
                .await
                .map_err(to_py_err)
                .and_then(values_to_py_list)
        })
    }

    pub(crate) fn get_system_time<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            rest.get_system_time()
                .await
                .map(|v| v.first().map(|t| t.ts.clone()).unwrap_or_default())
                .map_err(to_py_err)
        })
    }

    pub(crate) fn get_system_status<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        state: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            rest.get_system_status(state.as_deref())
                .await
                .map_err(to_py_err)
                .and_then(values_to_py_list)
        })
    }
}
