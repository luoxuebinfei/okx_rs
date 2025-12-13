# 设计说明：对齐官方文档与官方 Python SDK（python-okx）

## 1. 参考来源与对齐口径

本变更的“官方基准”由两部分构成：

1) OKX 官方文档（v5）：鉴权/WS 登录/端点路径与参数定义的**权威来源**。  
2) 官方 Python SDK（okxapi/python-okx）：作为“官方实现行为”的**可观测参考**，用于补齐端点族与核对细节（如路径常量、WS 登录签名拼接方式）。

当两者出现冲突时，优先级为：**官方文档 > 官方 SDK**；但若差异不影响正确性且会导致用户迁移成本，允许通过“兼容增强（开关/并行接口）”同时支持。

## 2. 已确认对齐的关键规则（摘录）

### 2.1 REST 签名

预签名串：`timestamp + method + requestPath + body`  
其中：`GET` 的 query 参数计入 `requestPath`，而不是 `body`。

### 2.2 WebSocket 登录签名

登录签名消息：`timestamp + 'GET' + '/users/self/verify'`  
登录 payload：`{"op":"login","args":[{"apiKey":...,"passphrase":...,"timestamp":...,"sign":...}]}`。

### 2.3 Demo Trading 请求头

官方文档要求：Demo Trading 请求必须带 `x-simulated-trading: 1`。

## 3. 缺失端点清单（以 python-okx 为准）

以下端点在官方 `python-okx` 的 `okx/consts.py` 中存在，且在对应模块中被实际调用；但在当前 Rust `okx-rest` 端点常量集中尚未实现（或未暴露）：

### 3.1 Account（Trading Account）

- `GET /api/v5/account/instruments`（python: `okx/Account.py`）
- `GET /api/v5/account/risk-state`（python: `okx/Account.py`）

### 3.2 Finance

#### Flexible Loan（python: `okx/Finance/FlexibleLoan.py`）

- `GET /api/v5/finance/flexible-loan/borrow-currencies`
- `GET /api/v5/finance/flexible-loan/collateral-assets`
- `GET /api/v5/finance/flexible-loan/max-loan`
- `GET /api/v5/finance/flexible-loan/loan-info`
- `GET /api/v5/finance/flexible-loan/loan-history`
- `GET /api/v5/finance/flexible-loan/interest-accrued`
- `GET /api/v5/finance/flexible-loan/max-collateral-redeem-amount`
- `POST /api/v5/finance/flexible-loan/adjust-collateral`

#### Staking-Defi（python: `okx/Finance/EthStaking.py`, `okx/Finance/SolStaking.py`）

ETH：
- `GET /api/v5/finance/staking-defi/eth/product-info`
- `GET /api/v5/finance/staking-defi/eth/balance`
- `GET /api/v5/finance/staking-defi/eth/apy-history`
- `POST /api/v5/finance/staking-defi/eth/purchase`
- `POST /api/v5/finance/staking-defi/eth/redeem`
- `GET /api/v5/finance/staking-defi/eth/purchase-redeem-history`

SOL：
- `GET /api/v5/finance/staking-defi/sol/product-info`
- `GET /api/v5/finance/staking-defi/sol/balance`
- `GET /api/v5/finance/staking-defi/sol/apy-history`
- `POST /api/v5/finance/staking-defi/sol/purchase`
- `POST /api/v5/finance/staking-defi/sol/redeem`
- `GET /api/v5/finance/staking-defi/sol/purchase-redeem-history`

#### Savings（python: `okx/Finance/Savings.py`）

- `GET /api/v5/finance/savings/lending-rate-history`

### 3.3 Market Data（python: `okx/MarketData.py`）

- `GET /api/v5/market/exchange-rate`
- `GET /api/v5/market/index-components`
- `GET /api/v5/market/platform-24-volume`

### 3.4 Public Data（python: `okx/PublicData.py`）

- `GET /api/v5/public/instrument-tick-bands`
- `GET /api/v5/public/option-trades`

> 注：官方 `python-okx` 的常量中包含 `/api/v5/public/liquidation-orders`，但在本仓库缓存的两份官方文档快照中未检索到对应的 REST 端点定义；同时 OKX WebSocket 存在 `liquidation-orders` 频道。故本变更不将其纳入“REST 对齐”验收范围，后续如需支持将以 WebSocket 能力单独补齐并配套规格与测试。

### 3.5 Trade（python: `okx/Trade.py`）

- `GET /api/v5/trade/one-click-repay-currency-list-v2`
- `POST /api/v5/trade/one-click-repay-v2`
- `GET /api/v5/trade/one-click-repay-history-v2`

## 4. 行为对齐策略（关键决策）

### 4.1 返回结构：类型化 vs 官方兼容

现状：
- Rust/绑定默认返回 `Vec<T>`（仅 `data`），并在 `code!=0` 时抛错。
- 官方 `python-okx` 返回完整 JSON（`code/msg/data`）并由用户自行处理。

方案（本变更采用，已决策）：
- 保持现有“类型化返回”不变；
- 在 Python 绑定侧对外提供 `*_raw` 方法族，返回与官方 `python-okx` 一致的完整响应结构；
- 允许后续补充一个“低层通用 raw 请求入口”（用于覆盖盲区与排障），但不以此替代 `*_raw` 的显式 API。

### 4.2 模拟盘请求头：安全优先

原则：默认不进入模拟盘（避免误操作真实账户），且在模拟盘模式下必须满足官方文档要求。

规则（本变更采用，已决策）：
- `simulated=false`：省略 `x-simulated-trading` 请求头（且不得发送 `x-simulated-trading: 1`）。
- `simulated=true`：必须发送 `x-simulated-trading: 1`。

### 4.3 查询参数编码

现状：Rust 使用 `serde_urlencoded`，更符合通用 URL 编码习惯；官方 `python-okx` 以字符串拼接为主，可能在特殊字符/数组参数场景存在差异。

本变更策略（已决策）：
- 不引入全局“查询参数兼容模式”，不以复刻官方 SDK 的非编码拼接为目标；
- 若后续出现可复现失败案例，则仅以端点级最小范围提供“逃生口”（例如允许传入已编码 query 或完整 requestPath），并配套测试。

## 5. 测试与回归策略

- REST 端点常量对齐：扩展 `okx-rest` 的端点路径单测覆盖上述新增端点。
- 绑定覆盖回归：沿用 `python-binding` 规格要求的覆盖回归测试，确保新增 trait 方法在 Python 同步/异步侧均可发现且可调用（无需真实网络）。
- 行为对齐：为 `x-simulated-trading` 规则与“官方兼容返回”增加最小单测。
