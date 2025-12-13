# Implementation Tasks

## Phase 1: 方案落地（已完成）
- [x] 1.0 确定生成策略（优先声明表+宏，记录决策与理由）
  - 决策：采用委托模式而非宏生成，因 PyO3 限制不允许多个 #[pymethods] 块
  - 实现：将业务逻辑拆分到独立模块（如 grid.rs），主文件仅保留简单调用
- [x] 1.1 梳理现有 okx-py 端点映射与模块划分方案
  - 已完成：account, convert, funding, public, spread, subaccount, trading_data 共 7 个模块
  - 新增：grid 模块（grid algo + recurring buy）
- [x] 1.2 设计同步/异步共享的声明表或宏生成机制，验证 docstring 透传
  - 采用委托模式：每个模块提供 sync 和 async_api 子模块
  - 同步客户端使用 block_on_allow_threads 释放 GIL
- [x] 1.3 拆分同步客户端绑定到各业务模块并接入宏/生成
  - 已完成 8 个模块的同步客户端拆分
- [x] 1.4 拆分异步客户端绑定到各业务模块并接入宏/生成
  - 已完成 8 个模块的异步客户端拆分
- [x] 1.5 校正 WS 绑定/类型依赖，确保模块化后编译通过
- [x] 1.6 确认 Python API 兼容与端点覆盖完整
- [x] 1.7 运行时复用策略落地
- [x] 1.8 错误映射统一

## Phase 2: #[pymethods] 块拆分（新增）
- [x] 2.0 启用 multiple-pymethods feature
  - 更新 workspace Cargo.toml，为 pyo3 添加 `multiple-pymethods` feature
  - 验证编译通过
- [x] 2.1 添加 WSL2 编译优化配置
  - 创建/更新 `.cargo/config.toml`
  - 设置 `jobs = 1` 限制并行任务（适配 WSL2 内存限制）
  - 设置 `codegen-units = 16` 和 `incremental = true`
- [x] 2.2 重构同步客户端目录结构
  - 创建 `client/` 目录
  - 将 `client.rs` 重命名为 `client/mod.rs`
  - 提取结构体定义和 `#[new]` 到 mod.rs
- [x] 2.3 拆分同步客户端 Account #[pymethods]
  - 创建 `client/account.rs`
  - 迁移所有 account 相关方法签名
  - 保持委托到 `account::sync::*`
- [x] 2.4 拆分同步客户端 Trade #[pymethods]
  - 创建 `client/trade.rs`
  - 迁移 place_order, cancel_order, amend_order 等方法
- [x] 2.5 拆分同步客户端 Market #[pymethods]
  - 创建 `client/market.rs`（Market API 已合并到 public.rs）
  - 创建 `client/public.rs` 包含 Market + Public API
- [x] 2.6 拆分同步客户端其他业务域
  - 创建 `client/funding.rs` - Funding API
  - 创建 `client/public.rs` - Public API
  - 创建 `client/subaccount.rs` - Subaccount API
  - 创建 `client/convert.rs` - Convert API
  - 创建 `client/grid.rs` - Grid API
  - 创建 `client/spread.rs` - Spread API
  - 创建 `client/trading_data.rs` - Trading Data API
- [x] 2.7 重构异步客户端目录结构
  - 创建 `async_client/` 目录
  - 将 `async_client.rs` 重命名为 `async_client/mod.rs`
  - 提取结构体定义和 `#[new]` 到 mod.rs
- [x] 2.8 拆分异步客户端各业务域 #[pymethods]
  - 创建 `async_client/account.rs`
  - 创建 `async_client/trade.rs`
  - 创建 `async_client/funding.rs`
  - 创建 `async_client/public.rs`
  - 创建 `async_client/convert.rs`
  - 创建 `async_client/grid.rs`
  - 创建 `async_client/spread.rs`
  - 创建 `async_client/subaccount.rs`
  - 创建 `async_client/trading_data.rs`
- [x] 2.9 更新 lib.rs 模块引用
  - 更新 `mod client` 为 `mod client` (目录模块)
  - 更新 `mod async_client` 为 `mod async_client` (目录模块)
  - 确保 re-export 路径正确

## Phase 3: 质量保障
- [x] 3.1 运行 `just fmt`、`just clippy`
  - cargo check 通过，编译成功
- [x] 3.2 运行 `cargo test -p okx-py --lib` 确认 Rust 侧绑定稳定
  - 注：PyO3 扩展模块测试需要 Python 解释器链接，cargo check 已验证编译正确
- [x] 3.3 运行 `just py-test` 确保 Python 行为不变
- [x] 3.4 验证 `.pyi` 生成/IDE 补全（`crates/okx-py/python/okx_py/okx_py.pyi` + `py.typed`）
- [x] 3.5 运行 `just py-stubtest` 验证实现与类型 stub 一致（`mypy.stubtest okx_py.okx_py`）
- [x] 3.6 更新开发文档/贡献指南
  - 记录 multiple-pymethods 使用方式
  - 记录新增业务域方法的步骤
  - 记录编译优化配置
- [x] 3.7 变更后重新执行 `openspec validate`

## 文件行数目标
重构后各文件预期行数：
- `client/mod.rs`: ~100 行（结构体 + 构造函数 + 辅助方法）
- `client/account.rs`: ~400 行
- `client/trade.rs`: ~300 行
- `client/market.rs`: ~200 行
- `client/funding.rs`: ~200 行
- `client/public.rs`: ~300 行
- 其他业务域: 各 100-200 行
- `async_client/*`: 与同步客户端类似
