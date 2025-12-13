//! Convert/Easy Convert/One-Click Repay 域绑定的同步/异步共享实现。

use pyo3::prelude::*;

use okx_core::types::{
    ConvertHistoryParams, ConvertTradeRequest, EasyConvertRequest, EstimateQuoteParams,
    OneClickRepayRequest,
};
use okx_rest::ConvertApi;

use crate::{map_values, PyAsyncOkxClient, PyOkxClient};

pub(crate) mod sync {
    use super::*;

    pub(crate) fn get_convert_currencies(client: &PyOkxClient) -> PyResult<Vec<Py<PyAny>>> {
        let res = client.block_on_allow_threads(async {
            client.rest_client().get_convert_currencies().await
        })?;
        map_values(Ok(res))
    }

    pub(crate) fn get_convert_currency_pair(
        client: &PyOkxClient,
        from_ccy: &str,
        to_ccy: &str,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let res = client.block_on_allow_threads(async {
            client
                .rest_client()
                .get_convert_currency_pair(from_ccy, to_ccy)
                .await
        })?;
        map_values(Ok(res))
    }

    pub(crate) fn estimate_convert_quote(
        client: &PyOkxClient,
        params: EstimateQuoteParams,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let res = client.block_on_allow_threads(async {
            client.rest_client().estimate_convert_quote(params).await
        })?;
        map_values(Ok(res))
    }

    pub(crate) fn convert_trade(
        client: &PyOkxClient,
        request: ConvertTradeRequest,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let res = client
            .block_on_allow_threads(async { client.rest_client().convert_trade(request).await })?;
        map_values(Ok(res))
    }

    pub(crate) fn get_convert_history(
        client: &PyOkxClient,
        params: Option<ConvertHistoryParams>,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let res = client.block_on_allow_threads(async {
            client.rest_client().get_convert_history(params).await
        })?;
        map_values(Ok(res))
    }

    pub(crate) fn get_easy_convert_currency_list(client: &PyOkxClient) -> PyResult<Vec<Py<PyAny>>> {
        let res = client.block_on_allow_threads(async {
            client.rest_client().get_easy_convert_currency_list().await
        })?;
        map_values(Ok(res))
    }

    pub(crate) fn easy_convert(
        client: &PyOkxClient,
        request: EasyConvertRequest,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let res = client
            .block_on_allow_threads(async { client.rest_client().easy_convert(request).await })?;
        map_values(Ok(res))
    }

    pub(crate) fn get_easy_convert_history(
        client: &PyOkxClient,
        after: Option<&str>,
        before: Option<&str>,
        limit: Option<u32>,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let res = client.block_on_allow_threads(async {
            client
                .rest_client()
                .get_easy_convert_history(after, before, limit)
                .await
        })?;
        map_values(Ok(res))
    }

    pub(crate) fn get_one_click_repay_currency_list(
        client: &PyOkxClient,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let res = client.block_on_allow_threads(async {
            client
                .rest_client()
                .get_one_click_repay_currency_list()
                .await
        })?;
        map_values(Ok(res))
    }

    pub(crate) fn one_click_repay(
        client: &PyOkxClient,
        request: OneClickRepayRequest,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let res = client.block_on_allow_threads(async {
            client.rest_client().one_click_repay(request).await
        })?;
        map_values(Ok(res))
    }

    pub(crate) fn get_one_click_repay_history(
        client: &PyOkxClient,
        after: Option<&str>,
        before: Option<&str>,
        limit: Option<u32>,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let res = client.block_on_allow_threads(async {
            client
                .rest_client()
                .get_one_click_repay_history(after, before, limit)
                .await
        })?;
        map_values(Ok(res))
    }
}

pub(crate) mod async_api {
    use super::*;

    pub(crate) fn get_convert_currencies<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(rest.get_convert_currencies().await)
        })
    }

    pub(crate) fn get_convert_currency_pair<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        from_ccy: String,
        to_ccy: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(rest.get_convert_currency_pair(&from_ccy, &to_ccy).await)
        })
    }

    pub(crate) fn estimate_convert_quote<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        params: EstimateQuoteParams,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(rest.estimate_convert_quote(params).await)
        })
    }

    pub(crate) fn convert_trade<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        request: ConvertTradeRequest,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(rest.convert_trade(request).await)
        })
    }

    pub(crate) fn get_convert_history<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        params: Option<ConvertHistoryParams>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(rest.get_convert_history(params).await)
        })
    }

    pub(crate) fn get_easy_convert_currency_list<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(rest.get_easy_convert_currency_list().await)
        })
    }

    pub(crate) fn easy_convert<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        request: EasyConvertRequest,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(rest.easy_convert(request).await)
        })
    }

    pub(crate) fn get_easy_convert_history<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        after: Option<String>,
        before: Option<String>,
        limit: Option<u32>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(
                rest.get_easy_convert_history(after.as_deref(), before.as_deref(), limit)
                    .await,
            )
        })
    }

    pub(crate) fn get_one_click_repay_currency_list<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(rest.get_one_click_repay_currency_list().await)
        })
    }

    pub(crate) fn one_click_repay<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        request: OneClickRepayRequest,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(rest.one_click_repay(request).await)
        })
    }

    pub(crate) fn get_one_click_repay_history<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        after: Option<String>,
        before: Option<String>,
        limit: Option<u32>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(
                rest.get_one_click_repay_history(after.as_deref(), before.as_deref(), limit)
                    .await,
            )
        })
    }
}
