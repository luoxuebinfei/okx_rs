//! Python type wrappers for OKX SDK types.

use pyo3::prelude::*;

use okx_core::types::{
    AccountConfig, AlgoOrder, AmendOrderResponse, AssetBalance, Balance, BalanceDetail,
    CancelAlgoOrderResponse, CancelOrderResponse, Candle, CurrencyInfo, DepositAddress,
    DepositRecord, Fill, FundingRate, FundsTransferResponse, IndexTicker, MarkPrice, Order,
    OrderBook, PlaceAlgoOrderResponse, PlaceOrderResponse, Position, Ticker, Trade,
    WithdrawalRecord, WithdrawalResponse,
};
use okx_core::{Config, Credentials};
use okx_rest::api::account::{
    AccountPositionRisk, BalanceRiskData, FeeRates, LeverageInfo, MaxAvailSize, MaxSize,
    PositionRiskData, SetLeverageResponse, SetPositionModeResponse,
};
use okx_rest::api::trade::ClosePositionResponse;

/// Python wrapper for Credentials.
#[pyclass(name = "Credentials")]
#[derive(Clone)]
pub struct PyCredentials {
    pub(crate) inner: Credentials,
}

#[pymethods]
impl PyCredentials {
    /// Create new credentials.
    ///
    /// Args:
    ///     api_key: The API key from OKX
    ///     secret_key: The secret key for signing
    ///     passphrase: The passphrase set when creating the API key
    #[new]
    fn new(api_key: String, secret_key: String, passphrase: String) -> Self {
        Self {
            inner: Credentials::new(api_key, secret_key, passphrase),
        }
    }

    fn __repr__(&self) -> String {
        format!(
            "Credentials(api_key='{}...')",
            &self.inner.api_key()[..8.min(self.inner.api_key().len())]
        )
    }
}

/// Python wrapper for Config.
#[pyclass(name = "Config")]
#[derive(Clone)]
pub struct PyConfig {
    pub(crate) inner: Config,
}

#[pymethods]
impl PyConfig {
    /// Create new configuration.
    ///
    /// Args:
    ///     credentials: API credentials
    ///     simulated: Whether to use simulated (demo) trading (default: False)
    ///     timeout_secs: Request timeout in seconds (default: 30)
    #[new]
    #[pyo3(signature = (credentials, simulated=false, timeout_secs=30))]
    fn new(credentials: PyCredentials, simulated: bool, timeout_secs: u64) -> Self {
        let config = Config::new(credentials.inner)
            .simulated(simulated)
            .with_timeout_secs(timeout_secs);
        Self { inner: config }
    }

    /// Check if simulated trading is enabled.
    #[getter]
    fn simulated(&self) -> bool {
        self.inner.is_simulated()
    }

    /// Get the REST API URL.
    #[getter]
    fn rest_url(&self) -> &str {
        self.inner.rest_url()
    }

    fn __repr__(&self) -> String {
        format!(
            "Config(simulated={}, rest_url='{}')",
            self.inner.is_simulated(),
            self.inner.rest_url()
        )
    }
}

#[cfg(test)]
mod tests {}

/// Python wrapper for Balance.
#[pyclass(name = "Balance")]
#[derive(Clone)]
pub struct PyBalance {
    inner: Balance,
}

impl From<Balance> for PyBalance {
    fn from(inner: Balance) -> Self {
        Self { inner }
    }
}

#[pymethods]
impl PyBalance {
    /// Total equity in USD.
    #[getter]
    fn total_eq(&self) -> &str {
        &self.inner.total_eq
    }

    /// Isolated margin equity in USD.
    #[getter]
    fn iso_eq(&self) -> &str {
        &self.inner.iso_eq
    }

    /// Margin ratio.
    #[getter]
    fn mgn_ratio(&self) -> &str {
        &self.inner.mgn_ratio
    }

    /// Get balance details as a list of dicts.
    #[getter]
    fn details(&self) -> Vec<PyBalanceDetail> {
        self.inner
            .details
            .iter()
            .map(|d| PyBalanceDetail::from(d.clone()))
            .collect()
    }

    fn __repr__(&self) -> String {
        format!(
            "Balance(total_eq='{}', currencies={})",
            self.inner.total_eq,
            self.inner.details.len()
        )
    }
}

/// Python wrapper for BalanceDetail.
#[pyclass(name = "BalanceDetail")]
#[derive(Clone)]
pub struct PyBalanceDetail {
    inner: BalanceDetail,
}

impl From<BalanceDetail> for PyBalanceDetail {
    fn from(inner: BalanceDetail) -> Self {
        Self { inner }
    }
}

#[pymethods]
impl PyBalanceDetail {
    /// Currency code.
    #[getter]
    fn ccy(&self) -> &str {
        &self.inner.ccy
    }

    /// Equity.
    #[getter]
    fn eq(&self) -> &str {
        &self.inner.eq
    }

    /// Available balance.
    #[getter]
    fn avail_bal(&self) -> &str {
        &self.inner.avail_bal
    }

    /// Frozen balance.
    #[getter]
    fn frozen_bal(&self) -> &str {
        &self.inner.frozen_bal
    }

    fn __repr__(&self) -> String {
        format!(
            "BalanceDetail(ccy='{}', eq='{}')",
            self.inner.ccy, self.inner.eq
        )
    }
}

/// Python wrapper for Position.
#[pyclass(name = "Position")]
#[derive(Clone)]
pub struct PyPosition {
    inner: Position,
}

impl From<Position> for PyPosition {
    fn from(inner: Position) -> Self {
        Self { inner }
    }
}

#[pymethods]
impl PyPosition {
    /// Instrument ID.
    #[getter]
    fn inst_id(&self) -> &str {
        &self.inner.inst_id
    }

    /// Position side.
    #[getter]
    fn pos_side(&self) -> &str {
        &self.inner.pos_side
    }

    /// Position quantity.
    #[getter]
    fn pos(&self) -> &str {
        &self.inner.pos
    }

    /// Average open price.
    #[getter]
    fn avg_px(&self) -> &str {
        &self.inner.avg_px
    }

    /// Unrealized PnL.
    #[getter]
    fn upl(&self) -> &str {
        &self.inner.upl
    }

    /// Leverage.
    #[getter]
    fn lever(&self) -> &str {
        &self.inner.lever
    }

    fn __repr__(&self) -> String {
        format!(
            "Position(inst_id='{}', pos='{}', upl='{}')",
            self.inner.inst_id, self.inner.pos, self.inner.upl
        )
    }
}

/// Python wrapper for Order.
#[pyclass(name = "Order")]
#[derive(Clone)]
pub struct PyOrder {
    inner: Order,
}

impl From<Order> for PyOrder {
    fn from(inner: Order) -> Self {
        Self { inner }
    }
}

#[pymethods]
impl PyOrder {
    /// Order ID.
    #[getter]
    fn ord_id(&self) -> &str {
        &self.inner.ord_id
    }

    /// Client order ID.
    #[getter]
    fn cl_ord_id(&self) -> &str {
        &self.inner.cl_ord_id
    }

    /// Instrument ID.
    #[getter]
    fn inst_id(&self) -> &str {
        &self.inner.inst_id
    }

    /// Order side.
    #[getter]
    fn side(&self) -> &str {
        &self.inner.side
    }

    /// Order type.
    #[getter]
    fn ord_type(&self) -> &str {
        &self.inner.ord_type
    }

    /// Order state.
    #[getter]
    fn state(&self) -> &str {
        &self.inner.state
    }

    /// Price.
    #[getter]
    fn px(&self) -> &str {
        &self.inner.px
    }

    /// Size.
    #[getter]
    fn sz(&self) -> &str {
        &self.inner.sz
    }

    /// Filled size.
    #[getter]
    fn acc_fill_sz(&self) -> &str {
        &self.inner.acc_fill_sz
    }

    /// Average fill price.
    #[getter]
    fn avg_px(&self) -> &str {
        &self.inner.avg_px
    }

    fn __repr__(&self) -> String {
        format!(
            "Order(ord_id='{}', inst_id='{}', side='{}', state='{}')",
            self.inner.ord_id, self.inner.inst_id, self.inner.side, self.inner.state
        )
    }
}

/// Python wrapper for Ticker.
#[pyclass(name = "Ticker")]
#[derive(Clone)]
pub struct PyTicker {
    inner: Ticker,
}

impl From<Ticker> for PyTicker {
    fn from(inner: Ticker) -> Self {
        Self { inner }
    }
}

#[pymethods]
impl PyTicker {
    /// Instrument ID.
    #[getter]
    fn inst_id(&self) -> &str {
        &self.inner.inst_id
    }

    /// Last traded price.
    #[getter]
    fn last(&self) -> &str {
        &self.inner.last
    }

    /// Best ask price.
    #[getter]
    fn ask_px(&self) -> &str {
        &self.inner.ask_px
    }

    /// Best bid price.
    #[getter]
    fn bid_px(&self) -> &str {
        &self.inner.bid_px
    }

    /// 24h high.
    #[getter]
    fn high_24h(&self) -> &str {
        &self.inner.high_24h
    }

    /// 24h low.
    #[getter]
    fn low_24h(&self) -> &str {
        &self.inner.low_24h
    }

    /// 24h volume.
    #[getter]
    fn vol_24h(&self) -> &str {
        &self.inner.vol_24h
    }

    fn __repr__(&self) -> String {
        format!(
            "Ticker(inst_id='{}', last='{}', bid='{}', ask='{}')",
            self.inner.inst_id, self.inner.last, self.inner.bid_px, self.inner.ask_px
        )
    }
}

/// 账户配置。
#[pyclass(name = "AccountConfig")]
#[derive(Clone)]
pub struct PyAccountConfig {
    #[pyo3(get)]
    pub uid: String,
    #[pyo3(get)]
    pub main_uid: String,
    #[pyo3(get)]
    pub acct_lv: String,
    #[pyo3(get)]
    pub pos_mode: String,
    #[pyo3(get)]
    pub auto_loan: bool,
    #[pyo3(get)]
    pub greeks_type: String,
    #[pyo3(get)]
    pub level: String,
    #[pyo3(get)]
    pub level_tmp: String,
    #[pyo3(get)]
    pub ct_iso_mode: String,
    #[pyo3(get)]
    pub mgn_iso_mode: String,
    #[pyo3(get)]
    pub risk_offset_type: String,
    #[pyo3(get)]
    pub spot_offset_type: String,
    #[pyo3(get)]
    pub label: String,
    #[pyo3(get)]
    pub role_type: String,
    #[pyo3(get)]
    pub spot_role_type: String,
    #[pyo3(get)]
    pub opauth: String,
    #[pyo3(get)]
    pub kyc_lv: String,
    #[pyo3(get)]
    pub enable_spot_borrow: bool,
    #[pyo3(get)]
    pub spot_borrow_auto_repay: bool,
}

impl From<AccountConfig> for PyAccountConfig {
    fn from(cfg: AccountConfig) -> Self {
        Self {
            uid: cfg.uid,
            main_uid: cfg.main_uid,
            acct_lv: cfg.acct_lv,
            pos_mode: cfg.pos_mode,
            auto_loan: cfg.auto_loan,
            greeks_type: cfg.greeks_type,
            level: cfg.level,
            level_tmp: cfg.level_tmp,
            ct_iso_mode: cfg.ct_iso_mode,
            mgn_iso_mode: cfg.mgn_iso_mode,
            risk_offset_type: cfg.risk_offset_type,
            spot_offset_type: cfg.spot_offset_type,
            label: cfg.label,
            role_type: cfg.role_type,
            spot_role_type: cfg.spot_role_type,
            opauth: cfg.opauth,
            kyc_lv: cfg.kyc_lv,
            enable_spot_borrow: cfg.enable_spot_borrow,
            spot_borrow_auto_repay: cfg.spot_borrow_auto_repay,
        }
    }
}

#[pymethods]
impl PyAccountConfig {
    fn __repr__(&self) -> String {
        format!(
            "AccountConfig(uid='{}', acct_lv='{}', pos_mode='{}')",
            self.uid, self.acct_lv, self.pos_mode
        )
    }
}

/// 调整杠杆后的返回。
#[pyclass(name = "SetLeverageResult")]
#[derive(Clone)]
pub struct PySetLeverageResult {
    #[pyo3(get)]
    pub lever: String,
    #[pyo3(get)]
    pub mgn_mode: String,
    #[pyo3(get)]
    pub inst_id: String,
    #[pyo3(get)]
    pub pos_side: String,
}

impl From<SetLeverageResponse> for PySetLeverageResult {
    fn from(res: SetLeverageResponse) -> Self {
        Self {
            lever: res.lever,
            mgn_mode: res.mgn_mode,
            inst_id: res.inst_id,
            pos_side: res.pos_side,
        }
    }
}

#[pymethods]
impl PySetLeverageResult {
    fn __repr__(&self) -> String {
        format!(
            "SetLeverageResult(inst_id='{}', lever='{}', pos_side='{}')",
            self.inst_id, self.lever, self.pos_side
        )
    }
}

/// 杠杆信息。
#[pyclass(name = "LeverageInfo")]
#[derive(Clone)]
pub struct PyLeverageInfo {
    #[pyo3(get)]
    pub inst_id: String,
    #[pyo3(get)]
    pub mgn_mode: String,
    #[pyo3(get)]
    pub pos_side: String,
    #[pyo3(get)]
    pub lever: String,
}

impl From<LeverageInfo> for PyLeverageInfo {
    fn from(info: LeverageInfo) -> Self {
        Self {
            inst_id: info.inst_id,
            mgn_mode: info.mgn_mode,
            pos_side: info.pos_side,
            lever: info.lever,
        }
    }
}

#[pymethods]
impl PyLeverageInfo {
    fn __repr__(&self) -> String {
        format!(
            "LeverageInfo(inst_id='{}', mgn_mode='{}', lever='{}')",
            self.inst_id, self.mgn_mode, self.lever
        )
    }
}

/// 最大可下单张数。
#[pyclass(name = "MaxSize")]
#[derive(Clone)]
pub struct PyMaxSize {
    #[pyo3(get)]
    pub inst_id: String,
    #[pyo3(get)]
    pub ccy: String,
    #[pyo3(get)]
    pub max_buy: String,
    #[pyo3(get)]
    pub max_sell: String,
}

impl From<MaxSize> for PyMaxSize {
    fn from(v: MaxSize) -> Self {
        Self {
            inst_id: v.inst_id,
            ccy: v.ccy,
            max_buy: v.max_buy,
            max_sell: v.max_sell,
        }
    }
}

#[pymethods]
impl PyMaxSize {
    fn __repr__(&self) -> String {
        format!(
            "MaxSize(inst_id='{}', buy='{}', sell='{}')",
            self.inst_id, self.max_buy, self.max_sell
        )
    }
}

/// 最大可用张数。
#[pyclass(name = "MaxAvailSize")]
#[derive(Clone)]
pub struct PyMaxAvailSize {
    #[pyo3(get)]
    pub inst_id: String,
    #[pyo3(get)]
    pub avail_buy: String,
    #[pyo3(get)]
    pub avail_sell: String,
}

impl From<MaxAvailSize> for PyMaxAvailSize {
    fn from(v: MaxAvailSize) -> Self {
        Self {
            inst_id: v.inst_id,
            avail_buy: v.avail_buy,
            avail_sell: v.avail_sell,
        }
    }
}

#[pymethods]
impl PyMaxAvailSize {
    fn __repr__(&self) -> String {
        format!(
            "MaxAvailSize(inst_id='{}', buy='{}', sell='{}')",
            self.inst_id, self.avail_buy, self.avail_sell
        )
    }
}

/// 手续费率。
#[pyclass(name = "FeeRates")]
#[derive(Clone)]
pub struct PyFeeRates {
    #[pyo3(get)]
    pub inst_type: String,
    #[pyo3(get)]
    pub maker: String,
    #[pyo3(get)]
    pub taker: String,
    #[pyo3(get)]
    pub maker_u: String,
    #[pyo3(get)]
    pub taker_u: String,
    #[pyo3(get)]
    pub maker_usdc: String,
    #[pyo3(get)]
    pub taker_usdc: String,
    #[pyo3(get)]
    pub level: String,
    #[pyo3(get)]
    pub ts: String,
}

impl From<FeeRates> for PyFeeRates {
    fn from(v: FeeRates) -> Self {
        Self {
            inst_type: v.inst_type,
            maker: v.maker,
            taker: v.taker,
            maker_u: v.maker_u,
            taker_u: v.taker_u,
            maker_usdc: v.maker_usdc,
            taker_usdc: v.taker_usdc,
            level: v.level,
            ts: v.ts,
        }
    }
}

#[pymethods]
impl PyFeeRates {
    fn __repr__(&self) -> String {
        format!(
            "FeeRates(inst_type='{}', maker='{}', taker='{}')",
            self.inst_type, self.maker, self.taker
        )
    }
}

/// 调整持仓模式结果。
#[pyclass(name = "SetPositionModeResult")]
#[derive(Clone)]
pub struct PySetPositionModeResult {
    #[pyo3(get)]
    pub pos_mode: String,
}

impl From<SetPositionModeResponse> for PySetPositionModeResult {
    fn from(v: SetPositionModeResponse) -> Self {
        Self {
            pos_mode: v.pos_mode,
        }
    }
}

#[pymethods]
impl PySetPositionModeResult {
    fn __repr__(&self) -> String {
        format!("SetPositionModeResult(pos_mode='{}')", self.pos_mode)
    }
}

/// 资产风险信息。
#[pyclass(name = "BalanceRiskData")]
#[derive(Clone)]
pub struct PyBalanceRiskData {
    #[pyo3(get)]
    pub ccy: String,
    #[pyo3(get)]
    pub eq: String,
    #[pyo3(get)]
    pub dis_eq: String,
}

impl From<BalanceRiskData> for PyBalanceRiskData {
    fn from(v: BalanceRiskData) -> Self {
        Self {
            ccy: v.ccy,
            eq: v.eq,
            dis_eq: v.dis_eq,
        }
    }
}

/// 持仓风险信息。
#[pyclass(name = "PositionRiskData")]
#[derive(Clone)]
pub struct PyPositionRiskData {
    #[pyo3(get)]
    pub inst_id: String,
    #[pyo3(get)]
    pub inst_type: String,
    #[pyo3(get)]
    pub mgn_mode: String,
    #[pyo3(get)]
    pub pos_side: String,
    #[pyo3(get)]
    pub pos: String,
    #[pyo3(get)]
    pub base_bal: String,
    #[pyo3(get)]
    pub quote_bal: String,
    #[pyo3(get)]
    pub pos_ccy: String,
    #[pyo3(get)]
    pub ccy: String,
    #[pyo3(get)]
    pub notional_ccy: String,
    #[pyo3(get)]
    pub notional_usd: String,
}

impl From<PositionRiskData> for PyPositionRiskData {
    fn from(v: PositionRiskData) -> Self {
        Self {
            inst_id: v.inst_id,
            inst_type: v.inst_type,
            mgn_mode: v.mgn_mode,
            pos_side: v.pos_side,
            pos: v.pos,
            base_bal: v.base_bal,
            quote_bal: v.quote_bal,
            pos_ccy: v.pos_ccy,
            ccy: v.ccy,
            notional_ccy: v.notional_ccy,
            notional_usd: v.notional_usd,
        }
    }
}

/// 账户持仓风险。
#[pyclass(name = "AccountPositionRisk")]
#[derive(Clone)]
pub struct PyAccountPositionRisk {
    #[pyo3(get)]
    pub adj_eq: String,
    #[pyo3(get)]
    pub bal_data: Vec<PyBalanceRiskData>,
    #[pyo3(get)]
    pub pos_data: Vec<PyPositionRiskData>,
    #[pyo3(get)]
    pub ts: String,
}

impl From<AccountPositionRisk> for PyAccountPositionRisk {
    fn from(v: AccountPositionRisk) -> Self {
        Self {
            adj_eq: v.adj_eq,
            bal_data: v
                .bal_data
                .into_iter()
                .map(PyBalanceRiskData::from)
                .collect(),
            pos_data: v
                .pos_data
                .into_iter()
                .map(PyPositionRiskData::from)
                .collect(),
            ts: v.ts,
        }
    }
}

#[pymethods]
impl PyAccountPositionRisk {
    fn __repr__(&self) -> String {
        format!(
            "AccountPositionRisk(adj_eq='{}', balances={}, positions={})",
            self.adj_eq,
            self.bal_data.len(),
            self.pos_data.len()
        )
    }
}

/// 成交明细。
#[pyclass(name = "Fill")]
#[derive(Clone)]
pub struct PyFill {
    #[pyo3(get)]
    pub inst_type: String,
    #[pyo3(get)]
    pub inst_id: String,
    #[pyo3(get)]
    pub trade_id: String,
    #[pyo3(get)]
    pub ord_id: String,
    #[pyo3(get)]
    pub cl_ord_id: String,
    #[pyo3(get)]
    pub bill_id: String,
    #[pyo3(get)]
    pub tag: String,
    #[pyo3(get)]
    pub fill_px: String,
    #[pyo3(get)]
    pub fill_sz: String,
    #[pyo3(get)]
    pub fill_pnl: String,
    #[pyo3(get)]
    pub side: String,
    #[pyo3(get)]
    pub pos_side: String,
    #[pyo3(get)]
    pub exec_type: String,
    #[pyo3(get)]
    pub fee_ccy: String,
    #[pyo3(get)]
    pub fee: String,
    #[pyo3(get)]
    pub ts: String,
    #[pyo3(get)]
    pub fill_idx_px: String,
    #[pyo3(get)]
    pub fill_mark_px: String,
    #[pyo3(get)]
    pub fill_time: String,
    #[pyo3(get)]
    pub fill_mark_vol: String,
    #[pyo3(get)]
    pub fill_fwd_px: String,
}

impl From<Fill> for PyFill {
    fn from(v: Fill) -> Self {
        Self {
            inst_type: v.inst_type,
            inst_id: v.inst_id,
            trade_id: v.trade_id,
            ord_id: v.ord_id,
            cl_ord_id: v.cl_ord_id,
            bill_id: v.bill_id,
            tag: v.tag,
            fill_px: v.fill_px,
            fill_sz: v.fill_sz,
            fill_pnl: v.fill_pnl,
            side: v.side,
            pos_side: v.pos_side,
            exec_type: v.exec_type,
            fee_ccy: v.fee_ccy,
            fee: v.fee,
            ts: v.ts,
            fill_idx_px: v.fill_idx_px,
            fill_mark_px: v.fill_mark_px,
            fill_time: v.fill_time,
            fill_mark_vol: v.fill_mark_vol,
            fill_fwd_px: v.fill_fwd_px,
        }
    }
}

#[pymethods]
impl PyFill {
    fn __repr__(&self) -> String {
        format!(
            "Fill(ord_id='{}', inst_id='{}', px='{}', sz='{}')",
            self.ord_id, self.inst_id, self.fill_px, self.fill_sz
        )
    }
}

/// 下单返回。
#[pyclass(name = "PlaceOrderResult")]
#[derive(Clone)]
pub struct PyPlaceOrderResult {
    #[pyo3(get)]
    pub ord_id: String,
    #[pyo3(get)]
    pub cl_ord_id: String,
    #[pyo3(get)]
    pub tag: String,
    #[pyo3(get)]
    pub s_code: String,
    #[pyo3(get)]
    pub s_msg: String,
}

impl From<PlaceOrderResponse> for PyPlaceOrderResult {
    fn from(v: PlaceOrderResponse) -> Self {
        Self {
            ord_id: v.ord_id,
            cl_ord_id: v.cl_ord_id,
            tag: v.tag,
            s_code: v.s_code,
            s_msg: v.s_msg,
        }
    }
}

/// 撤单返回。
#[pyclass(name = "CancelOrderResult")]
#[derive(Clone)]
pub struct PyCancelOrderResult {
    #[pyo3(get)]
    pub ord_id: String,
    #[pyo3(get)]
    pub cl_ord_id: String,
    #[pyo3(get)]
    pub s_code: String,
    #[pyo3(get)]
    pub s_msg: String,
}

impl From<CancelOrderResponse> for PyCancelOrderResult {
    fn from(v: CancelOrderResponse) -> Self {
        Self {
            ord_id: v.ord_id,
            cl_ord_id: v.cl_ord_id,
            s_code: v.s_code,
            s_msg: v.s_msg,
        }
    }
}

/// 改单返回。
#[pyclass(name = "AmendOrderResult")]
#[derive(Clone)]
pub struct PyAmendOrderResult {
    #[pyo3(get)]
    pub ord_id: String,
    #[pyo3(get)]
    pub cl_ord_id: String,
    #[pyo3(get)]
    pub req_id: String,
    #[pyo3(get)]
    pub s_code: String,
    #[pyo3(get)]
    pub s_msg: String,
}

impl From<AmendOrderResponse> for PyAmendOrderResult {
    fn from(v: AmendOrderResponse) -> Self {
        Self {
            ord_id: v.ord_id,
            cl_ord_id: v.cl_ord_id,
            req_id: v.req_id,
            s_code: v.s_code,
            s_msg: v.s_msg,
        }
    }
}

/// 算法单下单返回。
#[pyclass(name = "PlaceAlgoOrderResult")]
#[derive(Clone)]
pub struct PyPlaceAlgoOrderResult {
    #[pyo3(get)]
    pub algo_id: String,
    #[pyo3(get)]
    pub algo_cl_ord_id: String,
    #[pyo3(get)]
    pub s_code: String,
    #[pyo3(get)]
    pub s_msg: String,
}

impl From<PlaceAlgoOrderResponse> for PyPlaceAlgoOrderResult {
    fn from(v: PlaceAlgoOrderResponse) -> Self {
        Self {
            algo_id: v.algo_id,
            algo_cl_ord_id: v.algo_cl_ord_id,
            s_code: v.s_code,
            s_msg: v.s_msg,
        }
    }
}

/// 算法单撤单返回。
#[pyclass(name = "CancelAlgoOrderResult")]
#[derive(Clone)]
pub struct PyCancelAlgoOrderResult {
    #[pyo3(get)]
    pub algo_id: String,
    #[pyo3(get)]
    pub s_code: String,
    #[pyo3(get)]
    pub s_msg: String,
}

impl From<CancelAlgoOrderResponse> for PyCancelAlgoOrderResult {
    fn from(v: CancelAlgoOrderResponse) -> Self {
        Self {
            algo_id: v.algo_id,
            s_code: v.s_code,
            s_msg: v.s_msg,
        }
    }
}

/// 平仓返回。
#[pyclass(name = "ClosePositionResult")]
#[derive(Clone)]
pub struct PyClosePositionResult {
    #[pyo3(get)]
    pub inst_id: String,
    #[pyo3(get)]
    pub pos_side: String,
    #[pyo3(get)]
    pub cl_ord_id: String,
    #[pyo3(get)]
    pub tag: String,
}

impl From<ClosePositionResponse> for PyClosePositionResult {
    fn from(v: ClosePositionResponse) -> Self {
        Self {
            inst_id: v.inst_id,
            pos_side: v.pos_side,
            cl_ord_id: v.cl_ord_id,
            tag: v.tag,
        }
    }
}

/// 算法单详情。
#[pyclass(name = "AlgoOrder")]
#[derive(Clone)]
pub struct PyAlgoOrder {
    #[pyo3(get)]
    pub inst_type: String,
    #[pyo3(get)]
    pub inst_id: String,
    #[pyo3(get)]
    pub algo_id: String,
    #[pyo3(get)]
    pub algo_cl_ord_id: String,
    #[pyo3(get)]
    pub ord_type: String,
    #[pyo3(get)]
    pub side: String,
    #[pyo3(get)]
    pub pos_side: String,
    #[pyo3(get)]
    pub td_mode: String,
    #[pyo3(get)]
    pub sz: String,
    #[pyo3(get)]
    pub ccy: String,
    #[pyo3(get)]
    pub state: String,
    #[pyo3(get)]
    pub trigger_px: String,
    #[pyo3(get)]
    pub order_px: String,
    #[pyo3(get)]
    pub actual_px: String,
    #[pyo3(get)]
    pub actual_sz: String,
    #[pyo3(get)]
    pub actual_side: String,
    #[pyo3(get)]
    pub trigger_px_type: String,
    #[pyo3(get)]
    pub tp_trigger_px: String,
    #[pyo3(get)]
    pub tp_ord_px: String,
    #[pyo3(get)]
    pub sl_trigger_px: String,
    #[pyo3(get)]
    pub sl_ord_px: String,
    #[pyo3(get)]
    pub trigger_time: String,
    #[pyo3(get)]
    pub c_time: String,
}

impl From<AlgoOrder> for PyAlgoOrder {
    fn from(v: AlgoOrder) -> Self {
        Self {
            inst_type: v.inst_type,
            inst_id: v.inst_id,
            algo_id: v.algo_id,
            algo_cl_ord_id: v.algo_cl_ord_id,
            ord_type: v.ord_type,
            side: v.side,
            pos_side: v.pos_side,
            td_mode: v.td_mode,
            sz: v.sz,
            ccy: v.ccy,
            state: v.state,
            trigger_px: v.trigger_px,
            order_px: v.order_px,
            actual_px: v.actual_px,
            actual_sz: v.actual_sz,
            actual_side: v.actual_side,
            trigger_px_type: v.trigger_px_type,
            tp_trigger_px: v.tp_trigger_px,
            tp_ord_px: v.tp_ord_px,
            sl_trigger_px: v.sl_trigger_px,
            sl_ord_px: v.sl_ord_px,
            trigger_time: v.trigger_time,
            c_time: v.c_time,
        }
    }
}

/// 资金账户余额。
#[pyclass(name = "AssetBalance")]
#[derive(Clone)]
pub struct PyAssetBalance {
    #[pyo3(get)]
    pub ccy: String,
    #[pyo3(get)]
    pub bal: String,
    #[pyo3(get)]
    pub frozen_bal: String,
    #[pyo3(get)]
    pub avail_bal: String,
}

impl From<AssetBalance> for PyAssetBalance {
    fn from(v: AssetBalance) -> Self {
        Self {
            ccy: v.ccy,
            bal: v.bal,
            frozen_bal: v.frozen_bal,
            avail_bal: v.avail_bal,
        }
    }
}

/// 充值地址。
#[pyclass(name = "DepositAddress")]
#[derive(Clone)]
pub struct PyDepositAddress {
    #[pyo3(get)]
    pub ccy: String,
    #[pyo3(get)]
    pub chain: String,
    #[pyo3(get)]
    pub addr: String,
    #[pyo3(get)]
    pub tag: String,
    #[pyo3(get)]
    pub pmt_id: String,
    #[pyo3(get)]
    pub memo: String,
    #[pyo3(get)]
    pub ct_addr: String,
    #[pyo3(get)]
    pub selected: bool,
}

impl From<DepositAddress> for PyDepositAddress {
    fn from(v: DepositAddress) -> Self {
        Self {
            ccy: v.ccy,
            chain: v.chain,
            addr: v.addr,
            tag: v.tag,
            pmt_id: v.pmt_id,
            memo: v.memo,
            ct_addr: v.ct_addr,
            selected: v.selected,
        }
    }
}

/// 充值记录。
#[pyclass(name = "DepositRecord")]
#[derive(Clone)]
pub struct PyDepositRecord {
    #[pyo3(get)]
    pub ccy: String,
    #[pyo3(get)]
    pub chain: String,
    #[pyo3(get)]
    pub amt: String,
    #[pyo3(get)]
    pub to: String,
    #[pyo3(get)]
    pub tx_id: String,
    #[pyo3(get)]
    pub dep_id: String,
    #[pyo3(get)]
    pub from: String,
    #[pyo3(get)]
    pub state: String,
    #[pyo3(get)]
    pub ts: String,
    #[pyo3(get)]
    pub actual_dep_blk_confirm: String,
}

impl From<DepositRecord> for PyDepositRecord {
    fn from(v: DepositRecord) -> Self {
        Self {
            ccy: v.ccy,
            chain: v.chain,
            amt: v.amt,
            to: v.to,
            tx_id: v.tx_id,
            dep_id: v.dep_id,
            from: v.from,
            state: v.state,
            ts: v.ts,
            actual_dep_blk_confirm: v.actual_dep_blk_confirm,
        }
    }
}

/// 提现记录。
#[pyclass(name = "WithdrawalRecord")]
#[derive(Clone)]
pub struct PyWithdrawalRecord {
    #[pyo3(get)]
    pub ccy: String,
    #[pyo3(get)]
    pub chain: String,
    #[pyo3(get)]
    pub amt: String,
    #[pyo3(get)]
    pub to: String,
    #[pyo3(get)]
    pub tx_id: String,
    #[pyo3(get)]
    pub wd_id: String,
    #[pyo3(get)]
    pub client_id: String,
    #[pyo3(get)]
    pub fee: String,
    #[pyo3(get)]
    pub state: String,
    #[pyo3(get)]
    pub ts: String,
}

impl From<WithdrawalRecord> for PyWithdrawalRecord {
    fn from(v: WithdrawalRecord) -> Self {
        Self {
            ccy: v.ccy,
            chain: v.chain,
            amt: v.amt,
            to: v.to,
            tx_id: v.tx_id,
            wd_id: v.wd_id,
            client_id: v.client_id,
            fee: v.fee,
            state: v.state,
            ts: v.ts,
        }
    }
}

/// 资金划转结果。
#[pyclass(name = "FundsTransferResult")]
#[derive(Clone)]
pub struct PyFundsTransferResult {
    #[pyo3(get)]
    pub trans_id: String,
    #[pyo3(get)]
    pub ccy: String,
    #[pyo3(get)]
    pub client_id: String,
    #[pyo3(get)]
    pub from: String,
    #[pyo3(get)]
    pub amt: String,
    #[pyo3(get)]
    pub to: String,
}

impl From<FundsTransferResponse> for PyFundsTransferResult {
    fn from(v: FundsTransferResponse) -> Self {
        Self {
            trans_id: v.trans_id,
            ccy: v.ccy,
            client_id: v.client_id,
            from: v.from,
            amt: v.amt,
            to: v.to,
        }
    }
}

/// 提现结果。
#[pyclass(name = "WithdrawalResult")]
#[derive(Clone)]
pub struct PyWithdrawalResult {
    #[pyo3(get)]
    pub wd_id: String,
    #[pyo3(get)]
    pub ccy: String,
    #[pyo3(get)]
    pub chain: String,
    #[pyo3(get)]
    pub amt: String,
    #[pyo3(get)]
    pub client_id: String,
}

impl From<WithdrawalResponse> for PyWithdrawalResult {
    fn from(v: WithdrawalResponse) -> Self {
        Self {
            wd_id: v.wd_id,
            ccy: v.ccy,
            chain: v.chain,
            amt: v.amt,
            client_id: v.client_id,
        }
    }
}

/// 币种信息。
#[pyclass(name = "CurrencyInfo")]
#[derive(Clone)]
pub struct PyCurrencyInfo {
    #[pyo3(get)]
    pub ccy: String,
    #[pyo3(get)]
    pub name: String,
    #[pyo3(get)]
    pub logo_link: String,
    #[pyo3(get)]
    pub chain: String,
    #[pyo3(get)]
    pub can_dep: bool,
    #[pyo3(get)]
    pub can_wd: bool,
    #[pyo3(get)]
    pub can_internal: bool,
    #[pyo3(get)]
    pub min_wd: String,
    #[pyo3(get)]
    pub min_fee: String,
    #[pyo3(get)]
    pub max_fee: String,
    #[pyo3(get)]
    pub min_dep: String,
    #[pyo3(get)]
    pub min_dep_arrive_confirm: String,
    #[pyo3(get)]
    pub wd_tick_sz: String,
    #[pyo3(get)]
    pub wd_quota: String,
    #[pyo3(get)]
    pub used_wd_quota: String,
    #[pyo3(get)]
    pub main_net: bool,
}

impl From<CurrencyInfo> for PyCurrencyInfo {
    fn from(v: CurrencyInfo) -> Self {
        Self {
            ccy: v.ccy,
            name: v.name,
            logo_link: v.logo_link,
            chain: v.chain,
            can_dep: v.can_dep,
            can_wd: v.can_wd,
            can_internal: v.can_internal,
            min_wd: v.min_wd,
            min_fee: v.min_fee,
            max_fee: v.max_fee,
            min_dep: v.min_dep,
            min_dep_arrive_confirm: v.min_dep_arrive_confirm,
            wd_tick_sz: v.wd_tick_sz,
            wd_quota: v.wd_quota,
            used_wd_quota: v.used_wd_quota,
            main_net: v.main_net,
        }
    }
}

/// 深度数据。
#[pyclass(name = "OrderBook")]
#[derive(Clone)]
pub struct PyOrderBook {
    #[pyo3(get)]
    pub asks: Vec<Vec<String>>,
    #[pyo3(get)]
    pub bids: Vec<Vec<String>>,
    #[pyo3(get)]
    pub ts: String,
}

impl From<OrderBook> for PyOrderBook {
    fn from(v: OrderBook) -> Self {
        Self {
            asks: v.asks,
            bids: v.bids,
            ts: v.ts,
        }
    }
}

/// K 线数据。
#[pyclass(name = "Candle")]
#[derive(Clone)]
pub struct PyCandle {
    #[pyo3(get)]
    pub ts: String,
    #[pyo3(get)]
    pub open: String,
    #[pyo3(get)]
    pub high: String,
    #[pyo3(get)]
    pub low: String,
    #[pyo3(get)]
    pub close: String,
    #[pyo3(get)]
    pub vol: String,
    #[pyo3(get)]
    pub vol_ccy: String,
    #[pyo3(get)]
    pub vol_ccy_quote: String,
    #[pyo3(get)]
    pub confirm: String,
}

impl From<Candle> for PyCandle {
    fn from(v: Candle) -> Self {
        Self {
            ts: v.ts,
            open: v.open,
            high: v.high,
            low: v.low,
            close: v.close,
            vol: v.vol,
            vol_ccy: v.vol_ccy,
            vol_ccy_quote: v.vol_ccy_quote,
            confirm: v.confirm,
        }
    }
}

/// 行情成交。
#[pyclass(name = "PublicTrade")]
#[derive(Clone)]
pub struct PyPublicTrade {
    #[pyo3(get)]
    pub inst_id: String,
    #[pyo3(get)]
    pub trade_id: String,
    #[pyo3(get)]
    pub px: String,
    #[pyo3(get)]
    pub sz: String,
    #[pyo3(get)]
    pub side: String,
    #[pyo3(get)]
    pub ts: String,
    #[pyo3(get)]
    pub count: String,
}

impl From<Trade> for PyPublicTrade {
    fn from(v: Trade) -> Self {
        Self {
            inst_id: v.inst_id,
            trade_id: v.trade_id,
            px: v.px,
            sz: v.sz,
            side: v.side,
            ts: v.ts,
            count: v.count,
        }
    }
}

/// 指数或标记价格。
#[pyclass(name = "MarkPrice")]
#[derive(Clone)]
pub struct PyMarkPrice {
    #[pyo3(get)]
    pub inst_type: String,
    #[pyo3(get)]
    pub inst_id: String,
    #[pyo3(get)]
    pub mark_px: String,
    #[pyo3(get)]
    pub ts: String,
}

impl From<MarkPrice> for PyMarkPrice {
    fn from(v: MarkPrice) -> Self {
        Self {
            inst_type: v.inst_type,
            inst_id: v.inst_id,
            mark_px: v.mark_px,
            ts: v.ts,
        }
    }
}

/// 资金费率。
#[pyclass(name = "FundingRate")]
#[derive(Clone)]
pub struct PyFundingRate {
    #[pyo3(get)]
    pub inst_type: String,
    #[pyo3(get)]
    pub inst_id: String,
    #[pyo3(get)]
    pub funding_rate: String,
    #[pyo3(get)]
    pub next_funding_rate: String,
    #[pyo3(get)]
    pub funding_time: String,
    #[pyo3(get)]
    pub next_funding_time: String,
    #[pyo3(get)]
    pub min_funding_rate: String,
    #[pyo3(get)]
    pub max_funding_rate: String,
    #[pyo3(get)]
    pub settle_funding_rate: String,
    #[pyo3(get)]
    pub premium: String,
    #[pyo3(get)]
    pub settle_state: String,
    #[pyo3(get)]
    pub method: String,
}

impl From<FundingRate> for PyFundingRate {
    fn from(v: FundingRate) -> Self {
        Self {
            inst_type: v.inst_type,
            inst_id: v.inst_id,
            funding_rate: v.funding_rate,
            next_funding_rate: v.next_funding_rate,
            funding_time: v.funding_time,
            next_funding_time: v.next_funding_time,
            min_funding_rate: v.min_funding_rate,
            max_funding_rate: v.max_funding_rate,
            settle_funding_rate: v.settle_funding_rate,
            premium: v.premium,
            settle_state: v.settle_state,
            method: v.method,
        }
    }
}

/// 指数行情。
#[pyclass(name = "IndexTicker")]
#[derive(Clone)]
pub struct PyIndexTicker {
    #[pyo3(get)]
    pub inst_id: String,
    #[pyo3(get)]
    pub idx_px: String,
    #[pyo3(get)]
    pub high_24h: String,
    #[pyo3(get)]
    pub low_24h: String,
    #[pyo3(get)]
    pub sod_utc_0: String,
    #[pyo3(get)]
    pub sod_utc_8: String,
    #[pyo3(get)]
    pub ts: String,
}

impl From<IndexTicker> for PyIndexTicker {
    fn from(v: IndexTicker) -> Self {
        Self {
            inst_id: v.inst_id,
            idx_px: v.idx_px,
            high_24h: v.high_24h,
            low_24h: v.low_24h,
            sod_utc_0: v.sod_utc_0,
            sod_utc_8: v.sod_utc_8,
            ts: v.ts,
        }
    }
}
