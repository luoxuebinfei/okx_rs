# Implementation Tasks

## Phase 0: 变更对齐与验证口径
- [x] 0.1 固化缺口清单（以 `okx-rest` trait 方法为基准）
  - 记录当前统计口径与结果（231 总方法、49 合并缺口、`get_trades` 冲突）
  - 明确“命名映射表”包含的例外项（见 design.md）
- [x] 0.2 确认实现只做“薄包装”
  - Python 侧仅委托调用 `okx-rest` 的对应 trait 方法
  - 不新增/修改任何 OKX 端点与参数语义

## Phase 1: 新增业务域绑定（同步 + 异步）
- [x] 1.1 Block RFQ（BlockRfqApi）
  - 新增 `crates/okx-py/src/block_rfq.rs`（sync + async_api）
  - 新增 `crates/okx-py/src/client/block_rfq.rs`
  - 新增 `crates/okx-py/src/async_client/block_rfq.rs`
  - 处理 `get_trades` 冲突：以 `get_block_rfq_trades` 暴露
- [x] 1.2 Finance（FinanceApi）
  - 新增 `crates/okx-py/src/finance.rs`（sync + async_api）
  - 新增 `crates/okx-py/src/client/finance.rs`
  - 新增 `crates/okx-py/src/async_client/finance.rs`
- [x] 1.3 Copy Trading（CopyTradingApi）
  - 新增 `crates/okx-py/src/copy_trading.rs`（sync + async_api）
  - 新增 `crates/okx-py/src/client/copy_trading.rs`
  - 新增 `crates/okx-py/src/async_client/copy_trading.rs`
- [x] 1.4 Broker（BrokerApi）
  - 新增 `crates/okx-py/src/broker.rs`（sync + async_api）
  - 新增 `crates/okx-py/src/client/broker.rs`
  - 新增 `crates/okx-py/src/async_client/broker.rs`

## Phase 2: 同步/异步一致性与映射收敛
- [x] 2.1 异步补齐 `get_asset_balances`
  - 在 `AsyncOkxClient` 增加 `get_asset_balances`（调用现有实现）
  - 保留既有 `get_funding_balance` 作为兼容别名（不移除）
- [x] 2.2 子账户资金余额映射确认
  - 保持 `get_subaccount_funding_balance` 作为对外方法
  - 在覆盖测试映射表中声明 `(SubaccountApi, get_funding_balance) -> get_subaccount_funding_balance`

## Phase 3: 模块聚合与类型存根
- [x] 3.1 更新 `crates/okx-py/src/lib.rs`
  - `mod block_rfq; mod finance; mod copy_trading; mod broker;`
- [x] 3.2 更新 `crates/okx-py/src/client/mod.rs` 与 `crates/okx-py/src/async_client/mod.rs`
  - 增加新业务域子模块声明
- [x] 3.3 更新 `.pyi` 类型存根
  - `crates/okx-py/python/okx_py/okx_py.pyi` 补齐新增方法（必要时返回 `Any`）
  - 若存在命名映射（如 `get_block_rfq_trades`），在存根中明确

## Phase 4: 测试与防回退
- [x] 4.1 新增“覆盖回归测试”
  - 解析 `crates/okx-rest/src/api/*.rs` trait 方法
  - 采集 `OkxClient` / `AsyncOkxClient` 方法集合
  - 应用映射表（含 `get_trades` 冲突与子账户余额命名）
  - 断言缺口为 0，并输出差集便于定位
- [x] 4.2 增强表面契约测试
  - 为新增业务域补充至少 1 个“hasattr/可调用”回归用例（不做真实网络请求）

## Phase 5: 本地检查入口（与 CI 对齐）
- [x] 5.1 运行 `just fmt`、`just check`
- [x] 5.2 运行 `cargo test -p okx-py --lib`
- [x] 5.3 运行 `just py-test`
- [x] 5.4 运行 `openspec validate update-okx-py-rest-coverage --strict`
