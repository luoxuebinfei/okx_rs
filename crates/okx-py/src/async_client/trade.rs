//! Trade API #[pymethods] 块（异步）

use pyo3::prelude::*;

use okx_core::types::{
    AmendOrderRequest, CancelAlgoOrderRequest, CancelOrderRequest, PlaceAlgoOrderRequest,
    PlaceOrderRequest,
};
use okx_rest::api::trade::{
    AmendAlgoOrderRequest, ClosePositionRequest, GetAlgoOrderDetailsParams,
    GetAlgoOrdersHistoryParams, GetAlgoOrdersParams, GetFillsHistoryParams, GetFillsParams,
    GetOrderParams, GetOrdersHistoryArchiveParams, GetOrdersHistoryParams, GetOrdersPendingParams,
    OneClickRepayHistoryV2Params, OneClickRepayV2Request,
};
use okx_rest::TradeApi;

use crate::types::*;
use crate::{to_py_err, values_to_py_list};

use super::PyAsyncOkxClient;

// 为了保持 Python 方法签名直观，这里使用了较长的元组参数列表。
// 通过类型别名降低 clippy 的 `type_complexity` 噪音，同时提升可读性。
type PlaceBatchOrderArgs = (
    String,
    String,
    String,
    String,
    String,
    Option<String>,
    Option<String>,
);

type AmendBatchOrderArgs = (
    String,
    Option<String>,
    Option<String>,
    Option<String>,
    Option<String>,
    Option<String>,
    Option<String>,
    Option<String>,
    Option<String>,
    Option<String>,
    Option<String>,
    Option<String>,
);

#[pymethods]
impl PyAsyncOkxClient {
    /// 下单（单笔，异步）。
    #[pyo3(signature = (inst_id, td_mode, side, ord_type, sz, px=None, cl_ord_id=None))]
    fn place_order<'py>(
        &self,
        py: Python<'py>,
        inst_id: String,
        td_mode: String,
        side: String,
        ord_type: String,
        sz: String,
        px: Option<String>,
        cl_ord_id: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let request = PlaceOrderRequest {
            inst_id,
            td_mode,
            side,
            ord_type,
            sz,
            px,
            cl_ord_id,
            ccy: None,
            tag: None,
            pos_side: None,
            reduce_only: None,
            tgt_ccy: None,
            tp_trigger_px: None,
            tp_ord_px: None,
            sl_trigger_px: None,
            sl_ord_px: None,
            tp_trigger_px_type: None,
            sl_trigger_px_type: None,
            quick_mgn_type: None,
            stp_id: None,
            stp_mode: None,
            attach_algo_ords: None,
        };
        let rest = self.rest_client();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            rest.place_order(request)
                .await
                .map(|v| v.first().map(|r| r.ord_id.clone()).unwrap_or_default())
                .map_err(to_py_err)
        })
    }

    /// 批量下单（异步）。
    #[pyo3(signature = (orders))]
    fn place_batch_orders<'py>(
        &self,
        py: Python<'py>,
        orders: Vec<PlaceBatchOrderArgs>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let requests: Vec<PlaceOrderRequest> = orders
            .into_iter()
            .map(
                |(inst_id, td_mode, side, ord_type, sz, px, cl_ord_id)| PlaceOrderRequest {
                    inst_id,
                    td_mode,
                    side,
                    ord_type,
                    sz,
                    px,
                    cl_ord_id,
                    ccy: None,
                    tag: None,
                    pos_side: None,
                    reduce_only: None,
                    tgt_ccy: None,
                    tp_trigger_px: None,
                    tp_ord_px: None,
                    sl_trigger_px: None,
                    sl_ord_px: None,
                    tp_trigger_px_type: None,
                    sl_trigger_px_type: None,
                    quick_mgn_type: None,
                    stp_id: None,
                    stp_mode: None,
                    attach_algo_ords: None,
                },
            )
            .collect();
        let rest = self.rest_client();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            rest.place_batch_orders(requests)
                .await
                .map(|v| {
                    v.into_iter()
                        .map(PyPlaceOrderResult::from)
                        .collect::<Vec<_>>()
                })
                .map_err(to_py_err)
        })
    }

    /// 撤单（异步）。
    #[pyo3(signature = (inst_id, ord_id=None, cl_ord_id=None))]
    fn cancel_order<'py>(
        &self,
        py: Python<'py>,
        inst_id: String,
        ord_id: Option<String>,
        cl_ord_id: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let request = CancelOrderRequest {
            inst_id,
            ord_id,
            cl_ord_id,
        };
        let rest = self.rest_client();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            rest.cancel_order(request)
                .await
                .map(|v| v.first().map(|r| r.ord_id.clone()).unwrap_or_default())
                .map_err(to_py_err)
        })
    }

    /// 批量撤单（异步）。
    #[pyo3(signature = (orders))]
    fn cancel_batch_orders<'py>(
        &self,
        py: Python<'py>,
        orders: Vec<(String, Option<String>, Option<String>)>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let requests: Vec<CancelOrderRequest> = orders
            .into_iter()
            .map(|(inst_id, ord_id, cl_ord_id)| CancelOrderRequest {
                inst_id,
                ord_id,
                cl_ord_id,
            })
            .collect();
        let rest = self.rest_client();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            rest.cancel_batch_orders(requests)
                .await
                .map(|v| {
                    v.into_iter()
                        .map(PyCancelOrderResult::from)
                        .collect::<Vec<_>>()
                })
                .map_err(to_py_err)
        })
    }

    /// 改单（异步）。
    #[pyo3(signature = (inst_id, ord_id=None, cl_ord_id=None, req_id=None, new_sz=None, new_px=None, new_tp_trigger_px=None, new_tp_ord_px=None, new_sl_trigger_px=None, new_sl_ord_px=None, new_tp_trigger_px_type=None, new_sl_trigger_px_type=None))]
    fn amend_order<'py>(
        &self,
        py: Python<'py>,
        inst_id: String,
        ord_id: Option<String>,
        cl_ord_id: Option<String>,
        req_id: Option<String>,
        new_sz: Option<String>,
        new_px: Option<String>,
        new_tp_trigger_px: Option<String>,
        new_tp_ord_px: Option<String>,
        new_sl_trigger_px: Option<String>,
        new_sl_ord_px: Option<String>,
        new_tp_trigger_px_type: Option<String>,
        new_sl_trigger_px_type: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let request = AmendOrderRequest {
            inst_id,
            ord_id,
            cl_ord_id,
            req_id,
            new_sz,
            new_px,
            new_tp_trigger_px,
            new_tp_ord_px,
            new_sl_trigger_px,
            new_sl_ord_px,
            new_tp_trigger_px_type,
            new_sl_trigger_px_type,
        };
        let rest = self.rest_client();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            rest.amend_order(request)
                .await
                .map(|mut v| v.pop().map(PyAmendOrderResult::from))
                .map_err(to_py_err)
        })
    }

    /// 查询订单详情（异步）。
    #[pyo3(signature = (inst_id, ord_id=None, cl_ord_id=None))]
    fn get_order<'py>(
        &self,
        py: Python<'py>,
        inst_id: String,
        ord_id: Option<String>,
        cl_ord_id: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let params = GetOrderParams {
            inst_id,
            ord_id,
            cl_ord_id,
        };
        let rest = self.rest_client();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            rest.get_order(params)
                .await
                .map(|v| v.into_iter().next().map(PyOrder::from))
                .map_err(to_py_err)
        })
    }

    /// 查询挂单（异步）。
    #[pyo3(signature = (inst_type=None, inst_id=None))]
    fn get_orders_pending<'py>(
        &self,
        py: Python<'py>,
        inst_type: Option<String>,
        inst_id: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let params = if inst_type.is_some() || inst_id.is_some() {
            Some(GetOrdersPendingParams {
                inst_type,
                inst_id,
                ..Default::default()
            })
        } else {
            None
        };
        let rest = self.rest_client();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            rest.get_orders_pending(params)
                .await
                .map(|v| v.into_iter().map(PyOrder::from).collect::<Vec<_>>())
                .map_err(to_py_err)
        })
    }

    /// 查询历史订单（近 7 天，异步）。
    #[pyo3(signature = (inst_type, uly=None, inst_family=None, inst_id=None, ord_type=None, state=None, category=None, after=None, before=None, begin=None, end=None, limit=None))]
    fn get_orders_history<'py>(
        &self,
        py: Python<'py>,
        inst_type: String,
        uly: Option<String>,
        inst_family: Option<String>,
        inst_id: Option<String>,
        ord_type: Option<String>,
        state: Option<String>,
        category: Option<String>,
        after: Option<String>,
        before: Option<String>,
        begin: Option<String>,
        end: Option<String>,
        limit: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let params = GetOrdersHistoryParams {
            inst_type,
            uly,
            inst_family,
            inst_id,
            ord_type,
            state,
            category,
            after,
            before,
            begin,
            end,
            limit,
        };
        let rest = self.rest_client();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            rest.get_orders_history(params)
                .await
                .map(|v| v.into_iter().map(PyOrder::from).collect::<Vec<_>>())
                .map_err(to_py_err)
        })
    }

    /// 查询历史订单归档（近 3 个月，异步）。
    #[pyo3(signature = (inst_type, uly=None, inst_family=None, inst_id=None, ord_type=None, state=None, category=None, after=None, before=None, begin=None, end=None, limit=None))]
    fn get_orders_history_archive<'py>(
        &self,
        py: Python<'py>,
        inst_type: String,
        uly: Option<String>,
        inst_family: Option<String>,
        inst_id: Option<String>,
        ord_type: Option<String>,
        state: Option<String>,
        category: Option<String>,
        after: Option<String>,
        before: Option<String>,
        begin: Option<String>,
        end: Option<String>,
        limit: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let params = GetOrdersHistoryArchiveParams {
            inst_type,
            uly,
            inst_family,
            inst_id,
            ord_type,
            state,
            category,
            after,
            before,
            begin,
            end,
            limit,
        };
        let rest = self.rest_client();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            rest.get_orders_history_archive(params)
                .await
                .map(|v| v.into_iter().map(PyOrder::from).collect::<Vec<_>>())
                .map_err(to_py_err)
        })
    }

    /// 查询成交明细（异步）。
    #[pyo3(signature = (inst_type=None, uly=None, inst_family=None, inst_id=None, ord_id=None, after=None, before=None, begin=None, end=None, limit=None))]
    fn get_fills<'py>(
        &self,
        py: Python<'py>,
        inst_type: Option<String>,
        uly: Option<String>,
        inst_family: Option<String>,
        inst_id: Option<String>,
        ord_id: Option<String>,
        after: Option<String>,
        before: Option<String>,
        begin: Option<String>,
        end: Option<String>,
        limit: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let params = GetFillsParams {
            inst_type,
            uly,
            inst_family,
            inst_id,
            ord_id,
            after,
            before,
            begin,
            end,
            limit,
        };
        let rest = self.rest_client();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            rest.get_fills(Some(params))
                .await
                .map(|v| v.into_iter().map(PyFill::from).collect::<Vec<_>>())
                .map_err(to_py_err)
        })
    }

    /// 查询历史成交（近 3 个月，异步）。
    #[pyo3(signature = (inst_type, uly=None, inst_family=None, inst_id=None, ord_id=None, after=None, before=None, limit=None))]
    fn get_fills_history<'py>(
        &self,
        py: Python<'py>,
        inst_type: String,
        uly: Option<String>,
        inst_family: Option<String>,
        inst_id: Option<String>,
        ord_id: Option<String>,
        after: Option<String>,
        before: Option<String>,
        limit: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let params = GetFillsHistoryParams {
            inst_type,
            uly,
            inst_family,
            inst_id,
            ord_id,
            after,
            before,
            limit,
        };
        let rest = self.rest_client();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            rest.get_fills_history(params)
                .await
                .map(|v| v.into_iter().map(PyFill::from).collect::<Vec<_>>())
                .map_err(to_py_err)
        })
    }
}

// Trade API 续 - 算法单
#[pymethods]
impl PyAsyncOkxClient {
    /// 下算法单（异步）。
    #[pyo3(signature = (inst_id, td_mode, side, ord_type, sz, ccy=None, pos_side=None, reduce_only=None, tgt_ccy=None, algo_cl_ord_id=None, trigger_px=None, order_px=None, trigger_px_type=None, tp_trigger_px=None, tp_ord_px=None, tp_trigger_px_type=None, sl_trigger_px=None, sl_ord_px=None, sl_trigger_px_type=None, callback_ratio=None, callback_spread=None, active_px=None))]
    fn place_algo_order<'py>(
        &self,
        py: Python<'py>,
        inst_id: String,
        td_mode: String,
        side: String,
        ord_type: String,
        sz: String,
        ccy: Option<String>,
        pos_side: Option<String>,
        reduce_only: Option<bool>,
        tgt_ccy: Option<String>,
        algo_cl_ord_id: Option<String>,
        trigger_px: Option<String>,
        order_px: Option<String>,
        trigger_px_type: Option<String>,
        tp_trigger_px: Option<String>,
        tp_ord_px: Option<String>,
        tp_trigger_px_type: Option<String>,
        sl_trigger_px: Option<String>,
        sl_ord_px: Option<String>,
        sl_trigger_px_type: Option<String>,
        callback_ratio: Option<String>,
        callback_spread: Option<String>,
        active_px: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let request = PlaceAlgoOrderRequest {
            inst_id,
            td_mode,
            side,
            ord_type,
            sz,
            ccy,
            pos_side,
            reduce_only,
            tgt_ccy,
            algo_cl_ord_id,
            trigger_px,
            order_px,
            trigger_px_type,
            tp_trigger_px,
            tp_ord_px,
            tp_trigger_px_type,
            sl_trigger_px,
            sl_ord_px,
            sl_trigger_px_type,
            callback_ratio,
            callback_spread,
            active_px,
        };
        let rest = self.rest_client();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            rest.place_algo_order(request)
                .await
                .map(|v| {
                    v.into_iter()
                        .map(PyPlaceAlgoOrderResult::from)
                        .collect::<Vec<_>>()
                })
                .map_err(to_py_err)
        })
    }

    /// 批量撤算法单（异步）。
    #[pyo3(signature = (requests))]
    fn cancel_algo_orders<'py>(
        &self,
        py: Python<'py>,
        requests: Vec<(String, String)>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let reqs: Vec<CancelAlgoOrderRequest> = requests
            .into_iter()
            .map(|(inst_id, algo_id)| CancelAlgoOrderRequest { inst_id, algo_id })
            .collect();
        let rest = self.rest_client();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            rest.cancel_algo_orders(reqs)
                .await
                .map(|v| {
                    v.into_iter()
                        .map(PyCancelAlgoOrderResult::from)
                        .collect::<Vec<_>>()
                })
                .map_err(to_py_err)
        })
    }

    /// 修改算法单（异步）。
    #[pyo3(signature = (inst_id=None, algo_id=None, algo_cl_ord_id=None, cxl_on_fail=None, req_id=None, new_sz=None, new_tp_trigger_px=None, new_tp_ord_px=None, new_sl_trigger_px=None, new_sl_ord_px=None, new_tp_trigger_px_type=None, new_sl_trigger_px_type=None))]
    fn amend_algo_order<'py>(
        &self,
        py: Python<'py>,
        inst_id: Option<String>,
        algo_id: Option<String>,
        algo_cl_ord_id: Option<String>,
        cxl_on_fail: Option<String>,
        req_id: Option<String>,
        new_sz: Option<String>,
        new_tp_trigger_px: Option<String>,
        new_tp_ord_px: Option<String>,
        new_sl_trigger_px: Option<String>,
        new_sl_ord_px: Option<String>,
        new_tp_trigger_px_type: Option<String>,
        new_sl_trigger_px_type: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let request = AmendAlgoOrderRequest {
            inst_id,
            algo_id,
            algo_cl_ord_id,
            cxl_on_fail,
            req_id,
            new_sz,
            new_tp_trigger_px,
            new_tp_ord_px,
            new_sl_trigger_px,
            new_sl_ord_px,
            new_tp_trigger_px_type,
            new_sl_trigger_px_type,
        };
        let rest = self.rest_client();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            rest.amend_algo_order(request)
                .await
                .map_err(to_py_err)
                .and_then(values_to_py_list)
        })
    }

    /// 查询在途算法单（异步）。
    #[pyo3(signature = (ord_type, algo_id=None, inst_type=None, inst_id=None, after=None, before=None, limit=None))]
    fn get_algo_orders_pending<'py>(
        &self,
        py: Python<'py>,
        ord_type: String,
        algo_id: Option<String>,
        inst_type: Option<String>,
        inst_id: Option<String>,
        after: Option<String>,
        before: Option<String>,
        limit: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let params = GetAlgoOrdersParams {
            ord_type,
            algo_id,
            inst_type,
            inst_id,
            after,
            before,
            limit,
        };
        let rest = self.rest_client();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            rest.get_algo_orders_pending(params)
                .await
                .map(|v| v.into_iter().map(PyAlgoOrder::from).collect::<Vec<_>>())
                .map_err(to_py_err)
        })
    }

    /// 查询历史算法单（异步）。
    #[pyo3(signature = (ord_type, state=None, algo_id=None, inst_type=None, inst_id=None, after=None, before=None, limit=None))]
    fn get_algo_orders_history<'py>(
        &self,
        py: Python<'py>,
        ord_type: String,
        state: Option<String>,
        algo_id: Option<String>,
        inst_type: Option<String>,
        inst_id: Option<String>,
        after: Option<String>,
        before: Option<String>,
        limit: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let params = GetAlgoOrdersHistoryParams {
            ord_type,
            state,
            algo_id,
            inst_type,
            inst_id,
            after,
            before,
            limit,
        };
        let rest = self.rest_client();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            rest.get_algo_orders_history(params)
                .await
                .map(|v| v.into_iter().map(PyAlgoOrder::from).collect::<Vec<_>>())
                .map_err(to_py_err)
        })
    }

    /// 获取算法单详情（异步）。
    #[pyo3(signature = (algo_id=None, algo_cl_ord_id=None))]
    fn get_algo_order_details<'py>(
        &self,
        py: Python<'py>,
        algo_id: Option<String>,
        algo_cl_ord_id: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let params = GetAlgoOrderDetailsParams {
            algo_id,
            algo_cl_ord_id,
        };
        let rest = self.rest_client();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            rest.get_algo_order_details(params)
                .await
                .map(|v| v.into_iter().map(PyAlgoOrder::from).collect::<Vec<_>>())
                .map_err(to_py_err)
        })
    }

    /// 一键平仓（异步）。
    #[pyo3(signature = (inst_id, mgn_mode, pos_side=None, ccy=None, auto_cancel=None, cl_ord_id=None, tag=None))]
    fn close_position<'py>(
        &self,
        py: Python<'py>,
        inst_id: String,
        mgn_mode: String,
        pos_side: Option<String>,
        ccy: Option<String>,
        auto_cancel: Option<bool>,
        cl_ord_id: Option<String>,
        tag: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let request = ClosePositionRequest {
            inst_id,
            mgn_mode,
            pos_side,
            ccy,
            auto_cancel,
            cl_ord_id,
            tag,
        };
        let rest = self.rest_client();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            rest.close_position(request)
                .await
                .map(|mut v| v.pop().map(PyClosePositionResult::from))
                .map_err(to_py_err)
        })
    }

    /// 批量改单（异步）。
    #[pyo3(signature = (orders))]
    fn amend_batch_orders<'py>(
        &self,
        py: Python<'py>,
        orders: Vec<AmendBatchOrderArgs>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let requests: Vec<AmendOrderRequest> = orders
            .into_iter()
            .map(
                |(
                    inst_id,
                    ord_id,
                    cl_ord_id,
                    req_id,
                    new_sz,
                    new_px,
                    new_tp_trigger_px,
                    new_tp_ord_px,
                    new_sl_trigger_px,
                    new_sl_ord_px,
                    new_tp_trigger_px_type,
                    new_sl_trigger_px_type,
                )| AmendOrderRequest {
                    inst_id,
                    ord_id,
                    cl_ord_id,
                    req_id,
                    new_sz,
                    new_px,
                    new_tp_trigger_px,
                    new_tp_ord_px,
                    new_sl_trigger_px,
                    new_sl_ord_px,
                    new_tp_trigger_px_type,
                    new_sl_trigger_px_type,
                },
            )
            .collect();
        let rest = self.rest_client();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            rest.amend_batch_orders(requests)
                .await
                .map(|v| {
                    v.into_iter()
                        .map(PyAmendOrderResult::from)
                        .collect::<Vec<_>>()
                })
                .map_err(to_py_err)
        })
    }

    /// 全量撤单（异步）。
    #[pyo3(signature = (request_json))]
    fn mass_cancel<'py>(
        &self,
        py: Python<'py>,
        request_json: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        use crate::parse_json_value;
        let request = parse_json_value(Some(&request_json), "request")?
            .ok_or_else(|| pyo3::exceptions::PyValueError::new_err("request 不能为空"))?;
        let rest = self.rest_client();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            rest.mass_cancel(request)
                .await
                .map_err(to_py_err)
                .and_then(values_to_py_list)
        })
    }

    /// 定时全撤（异步）。
    #[pyo3(signature = (request_json))]
    fn cancel_all_after<'py>(
        &self,
        py: Python<'py>,
        request_json: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        use crate::parse_json_value;
        let request = parse_json_value(Some(&request_json), "request")?
            .ok_or_else(|| pyo3::exceptions::PyValueError::new_err("request 不能为空"))?;
        let rest = self.rest_client();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            rest.cancel_all_after(request)
                .await
                .map_err(to_py_err)
                .and_then(values_to_py_list)
        })
    }

    /// 下单预检查（异步）。
    #[pyo3(signature = (request_json))]
    fn order_precheck<'py>(
        &self,
        py: Python<'py>,
        request_json: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        use crate::parse_json_value;
        let request = parse_json_value(Some(&request_json), "request")?
            .ok_or_else(|| pyo3::exceptions::PyValueError::new_err("request 不能为空"))?;
        let rest = self.rest_client();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            rest.order_precheck(request)
                .await
                .map_err(to_py_err)
                .and_then(values_to_py_list)
        })
    }

    /// 获取一键还债支持币种列表 v2（异步）。
    fn get_one_click_repay_currency_list_v2<'py>(
        &self,
        py: Python<'py>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rest = self.rest_client();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            rest.get_one_click_repay_currency_list_v2()
                .await
                .map_err(to_py_err)
                .and_then(values_to_py_list)
        })
    }

    /// 一键还债 v2（异步）。
    #[pyo3(signature = (debt_ccy, repay_ccy_list))]
    fn one_click_repay_v2<'py>(
        &self,
        py: Python<'py>,
        debt_ccy: String,
        repay_ccy_list: Vec<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let request = OneClickRepayV2Request {
            debt_ccy,
            repay_ccy_list,
        };
        let rest = self.rest_client();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            rest.one_click_repay_v2(request)
                .await
                .map_err(to_py_err)
                .and_then(values_to_py_list)
        })
    }

    /// 获取一键还债历史 v2（异步）。
    #[pyo3(signature = (after=None, before=None, limit=None))]
    fn get_one_click_repay_history_v2<'py>(
        &self,
        py: Python<'py>,
        after: Option<String>,
        before: Option<String>,
        limit: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let params = if after.is_some() || before.is_some() || limit.is_some() {
            Some(OneClickRepayHistoryV2Params {
                after,
                before,
                limit,
            })
        } else {
            None
        };
        let rest = self.rest_client();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            rest.get_one_click_repay_history_v2(params)
                .await
                .map_err(to_py_err)
                .and_then(values_to_py_list)
        })
    }
}
