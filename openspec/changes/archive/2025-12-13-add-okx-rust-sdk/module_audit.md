# 模块排查与纳入计划（9.6）

本文件用于补齐 `openspec/changes/add-okx-rust-sdk/tasks.md` 中 **9.6** 的排查结论与后续计划，便于决策追溯。

## 排查范围

任务 9.6 提到的“官方文档未覆盖模块/家族”主要包括：
- SubAccount
- Convert（闪兑）
- CopyTrading/策略
- Broker

## 当前实现现状（以仓库代码为准）

### Rust REST

`crates/okx-rest/src/api/` 已包含对应模块实现：
- `subaccount.rs`：SubAccount 家族
- `convert.rs`：Convert（闪兑）家族
- `copy_trading.rs`：CopyTrading 家族
- `broker.rs`：Broker 家族

同时还包含与“文档覆盖感知差异”常伴随出现的扩展域：
- `finance.rs`：staking-defi、savings、simple earn 等
- `block_rfq.rs`、`spread.rs`、`grid.rs`、`trading_data.rs` 等

### Python REST 绑定

`crates/okx-py/src/client/` 与 `crates/okx-py/src/async_client/` 已按业务域拆分并暴露对应方法：
- `subaccount.rs` / `async_client/subaccount.rs`
- `convert.rs` / `async_client/convert.rs`
- `copy_trading.rs` / `async_client/copy_trading.rs`
- `broker.rs` / `async_client/broker.rs`

### Python 类型存根

类型存根已随绑定导出同步更新：
- `crates/okx-py/python/okx_py/okx_py.pyi`
- `crates/okx-py/python/okx_py/py.typed`

并可通过 `just py-stubtest`（内部调用 `mypy.stubtest okx_py.okx_py`）校验一致性。

## 结论

9.6 中点名的 SubAccount / Convert / CopyTrading / Broker 家族 **均已纳入**（Rust REST + Python 绑定 + 类型存根）。

## 后续建议（如需新增/扩展）

当前无需为 9.6 额外创建新的变更提案。若后续出现以下情况，建议按 OpenSpec 流程新建独立 change-id：
- OKX 新增家族/端点，且不在现有 `crates/okx-rest/src/api/` 模块范围内
- WS 侧新增重要频道需要对齐（例如新增频道族或订阅参数变化）
- Python 绑定需要做破坏性调整（命名、参数语义、返回结构）

