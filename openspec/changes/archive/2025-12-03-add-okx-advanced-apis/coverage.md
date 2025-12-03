# 覆盖表（REST / WS）

## REST 覆盖（依据 python-okx `consts.py` 与官方文档路径）
- SubAccount、Convert（含 easy-convert/one-click-repay）、CopyTrading、Grid/TradingBot+定投、Block/RFQ（含 MMP）、Spread、Broker、TradingData、Finance/SimpleEarn 全量端点已落地，对应实现与路径校验见 `crates/okx-rest/tests/test_private_api.rs` 与 `test_python_sdk_paths.rs`。
- Trade 扩展：批量撤单/mass-cancel、cancel-all-after、order-precheck 已实现。
- Account 扩展：新增 `position/margin-balance`（调保证金）、`set-riskOffset-type`、`set-auto-loan`，并补充序列化测试。
- Status 模块：新增 `/api/v5/system/status`。

## WS 覆盖
- 公共频道：tickers/books/books5/books50-l2-tbt/books-l2-tbt/trades/candles/mark-price/index-tickers/open-interest/option-summary/funding-rate/estimated-price/price-limit/mark-price-candles/index-candles，已在 `crates/okx-ws/src/channel.rs` 枚举并在 `tests/test_channel.rs` 校验。
- 私有频道与高级频道：orders、orders-algo、orders-advanced-algo、liquidation-warning、balance_and_position、account-greeks、positions、fills、multiple call-auction/option-trades、block-trades/spread-trades/grid/copy/recurring-buy/strategy 等，均在 Channel 枚举与订阅恢复逻辑中覆盖。

## 类型与优先级
- 新增请求类型：`AdjustmentMarginRequest`、`SetRiskOffsetTypeRequest`、`SetAutoLoanRequest`；状态查询保持 `serde_json::Value` 以贴合官方返回。
- core 层无需新增模型，现有 `Value` 即可承载可变字段响应。
- 优先级批次：第 1 批（SubAccount/Convert/Trade+Account 扩展）与第 2 批（Grid/CopyTrading/Block/RFQ/Spread）已完成；第 3 批（Broker/TradingData/Status/Finance）已覆盖并补充 Status 模块。
