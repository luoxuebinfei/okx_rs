# Change: OKX 高级接口补齐

## Why
- 现有仓库仅实现基础 Account/Funding/Trade/Market/Public 与少量 WS 频道，未覆盖官方文档与 Python SDK 的高级家族（子账户、闪兑/一键还债、复制交易、网格/策略、RFQ/Block、Spread、Broker、Status、TradingData、SimpleEarn/Finance 等），功能缺口影响对齐要求。
- 需要通过统一提案明确要补齐的模块范围、优先级与测试口径，避免碎片化开发。

## What Changes
- 补齐缺失的 REST 模块家族：SubAccount、Convert 闪兑/一键还债、CopyTrading、Grid/TradingBot（含 recurring buy）、Block/RFQ、Spread、Broker(ND/FD)、TradingData、Status、SimpleEarn/Finance（staking/defi/savings）、Signal bot 等。
- 补齐 Trade/Account 扩展端点：mass cancel、Cancel All After、order precheck、easy-convert/one-click repay、account rate limit、加/减保证金、风险偏移、MMP、搬仓、抵押资产、自动收益/结算币种等。
- 补齐 WebSocket 频道：fills/order 事件、advanced algo、全市场 trades/call-auction/option trades，以及 block/spread/grid/copy/recurring-buy/strategy 类频道。
- 对应新增 Rust trait/类型、WS channel 枚举、Python 绑定及测试/示例。

## Impact
- 影响 crates：okx-rest、okx-core（新增类型）、okx-ws、okx-py。
- 影响说明：API 行为新增（非破坏），需要新增测试用例与文档。
