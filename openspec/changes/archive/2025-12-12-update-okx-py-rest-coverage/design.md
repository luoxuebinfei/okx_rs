# Design: okx-py REST 绑定覆盖补齐

## 目标
1) Python 侧可调用覆盖 `okx-rest` 已实现的全部 REST trait 方法能力（同步 + 异步）。
2) 同步/异步客户端保持方法集合与参数行为一致（允许少量、显式记录的例外）。
3) 处理方法名冲突与历史别名，避免产生不可预期的“同名不同义”。

## 覆盖口径
### 基准清单
以 `crates/okx-rest/src/api/*.rs` 中所有 `pub trait *Api` 的方法名作为“目标清单”。

### 覆盖判定
默认规则：Python 方法名与 Rust trait 方法名一致，即视为覆盖。

例外规则：当存在下列情况时，允许使用“显式映射表”完成覆盖：
- **Rust 方法名冲突**（同名方法存在于多个 trait）
- **历史命名/语义更清晰**：Python 已存在更清晰的方法名，且改名会造成混淆或潜在破坏

显式映射表应由测试代码维护，形式为：
`(trait_name, rust_method_name) -> python_method_name`

## 方法名冲突策略
### 1) `get_trades`（MarketApi vs BlockRfqApi）
- 约束：不能破坏既有 `get_trades`（已被 Market 语义占用）。
- 决策：为 `BlockRfqApi.get_trades` 暴露一个不冲突的 Python 方法名：
  - Python 方法名：`get_block_rfq_trades`
- 映射表新增：
  - `(BlockRfqApi, get_trades) -> get_block_rfq_trades`

理由：
- 可读性强，避免引入短前缀导致的歧义；
- 保持现有用户对 `get_trades` 的认知不变；
- 扩展性好，未来若出现更多冲突可沿用 `<域>_<方法>` 的策略。

## 资金余额相关命名策略
当前存在潜在混淆：
- `FundingApi.get_asset_balances`（资金账户余额）
- `SubaccountApi.get_funding_balance`（子账户资金账户余额）
- 历史上异步客户端曾引入 `get_funding_balance` 作为 `get_asset_balances` 的别名

为避免“同名不同义”，本变更采用：
- `FundingApi.get_asset_balances`：
  - 同步：保持 `get_asset_balances`
  - 异步：新增 `get_asset_balances`（并保留既有 `get_funding_balance` 作为兼容别名）
- `SubaccountApi.get_funding_balance`：
  - 对外继续使用更明确的 `get_subaccount_funding_balance`
  - 映射表新增：
    - `(SubaccountApi, get_funding_balance) -> get_subaccount_funding_balance`

说明：
- 不在 Python 顶层强行引入 `get_funding_balance(sub_acct, ...)`，避免与现有别名冲突；
- 通过映射表保证“能力覆盖”与“命名清晰”同时成立；
- 未来若要彻底收敛别名，可另起变更提案进行破坏性整理（含迁移说明与版本策略）。

## 代码组织策略（延续既有模式）
每个新增业务域按既有结构实现：
- `crates/okx-py/src/<domain>.rs`：同步/异步共享实现（`sync` + `async_api` 子模块）
- `crates/okx-py/src/client/<domain>.rs`：同步 `#[pymethods]` 壳
- `crates/okx-py/src/async_client/<domain>.rs`：异步 `#[pymethods]` 壳

## 回归测试策略
新增一个“覆盖回归测试”，思路：
1) 在测试中解析 `crates/okx-rest/src/api/*.rs` 的 trait 方法清单；
2) 采集 Python 客户端实例的可用方法集合；
3) 应用映射表（含冲突/历史命名）；
4) 断言所有 trait 方法均被覆盖（差集为空）；
5) 断言同步/异步方法集合一致（允许少量显式例外）。

该测试的目标是防回退，不要求对每个方法做真实网络调用。

