## Context
okx-py 绑定已完成第一阶段模块化（业务逻辑拆分到 account.rs、funding.rs 等），但主文件（client.rs、async_client.rs）仍承载所有 `#[pymethods]` 方法签名，单文件行数过大。需求是进一步拆分 `#[pymethods]` 块，使每个业务域的方法签名也独立成文件。

## Goals / Non-Goals
- Goals:
  - 启用 `multiple-pymethods` feature，将 `#[pymethods]` 块按业务域拆分到独立文件。
  - 主文件（client.rs、async_client.rs）仅保留结构体定义、构造函数和模块引用。
  - 单个 `#[pymethods]` 文件行数控制在 500 行以内。
  - 保持 Python API 兼容并确保测试通过。
  - 添加 WSL2 编译优化配置。
- Non-Goals:
  - 不新增/删除 API 端点。
  - 不改变 ws 协议或业务语义。
  - 不引入宏生成方案（保持代码直观可调试）。

## Decisions
- Decision: 启用 PyO3 `multiple-pymethods` feature，采用 Polars 项目验证的模块化模式。
- Decision: 将 `client.rs` 重构为 `client/mod.rs` + `client/{account,trade,market,funding,...}.rs` 目录结构。
- Decision: 将 `async_client.rs` 重构为 `async_client/mod.rs` + `async_client/{account,trade,market,funding,...}.rs` 目录结构。
- Decision: 每个业务域文件包含一个 `#[pymethods]` 块，方法实现委托到对应的域模块（如 `account::sync::get_balance`）。
- Decision: 主模块文件（mod.rs）仅包含结构体定义、`#[new]` 构造函数和子模块声明。
- Decision: 添加 `.cargo/config.toml` 编译优化配置，限制 WSL2 环境下的并行任务数。

## Architecture

### 目录结构（重构后）
```
crates/okx-py/src/
├── lib.rs                    # 模块注册
├── client/                   # 同步客户端
│   ├── mod.rs               # PyOkxClient 结构体定义 + #[new]
│   ├── account.rs           # #[pymethods] Account API
│   ├── trade.rs             # #[pymethods] Trade API
│   ├── market.rs            # #[pymethods] Market API
│   ├── funding.rs           # #[pymethods] Funding API
│   ├── public.rs            # #[pymethods] Public API
│   └── ...                  # 其他业务域
├── async_client/            # 异步客户端（同上结构）
│   ├── mod.rs
│   ├── account.rs
│   └── ...
├── account.rs               # 业务逻辑（已有，保持不变）
├── funding.rs               # 业务逻辑（已有，保持不变）
├── ...                      # 其他业务逻辑模块
├── types.rs                 # 类型定义
└── ws_client.rs             # WebSocket 客户端
```

### 代码示例

**client/mod.rs（主文件）：**
```rust
use pyo3::prelude::*;
use tokio::runtime::Runtime;
use okx_rest::OkxRestClient;

mod account;
mod trade;
mod market;
mod funding;
// ... 其他模块

#[pyclass(name = "OkxClient")]
pub struct PyOkxClient {
    pub(crate) client: OkxRestClient,
    pub(crate) runtime: Runtime,
}

#[pymethods]
impl PyOkxClient {
    #[new]
    fn new(config: PyConfig) -> PyResult<Self> {
        let runtime = Runtime::new()
            .map_err(|e| PyRuntimeError::new_err(format!("Failed to create runtime: {}", e)))?;
        let client = OkxRestClient::new(config.inner);
        Ok(Self { client, runtime })
    }
}
```

**client/account.rs（业务域方法）：**
```rust
use pyo3::prelude::*;
use crate::client::PyOkxClient;
use crate::account as account_impl;
use crate::types::*;

#[pymethods]
impl PyOkxClient {
    #[pyo3(signature = (ccy=None))]
    fn get_balance(&self, ccy: Option<&str>) -> PyResult<Vec<PyBalance>> {
        account_impl::sync::get_balance(self, ccy)
    }

    #[pyo3(signature = (inst_type=None, inst_id=None))]
    fn get_positions(&self, inst_type: Option<&str>, inst_id: Option<&str>) -> PyResult<Vec<PyPosition>> {
        account_impl::sync::get_positions(self, inst_type, inst_id)
    }
    // ... 其他 account 方法
}
```

## Risks / Trade-offs
- 风险：`multiple-pymethods` 依赖 `inventory` crate，不支持 Wasm 平台 → 本项目无 Wasm 需求，可接受。
- 风险：编译时间略有增加 → 通过 `.cargo/config.toml` 优化配置缓解。
- 风险：拆分后可能出现循环依赖 → 保持单向依赖：`client/*.rs` → `account.rs` 等业务模块。
- 风险：IDE 对多 `#[pymethods]` 块支持可能不完善 → rust-analyzer 已良好支持。

## Migration Plan
1. 更新 `Cargo.toml`，启用 `multiple-pymethods` feature。
2. 添加 `.cargo/config.toml` 编译优化配置。
3. 创建 `client/` 目录结构，迁移 `PyOkxClient` 结构体定义。
4. 按业务域拆分 `#[pymethods]` 块到独立文件。
5. 对 `async_client` 执行相同重构。
6. 运行测试验证 Python API 兼容性。
7. 更新文档与贡献指南。

## Open Questions
- （暂无）
