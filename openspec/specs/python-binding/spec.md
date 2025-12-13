# python-binding Specification

## Purpose
TBD - created by archiving change update-okx-py-rest-coverage. Update Purpose after archive.
## Requirements
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

### Requirement: 启用 multiple-pymethods feature
系统 SHALL 启用 PyO3 的 `multiple-pymethods` feature，允许同一 `#[pyclass]` 拥有多个 `#[pymethods]` 块。

#### Scenario: Feature 启用验证
- **WHEN** 查看 `Cargo.toml` 配置
- **THEN** pyo3 依赖包含 `multiple-pymethods` feature

### Requirement: #[pymethods] 块按业务域拆分
系统 SHALL 将 `PyOkxClient` 和 `PyAsyncOkxClient` 的 `#[pymethods]` 块按业务域拆分到独立文件。

#### Scenario: 同步客户端目录结构
- **WHEN** 查看 `crates/okx-py/src/client/` 目录
- **THEN** 存在 `mod.rs`、`account.rs`、`trade.rs`、`market.rs` 等业务域文件
- **AND** 每个业务域文件包含一个 `#[pymethods]` 块
- **AND** 单个文件行数不超过 500 行

#### Scenario: 异步客户端目录结构
- **WHEN** 查看 `crates/okx-py/src/async_client/` 目录
- **THEN** 存在与同步客户端对应的业务域文件结构

### Requirement: 主模块文件精简
系统 SHALL 将主模块文件（`client/mod.rs`、`async_client/mod.rs`）精简为仅包含结构体定义、构造函数和子模块声明。

#### Scenario: 主模块内容验证
- **WHEN** 查看 `client/mod.rs` 源码
- **THEN** 文件仅包含 `PyOkxClient` 结构体定义、`#[new]` 构造函数和 `mod` 声明
- **AND** 文件行数不超过 150 行

### Requirement: Python 绑定按业务域模块化（保留）
系统 SHALL 将 PyO3 REST 绑定按业务域拆分（如 account/funding/market/public/trade/subaccount/convert/grid 等），主文件仅保留结构体定义、运行时构造与模块聚合。

#### Scenario: 域模块拆分
- **WHEN** 查看 `OkxClient` / `AsyncOkxClient` 源码
- **THEN** REST 端点分布在对应域模块，主文件不再承载端点实现，任一单个域模块文件行数不超过 500 行。

### Requirement: 绑定去重复且同步/异步一致（保留）
系统 SHALL 使用委托模式为同步与异步客户端实现 PyO3 包装，保证 Python 方法名、参数序列和返回处理与 Rust 客户端保持一致。

#### Scenario: 委托模式实现
- **WHEN** 新增或修改任一 REST 端点绑定
- **THEN** 仅需更新对应业务域的 `#[pymethods]` 文件和业务逻辑模块，同步与异步绑定保持等效方法签名。

### Requirement: 绑定兼容性与测试保障（保留）
系统 SHALL 在模块化后维持现有 Python API 兼容，并通过 Rust 与 Python 测试回归。

#### Scenario: 功能与测试通过
- **WHEN** 运行 `cargo test -p okx-py --lib` 与 `just py-test`
- **THEN** 测试全部通过，Python 调用行为与变更前保持一致（端点覆盖、参数/返回格式不变）。

### Requirement: WS 绑定兼容与依赖校正（保留）
系统 SHALL 在模块化过程中保持 `WsClient` 绑定接口与行为不变。

#### Scenario: WS 兼容验证
- **WHEN** 完成模块拆分并构建 okx-py
- **THEN** `WsClient` 相关编译/测试通过，公开 API 与事件处理行为与变更前一致。

### Requirement: WSL2 编译优化配置
系统 SHALL 提供编译优化配置，适配 WSL2 资源受限环境。

#### Scenario: 编译配置验证
- **WHEN** 查看 `.cargo/config.toml`
- **THEN** 包含 `jobs = 2` 限制并行任务数
- **AND** 包含 `codegen-units` 和 `incremental` 优化配置

