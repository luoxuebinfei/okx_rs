//! Finance（财务产品）域绑定的同步/异步共享实现。

use pyo3::prelude::*;
use serde_json::Value;

use okx_rest::FinanceApi;

use crate::{
    map_values, parse_json_value, parse_required_json_value, PyAsyncOkxClient, PyOkxClient,
};

fn parse_params(params_json: Option<&str>) -> PyResult<Option<Value>> {
    parse_json_value(params_json, "params_json")
}

fn parse_request(request_json: &str) -> PyResult<Value> {
    parse_required_json_value(request_json, "request_json")
}

pub(crate) mod sync {
    use super::*;

    pub(crate) fn defi_get_offers(
        client: &PyOkxClient,
        params_json: Option<&str>,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let params = parse_params(params_json)?;
        let res = client
            .block_on_allow_threads(async { client.rest_client().defi_get_offers(params).await })?;
        map_values(Ok(res))
    }

    pub(crate) fn defi_purchase(
        client: &PyOkxClient,
        request_json: &str,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let request = parse_request(request_json)?;
        let res = client
            .block_on_allow_threads(async { client.rest_client().defi_purchase(request).await })?;
        map_values(Ok(res))
    }

    pub(crate) fn defi_redeem(
        client: &PyOkxClient,
        request_json: &str,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let request = parse_request(request_json)?;
        let res = client
            .block_on_allow_threads(async { client.rest_client().defi_redeem(request).await })?;
        map_values(Ok(res))
    }

    pub(crate) fn defi_cancel(
        client: &PyOkxClient,
        request_json: &str,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let request = parse_request(request_json)?;
        let res = client
            .block_on_allow_threads(async { client.rest_client().defi_cancel(request).await })?;
        map_values(Ok(res))
    }

    pub(crate) fn defi_orders_active(
        client: &PyOkxClient,
        params_json: Option<&str>,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let params = parse_params(params_json)?;
        let res = client.block_on_allow_threads(async {
            client.rest_client().defi_orders_active(params).await
        })?;
        map_values(Ok(res))
    }

    pub(crate) fn defi_orders_history(
        client: &PyOkxClient,
        params_json: Option<&str>,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let params = parse_params(params_json)?;
        let res = client.block_on_allow_threads(async {
            client.rest_client().defi_orders_history(params).await
        })?;
        map_values(Ok(res))
    }

    pub(crate) fn saving_balance(
        client: &PyOkxClient,
        params_json: Option<&str>,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let params = parse_params(params_json)?;
        let res = client
            .block_on_allow_threads(async { client.rest_client().saving_balance(params).await })?;
        map_values(Ok(res))
    }

    pub(crate) fn saving_purchase_redemption(
        client: &PyOkxClient,
        request_json: &str,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let request = parse_request(request_json)?;
        let res = client.block_on_allow_threads(async {
            client
                .rest_client()
                .saving_purchase_redemption(request)
                .await
        })?;
        map_values(Ok(res))
    }

    pub(crate) fn saving_set_lending_rate(
        client: &PyOkxClient,
        request_json: &str,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let request = parse_request(request_json)?;
        let res = client.block_on_allow_threads(async {
            client.rest_client().saving_set_lending_rate(request).await
        })?;
        map_values(Ok(res))
    }

    pub(crate) fn saving_lending_history(
        client: &PyOkxClient,
        params_json: Option<&str>,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let params = parse_params(params_json)?;
        let res = client.block_on_allow_threads(async {
            client.rest_client().saving_lending_history(params).await
        })?;
        map_values(Ok(res))
    }

    pub(crate) fn saving_public_lending_rate(
        client: &PyOkxClient,
        params_json: Option<&str>,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let params = parse_params(params_json)?;
        let res = client.block_on_allow_threads(async {
            client
                .rest_client()
                .saving_public_lending_rate(params)
                .await
        })?;
        map_values(Ok(res))
    }

    pub(crate) fn saving_lending_rate_history(
        client: &PyOkxClient,
        params_json: Option<&str>,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let params = parse_params(params_json)?;
        let res = client.block_on_allow_threads(async {
            client
                .rest_client()
                .saving_lending_rate_history(params)
                .await
        })?;
        map_values(Ok(res))
    }

    pub(crate) fn flexible_loan_borrow_currencies(
        client: &PyOkxClient,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let res = client.block_on_allow_threads(async {
            client.rest_client().flexible_loan_borrow_currencies().await
        })?;
        map_values(Ok(res))
    }

    pub(crate) fn flexible_loan_collateral_assets(
        client: &PyOkxClient,
        params_json: Option<&str>,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let params = parse_params(params_json)?;
        let res = client.block_on_allow_threads(async {
            client
                .rest_client()
                .flexible_loan_collateral_assets(params)
                .await
        })?;
        map_values(Ok(res))
    }

    pub(crate) fn flexible_loan_max_loan(
        client: &PyOkxClient,
        request_json: &str,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let request = parse_request(request_json)?;
        let res = client.block_on_allow_threads(async {
            client.rest_client().flexible_loan_max_loan(request).await
        })?;
        map_values(Ok(res))
    }

    pub(crate) fn flexible_loan_max_collateral_redeem_amount(
        client: &PyOkxClient,
        params_json: Option<&str>,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let params = parse_params(params_json)?;
        let res = client.block_on_allow_threads(async {
            client
                .rest_client()
                .flexible_loan_max_collateral_redeem_amount(params)
                .await
        })?;
        map_values(Ok(res))
    }

    pub(crate) fn flexible_loan_adjust_collateral(
        client: &PyOkxClient,
        request_json: &str,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let request = parse_request(request_json)?;
        let res = client.block_on_allow_threads(async {
            client
                .rest_client()
                .flexible_loan_adjust_collateral(request)
                .await
        })?;
        map_values(Ok(res))
    }

    pub(crate) fn flexible_loan_loan_info(client: &PyOkxClient) -> PyResult<Vec<Py<PyAny>>> {
        let res = client.block_on_allow_threads(async {
            client.rest_client().flexible_loan_loan_info().await
        })?;
        map_values(Ok(res))
    }

    pub(crate) fn flexible_loan_loan_history(
        client: &PyOkxClient,
        params_json: Option<&str>,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let params = parse_params(params_json)?;
        let res = client.block_on_allow_threads(async {
            client
                .rest_client()
                .flexible_loan_loan_history(params)
                .await
        })?;
        map_values(Ok(res))
    }

    pub(crate) fn flexible_loan_interest_accrued(
        client: &PyOkxClient,
        params_json: Option<&str>,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let params = parse_params(params_json)?;
        let res = client.block_on_allow_threads(async {
            client
                .rest_client()
                .flexible_loan_interest_accrued(params)
                .await
        })?;
        map_values(Ok(res))
    }

    pub(crate) fn staking_defi_eth_product_info(client: &PyOkxClient) -> PyResult<Vec<Py<PyAny>>> {
        let res = client.block_on_allow_threads(async {
            client.rest_client().staking_defi_eth_product_info().await
        })?;
        map_values(Ok(res))
    }

    pub(crate) fn staking_defi_eth_balance(client: &PyOkxClient) -> PyResult<Vec<Py<PyAny>>> {
        let res = client.block_on_allow_threads(async {
            client.rest_client().staking_defi_eth_balance().await
        })?;
        map_values(Ok(res))
    }

    pub(crate) fn staking_defi_eth_purchase(
        client: &PyOkxClient,
        request_json: &str,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let request = parse_request(request_json)?;
        let res = client.block_on_allow_threads(async {
            client
                .rest_client()
                .staking_defi_eth_purchase(request)
                .await
        })?;
        map_values(Ok(res))
    }

    pub(crate) fn staking_defi_eth_redeem(
        client: &PyOkxClient,
        request_json: &str,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let request = parse_request(request_json)?;
        let res = client.block_on_allow_threads(async {
            client.rest_client().staking_defi_eth_redeem(request).await
        })?;
        map_values(Ok(res))
    }

    pub(crate) fn staking_defi_eth_purchase_redeem_history(
        client: &PyOkxClient,
        params_json: Option<&str>,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let params = parse_params(params_json)?;
        let res = client.block_on_allow_threads(async {
            client
                .rest_client()
                .staking_defi_eth_purchase_redeem_history(params)
                .await
        })?;
        map_values(Ok(res))
    }

    pub(crate) fn staking_defi_eth_apy_history(
        client: &PyOkxClient,
        params_json: &str,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let params = parse_required_json_value(params_json, "params_json")?;
        let res = client.block_on_allow_threads(async {
            client
                .rest_client()
                .staking_defi_eth_apy_history(params)
                .await
        })?;
        map_values(Ok(res))
    }

    pub(crate) fn staking_defi_sol_product_info(client: &PyOkxClient) -> PyResult<Vec<Py<PyAny>>> {
        let res = client.block_on_allow_threads(async {
            client.rest_client().staking_defi_sol_product_info().await
        })?;
        map_values(Ok(res))
    }

    pub(crate) fn staking_defi_sol_balance(client: &PyOkxClient) -> PyResult<Vec<Py<PyAny>>> {
        let res = client.block_on_allow_threads(async {
            client.rest_client().staking_defi_sol_balance().await
        })?;
        map_values(Ok(res))
    }

    pub(crate) fn staking_defi_sol_purchase(
        client: &PyOkxClient,
        request_json: &str,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let request = parse_request(request_json)?;
        let res = client.block_on_allow_threads(async {
            client
                .rest_client()
                .staking_defi_sol_purchase(request)
                .await
        })?;
        map_values(Ok(res))
    }

    pub(crate) fn staking_defi_sol_redeem(
        client: &PyOkxClient,
        request_json: &str,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let request = parse_request(request_json)?;
        let res = client.block_on_allow_threads(async {
            client.rest_client().staking_defi_sol_redeem(request).await
        })?;
        map_values(Ok(res))
    }

    pub(crate) fn staking_defi_sol_purchase_redeem_history(
        client: &PyOkxClient,
        params_json: Option<&str>,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let params = parse_params(params_json)?;
        let res = client.block_on_allow_threads(async {
            client
                .rest_client()
                .staking_defi_sol_purchase_redeem_history(params)
                .await
        })?;
        map_values(Ok(res))
    }

    pub(crate) fn staking_defi_sol_apy_history(
        client: &PyOkxClient,
        params_json: &str,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let params = parse_required_json_value(params_json, "params_json")?;
        let res = client.block_on_allow_threads(async {
            client
                .rest_client()
                .staking_defi_sol_apy_history(params)
                .await
        })?;
        map_values(Ok(res))
    }

    pub(crate) fn simple_earn_get_lending_offers(
        client: &PyOkxClient,
        params_json: Option<&str>,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let params = parse_params(params_json)?;
        let res = client.block_on_allow_threads(async {
            client
                .rest_client()
                .simple_earn_get_lending_offers(params)
                .await
        })?;
        map_values(Ok(res))
    }

    pub(crate) fn simple_earn_get_lending_apy_history(
        client: &PyOkxClient,
        params_json: Option<&str>,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let params = parse_params(params_json)?;
        let res = client.block_on_allow_threads(async {
            client
                .rest_client()
                .simple_earn_get_lending_apy_history(params)
                .await
        })?;
        map_values(Ok(res))
    }

    pub(crate) fn simple_earn_get_pending_lending_volume(
        client: &PyOkxClient,
        params_json: Option<&str>,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let params = parse_params(params_json)?;
        let res = client.block_on_allow_threads(async {
            client
                .rest_client()
                .simple_earn_get_pending_lending_volume(params)
                .await
        })?;
        map_values(Ok(res))
    }

    pub(crate) fn simple_earn_place_lending_order(
        client: &PyOkxClient,
        request_json: &str,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let request = parse_request(request_json)?;
        let res = client.block_on_allow_threads(async {
            client
                .rest_client()
                .simple_earn_place_lending_order(request)
                .await
        })?;
        map_values(Ok(res))
    }

    pub(crate) fn simple_earn_amend_lending_order(
        client: &PyOkxClient,
        request_json: &str,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let request = parse_request(request_json)?;
        let res = client.block_on_allow_threads(async {
            client
                .rest_client()
                .simple_earn_amend_lending_order(request)
                .await
        })?;
        map_values(Ok(res))
    }

    pub(crate) fn simple_earn_get_lending_orders_list(
        client: &PyOkxClient,
        params_json: Option<&str>,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let params = parse_params(params_json)?;
        let res = client.block_on_allow_threads(async {
            client
                .rest_client()
                .simple_earn_get_lending_orders_list(params)
                .await
        })?;
        map_values(Ok(res))
    }

    pub(crate) fn simple_earn_get_lending_sub_orders(
        client: &PyOkxClient,
        params_json: Option<&str>,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let params = parse_params(params_json)?;
        let res = client.block_on_allow_threads(async {
            client
                .rest_client()
                .simple_earn_get_lending_sub_orders(params)
                .await
        })?;
        map_values(Ok(res))
    }
}

pub(crate) mod async_api {
    use super::*;

    pub(crate) fn defi_get_offers<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        params_json: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        let params = parse_params(params_json.as_deref())?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(rest.defi_get_offers(params).await)
        })
    }

    pub(crate) fn defi_purchase<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        request_json: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        let request = parse_request(&request_json)?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(rest.defi_purchase(request).await)
        })
    }

    pub(crate) fn defi_redeem<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        request_json: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        let request = parse_request(&request_json)?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(rest.defi_redeem(request).await)
        })
    }

    pub(crate) fn defi_cancel<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        request_json: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        let request = parse_request(&request_json)?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(rest.defi_cancel(request).await)
        })
    }

    pub(crate) fn defi_orders_active<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        params_json: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        let params = parse_params(params_json.as_deref())?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(rest.defi_orders_active(params).await)
        })
    }

    pub(crate) fn defi_orders_history<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        params_json: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        let params = parse_params(params_json.as_deref())?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(rest.defi_orders_history(params).await)
        })
    }

    pub(crate) fn saving_balance<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        params_json: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        let params = parse_params(params_json.as_deref())?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(rest.saving_balance(params).await)
        })
    }

    pub(crate) fn saving_purchase_redemption<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        request_json: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        let request = parse_request(&request_json)?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(rest.saving_purchase_redemption(request).await)
        })
    }

    pub(crate) fn saving_set_lending_rate<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        request_json: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        let request = parse_request(&request_json)?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(rest.saving_set_lending_rate(request).await)
        })
    }

    pub(crate) fn saving_lending_history<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        params_json: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        let params = parse_params(params_json.as_deref())?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(rest.saving_lending_history(params).await)
        })
    }

    pub(crate) fn saving_public_lending_rate<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        params_json: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        let params = parse_params(params_json.as_deref())?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(rest.saving_public_lending_rate(params).await)
        })
    }

    pub(crate) fn saving_lending_rate_history<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        params_json: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        let params = parse_params(params_json.as_deref())?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(rest.saving_lending_rate_history(params).await)
        })
    }

    pub(crate) fn flexible_loan_borrow_currencies<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(rest.flexible_loan_borrow_currencies().await)
        })
    }

    pub(crate) fn flexible_loan_collateral_assets<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        params_json: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        let params = parse_params(params_json.as_deref())?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(rest.flexible_loan_collateral_assets(params).await)
        })
    }

    pub(crate) fn flexible_loan_max_loan<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        request_json: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        let request = parse_request(&request_json)?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(rest.flexible_loan_max_loan(request).await)
        })
    }

    pub(crate) fn flexible_loan_max_collateral_redeem_amount<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        params_json: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        let params = parse_params(params_json.as_deref())?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(
                rest.flexible_loan_max_collateral_redeem_amount(params)
                    .await,
            )
        })
    }

    pub(crate) fn flexible_loan_adjust_collateral<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        request_json: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        let request = parse_request(&request_json)?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(rest.flexible_loan_adjust_collateral(request).await)
        })
    }

    pub(crate) fn flexible_loan_loan_info<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(rest.flexible_loan_loan_info().await)
        })
    }

    pub(crate) fn flexible_loan_loan_history<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        params_json: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        let params = parse_params(params_json.as_deref())?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(rest.flexible_loan_loan_history(params).await)
        })
    }

    pub(crate) fn flexible_loan_interest_accrued<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        params_json: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        let params = parse_params(params_json.as_deref())?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(rest.flexible_loan_interest_accrued(params).await)
        })
    }

    pub(crate) fn staking_defi_eth_product_info<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(rest.staking_defi_eth_product_info().await)
        })
    }

    pub(crate) fn staking_defi_eth_balance<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(rest.staking_defi_eth_balance().await)
        })
    }

    pub(crate) fn staking_defi_eth_purchase<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        request_json: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        let request = parse_request(&request_json)?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(rest.staking_defi_eth_purchase(request).await)
        })
    }

    pub(crate) fn staking_defi_eth_redeem<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        request_json: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        let request = parse_request(&request_json)?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(rest.staking_defi_eth_redeem(request).await)
        })
    }

    pub(crate) fn staking_defi_eth_purchase_redeem_history<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        params_json: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        let params = parse_params(params_json.as_deref())?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(rest.staking_defi_eth_purchase_redeem_history(params).await)
        })
    }

    pub(crate) fn staking_defi_eth_apy_history<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        params_json: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        let params = parse_required_json_value(&params_json, "params_json")?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(rest.staking_defi_eth_apy_history(params).await)
        })
    }

    pub(crate) fn staking_defi_sol_product_info<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(rest.staking_defi_sol_product_info().await)
        })
    }

    pub(crate) fn staking_defi_sol_balance<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(rest.staking_defi_sol_balance().await)
        })
    }

    pub(crate) fn staking_defi_sol_purchase<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        request_json: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        let request = parse_request(&request_json)?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(rest.staking_defi_sol_purchase(request).await)
        })
    }

    pub(crate) fn staking_defi_sol_redeem<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        request_json: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        let request = parse_request(&request_json)?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(rest.staking_defi_sol_redeem(request).await)
        })
    }

    pub(crate) fn staking_defi_sol_purchase_redeem_history<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        params_json: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        let params = parse_params(params_json.as_deref())?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(rest.staking_defi_sol_purchase_redeem_history(params).await)
        })
    }

    pub(crate) fn staking_defi_sol_apy_history<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        params_json: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        let params = parse_required_json_value(&params_json, "params_json")?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(rest.staking_defi_sol_apy_history(params).await)
        })
    }

    pub(crate) fn simple_earn_get_lending_offers<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        params_json: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        let params = parse_params(params_json.as_deref())?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(rest.simple_earn_get_lending_offers(params).await)
        })
    }

    pub(crate) fn simple_earn_get_lending_apy_history<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        params_json: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        let params = parse_params(params_json.as_deref())?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(rest.simple_earn_get_lending_apy_history(params).await)
        })
    }

    pub(crate) fn simple_earn_get_pending_lending_volume<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        params_json: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        let params = parse_params(params_json.as_deref())?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(rest.simple_earn_get_pending_lending_volume(params).await)
        })
    }

    pub(crate) fn simple_earn_place_lending_order<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        request_json: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        let request = parse_request(&request_json)?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(rest.simple_earn_place_lending_order(request).await)
        })
    }

    pub(crate) fn simple_earn_amend_lending_order<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        request_json: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        let request = parse_request(&request_json)?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(rest.simple_earn_amend_lending_order(request).await)
        })
    }

    pub(crate) fn simple_earn_get_lending_orders_list<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        params_json: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        let params = parse_params(params_json.as_deref())?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(rest.simple_earn_get_lending_orders_list(params).await)
        })
    }

    pub(crate) fn simple_earn_get_lending_sub_orders<'py>(
        client: &PyAsyncOkxClient,
        py: Python<'py>,
        params_json: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = client.rest_client();
        let params = parse_params(params_json.as_deref())?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            map_values(rest.simple_earn_get_lending_sub_orders(params).await)
        })
    }
}
