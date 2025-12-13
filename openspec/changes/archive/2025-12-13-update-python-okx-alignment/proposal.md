# 变更提案：对齐官方文档与官方 Python SDK（python-okx）

## Why

当前 Rust SDK（`okx-core` / `okx-rest` / `okx-ws`）与 Python 绑定（`okx-py`）在鉴权主流程上已符合官方规范，但与官方参考实现 `python-okx` 仍存在可验证的差异：

1. **REST 端点覆盖缺口**：相对官方 `python-okx` 仍缺少若干 v5 端点（集中在 Account/Finance/Market/Public/Trade）。
2. **Python 侧“官方兼容”体验不足**：现有绑定更偏向 Rust 类型化返回（仅返回 `data`），与官方 `python-okx` “返回完整 JSON（含 code/msg/data）”的使用习惯不一致。
3. **模拟盘请求头与对齐策略需要明确**：官方文档要求 Demo Trading 请求必须携带 `x-simulated-trading: 1`，官方 `python-okx` 也始终携带该头（值为 flag）。本项目当前行为与官方并不完全一致，需在安全前提下明确定义对齐策略。

本变更的目标是：**在不牺牲安全与可维护性的前提下**，把“端点覆盖 + 绑定能力 + 行为对齐策略”沉淀为可执行的规范与任务清单，作为后续实现与回归测试的唯一来源。

## What Changes

### 1) 补齐相对 `python-okx` 的缺失 REST 端点（v5）

以官方 `python-okx`（okxapi/python-okx）为基准，补齐以下端点族（完整清单见 `design.md`）：

- Account：`/api/v5/account/instruments`、`/api/v5/account/risk-state`
- Finance：
  - Flexible Loan：`/api/v5/finance/flexible-loan/*`（8 个）
  - Staking-Defi（ETH/SOL）：`/api/v5/finance/staking-defi/{eth,sol}/*`（12 个）
  - Savings：`/api/v5/finance/savings/lending-rate-history`
- Market：`/api/v5/market/exchange-rate`、`/api/v5/market/index-components`、`/api/v5/market/platform-24-volume`
- Public：`/api/v5/public/instrument-tick-bands`、`/api/v5/public/option-trades`
- Trade：`/api/v5/trade/one-click-repay-*-v2`（3 个）

### 2) Python 绑定同步补齐（覆盖新增能力）

在 `okx_py.OkxClient` 与 `okx_py.AsyncOkxClient` 中暴露新增端点对应方法，并保证同步/异步表面契约一致。

### 3) 行为对齐策略（最小破坏 + 可选官方兼容）

为避免破坏现有用户代码，本变更采用“**兼容增强**”策略：

- 保持现有“类型化返回（只返回 data）”能力不变；
- **新增**“官方兼容返回”能力：提供 `*_raw` 方法或 `raw=True` 参数，返回与官方 `python-okx` 一致的完整响应结构（含 `code/msg/data` 等）。
- 明确 `x-simulated-trading` 的发送规则与默认值：默认以安全为先（默认不进入模拟盘），并在需要时严格满足官方文档要求。

## Non-Goals

- 不对现有已实现端点进行大规模重构或重命名（除非为对齐官方行为且能证明必要）。
- 不承诺一次性覆盖所有官方文档端点；本变更以“对齐官方 `python-okx`”为验收边界。

## Impact

- **对外 API**：新增方法/参数（向后兼容）；若选择调整默认模拟盘行为，将明确列为破坏性变更并要求显式评审。
- **测试**：需要补充端点常量对齐测试与绑定覆盖回归测试，确保后续文档/SDK更新时可快速发现差异。

## Success Criteria

- Rust REST：缺失端点全部补齐（路径、方法、参数、响应结构经官方文档/官方 SDK 双重验证），并有测试覆盖。
- Python 绑定：新增端点在同步与异步客户端均可调用，且覆盖测试通过。
- 行为对齐：官方兼容返回能力可用；`x-simulated-trading` 规则写入规范并有测试验证。
