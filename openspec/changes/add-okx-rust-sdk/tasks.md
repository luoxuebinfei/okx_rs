# Tasks: OKX Rust SDK 实现任务

## 1. 项目基础设施

- [x] 1.1 配置 Cargo Workspace 结构
- [x] 1.2 创建 okx-core crate 骨架
- [x] 1.3 创建 okx-rest crate 骨架
- [x] 1.4 创建 okx-ws crate 骨架
- [x] 1.5 创建 okx-py crate 骨架（PyO3）
- [x] 1.6 配置 CI/CD（GitHub Actions）
- [x] 1.7 配置代码格式化（rustfmt）和静态检查（clippy）

## 2. okx-core 实现

- [x] 2.1 定义基础配置结构 (`Config`, `Credentials`)
- [x] 2.2 实现 HMAC-SHA256 签名器 (`Signer`)
- [x] 2.3 实现认证头生成 (`AuthHeaders`)
- [x] 2.4 定义错误类型 (`OkxError`)
- [x] 2.5 定义通用响应类型 (`OkxResponse<T>`)
- [x] 2.6 定义 API 端点常量模块
- [x] 2.7 定义账户相关类型 (`Balance`, `Position`, `AccountConfig`)
- [x] 2.8 定义交易相关类型 (`Order`, `Fill`, `AlgoOrder`)
- [x] 2.9 定义行情相关类型 (`Ticker`, `OrderBook`, `Candle`, `Trade`)
- [x] 2.10 定义资金相关类型 (`AssetBalance`, `DepositAddress`, `WithdrawalRecord`)
- [x] 2.11 编写 okx-core 单元测试 (31 tests)

## 3. okx-rest 实现

- [x] 3.1 实现 HTTP 客户端基类 (`OkxClient`)
- [x] 3.2 实现请求签名中间件
- [x] 3.3 实现 AccountAPI
  - [x] 3.3.1 get_balance
  - [x] 3.3.2 get_positions
  - [x] 3.3.3 set_leverage
  - [x] 3.3.4 get_account_config
  - [x] 3.3.5 get_leverage_info, get_max_size, get_max_avail_size, get_fee_rates, set_position_mode, get_account_position_risk
- [x] 3.4 实现 TradeAPI
  - [x] 3.4.1 place_order
  - [x] 3.4.2 cancel_order
  - [x] 3.4.3 amend_order
  - [x] 3.4.4 get_order
  - [x] 3.4.5 get_order_list (get_orders_pending, get_orders_history)
  - [x] 3.4.6 place_algo_order, cancel_algo_orders, get_algo_orders_pending, get_algo_orders_history
  - [x] 3.4.7 close_position
- [x] 3.5 实现 FundingAPI
  - [x] 3.5.1 get_asset_balances
  - [x] 3.5.2 funds_transfer
  - [x] 3.5.3 withdrawal
  - [x] 3.5.4 get_deposit_address, get_deposit_history, get_withdrawal_history
  - [x] 3.5.5 get_currencies
- [x] 3.6 实现 MarketDataAPI
  - [x] 3.6.1 get_tickers
  - [x] 3.6.2 get_orderbook
  - [x] 3.6.3 get_candlesticks
  - [x] 3.6.4 get_trades
  - [x] 3.6.5 get_index_tickers
- [x] 3.7 实现 PublicDataAPI
  - [x] 3.7.1 get_instruments
  - [x] 3.7.2 get_funding_rate, get_funding_rate_history
  - [x] 3.7.3 get_system_time
  - [x] 3.7.4 get_mark_price
- [x] 3.8 实现连接池和超时配置 (通过 reqwest 内置支持)
- [x] 3.9 实现代理支持 (Config.with_proxy() + OkxRestClient 自动配置)
- [x] 3.10 编写 okx-rest 集成测试 (13 tests)

## 4. okx-ws 实现

- [x] 4.1 实现 WebSocket 连接管理器
- [x] 4.2 实现消息编解码
- [x] 4.3 实现 WsPublic（公共频道）
  - [x] 4.3.1 订阅 tickers
  - [x] 4.3.2 订阅 orderbook
  - [x] 4.3.3 订阅 trades
  - [x] 4.3.4 订阅 candles
- [x] 4.4 实现 WsPrivate（私有频道）
  - [x] 4.4.1 WebSocket 认证登录
  - [x] 4.4.2 订阅 account
  - [x] 4.4.3 订阅 positions
  - [x] 4.4.4 订阅 orders
- [x] 4.5 实现自动重连机制 (ReconnectingWsClient + 指数退避)
- [x] 4.6 实现心跳维护（ping/pong）
- [x] 4.7 实现订阅状态恢复 (ReconnectingWsClient 自动恢复订阅)
- [x] 4.8 编写 okx-ws 集成测试 (17 tests)

## 5. okx-py Python 绑定

- [x] 5.1 配置 PyO3 和 maturin
- [x] 5.2 配置 pyproject.toml
- [x] 5.3 暴露核心类型到 Python
  - [x] 5.3.1 Config, Credentials
  - [x] 5.3.2 Balance, Position
  - [x] 5.3.3 Order, Fill
  - [x] 5.3.4 Ticker, OrderBook
- [x] 5.4 实现同步 OkxClient 包装
- [x] 5.5 实现异步 OkxAsyncClient（pyo3-async-runtimes）
- [x] 5.6 实现 WebSocket 客户端绑定 (WsClient)
- [x] 5.7 编写 Python 单元测试
- [x] 5.8 编写 Python 使用示例
- [x] 5.9 生成 Python 类型存根（.pyi）

## 6. 文档与发布

- [x] 6.1 编写 README.md（中英文）
- [x] 6.2 编写 API 文档（rustdoc）
- [x] 6.3 编写 Python 文档
- [x] 6.4 编写使用示例
- [x] 6.5 配置 crates.io 发布
- [x] 6.6 配置 PyPI 发布
- [x] 6.7 编写 CHANGELOG

## 7. 质量保证

- [x] 7.1 设置测试覆盖率统计
- [x] 7.2 配置性能基准测试（criterion）
- [x] 7.3 与官方 Python SDK 功能对比测试
- [x] 7.4 安全审计（依赖审计）

## 8. 对齐官方 Python SDK 缺口（待实现）
- [x] 8.1 Account：max-withdrawal、position-builder、positions-history、set-greeks、set-isolated-mode、set-account-level、borrow/repay 及历史、account bills（7d/3m）、interest/VIP accrued、simulated margin、position-tiers、greeks
- [x] 8.2 Trade：orders-history-archive、fills-history、algo 详情/修改（amend-algos）、get_algo_order_details
- [x] 8.3 Funding：non-tradable-assets、transfer-state、purchase_redempt、bills、lightning 存取、lending 设置/历史/汇总、cancel-withdrawal、convert-dust-assets、asset-valuation、saving-balance、deposit/withdraw status
- [x] 8.4 Market/Public：历史/指数/标记 K 线、历史成交、轻量深度、块交易系列、option family trades、delivery-exercise history、open-interest、public position-tiers

## Dependencies

- 任务 3.x 依赖 2.1-2.6 完成
- 任务 4.x 依赖 2.1-2.6 完成
- 任务 5.x 依赖 3.x 和 4.x 完成
- 任务 6.x 可与 3-5 并行

## Parallelizable Work

- 2.7-2.10（类型定义）可并行
- 3.3-3.7（各 API 模块）可并行
- 4.3 和 4.4（公共/私有 WebSocket）可并行
- 5.3.1-5.3.4（类型绑定）可并行

## 完成统计

- 第 1 阶段（基础设施）: **全部完成** (7/7)
- 第 2 阶段（okx-core）: **全部完成** (11/11)
  - 单元测试: 31 tests passed
- 第 3 阶段（okx-rest）: **全部完成** (10/10)
  - AccountAPI: 全部完成
  - TradeAPI: 全部完成（含 Algo Orders）
  - FundingAPI: 全部完成
  - MarketDataAPI: 全部完成
  - PublicDataAPI: 全部完成
  - 代理支持: 已完成
  - 集成测试: 13 tests passed
- 第 4 阶段（okx-ws）: **全部完成** (8/8)
  - 公共/私有频道: 已完成
  - 自动重连: 已完成 (ReconnectingWsClient)
  - 订阅状态恢复: 已完成
  - 心跳维护: 已完成
  - 集成测试: 17 tests passed
- 第 5 阶段（okx-py）: **全部完成** (9/9)
  - 同步客户端 (OkxClient): 已完成
  - 异步客户端 (AsyncOkxClient): 已完成
  - WebSocket 客户端 (WsClient): 已完成
  - Python 测试: 已完成
  - Python 示例: 已完成
  - 类型存根 (.pyi): 已完成
- 第 6-7 阶段（文档/质量）: **全部完成**

### 测试统计
- okx-core: 34 tests (3 inline + 31 integration)
- okx-rest: 13 tests
- okx-ws: 20 tests (3 inline + 17 integration)
- Doc tests: 6 tests
- **Total: 73 tests passed**
