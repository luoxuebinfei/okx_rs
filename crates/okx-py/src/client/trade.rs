//! Trade API #[pymethods] 块

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

use super::PyOkxClient;

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
impl PyOkxClient {
    // ==================== Trade API ====================

    /// 下单（单笔）。
    #[pyo3(signature = (inst_id, td_mode, side, ord_type, sz, px=None, cl_ord_id=None))]
    fn place_order(
        &self,
        inst_id: &str,
        td_mode: &str,
        side: &str,
        ord_type: &str,
        sz: &str,
        px: Option<&str>,
        cl_ord_id: Option<&str>,
    ) -> PyResult<String> {
        let request = PlaceOrderRequest {
            inst_id: inst_id.to_string(),
            td_mode: td_mode.to_string(),
            side: side.to_string(),
            ord_type: ord_type.to_string(),
            sz: sz.to_string(),
            px: px.map(String::from),
            cl_ord_id: cl_ord_id.map(String::from),
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

        self.runtime.block_on(async {
            self.client
                .place_order(request)
                .await
                .map(|v| v.first().map(|r| r.ord_id.clone()).unwrap_or_default())
                .map_err(to_py_err)
        })
    }

    /// 批量下单。
    #[pyo3(signature = (orders))]
    fn place_batch_orders(
        &self,
        orders: Vec<PlaceBatchOrderArgs>,
    ) -> PyResult<Vec<PyPlaceOrderResult>> {
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

        self.runtime.block_on(async {
            self.client
                .place_batch_orders(requests)
                .await
                .map(|v| v.into_iter().map(PyPlaceOrderResult::from).collect())
                .map_err(to_py_err)
        })
    }

    /// 撤单。
    #[pyo3(signature = (inst_id, ord_id=None, cl_ord_id=None))]
    fn cancel_order(
        &self,
        inst_id: &str,
        ord_id: Option<&str>,
        cl_ord_id: Option<&str>,
    ) -> PyResult<String> {
        let request = CancelOrderRequest {
            inst_id: inst_id.to_string(),
            ord_id: ord_id.map(String::from),
            cl_ord_id: cl_ord_id.map(String::from),
        };

        self.runtime.block_on(async {
            self.client
                .cancel_order(request)
                .await
                .map(|v| v.first().map(|r| r.ord_id.clone()).unwrap_or_default())
                .map_err(to_py_err)
        })
    }

    /// 批量撤单。
    #[pyo3(signature = (orders))]
    fn cancel_batch_orders(
        &self,
        orders: Vec<(String, Option<String>, Option<String>)>,
    ) -> PyResult<Vec<PyCancelOrderResult>> {
        let requests: Vec<CancelOrderRequest> = orders
            .into_iter()
            .map(|(inst_id, ord_id, cl_ord_id)| CancelOrderRequest {
                inst_id,
                ord_id,
                cl_ord_id,
            })
            .collect();

        self.runtime.block_on(async {
            self.client
                .cancel_batch_orders(requests)
                .await
                .map(|v| v.into_iter().map(PyCancelOrderResult::from).collect())
                .map_err(to_py_err)
        })
    }

    /// 改单。
    #[pyo3(
        signature = (inst_id, ord_id=None, cl_ord_id=None, req_id=None, new_sz=None, new_px=None, new_tp_trigger_px=None, new_tp_ord_px=None, new_sl_trigger_px=None, new_sl_ord_px=None, new_tp_trigger_px_type=None, new_sl_trigger_px_type=None)
    )]
    fn amend_order(
        &self,
        inst_id: &str,
        ord_id: Option<&str>,
        cl_ord_id: Option<&str>,
        req_id: Option<&str>,
        new_sz: Option<&str>,
        new_px: Option<&str>,
        new_tp_trigger_px: Option<&str>,
        new_tp_ord_px: Option<&str>,
        new_sl_trigger_px: Option<&str>,
        new_sl_ord_px: Option<&str>,
        new_tp_trigger_px_type: Option<&str>,
        new_sl_trigger_px_type: Option<&str>,
    ) -> PyResult<Option<PyAmendOrderResult>> {
        let request = AmendOrderRequest {
            inst_id: inst_id.to_string(),
            ord_id: ord_id.map(String::from),
            cl_ord_id: cl_ord_id.map(String::from),
            req_id: req_id.map(String::from),
            new_sz: new_sz.map(String::from),
            new_px: new_px.map(String::from),
            new_tp_trigger_px: new_tp_trigger_px.map(String::from),
            new_tp_ord_px: new_tp_ord_px.map(String::from),
            new_sl_trigger_px: new_sl_trigger_px.map(String::from),
            new_sl_ord_px: new_sl_ord_px.map(String::from),
            new_tp_trigger_px_type: new_tp_trigger_px_type.map(String::from),
            new_sl_trigger_px_type: new_sl_trigger_px_type.map(String::from),
        };

        self.runtime.block_on(async {
            self.client
                .amend_order(request)
                .await
                .map(|mut v| v.pop().map(PyAmendOrderResult::from))
                .map_err(to_py_err)
        })
    }

    /// 批量改单。
    #[pyo3(signature = (orders))]
    fn amend_batch_orders(
        &self,
        orders: Vec<AmendBatchOrderArgs>,
    ) -> PyResult<Vec<PyAmendOrderResult>> {
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

        self.runtime.block_on(async {
            self.client
                .amend_batch_orders(requests)
                .await
                .map(|v| v.into_iter().map(PyAmendOrderResult::from).collect())
                .map_err(to_py_err)
        })
    }

    /// 查询订单详情。
    #[pyo3(signature = (inst_id, ord_id=None, cl_ord_id=None))]
    fn get_order(
        &self,
        inst_id: &str,
        ord_id: Option<&str>,
        cl_ord_id: Option<&str>,
    ) -> PyResult<Option<PyOrder>> {
        let params = GetOrderParams {
            inst_id: inst_id.to_string(),
            ord_id: ord_id.map(String::from),
            cl_ord_id: cl_ord_id.map(String::from),
        };

        self.runtime.block_on(async {
            self.client
                .get_order(params)
                .await
                .map(|v| v.into_iter().next().map(PyOrder::from))
                .map_err(to_py_err)
        })
    }

    /// 查询挂单。
    #[pyo3(signature = (inst_type=None, inst_id=None))]
    fn get_orders_pending(
        &self,
        inst_type: Option<&str>,
        inst_id: Option<&str>,
    ) -> PyResult<Vec<PyOrder>> {
        let params = if inst_type.is_some() || inst_id.is_some() {
            Some(GetOrdersPendingParams {
                inst_type: inst_type.map(String::from),
                inst_id: inst_id.map(String::from),
                ..Default::default()
            })
        } else {
            None
        };

        self.runtime.block_on(async {
            self.client
                .get_orders_pending(params)
                .await
                .map(|v| v.into_iter().map(PyOrder::from).collect())
                .map_err(to_py_err)
        })
    }

    /// 查询历史订单（近 7 天）。
    #[pyo3(
        signature = (inst_type, uly=None, inst_family=None, inst_id=None, ord_type=None, state=None, category=None, after=None, before=None, begin=None, end=None, limit=None)
    )]
    fn get_orders_history(
        &self,
        inst_type: &str,
        uly: Option<&str>,
        inst_family: Option<&str>,
        inst_id: Option<&str>,
        ord_type: Option<&str>,
        state: Option<&str>,
        category: Option<&str>,
        after: Option<&str>,
        before: Option<&str>,
        begin: Option<&str>,
        end: Option<&str>,
        limit: Option<&str>,
    ) -> PyResult<Vec<PyOrder>> {
        let params = GetOrdersHistoryParams {
            inst_type: inst_type.to_string(),
            uly: uly.map(String::from),
            inst_family: inst_family.map(String::from),
            inst_id: inst_id.map(String::from),
            ord_type: ord_type.map(String::from),
            state: state.map(String::from),
            category: category.map(String::from),
            after: after.map(String::from),
            before: before.map(String::from),
            begin: begin.map(String::from),
            end: end.map(String::from),
            limit: limit.map(String::from),
        };

        self.runtime.block_on(async {
            self.client
                .get_orders_history(params)
                .await
                .map(|v| v.into_iter().map(PyOrder::from).collect())
                .map_err(to_py_err)
        })
    }

    /// 查询历史订单归档（近 3 个月）。
    #[pyo3(
        signature = (inst_type, uly=None, inst_family=None, inst_id=None, ord_type=None, state=None, category=None, after=None, before=None, begin=None, end=None, limit=None)
    )]
    fn get_orders_history_archive(
        &self,
        inst_type: &str,
        uly: Option<&str>,
        inst_family: Option<&str>,
        inst_id: Option<&str>,
        ord_type: Option<&str>,
        state: Option<&str>,
        category: Option<&str>,
        after: Option<&str>,
        before: Option<&str>,
        begin: Option<&str>,
        end: Option<&str>,
        limit: Option<&str>,
    ) -> PyResult<Vec<PyOrder>> {
        let params = GetOrdersHistoryArchiveParams {
            inst_type: inst_type.to_string(),
            uly: uly.map(String::from),
            inst_family: inst_family.map(String::from),
            inst_id: inst_id.map(String::from),
            ord_type: ord_type.map(String::from),
            state: state.map(String::from),
            category: category.map(String::from),
            after: after.map(String::from),
            before: before.map(String::from),
            begin: begin.map(String::from),
            end: end.map(String::from),
            limit: limit.map(String::from),
        };

        self.runtime.block_on(async {
            self.client
                .get_orders_history_archive(params)
                .await
                .map(|v| v.into_iter().map(PyOrder::from).collect())
                .map_err(to_py_err)
        })
    }

    /// 查询成交明细。
    #[pyo3(
        signature = (inst_type=None, uly=None, inst_family=None, inst_id=None, ord_id=None, after=None, before=None, begin=None, end=None, limit=None)
    )]
    fn get_fills(
        &self,
        inst_type: Option<&str>,
        uly: Option<&str>,
        inst_family: Option<&str>,
        inst_id: Option<&str>,
        ord_id: Option<&str>,
        after: Option<&str>,
        before: Option<&str>,
        begin: Option<&str>,
        end: Option<&str>,
        limit: Option<&str>,
    ) -> PyResult<Vec<PyFill>> {
        let params = GetFillsParams {
            inst_type: inst_type.map(String::from),
            uly: uly.map(String::from),
            inst_family: inst_family.map(String::from),
            inst_id: inst_id.map(String::from),
            ord_id: ord_id.map(String::from),
            after: after.map(String::from),
            before: before.map(String::from),
            begin: begin.map(String::from),
            end: end.map(String::from),
            limit: limit.map(String::from),
        };

        self.runtime.block_on(async {
            self.client
                .get_fills(Some(params))
                .await
                .map(|v| v.into_iter().map(PyFill::from).collect())
                .map_err(to_py_err)
        })
    }

    /// 查询历史成交（近 3 个月）。
    #[pyo3(
        signature = (inst_type, uly=None, inst_family=None, inst_id=None, ord_id=None, after=None, before=None, limit=None)
    )]
    fn get_fills_history(
        &self,
        inst_type: &str,
        uly: Option<&str>,
        inst_family: Option<&str>,
        inst_id: Option<&str>,
        ord_id: Option<&str>,
        after: Option<&str>,
        before: Option<&str>,
        limit: Option<&str>,
    ) -> PyResult<Vec<PyFill>> {
        let params = GetFillsHistoryParams {
            inst_type: inst_type.to_string(),
            uly: uly.map(String::from),
            inst_family: inst_family.map(String::from),
            inst_id: inst_id.map(String::from),
            ord_id: ord_id.map(String::from),
            after: after.map(String::from),
            before: before.map(String::from),
            limit: limit.map(String::from),
        };

        self.runtime.block_on(async {
            self.client
                .get_fills_history(params)
                .await
                .map(|v| v.into_iter().map(PyFill::from).collect())
                .map_err(to_py_err)
        })
    }

    /// 下算法单。
    #[pyo3(
        signature = (inst_id, td_mode, side, ord_type, sz, ccy=None, pos_side=None, reduce_only=None, tgt_ccy=None, algo_cl_ord_id=None, trigger_px=None, order_px=None, trigger_px_type=None, tp_trigger_px=None, tp_ord_px=None, tp_trigger_px_type=None, sl_trigger_px=None, sl_ord_px=None, sl_trigger_px_type=None, callback_ratio=None, callback_spread=None, active_px=None)
    )]
    fn place_algo_order(
        &self,
        inst_id: &str,
        td_mode: &str,
        side: &str,
        ord_type: &str,
        sz: &str,
        ccy: Option<&str>,
        pos_side: Option<&str>,
        reduce_only: Option<bool>,
        tgt_ccy: Option<&str>,
        algo_cl_ord_id: Option<&str>,
        trigger_px: Option<&str>,
        order_px: Option<&str>,
        trigger_px_type: Option<&str>,
        tp_trigger_px: Option<&str>,
        tp_ord_px: Option<&str>,
        tp_trigger_px_type: Option<&str>,
        sl_trigger_px: Option<&str>,
        sl_ord_px: Option<&str>,
        sl_trigger_px_type: Option<&str>,
        callback_ratio: Option<&str>,
        callback_spread: Option<&str>,
        active_px: Option<&str>,
    ) -> PyResult<Vec<PyPlaceAlgoOrderResult>> {
        let request = PlaceAlgoOrderRequest {
            inst_id: inst_id.to_string(),
            td_mode: td_mode.to_string(),
            side: side.to_string(),
            ord_type: ord_type.to_string(),
            sz: sz.to_string(),
            ccy: ccy.map(String::from),
            pos_side: pos_side.map(String::from),
            reduce_only,
            tgt_ccy: tgt_ccy.map(String::from),
            algo_cl_ord_id: algo_cl_ord_id.map(String::from),
            trigger_px: trigger_px.map(String::from),
            order_px: order_px.map(String::from),
            trigger_px_type: trigger_px_type.map(String::from),
            tp_trigger_px: tp_trigger_px.map(String::from),
            tp_ord_px: tp_ord_px.map(String::from),
            tp_trigger_px_type: tp_trigger_px_type.map(String::from),
            sl_trigger_px: sl_trigger_px.map(String::from),
            sl_ord_px: sl_ord_px.map(String::from),
            sl_trigger_px_type: sl_trigger_px_type.map(String::from),
            callback_ratio: callback_ratio.map(String::from),
            callback_spread: callback_spread.map(String::from),
            active_px: active_px.map(String::from),
        };

        self.runtime.block_on(async {
            self.client
                .place_algo_order(request)
                .await
                .map(|v| v.into_iter().map(PyPlaceAlgoOrderResult::from).collect())
                .map_err(to_py_err)
        })
    }

    /// 批量撤算法单。
    #[pyo3(signature = (requests))]
    fn cancel_algo_orders(
        &self,
        requests: Vec<(String, String)>,
    ) -> PyResult<Vec<PyCancelAlgoOrderResult>> {
        let reqs: Vec<CancelAlgoOrderRequest> = requests
            .into_iter()
            .map(|(inst_id, algo_id)| CancelAlgoOrderRequest { inst_id, algo_id })
            .collect();

        self.runtime.block_on(async {
            self.client
                .cancel_algo_orders(reqs)
                .await
                .map(|v| v.into_iter().map(PyCancelAlgoOrderResult::from).collect())
                .map_err(to_py_err)
        })
    }

    /// 修改算法单。
    #[pyo3(
        signature = (inst_id=None, algo_id=None, algo_cl_ord_id=None, cxl_on_fail=None, req_id=None, new_sz=None, new_tp_trigger_px=None, new_tp_ord_px=None, new_sl_trigger_px=None, new_sl_ord_px=None, new_tp_trigger_px_type=None, new_sl_trigger_px_type=None)
    )]
    fn amend_algo_order(
        &self,
        inst_id: Option<&str>,
        algo_id: Option<&str>,
        algo_cl_ord_id: Option<&str>,
        cxl_on_fail: Option<&str>,
        req_id: Option<&str>,
        new_sz: Option<&str>,
        new_tp_trigger_px: Option<&str>,
        new_tp_ord_px: Option<&str>,
        new_sl_trigger_px: Option<&str>,
        new_sl_ord_px: Option<&str>,
        new_tp_trigger_px_type: Option<&str>,
        new_sl_trigger_px_type: Option<&str>,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let request = AmendAlgoOrderRequest {
            inst_id: inst_id.map(String::from),
            algo_id: algo_id.map(String::from),
            algo_cl_ord_id: algo_cl_ord_id.map(String::from),
            cxl_on_fail: cxl_on_fail.map(String::from),
            req_id: req_id.map(String::from),
            new_sz: new_sz.map(String::from),
            new_tp_trigger_px: new_tp_trigger_px.map(String::from),
            new_tp_ord_px: new_tp_ord_px.map(String::from),
            new_sl_trigger_px: new_sl_trigger_px.map(String::from),
            new_sl_ord_px: new_sl_ord_px.map(String::from),
            new_tp_trigger_px_type: new_tp_trigger_px_type.map(String::from),
            new_sl_trigger_px_type: new_sl_trigger_px_type.map(String::from),
        };

        self.runtime.block_on(async {
            self.client
                .amend_algo_order(request)
                .await
                .map_err(to_py_err)
                .and_then(values_to_py_list)
        })
    }

    /// 查询在途算法单。
    #[pyo3(signature = (ord_type, algo_id=None, inst_type=None, inst_id=None, after=None, before=None, limit=None))]
    fn get_algo_orders_pending(
        &self,
        ord_type: &str,
        algo_id: Option<&str>,
        inst_type: Option<&str>,
        inst_id: Option<&str>,
        after: Option<&str>,
        before: Option<&str>,
        limit: Option<&str>,
    ) -> PyResult<Vec<PyAlgoOrder>> {
        let params = GetAlgoOrdersParams {
            ord_type: ord_type.to_string(),
            algo_id: algo_id.map(String::from),
            inst_type: inst_type.map(String::from),
            inst_id: inst_id.map(String::from),
            after: after.map(String::from),
            before: before.map(String::from),
            limit: limit.map(String::from),
        };

        self.runtime.block_on(async {
            self.client
                .get_algo_orders_pending(params)
                .await
                .map(|v| v.into_iter().map(PyAlgoOrder::from).collect())
                .map_err(to_py_err)
        })
    }

    /// 查询历史算法单。
    #[pyo3(signature = (ord_type, state=None, algo_id=None, inst_type=None, inst_id=None, after=None, before=None, limit=None))]
    fn get_algo_orders_history(
        &self,
        ord_type: &str,
        state: Option<&str>,
        algo_id: Option<&str>,
        inst_type: Option<&str>,
        inst_id: Option<&str>,
        after: Option<&str>,
        before: Option<&str>,
        limit: Option<&str>,
    ) -> PyResult<Vec<PyAlgoOrder>> {
        let params = GetAlgoOrdersHistoryParams {
            ord_type: ord_type.to_string(),
            state: state.map(String::from),
            algo_id: algo_id.map(String::from),
            inst_type: inst_type.map(String::from),
            inst_id: inst_id.map(String::from),
            after: after.map(String::from),
            before: before.map(String::from),
            limit: limit.map(String::from),
        };

        self.runtime.block_on(async {
            self.client
                .get_algo_orders_history(params)
                .await
                .map(|v| v.into_iter().map(PyAlgoOrder::from).collect())
                .map_err(to_py_err)
        })
    }

    /// 获取算法单详情。
    #[pyo3(signature = (algo_id=None, algo_cl_ord_id=None))]
    fn get_algo_order_details(
        &self,
        algo_id: Option<&str>,
        algo_cl_ord_id: Option<&str>,
    ) -> PyResult<Vec<PyAlgoOrder>> {
        let params = GetAlgoOrderDetailsParams {
            algo_id: algo_id.map(String::from),
            algo_cl_ord_id: algo_cl_ord_id.map(String::from),
        };

        self.runtime.block_on(async {
            self.client
                .get_algo_order_details(params)
                .await
                .map(|v| v.into_iter().map(PyAlgoOrder::from).collect())
                .map_err(to_py_err)
        })
    }

    /// 一键平仓。
    #[pyo3(signature = (inst_id, mgn_mode, pos_side=None, ccy=None, auto_cancel=None, cl_ord_id=None, tag=None))]
    fn close_position(
        &self,
        inst_id: &str,
        mgn_mode: &str,
        pos_side: Option<&str>,
        ccy: Option<&str>,
        auto_cancel: Option<bool>,
        cl_ord_id: Option<&str>,
        tag: Option<&str>,
    ) -> PyResult<Option<PyClosePositionResult>> {
        let request = ClosePositionRequest {
            inst_id: inst_id.to_string(),
            mgn_mode: mgn_mode.to_string(),
            pos_side: pos_side.map(String::from),
            ccy: ccy.map(String::from),
            auto_cancel,
            cl_ord_id: cl_ord_id.map(String::from),
            tag: tag.map(String::from),
        };

        self.runtime.block_on(async {
            self.client
                .close_position(request)
                .await
                .map(|mut v| v.pop().map(PyClosePositionResult::from))
                .map_err(to_py_err)
        })
    }

    /// 全量撤单。
    #[pyo3(signature = (request_json))]
    fn mass_cancel(&self, request_json: &str) -> PyResult<Vec<Py<PyAny>>> {
        use crate::parse_json_value;
        let request = parse_json_value(Some(request_json), "request")?
            .ok_or_else(|| pyo3::exceptions::PyValueError::new_err("request 不能为空"))?;
        self.runtime
            .block_on(async { self.client.mass_cancel(request).await })
            .map_err(to_py_err)
            .and_then(values_to_py_list)
    }

    /// 定时全撤。
    #[pyo3(signature = (request_json))]
    fn cancel_all_after(&self, request_json: &str) -> PyResult<Vec<Py<PyAny>>> {
        use crate::parse_json_value;
        let request = parse_json_value(Some(request_json), "request")?
            .ok_or_else(|| pyo3::exceptions::PyValueError::new_err("request 不能为空"))?;
        self.runtime
            .block_on(async { self.client.cancel_all_after(request).await })
            .map_err(to_py_err)
            .and_then(values_to_py_list)
    }

    /// 下单预检查。
    #[pyo3(signature = (request_json))]
    fn order_precheck(&self, request_json: &str) -> PyResult<Vec<Py<PyAny>>> {
        use crate::parse_json_value;
        let request = parse_json_value(Some(request_json), "request")?
            .ok_or_else(|| pyo3::exceptions::PyValueError::new_err("request 不能为空"))?;
        self.runtime
            .block_on(async { self.client.order_precheck(request).await })
            .map_err(to_py_err)
            .and_then(values_to_py_list)
    }

    /// 获取一键还债支持币种列表 v2。
    fn get_one_click_repay_currency_list_v2(&self) -> PyResult<Vec<Py<PyAny>>> {
        self.runtime
            .block_on(async { self.client.get_one_click_repay_currency_list_v2().await })
            .map_err(to_py_err)
            .and_then(values_to_py_list)
    }

    /// 一键还债 v2。
    #[pyo3(signature = (debt_ccy, repay_ccy_list))]
    fn one_click_repay_v2(
        &self,
        debt_ccy: &str,
        repay_ccy_list: Vec<String>,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let request = OneClickRepayV2Request {
            debt_ccy: debt_ccy.to_string(),
            repay_ccy_list,
        };
        self.runtime
            .block_on(async { self.client.one_click_repay_v2(request).await })
            .map_err(to_py_err)
            .and_then(values_to_py_list)
    }

    /// 获取一键还债历史 v2。
    #[pyo3(signature = (after=None, before=None, limit=None))]
    fn get_one_click_repay_history_v2(
        &self,
        after: Option<&str>,
        before: Option<&str>,
        limit: Option<&str>,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let params = if after.is_some() || before.is_some() || limit.is_some() {
            Some(OneClickRepayHistoryV2Params {
                after: after.map(String::from),
                before: before.map(String::from),
                limit: limit.map(String::from),
            })
        } else {
            None
        };

        self.runtime
            .block_on(async { self.client.get_one_click_repay_history_v2(params).await })
            .map_err(to_py_err)
            .and_then(values_to_py_list)
    }
}
