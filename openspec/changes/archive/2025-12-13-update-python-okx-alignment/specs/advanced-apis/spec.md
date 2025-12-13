## MODIFIED Requirements

### Requirement: 高级 REST 模块补齐
系统 SHALL 提供官方文档列出的高级 REST 模块（含 SubAccount、Convert 闪兑/一键还债、CopyTrading、Grid/TradingBot+recurring buy、Block/RFQ、Spread、Broker、TradingData、Status、SimpleEarn/Finance/Staking/DeFi/Savings、Signal bot 等），端点路径、参数与返回结构需与官方一致。

本变更额外要求：系统 MUST 覆盖官方 `python-okx` 已封装且当前缺失的高级端点族，包括但不限于：
- Finance / Flexible Loan：`/api/v5/finance/flexible-loan/*`
- Finance / Staking-Defi（ETH/SOL）：`/api/v5/finance/staking-defi/{eth,sol}/*`
- Finance / Savings：`/api/v5/finance/savings/lending-rate-history`

#### Scenario: 模块端点齐全
- **WHEN** 开发者使用新增模块调用上述任一端点
- **THEN** 请求路径、参数序列化和响应反序列化与官方文档匹配并通过测试验证

