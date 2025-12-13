//! Account API #[pymethods] 块（异步）

use pyo3::prelude::*;

use okx_rest::api::account::{
    GetFeeRatesParams, GetLeverageInfoParams, GetMaxAvailSizeParams, GetMaxLoanParams,
    GetMaxSizeParams, SetLeverageRequest,
};

use crate::account as account_impl;

use super::PyAsyncOkxClient;

#[pymethods]
impl PyAsyncOkxClient {
    /// Get account balance (async).
    #[pyo3(signature = (ccy=None))]
    fn get_balance<'py>(
        &self,
        py: Python<'py>,
        ccy: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        account_impl::async_api::get_balance(self, py, ccy)
    }

    /// Get positions (async).
    #[pyo3(signature = (inst_type=None, inst_id=None))]
    fn get_positions<'py>(
        &self,
        py: Python<'py>,
        inst_type: Option<String>,
        inst_id: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        account_impl::async_api::get_positions(self, py, inst_type, inst_id)
    }

    /// 获取账户配置（异步）。
    fn get_account_config<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        account_impl::async_api::get_account_config(self, py)
    }

    /// 设置杠杆（异步）。
    #[pyo3(signature = (lever, mgn_mode, inst_id=None, ccy=None, pos_side=None))]
    fn set_leverage<'py>(
        &self,
        py: Python<'py>,
        lever: String,
        mgn_mode: String,
        inst_id: Option<String>,
        ccy: Option<String>,
        pos_side: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let request = SetLeverageRequest {
            inst_id,
            ccy,
            lever,
            mgn_mode,
            pos_side,
        };
        account_impl::async_api::set_leverage(self, py, request)
    }

    /// 查询杠杆信息（异步）。
    #[pyo3(signature = (mgn_mode, ccy=None, inst_id=None))]
    fn get_leverage_info<'py>(
        &self,
        py: Python<'py>,
        mgn_mode: String,
        ccy: Option<String>,
        inst_id: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let params = GetLeverageInfoParams {
            mgn_mode,
            ccy,
            inst_id,
        };
        account_impl::async_api::get_leverage_info(self, py, params)
    }

    /// 查询最大可下单张数（异步）。
    #[pyo3(signature = (inst_id, td_mode, ccy=None, px=None, leverage=None))]
    fn get_max_size<'py>(
        &self,
        py: Python<'py>,
        inst_id: String,
        td_mode: String,
        ccy: Option<String>,
        px: Option<String>,
        leverage: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let params = GetMaxSizeParams {
            inst_id,
            td_mode,
            ccy,
            px,
            leverage,
        };
        account_impl::async_api::get_max_size(self, py, params)
    }

    /// 查询最大可用张数（异步）。
    #[pyo3(signature = (inst_id, td_mode, ccy=None, reduce_only=None, quick_mgn_type=None))]
    fn get_max_avail_size<'py>(
        &self,
        py: Python<'py>,
        inst_id: String,
        td_mode: String,
        ccy: Option<String>,
        reduce_only: Option<bool>,
        quick_mgn_type: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let params = GetMaxAvailSizeParams {
            inst_id,
            td_mode,
            ccy,
            reduce_only,
            quick_mgn_type,
        };
        account_impl::async_api::get_max_avail_size(self, py, params)
    }

    /// 查询最大可借（异步）。
    #[pyo3(signature = (inst_id, mgn_mode, mgn_ccy=None))]
    fn get_max_loan<'py>(
        &self,
        py: Python<'py>,
        inst_id: String,
        mgn_mode: String,
        mgn_ccy: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let params = GetMaxLoanParams {
            inst_id,
            mgn_mode,
            mgn_ccy,
        };
        account_impl::async_api::get_max_loan(self, py, params)
    }

    /// 查询借贷利率（异步）。
    #[pyo3(signature = (ccy=None))]
    fn get_interest_rate<'py>(
        &self,
        py: Python<'py>,
        ccy: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        account_impl::async_api::get_interest_rate(self, py, ccy)
    }

    /// 查询手续费率（异步）。
    #[pyo3(signature = (inst_type, inst_id=None, uly=None, inst_family=None))]
    fn get_fee_rates<'py>(
        &self,
        py: Python<'py>,
        inst_type: String,
        inst_id: Option<String>,
        uly: Option<String>,
        inst_family: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let params = GetFeeRatesParams {
            inst_type,
            inst_id,
            uly,
            inst_family,
        };
        account_impl::async_api::get_fee_rates(self, py, params)
    }

    /// 设置持仓模式（异步）。
    fn set_position_mode<'py>(
        &self,
        py: Python<'py>,
        pos_mode: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        account_impl::async_api::set_position_mode(self, py, pos_mode)
    }

    /// 获取账户风险（异步）。
    fn get_account_position_risk<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        account_impl::async_api::get_account_position_risk(self, py)
    }

    /// 获取历史持仓（异步）。
    #[pyo3(signature = (inst_type=None, inst_id=None, mgn_mode=None, type_=None, pos_id=None, after=None, before=None, limit=None))]
    fn get_positions_history<'py>(
        &self,
        py: Python<'py>,
        inst_type: Option<String>,
        inst_id: Option<String>,
        mgn_mode: Option<String>,
        type_: Option<String>,
        pos_id: Option<String>,
        after: Option<String>,
        before: Option<String>,
        limit: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        account_impl::async_api::get_positions_history(
            self, py, inst_type, inst_id, mgn_mode, type_, pos_id, after, before, limit,
        )
    }

    /// 查询最大可提额度（异步）。
    #[pyo3(signature = (ccy=None))]
    fn get_max_withdrawal<'py>(
        &self,
        py: Python<'py>,
        ccy: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        account_impl::async_api::get_max_withdrawal(self, py, ccy)
    }

    /// 账户账单（近 7 天，异步）。
    #[pyo3(signature = (inst_type=None, ccy=None, mgn_mode=None, ct_type=None, type_=None, sub_type=None, after=None, before=None, limit=None))]
    fn get_account_bills<'py>(
        &self,
        py: Python<'py>,
        inst_type: Option<String>,
        ccy: Option<String>,
        mgn_mode: Option<String>,
        ct_type: Option<String>,
        type_: Option<String>,
        sub_type: Option<String>,
        after: Option<String>,
        before: Option<String>,
        limit: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        account_impl::async_api::get_account_bills(
            self, py, inst_type, ccy, mgn_mode, ct_type, type_, sub_type, after, before, limit,
        )
    }

    /// 账户账单归档（异步）。
    #[pyo3(signature = (inst_type=None, ccy=None, mgn_mode=None, ct_type=None, type_=None, sub_type=None, after=None, before=None, limit=None, begin=None, end=None))]
    fn get_account_bills_archive<'py>(
        &self,
        py: Python<'py>,
        inst_type: Option<String>,
        ccy: Option<String>,
        mgn_mode: Option<String>,
        ct_type: Option<String>,
        type_: Option<String>,
        sub_type: Option<String>,
        after: Option<String>,
        before: Option<String>,
        limit: Option<String>,
        begin: Option<String>,
        end: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        account_impl::async_api::get_account_bills_archive(
            self, py, inst_type, ccy, mgn_mode, ct_type, type_, sub_type, after, before, limit,
            begin, end,
        )
    }

    /// 设置 Greeks（异步）。
    fn set_greeks<'py>(&self, py: Python<'py>, greeks_type: String) -> PyResult<Bound<'py, PyAny>> {
        account_impl::async_api::set_greeks(self, py, greeks_type)
    }

    /// 设置逐仓模式（异步）。
    #[pyo3(signature = (iso_mode, type_))]
    fn set_isolated_mode<'py>(
        &self,
        py: Python<'py>,
        iso_mode: String,
        type_: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        account_impl::async_api::set_isolated_mode(self, py, iso_mode, type_)
    }

    /// 调整持仓保证金（异步）。
    #[pyo3(signature = (inst_id, pos_side, type_, amt, loan_trans=None))]
    fn adjustment_margin<'py>(
        &self,
        py: Python<'py>,
        inst_id: String,
        pos_side: String,
        type_: String,
        amt: String,
        loan_trans: Option<bool>,
    ) -> PyResult<Bound<'py, PyAny>> {
        account_impl::async_api::adjustment_margin(
            self, py, inst_id, pos_side, type_, amt, loan_trans,
        )
    }

    /// 设置风险对冲类型（异步）。
    fn set_risk_offset_type<'py>(
        &self,
        py: Python<'py>,
        type_: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        account_impl::async_api::set_risk_offset_type(self, py, type_)
    }

    /// 设置自动借币（异步）。
    #[pyo3(signature = (auto_loan=None))]
    fn set_auto_loan<'py>(
        &self,
        py: Python<'py>,
        auto_loan: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        account_impl::async_api::set_auto_loan(self, py, auto_loan)
    }

    /// 激活期权交易（异步）。
    fn activate_option<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        account_impl::async_api::activate_option(self, py)
    }

    /// 设置自动还币（异步）。
    #[pyo3(signature = (auto_repay=None))]
    fn set_auto_repay<'py>(
        &self,
        py: Python<'py>,
        auto_repay: Option<bool>,
    ) -> PyResult<Bound<'py, PyAny>> {
        account_impl::async_api::set_auto_repay(self, py, auto_repay)
    }

    /// 借币利息抵扣额度及利率（异步）。
    #[pyo3(signature = (type_=None, ccy=None))]
    fn get_interest_limits<'py>(
        &self,
        py: Python<'py>,
        type_: Option<String>,
        ccy: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        account_impl::async_api::get_interest_limits(self, py, type_, ccy)
    }
}

// Account API 续 - 借币相关
#[pymethods]
impl PyAsyncOkxClient {
    /// 尊享借币订单列表（异步）。
    #[pyo3(signature = (ord_id=None, state=None, ccy=None, after=None, before=None, limit=None))]
    fn get_vip_loan_order_list<'py>(
        &self,
        py: Python<'py>,
        ord_id: Option<String>,
        state: Option<String>,
        ccy: Option<String>,
        after: Option<String>,
        before: Option<String>,
        limit: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        account_impl::async_api::get_vip_loan_order_list(
            self, py, ord_id, state, ccy, after, before, limit,
        )
    }

    /// 尊享借币订单详情（异步）。
    #[pyo3(signature = (ccy=None, ord_id=None, after=None, before=None, limit=None))]
    fn get_vip_loan_order_detail<'py>(
        &self,
        py: Python<'py>,
        ccy: Option<String>,
        ord_id: Option<String>,
        after: Option<String>,
        before: Option<String>,
        limit: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        account_impl::async_api::get_vip_loan_order_detail(
            self, py, ccy, ord_id, after, before, limit,
        )
    }

    /// 定期借币限额（异步）。
    fn get_fix_loan_borrowing_limit<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        account_impl::async_api::get_fix_loan_borrowing_limit(self, py)
    }

    /// 定期借币报价（异步）。
    #[pyo3(signature = (type_=None, ccy=None, amt=None, max_rate=None, term=None, ord_id=None))]
    fn get_fix_loan_borrowing_quote<'py>(
        &self,
        py: Python<'py>,
        type_: Option<String>,
        ccy: Option<String>,
        amt: Option<String>,
        max_rate: Option<String>,
        term: Option<String>,
        ord_id: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        account_impl::async_api::get_fix_loan_borrowing_quote(
            self, py, type_, ccy, amt, max_rate, term, ord_id,
        )
    }

    /// 定期借币下单（异步）。
    #[pyo3(signature = (ccy=None, amt=None, max_rate=None, term=None, reborrow=None, reborrow_rate=None))]
    fn place_fix_loan_borrowing_order<'py>(
        &self,
        py: Python<'py>,
        ccy: Option<String>,
        amt: Option<String>,
        max_rate: Option<String>,
        term: Option<String>,
        reborrow: Option<bool>,
        reborrow_rate: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        account_impl::async_api::place_fix_loan_borrowing_order(
            self,
            py,
            ccy,
            amt,
            max_rate,
            term,
            reborrow,
            reborrow_rate,
        )
    }

    /// 修改定期借币订单（异步）。
    #[pyo3(signature = (ord_id=None, reborrow=None, renew_max_rate=None))]
    fn amend_fix_loan_borrowing_order<'py>(
        &self,
        py: Python<'py>,
        ord_id: Option<String>,
        reborrow: Option<bool>,
        renew_max_rate: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        account_impl::async_api::amend_fix_loan_borrowing_order(
            self,
            py,
            ord_id,
            reborrow,
            renew_max_rate,
        )
    }

    /// 定期借币手动续借（异步）。
    #[pyo3(signature = (ord_id=None, max_rate=None))]
    fn fix_loan_manual_reborrow<'py>(
        &self,
        py: Python<'py>,
        ord_id: Option<String>,
        max_rate: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        account_impl::async_api::fix_loan_manual_reborrow(self, py, ord_id, max_rate)
    }

    /// 定期借币还币（异步）。
    #[pyo3(signature = (ord_id=None))]
    fn repay_fix_loan_borrowing_order<'py>(
        &self,
        py: Python<'py>,
        ord_id: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        account_impl::async_api::repay_fix_loan_borrowing_order(self, py, ord_id)
    }

    /// 定期借币订单列表（异步）。
    #[pyo3(signature = (ord_id=None, ccy=None, state=None, after=None, before=None, limit=None))]
    fn get_fix_loan_borrowing_orders_list<'py>(
        &self,
        py: Python<'py>,
        ord_id: Option<String>,
        ccy: Option<String>,
        state: Option<String>,
        after: Option<String>,
        before: Option<String>,
        limit: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        account_impl::async_api::get_fix_loan_borrowing_orders_list(
            self, py, ord_id, ccy, state, after, before, limit,
        )
    }

    /// 设置账户等级（异步）。
    fn set_account_level<'py>(
        &self,
        py: Python<'py>,
        acct_lv: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        account_impl::async_api::set_account_level(self, py, acct_lv)
    }

    /// 借币/还币（异步）。
    #[pyo3(signature = (ccy=None, side=None, amt=None, ord_id=None))]
    fn borrow_repay<'py>(
        &self,
        py: Python<'py>,
        ccy: Option<String>,
        side: Option<String>,
        amt: Option<String>,
        ord_id: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        account_impl::async_api::borrow_repay(self, py, ccy, side, amt, ord_id)
    }

    /// 借币/还币历史（异步）。
    #[pyo3(signature = (ccy=None, after=None, before=None, limit=None))]
    fn get_borrow_repay_history<'py>(
        &self,
        py: Python<'py>,
        ccy: Option<String>,
        after: Option<String>,
        before: Option<String>,
        limit: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        account_impl::async_api::get_borrow_repay_history(self, py, ccy, after, before, limit)
    }

    /// 现货手动借币/还币（异步）。
    #[pyo3(signature = (ccy=None, side=None, amt=None))]
    fn spot_manual_borrow_repay<'py>(
        &self,
        py: Python<'py>,
        ccy: Option<String>,
        side: Option<String>,
        amt: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        account_impl::async_api::spot_manual_borrow_repay(self, py, ccy, side, amt)
    }

    /// 现货借币/还币历史（异步）。
    #[pyo3(signature = (ccy=None, type_=None, after=None, before=None, limit=None))]
    fn spot_borrow_repay_history<'py>(
        &self,
        py: Python<'py>,
        ccy: Option<String>,
        type_: Option<String>,
        after: Option<String>,
        before: Option<String>,
        limit: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        account_impl::async_api::spot_borrow_repay_history(
            self, py, ccy, type_, after, before, limit,
        )
    }

    /// 已生息数据（异步）。
    #[pyo3(signature = (inst_id=None, ccy=None, mgn_mode=None, after=None, before=None, limit=None))]
    fn get_interest_accrued<'py>(
        &self,
        py: Python<'py>,
        inst_id: Option<String>,
        ccy: Option<String>,
        mgn_mode: Option<String>,
        after: Option<String>,
        before: Option<String>,
        limit: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        account_impl::async_api::get_interest_accrued(
            self, py, inst_id, ccy, mgn_mode, after, before, limit,
        )
    }

    /// VIP 已生息数据（异步）。
    #[pyo3(signature = (ccy=None, ord_id=None, after=None, before=None, limit=None))]
    fn get_vip_interest_accrued<'py>(
        &self,
        py: Python<'py>,
        ccy: Option<String>,
        ord_id: Option<String>,
        after: Option<String>,
        before: Option<String>,
        limit: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        account_impl::async_api::get_vip_interest_accrued(
            self, py, ccy, ord_id, after, before, limit,
        )
    }

    /// VIP 已扣息数据（异步）。
    #[pyo3(signature = (ccy=None, ord_id=None, after=None, before=None, limit=None))]
    fn get_vip_interest_deducted<'py>(
        &self,
        py: Python<'py>,
        ccy: Option<String>,
        ord_id: Option<String>,
        after: Option<String>,
        before: Option<String>,
        limit: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        account_impl::async_api::get_vip_interest_deducted(
            self, py, ccy, ord_id, after, before, limit,
        )
    }

    /// 模拟保证金计算（异步）。
    #[pyo3(signature = (inst_type, incl_real_pos=None, spot_offset_type=None, sim_pos_json=None))]
    fn get_simulated_margin<'py>(
        &self,
        py: Python<'py>,
        inst_type: String,
        incl_real_pos: Option<bool>,
        spot_offset_type: Option<String>,
        sim_pos_json: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        account_impl::async_api::get_simulated_margin(
            self,
            py,
            inst_type,
            incl_real_pos,
            spot_offset_type,
            sim_pos_json,
        )
    }

    /// 杠杆分层（异步）。
    fn get_account_position_tiers<'py>(
        &self,
        py: Python<'py>,
        inst_type: String,
        uly: Option<String>,
        inst_family: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        account_impl::async_api::get_account_position_tiers(self, py, inst_type, uly, inst_family)
    }

    /// 查询 Greeks（异步）。
    #[pyo3(signature = (ccy=None))]
    fn get_greeks<'py>(&self, py: Python<'py>, ccy: Option<String>) -> PyResult<Bound<'py, PyAny>> {
        account_impl::async_api::get_greeks(self, py, ccy)
    }

    /// Position Builder 构建模拟仓位（异步）。
    #[pyo3(signature = (acct_lv=None, incl_real_pos_and_eq=None, lever=None, greeks_type=None, sim_pos_json=None, sim_asset_json=None))]
    fn position_builder<'py>(
        &self,
        py: Python<'py>,
        acct_lv: Option<String>,
        incl_real_pos_and_eq: Option<bool>,
        lever: Option<String>,
        greeks_type: Option<String>,
        sim_pos_json: Option<String>,
        sim_asset_json: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        account_impl::async_api::position_builder(
            self,
            py,
            acct_lv,
            incl_real_pos_and_eq,
            lever,
            greeks_type,
            sim_pos_json,
            sim_asset_json,
        )
    }
}
