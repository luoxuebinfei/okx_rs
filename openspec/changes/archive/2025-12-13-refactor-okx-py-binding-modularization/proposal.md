# Change: okx-py 绑定模块化与去重复（Phase 2）

## Why
当前 okx-py 虽已完成业务逻辑拆分（account、funding 等模块），但主文件仍然过大（`client.rs` 约 2.9k 行、`async_client.rs` 约 3.5k 行）。所有 `#[pymethods]` 方法签名仍集中在单文件，评审、维护成本高。

## What Changes
- 启用 PyO3 `multiple-pymethods` feature，允许同一 `#[pyclass]` 拥有多个 `#[pymethods]` 块。
- 按业务域拆分 `#[pymethods]` 块到独立文件（如 `client/trade.rs`、`client/market.rs`），主文件仅保留结构体定义与构造函数。
- 业务逻辑继续委托到现有域模块（`account.rs`、`funding.rs` 等），新拆分的 `#[pymethods]` 文件仅负责方法签名与委托调用。
- 保持 Python API 兼容，同时完善测试入口（Rust + Python）覆盖拆分后的结构。
- 添加 WSL2 编译优化配置，限制并行任务数。

## Impact
- Affected specs: `python-binding`
- Affected code: `crates/okx-py/src/{client.rs,async_client.rs}` 拆分为目录结构、`Cargo.toml` 新增 feature。
- Dependencies: 新增 `inventory` crate（`multiple-pymethods` 依赖）。
- Build: `.cargo/config.toml` 添加编译优化配置。
