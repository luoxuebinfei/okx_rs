## 1. 评审与决策

- [x] 1.1 确认“官方兼容返回”对外形态（结论：对外采用 `*_raw`；并允许后续补充低层通用 raw 请求入口）
- [x] 1.2 确认 `x-simulated-trading` 在 `simulated=false` 时的发送策略（结论：省略该请求头；`simulated=true` 必须发送 `1`）
- [x] 1.3 明确本变更是否引入“查询参数兼容模式”（结论：不引入；仅在出现可复现失败案例时做端点级最小逃生口）

## 2. Rust：okx-rest 端点补齐（按模块）

### 2.1 Account
- [x] 2.1.1 增加 `/api/v5/account/instruments`、`/api/v5/account/risk-state` 的端点常量、参数结构体与方法实现
- [x] 2.1.2 补齐对应响应类型（如官方响应结构需要新增类型）

### 2.2 Finance
- [x] 2.2.1 增加 Flexible Loan 8 个端点的常量、参数与方法
- [x] 2.2.2 增加 Staking-Defi ETH/SOL 12 个端点的常量、参数与方法
- [x] 2.2.3 增加 Savings 的 `/api/v5/finance/savings/lending-rate-history`

### 2.3 Market
- [x] 2.3.1 增加 `/api/v5/market/exchange-rate`、`/api/v5/market/index-components`、`/api/v5/market/platform-24-volume`

### 2.4 Public
- [x] 2.4.1 增加 `/api/v5/public/instrument-tick-bands`、`/api/v5/public/option-trades`

### 2.5 Trade
- [x] 2.5.1 增加 one-click-repay v2 三个端点的常量、参数与方法

## 3. Rust：okx-core 行为对齐

- [x] 3.1 按评审决策调整 `x-simulated-trading` 发送规则并补充单测

## 4. Python 绑定：okx-py 暴露新增端点

- [x] 4.1 为同步客户端新增方法并与 Rust trait 一一对应
- [x] 4.2 为异步客户端新增方法并保证与同步客户端方法集合一致
- [x] 4.3 新增“官方兼容返回”接口（按 1.1 决策）

## 5. 测试与文档

- [x] 5.1 扩展 `okx-rest` 端点路径对齐单测覆盖新增端点
- [x] 5.2 扩展 Python 绑定覆盖回归测试与表面契约一致性测试
- [x] 5.3 更新 `docs/zh/python.md` 与 `docs/zh/rust-api.md`（仅补充新增端点与 raw 接口说明）
