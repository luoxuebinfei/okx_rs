## ADDED Requirements

### Requirement: 高级 REST 模块补齐
系统 SHALL 提供官方文档列出的高级 REST 模块（含 SubAccount、Convert 闪兑/一键还债、CopyTrading、Grid/TradingBot+recurring buy、Block/RFQ、Spread、Broker、TradingData、Status、SimpleEarn/Finance/Staking/DeFi/Savings、Signal bot 等），端点路径、参数与返回结构需与官方一致。

#### Scenario: 模块端点齐全
- **WHEN** 开发者使用新增模块调用任一官方对应端点
- **THEN** 请求路径、参数序列化和响应反序列化与官方文档匹配并通过集成测试

### Requirement: Trade/Account 扩展端点补齐
系统 SHALL 覆盖官方 Trade/Account 扩展端点（如 mass cancel、Cancel All After、order precheck、easy-convert/one-click repay、加/减保证金、杠杆预估、费率类型设置、自动借还、风险偏移、MMP、搬仓、抵押资产、自动收益/结算币种等），并提供对应类型定义与错误处理。

#### Scenario: 扩展端点可用
- **WHEN** 调用任一扩展端点执行或查询
- **THEN** 返回结构符合官方定义，错误码与约束处理一致并有测试覆盖

### Requirement: WebSocket 频道补齐
系统 SHALL 支持官方 WebSocket 频道全集（含 fills/order 事件、advanced algo、全市场 trades/call-auction/option trades、block/spread/grid/copy/recurring-buy/strategy 等），并在断线重连后恢复订阅。

#### Scenario: 频道订阅与恢复
- **WHEN** 客户端订阅任一新增频道并发生重连
- **THEN** 连接恢复后自动重新订阅，消息格式解析正确且通过集成测试
