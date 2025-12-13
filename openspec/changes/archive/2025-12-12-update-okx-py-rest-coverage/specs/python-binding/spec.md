# python-binding Specification (delta)

## Purpose
补齐 okx-py 的 Python REST 绑定覆盖缺口，并通过自动化回归测试避免后续回退。

## ADDED Requirements

### Requirement: Python REST 绑定覆盖完整（以 okx-rest trait 为基准）
系统 SHALL 使 `okx_py.OkxClient` 与 `okx_py.AsyncOkxClient` 可调用覆盖 `okx-rest` 中全部 `pub trait *Api` 方法的能力。

默认情况下，Python 方法名 SHALL 与 Rust trait 方法名一致。

如遇方法名冲突或历史命名需要保留，系统 SHALL 使用显式“方法映射表”声明覆盖关系，并纳入测试与类型存根/文档。

#### Scenario: 覆盖回归测试通过
- **WHEN** 运行 Python 测试中的“覆盖回归测试”
- **THEN** 从 `crates/okx-rest/src/api/*.rs` 解析得到的 trait 方法清单均被判定为已覆盖
- **AND** 若存在差集，测试输出差集方法名用于定位

---

### Requirement: Block RFQ / Finance / Copy Trading / Broker 域绑定补齐
系统 SHALL 为下列业务域提供同步与异步 Python 绑定：
- `BlockRfqApi`
- `FinanceApi`
- `CopyTradingApi`
- `BrokerApi`

#### Scenario: 新增业务域方法可发现
- **WHEN** 用户创建 `okx_py.OkxClient(...)` 与 `okx_py.AsyncOkxClient(...)`
- **THEN** 可通过 `hasattr` 发现上述业务域对应方法
- **AND** 方法可调用（不要求真实网络连接）

---

### Requirement: 方法名冲突处理（get_trades）
当 Rust 侧出现 trait 方法名冲突时，系统 SHALL 通过不冲突的 Python 方法名暴露所有能力，并在映射表中声明。

#### Scenario: BlockRfqApi.get_trades 可调用且不破坏 MarketApi.get_trades
- **WHEN** 用户调用市场域的 `get_trades`
- **THEN** 行为保持既有市场成交语义
- **AND** 用户可通过 `get_block_rfq_trades` 调用 Block RFQ 的成交查询能力

---

### Requirement: 同步/异步表面契约一致
系统 SHALL 保证 `OkxClient` 与 `AsyncOkxClient` 的方法集合保持一致（按映射表口径判定），并避免“同名不同义”。

#### Scenario: 同步/异步方法集合一致性验证
- **WHEN** 运行表面契约一致性测试
- **THEN** 同步/异步客户端的覆盖结果一致
- **AND** 任何例外均必须在映射表中显式声明

