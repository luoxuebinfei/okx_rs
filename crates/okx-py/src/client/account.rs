//! Account API #[pymethods] 块

use pyo3::prelude::*;

use okx_rest::api::account::{
    GetFeeRatesParams, GetLeverageInfoParams, GetMaxAvailSizeParams, GetMaxLoanParams,
    GetMaxSizeParams, SetLeverageRequest,
};

use crate::account as account_impl;
use crate::types::*;

use super::PyOkxClient;

#[pymethods]
impl PyOkxClient {
    // ==================== Account API ====================

    /// Get account balance.
    #[pyo3(signature = (ccy=None))]
    fn get_balance(&self, ccy: Option<&str>) -> PyResult<Vec<PyBalance>> {
        account_impl::sync::get_balance(self, ccy)
    }

    /// Get positions.
    #[pyo3(signature = (inst_type=None, inst_id=None))]
    fn get_positions(
        &self,
        inst_type: Option<&str>,
        inst_id: Option<&str>,
    ) -> PyResult<Vec<PyPosition>> {
        account_impl::sync::get_positions(self, inst_type, inst_id)
    }

    /// 获取当前账户可用交易产品信息。
    #[pyo3(signature = (inst_type, inst_family=None, inst_id=None))]
    fn get_account_instruments(
        &self,
        inst_type: &str,
        inst_family: Option<&str>,
        inst_id: Option<&str>,
    ) -> PyResult<Vec<Py<PyAny>>> {
        account_impl::sync::get_account_instruments(self, inst_type, inst_family, inst_id)
    }

    /// 获取账户配置。
    fn get_account_config(&self) -> PyResult<Vec<PyAccountConfig>> {
        account_impl::sync::get_account_config(self)
    }

    /// 设置杠杆。
    #[pyo3(signature = (lever, mgn_mode, inst_id=None, ccy=None, pos_side=None))]
    fn set_leverage(
        &self,
        lever: &str,
        mgn_mode: &str,
        inst_id: Option<&str>,
        ccy: Option<&str>,
        pos_side: Option<&str>,
    ) -> PyResult<Option<PySetLeverageResult>> {
        let request = SetLeverageRequest {
            inst_id: inst_id.map(String::from),
            ccy: ccy.map(String::from),
            lever: lever.to_string(),
            mgn_mode: mgn_mode.to_string(),
            pos_side: pos_side.map(String::from),
        };
        account_impl::sync::set_leverage(self, request)
    }

    /// 查询杠杆信息。
    #[pyo3(signature = (mgn_mode, ccy=None, inst_id=None))]
    fn get_leverage_info(
        &self,
        mgn_mode: &str,
        ccy: Option<&str>,
        inst_id: Option<&str>,
    ) -> PyResult<Vec<PyLeverageInfo>> {
        let params = GetLeverageInfoParams {
            mgn_mode: mgn_mode.to_string(),
            ccy: ccy.map(String::from),
            inst_id: inst_id.map(String::from),
        };
        account_impl::sync::get_leverage_info(self, params)
    }

    /// 查询最大可下单张数。
    #[pyo3(signature = (inst_id, td_mode, ccy=None, px=None, leverage=None))]
    fn get_max_size(
        &self,
        inst_id: &str,
        td_mode: &str,
        ccy: Option<&str>,
        px: Option<&str>,
        leverage: Option<&str>,
    ) -> PyResult<Option<PyMaxSize>> {
        let params = GetMaxSizeParams {
            inst_id: inst_id.to_string(),
            td_mode: td_mode.to_string(),
            ccy: ccy.map(String::from),
            px: px.map(String::from),
            leverage: leverage.map(String::from),
        };
        account_impl::sync::get_max_size(self, params).map(|mut v| v.pop())
    }

    /// 查询最大可用张数。
    #[pyo3(signature = (inst_id, td_mode, ccy=None, reduce_only=None, quick_mgn_type=None))]
    fn get_max_avail_size(
        &self,
        inst_id: &str,
        td_mode: &str,
        ccy: Option<&str>,
        reduce_only: Option<bool>,
        quick_mgn_type: Option<&str>,
    ) -> PyResult<Option<PyMaxAvailSize>> {
        let params = GetMaxAvailSizeParams {
            inst_id: inst_id.to_string(),
            td_mode: td_mode.to_string(),
            ccy: ccy.map(String::from),
            reduce_only,
            quick_mgn_type: quick_mgn_type.map(String::from),
        };
        account_impl::sync::get_max_avail_size(self, params).map(|mut v| v.pop())
    }

    /// 查询最大可借。
    #[pyo3(signature = (inst_id, mgn_mode, mgn_ccy=None))]
    fn get_max_loan(
        &self,
        inst_id: &str,
        mgn_mode: &str,
        mgn_ccy: Option<&str>,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let params = GetMaxLoanParams {
            inst_id: inst_id.to_string(),
            mgn_mode: mgn_mode.to_string(),
            mgn_ccy: mgn_ccy.map(String::from),
        };
        account_impl::sync::get_max_loan(self, params)
    }

    /// 查询借贷利率。
    #[pyo3(signature = (ccy=None))]
    fn get_interest_rate(&self, ccy: Option<&str>) -> PyResult<Vec<Py<PyAny>>> {
        account_impl::sync::get_interest_rate(self, ccy)
    }

    /// 查询手续费率。
    #[pyo3(signature = (inst_type, inst_id=None, uly=None, inst_family=None))]
    fn get_fee_rates(
        &self,
        inst_type: &str,
        inst_id: Option<&str>,
        uly: Option<&str>,
        inst_family: Option<&str>,
    ) -> PyResult<Vec<PyFeeRates>> {
        let params = GetFeeRatesParams {
            inst_type: inst_type.to_string(),
            inst_id: inst_id.map(String::from),
            uly: uly.map(String::from),
            inst_family: inst_family.map(String::from),
        };
        account_impl::sync::get_fee_rates(self, params)
    }

    /// 设置持仓模式。
    fn set_position_mode(&self, pos_mode: &str) -> PyResult<Option<PySetPositionModeResult>> {
        account_impl::sync::set_position_mode(self, pos_mode)
    }

    /// 获取账户风险。
    fn get_account_position_risk(&self) -> PyResult<Vec<PyAccountPositionRisk>> {
        account_impl::sync::get_account_position_risk(self)
    }

    /// 获取账户风险状态。
    fn get_account_risk_state(&self) -> PyResult<Vec<Py<PyAny>>> {
        account_impl::sync::get_account_risk_state(self)
    }

    /// 获取历史持仓。
    #[pyo3(signature = (inst_type=None, inst_id=None, mgn_mode=None, type_=None, pos_id=None, after=None, before=None, limit=None))]
    fn get_positions_history(
        &self,
        inst_type: Option<&str>,
        inst_id: Option<&str>,
        mgn_mode: Option<&str>,
        type_: Option<&str>,
        pos_id: Option<&str>,
        after: Option<&str>,
        before: Option<&str>,
        limit: Option<&str>,
    ) -> PyResult<Vec<Py<PyAny>>> {
        account_impl::sync::get_positions_history(
            self, inst_type, inst_id, mgn_mode, type_, pos_id, after, before, limit,
        )
    }

    /// 查询最大可提额度。
    #[pyo3(signature = (ccy=None))]
    fn get_max_withdrawal(&self, ccy: Option<&str>) -> PyResult<Vec<Py<PyAny>>> {
        account_impl::sync::get_max_withdrawal(self, ccy)
    }

    /// 查询账户账单（近 7 天）。
    #[pyo3(signature = (inst_type=None, ccy=None, mgn_mode=None, ct_type=None, type_=None, sub_type=None, after=None, before=None, limit=None))]
    fn get_account_bills(
        &self,
        inst_type: Option<&str>,
        ccy: Option<&str>,
        mgn_mode: Option<&str>,
        ct_type: Option<&str>,
        type_: Option<&str>,
        sub_type: Option<&str>,
        after: Option<&str>,
        before: Option<&str>,
        limit: Option<&str>,
    ) -> PyResult<Vec<Py<PyAny>>> {
        account_impl::sync::get_account_bills(
            self, inst_type, ccy, mgn_mode, ct_type, type_, sub_type, after, before, limit,
        )
    }

    /// 查询账户账单归档（近 3 个月）。
    #[pyo3(signature = (inst_type=None, ccy=None, mgn_mode=None, ct_type=None, type_=None, sub_type=None, after=None, before=None, limit=None, begin=None, end=None))]
    fn get_account_bills_archive(
        &self,
        inst_type: Option<&str>,
        ccy: Option<&str>,
        mgn_mode: Option<&str>,
        ct_type: Option<&str>,
        type_: Option<&str>,
        sub_type: Option<&str>,
        after: Option<&str>,
        before: Option<&str>,
        limit: Option<&str>,
        begin: Option<&str>,
        end: Option<&str>,
    ) -> PyResult<Vec<Py<PyAny>>> {
        account_impl::sync::get_account_bills_archive(
            self, inst_type, ccy, mgn_mode, ct_type, type_, sub_type, after, before, limit, begin,
            end,
        )
    }

    /// 设置 Greeks 显示方式。
    fn set_greeks(&self, greeks_type: &str) -> PyResult<Vec<Py<PyAny>>> {
        account_impl::sync::set_greeks(self, greeks_type)
    }

    /// 设置逐仓模式。
    #[pyo3(signature = (iso_mode, type_))]
    fn set_isolated_mode(&self, iso_mode: &str, type_: &str) -> PyResult<Vec<Py<PyAny>>> {
        account_impl::sync::set_isolated_mode(self, iso_mode, type_)
    }

    /// 调整持仓保证金。
    #[pyo3(signature = (inst_id, pos_side, type_, amt, loan_trans=None))]
    fn adjustment_margin(
        &self,
        inst_id: &str,
        pos_side: &str,
        type_: &str,
        amt: &str,
        loan_trans: Option<bool>,
    ) -> PyResult<Vec<Py<PyAny>>> {
        account_impl::sync::adjustment_margin(self, inst_id, pos_side, type_, amt, loan_trans)
    }

    /// 设置风险对冲类型。
    fn set_risk_offset_type(&self, type_: &str) -> PyResult<Vec<Py<PyAny>>> {
        account_impl::sync::set_risk_offset_type(self, type_)
    }

    /// 设置自动借币。
    #[pyo3(signature = (auto_loan=None))]
    fn set_auto_loan(&self, auto_loan: Option<&str>) -> PyResult<Vec<Py<PyAny>>> {
        account_impl::sync::set_auto_loan(self, auto_loan)
    }

    /// 激活期权交易。
    fn activate_option(&self) -> PyResult<Vec<Py<PyAny>>> {
        account_impl::sync::activate_option(self)
    }

    /// 设置自动还币。
    #[pyo3(signature = (auto_repay=None))]
    fn set_auto_repay(&self, auto_repay: Option<bool>) -> PyResult<Vec<Py<PyAny>>> {
        account_impl::sync::set_auto_repay(self, auto_repay)
    }

    /// 借币利息抵扣额度及利率。
    #[pyo3(signature = (type_=None, ccy=None))]
    fn get_interest_limits(
        &self,
        type_: Option<&str>,
        ccy: Option<&str>,
    ) -> PyResult<Vec<Py<PyAny>>> {
        account_impl::sync::get_interest_limits(self, type_, ccy)
    }
}

// Account API 续 - 借币相关
#[pymethods]
impl PyOkxClient {
    /// 尊享借币订单列表。
    #[pyo3(signature = (ord_id=None, state=None, ccy=None, after=None, before=None, limit=None))]
    fn get_vip_loan_order_list(
        &self,
        ord_id: Option<&str>,
        state: Option<&str>,
        ccy: Option<&str>,
        after: Option<&str>,
        before: Option<&str>,
        limit: Option<&str>,
    ) -> PyResult<Vec<Py<PyAny>>> {
        account_impl::sync::get_vip_loan_order_list(self, ord_id, state, ccy, after, before, limit)
    }

    /// 尊享借币订单详情。
    #[pyo3(signature = (ccy=None, ord_id=None, after=None, before=None, limit=None))]
    fn get_vip_loan_order_detail(
        &self,
        ccy: Option<&str>,
        ord_id: Option<&str>,
        after: Option<&str>,
        before: Option<&str>,
        limit: Option<&str>,
    ) -> PyResult<Vec<Py<PyAny>>> {
        account_impl::sync::get_vip_loan_order_detail(self, ccy, ord_id, after, before, limit)
    }

    /// 定期借币限额。
    fn get_fix_loan_borrowing_limit(&self) -> PyResult<Vec<Py<PyAny>>> {
        account_impl::sync::get_fix_loan_borrowing_limit(self)
    }

    /// 定期借币报价。
    #[pyo3(signature = (type_=None, ccy=None, amt=None, max_rate=None, term=None, ord_id=None))]
    fn get_fix_loan_borrowing_quote(
        &self,
        type_: Option<&str>,
        ccy: Option<&str>,
        amt: Option<&str>,
        max_rate: Option<&str>,
        term: Option<&str>,
        ord_id: Option<&str>,
    ) -> PyResult<Vec<Py<PyAny>>> {
        account_impl::sync::get_fix_loan_borrowing_quote(
            self, type_, ccy, amt, max_rate, term, ord_id,
        )
    }

    /// 定期借币下单。
    #[pyo3(signature = (ccy=None, amt=None, max_rate=None, term=None, reborrow=None, reborrow_rate=None))]
    fn place_fix_loan_borrowing_order(
        &self,
        ccy: Option<&str>,
        amt: Option<&str>,
        max_rate: Option<&str>,
        term: Option<&str>,
        reborrow: Option<bool>,
        reborrow_rate: Option<&str>,
    ) -> PyResult<Vec<Py<PyAny>>> {
        account_impl::sync::place_fix_loan_borrowing_order(
            self,
            ccy,
            amt,
            max_rate,
            term,
            reborrow,
            reborrow_rate,
        )
    }

    /// 修改定期借币订单。
    #[pyo3(signature = (ord_id=None, reborrow=None, renew_max_rate=None))]
    fn amend_fix_loan_borrowing_order(
        &self,
        ord_id: Option<&str>,
        reborrow: Option<bool>,
        renew_max_rate: Option<&str>,
    ) -> PyResult<Vec<Py<PyAny>>> {
        account_impl::sync::amend_fix_loan_borrowing_order(self, ord_id, reborrow, renew_max_rate)
    }

    /// 定期借币手动续借。
    #[pyo3(signature = (ord_id=None, max_rate=None))]
    fn fix_loan_manual_reborrow(
        &self,
        ord_id: Option<&str>,
        max_rate: Option<&str>,
    ) -> PyResult<Vec<Py<PyAny>>> {
        account_impl::sync::fix_loan_manual_reborrow(self, ord_id, max_rate)
    }

    /// 定期借币还币。
    #[pyo3(signature = (ord_id=None))]
    fn repay_fix_loan_borrowing_order(&self, ord_id: Option<&str>) -> PyResult<Vec<Py<PyAny>>> {
        account_impl::sync::repay_fix_loan_borrowing_order(self, ord_id)
    }

    /// 定期借币订单列表。
    #[pyo3(signature = (ord_id=None, ccy=None, state=None, after=None, before=None, limit=None))]
    fn get_fix_loan_borrowing_orders_list(
        &self,
        ord_id: Option<&str>,
        ccy: Option<&str>,
        state: Option<&str>,
        after: Option<&str>,
        before: Option<&str>,
        limit: Option<&str>,
    ) -> PyResult<Vec<Py<PyAny>>> {
        account_impl::sync::get_fix_loan_borrowing_orders_list(
            self, ord_id, ccy, state, after, before, limit,
        )
    }

    /// 设置账户等级。
    fn set_account_level(&self, acct_lv: &str) -> PyResult<Vec<Py<PyAny>>> {
        account_impl::sync::set_account_level(self, acct_lv)
    }

    /// 借币/还币。
    #[pyo3(signature = (ccy=None, side=None, amt=None, ord_id=None))]
    fn borrow_repay(
        &self,
        ccy: Option<&str>,
        side: Option<&str>,
        amt: Option<&str>,
        ord_id: Option<&str>,
    ) -> PyResult<Vec<Py<PyAny>>> {
        account_impl::sync::borrow_repay(self, ccy, side, amt, ord_id)
    }

    /// 借币/还币历史。
    #[pyo3(signature = (ccy=None, after=None, before=None, limit=None))]
    fn get_borrow_repay_history(
        &self,
        ccy: Option<&str>,
        after: Option<&str>,
        before: Option<&str>,
        limit: Option<&str>,
    ) -> PyResult<Vec<Py<PyAny>>> {
        account_impl::sync::get_borrow_repay_history(self, ccy, after, before, limit)
    }

    /// 现货手动借币/还币。
    #[pyo3(signature = (ccy=None, side=None, amt=None))]
    fn spot_manual_borrow_repay(
        &self,
        ccy: Option<&str>,
        side: Option<&str>,
        amt: Option<&str>,
    ) -> PyResult<Vec<Py<PyAny>>> {
        account_impl::sync::spot_manual_borrow_repay(self, ccy, side, amt)
    }

    /// 现货借币/还币历史。
    #[pyo3(signature = (ccy=None, type_=None, after=None, before=None, limit=None))]
    fn spot_borrow_repay_history(
        &self,
        ccy: Option<&str>,
        type_: Option<&str>,
        after: Option<&str>,
        before: Option<&str>,
        limit: Option<&str>,
    ) -> PyResult<Vec<Py<PyAny>>> {
        account_impl::sync::spot_borrow_repay_history(self, ccy, type_, after, before, limit)
    }

    /// 已生息数据。
    #[pyo3(signature = (inst_id=None, ccy=None, mgn_mode=None, after=None, before=None, limit=None))]
    fn get_interest_accrued(
        &self,
        inst_id: Option<&str>,
        ccy: Option<&str>,
        mgn_mode: Option<&str>,
        after: Option<&str>,
        before: Option<&str>,
        limit: Option<&str>,
    ) -> PyResult<Vec<Py<PyAny>>> {
        account_impl::sync::get_interest_accrued(self, inst_id, ccy, mgn_mode, after, before, limit)
    }

    /// VIP 已生息数据。
    #[pyo3(signature = (ccy=None, ord_id=None, after=None, before=None, limit=None))]
    fn get_vip_interest_accrued(
        &self,
        ccy: Option<&str>,
        ord_id: Option<&str>,
        after: Option<&str>,
        before: Option<&str>,
        limit: Option<&str>,
    ) -> PyResult<Vec<Py<PyAny>>> {
        account_impl::sync::get_vip_interest_accrued(self, ccy, ord_id, after, before, limit)
    }

    /// VIP 已扣息数据。
    #[pyo3(signature = (ccy=None, ord_id=None, after=None, before=None, limit=None))]
    fn get_vip_interest_deducted(
        &self,
        ccy: Option<&str>,
        ord_id: Option<&str>,
        after: Option<&str>,
        before: Option<&str>,
        limit: Option<&str>,
    ) -> PyResult<Vec<Py<PyAny>>> {
        account_impl::sync::get_vip_interest_deducted(self, ccy, ord_id, after, before, limit)
    }

    /// 模拟保证金计算。
    #[pyo3(signature = (inst_type, incl_real_pos=None, spot_offset_type=None, sim_pos_json=None))]
    fn get_simulated_margin(
        &self,
        inst_type: &str,
        incl_real_pos: Option<bool>,
        spot_offset_type: Option<&str>,
        sim_pos_json: Option<&str>,
    ) -> PyResult<Vec<Py<PyAny>>> {
        account_impl::sync::get_simulated_margin(
            self,
            inst_type,
            incl_real_pos,
            spot_offset_type,
            sim_pos_json,
        )
    }

    /// 杠杆分层。
    fn get_account_position_tiers(
        &self,
        inst_type: &str,
        uly: Option<&str>,
        inst_family: Option<&str>,
    ) -> PyResult<Vec<Py<PyAny>>> {
        account_impl::sync::get_account_position_tiers(self, inst_type, uly, inst_family)
    }

    /// 查询 Greeks。
    #[pyo3(signature = (ccy=None))]
    fn get_greeks(&self, ccy: Option<&str>) -> PyResult<Vec<Py<PyAny>>> {
        account_impl::sync::get_greeks(self, ccy)
    }

    /// Position Builder 构建模拟仓位。
    #[pyo3(signature = (acct_lv=None, incl_real_pos_and_eq=None, lever=None, greeks_type=None, sim_pos_json=None, sim_asset_json=None))]
    fn position_builder(
        &self,
        acct_lv: Option<&str>,
        incl_real_pos_and_eq: Option<bool>,
        lever: Option<&str>,
        greeks_type: Option<&str>,
        sim_pos_json: Option<&str>,
        sim_asset_json: Option<&str>,
    ) -> PyResult<Vec<Py<PyAny>>> {
        account_impl::sync::position_builder(
            self,
            acct_lv,
            incl_real_pos_and_eq,
            lever,
            greeks_type,
            sim_pos_json,
            sim_asset_json,
        )
    }
}
