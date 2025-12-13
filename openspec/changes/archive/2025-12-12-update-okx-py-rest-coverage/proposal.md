# Change: 补齐 okx-py Python REST 绑定覆盖缺口

## Why
当前 `okx_py.OkxClient` / `okx_py.AsyncOkxClient` 未完整暴露 `okx-rest` 已实现的全部 REST API 方法，导致部分业务域（尤其是金融产品、跟单、大宗 RFQ、经纪商）在 Python 侧不可用，且同步/异步表面契约存在不一致风险。

基于代码扫描统计（以 `crates/okx-rest/src/api/*.rs` 中 `pub trait *Api` 的方法为“目标清单”）：
- Rust REST trait 方法总数：231（去重后）
- 同步客户端缺口：48
- 异步客户端缺口：48
- 合并缺口（去重）：49（其中 2 个属于“命名/别名不一致”而非能力缺失）
- 另有 1 处 Rust 方法名冲突：`get_trades` 同时存在于 `MarketApi` 与 `BlockRfqApi`

## What Changes
- 为 `BlockRfqApi` / `FinanceApi` / `CopyTradingApi` / `BrokerApi` 增加 Python 绑定（同步 + 异步），补齐缺失方法。
- 处理 Rust 侧方法名冲突 `get_trades`：保持现有 `MarketApi.get_trades` 不变，为 `BlockRfqApi.get_trades` 提供一个不冲突的 Python 方法名（详见 design.md 的命名策略）。
- 统一同步/异步对 `FundingApi.get_asset_balances` 的可用性：在异步客户端补齐 `get_asset_balances`（保留现有 `get_funding_balance` 作为兼容别名）。
- 为 `SubaccountApi.get_funding_balance` 建立明确映射：继续以现有 `get_subaccount_funding_balance` 作为对外方法，并在覆盖测试中声明该映射关系，避免与资金账户“余额”语义混淆。
- 新增一套“覆盖回归测试”：自动解析 Rust trait 方法清单，并对照 Python 客户端方法表 + 显式映射表，确保后续不再回退。
- 更新 `.pyi` 类型存根与文档，确保 IDE 补全与用户可发现性。

## Impact
- 影响 specs：
  - `python-binding`（新增“REST 覆盖完整性/命名冲突/同步异步一致性”的要求）
- 影响代码：
  - `crates/okx-py/src/`：新增若干业务域模块（block_rfq/finance/copy_trading/broker）及对应 `client/`、`async_client/` 绑定文件
  - `crates/okx-py/python/okx_py/okx_py.pyi`：补齐新增方法签名（必要时使用 `Any`）
  - `crates/okx-py/tests/`：新增/增强覆盖测试与回归测试

## Breaking Changes
本变更目标为“只增不改”，尽量避免破坏性调整。

唯一需要明确的行为是：`SubaccountApi.get_funding_balance` 在 Python 侧继续使用 `get_subaccount_funding_balance` 命名（通过映射表保证覆盖），不会强行引入同名 `get_funding_balance` 以避免与资金账户余额的既有别名冲突/混淆。

## Acceptance Criteria
- `okx_py.OkxClient` 与 `okx_py.AsyncOkxClient` 均可调用：
  - `BlockRfqApi` 全部方法（含 `get_trades` 的冲突解决方案）
  - `FinanceApi` 全部 18 个方法
  - `CopyTradingApi` 全部 9 个方法
  - `BrokerApi` 全部 2 个方法
- 新增的“覆盖回归测试”通过，且对缺口数量为 0（以映射规则为准）。
- `just fmt`、`just check`、`cargo test -p okx-py --lib`、`just py-test` 通过。

## 附录 A：缺口清单（以 trait 方法名为基准）
说明：
- 标注为 `sync/async` 表示两端都缺；
- `FundingApi.get_asset_balances` 为异步侧“命名缺口”（已有能力但缺少同名方法）；
- `SubaccountApi.get_funding_balance` 为同步侧“命名缺口”（已有能力但使用更明确的 `get_subaccount_funding_balance`）；
- `BlockRfqApi.get_trades` 与 `MarketApi.get_trades` 存在 Rust 侧同名冲突，需通过别名暴露（不计入下表 18 项缺口中）。

### BlockRfqApi（18）
- `cancel_all_quotes`
- `cancel_all_rfqs`
- `cancel_batch_quotes`
- `cancel_batch_rfqs`
- `cancel_quote`
- `cancel_rfq`
- `create_quote`
- `create_rfq`
- `execute_quote`
- `get_counterparties`
- `get_mmp_config`
- `get_public_trades`
- `get_quote_products`
- `get_quotes`
- `get_rfqs`
- `reset_mmp`
- `set_marker_instrument`
- `set_mmp_config`

### FinanceApi（18）
- `defi_cancel`
- `defi_get_offers`
- `defi_orders_active`
- `defi_orders_history`
- `defi_purchase`
- `defi_redeem`
- `saving_balance`
- `saving_lending_history`
- `saving_public_lending_rate`
- `saving_purchase_redemption`
- `saving_set_lending_rate`
- `simple_earn_amend_lending_order`
- `simple_earn_get_lending_apy_history`
- `simple_earn_get_lending_offers`
- `simple_earn_get_lending_orders_list`
- `simple_earn_get_lending_sub_orders`
- `simple_earn_get_pending_lending_volume`
- `simple_earn_place_lending_order`

### CopyTradingApi（9）
- `amend_leading_instruments`
- `close_lead_position`
- `get_existing_lead_positions`
- `get_lead_position_history`
- `get_leading_instruments`
- `get_profit_sharing_details`
- `get_total_profit_sharing`
- `get_unrealized_profit_sharing_details`
- `place_lead_stop_order`

### BrokerApi（2）
- `fd_get_rebate_per_orders`
- `fd_rebate_per_orders`

### Funding/子账户命名缺口（2）
- `FundingApi.get_asset_balances`（async 侧缺同名方法）
- `SubaccountApi.get_funding_balance`（sync 侧缺同名方法；现有 `get_subaccount_funding_balance` 覆盖能力）
