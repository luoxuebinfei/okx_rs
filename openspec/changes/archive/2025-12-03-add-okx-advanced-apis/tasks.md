## 1. 信息收集与设计
- [x] 1.1 整理官方文档与 Python SDK 缺失端点列表，形成最终覆盖表（REST/WS 分栏）。
- [x] 1.2 设计新增类型与错误映射，确认是否需要 core 级别模型扩展。
- [x] 1.3 明确优先级批次：优先 SubAccount、Convert、Trade/Account 扩展；次级 Grid/CopyTrading/Block/RFQ/Spread；最后 Broker/TradingData/Status/Finance 家族。

## 2. REST 实现
- [x] 2.1 为新家族创建模块与 endpoints 常量，新增 trait 定义与客户端实现。
- [x] 2.2 实现 Trade/Account 扩展端点（mass cancel、Cancel All After、order precheck、easy-convert/one-click repay、加/减保证金、MMP、搬仓、抵押资产等）。
- [x] 2.3 补充参数/响应类型与 serde 标注，覆盖单元/集成测试。

## 3. WebSocket 扩展
- [x] 3.1 补充 Channel 枚举新增频道（fills/order 事件、advanced algo、call-auction/option trades、block/spread/grid/copy/recurring-buy/strategy）。
- [x] 3.2 实现订阅/消息解析测试，更新重连恢复逻辑的订阅集。

## 4. Python 绑定
- [x] 4.1 暴露新增 REST/WS 接口（同步/异步），更新类型与注释。
- [x] 4.2 补充 Python 侧示例与测试。

## 5. 文档与验证
- [x] 5.1 更新 README/CHANGELOG 高级接口章节。
- [x] 5.2 运行 openspec validate add-okx-advanced-apis --strict。
- [x] 5.3 整体测试回归（cargo test / python tests）。
