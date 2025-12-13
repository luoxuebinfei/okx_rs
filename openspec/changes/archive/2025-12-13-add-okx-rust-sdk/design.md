# Design: OKX Rust SDK 架构设计

## Context

OKX 是全球领先的加密货币交易所，提供现货、合约、期权等交易服务。官方 Python SDK (`python-okx`) 提供了完整的 API 覆盖，但在性能敏感场景下存在瓶颈。

本设计旨在构建一个高性能、类型安全的 Rust SDK，同时通过 PyO3 提供 Python 绑定，让 Python 用户也能享受 Rust 的性能优势。

### 约束条件
- 必须兼容 OKX API v5
- Python 绑定需支持 Python 3.9+
- 需支持生产环境和模拟交易环境
- 必须支持自动重连的 WebSocket

### 严格禁止猜测 API（最高优先级）

**所有 OKX API 实现必须经过验证，严禁凭借记忆猜测！**

验证方式：
1. 查询官方 Python SDK: https://github.com/okxapi/python-okx
2. 查询官方 API 文档: https://www.okx.com/docs-v5/en/
3. 使用 DeepWiki MCP 工具查询 SDK 细节
4. 使用 WebFetch/Exa 获取最新文档
5. 使用 Context7 MCP 查询 Rust 依赖库文档（tokio, reqwest, PyO3 等）

每个 API 实现前必须确认：
- 端点路径正确
- 请求方法正确 (GET/POST)
- 请求参数完整且类型正确
- 响应结构与官方一致

## Goals / Non-Goals

### Goals
- 实现与官方 Python SDK 功能对等的 Rust SDK
- 提供零成本抽象的 Python 绑定
- 支持异步 I/O，最大化并发性能
- 提供完善的错误处理和类型定义
- 支持 WebSocket 自动重连和心跳维护

### Non-Goals
- 不实现交易策略逻辑
- 不提供图形界面
- 不实现本地撮合引擎
- 暂不支持 Rust 2024 Edition（等待生态稳定）

## Decisions

### 1. Crate 组织结构

**决策**: 采用 Workspace 多 Crate 结构

```
okx_rs/
├── Cargo.toml              # Workspace root
├── crates/
│   ├── okx-core/           # 核心类型、认证、错误
│   ├── okx-rest/           # REST API 客户端
│   ├── okx-ws/             # WebSocket 客户端
│   └── okx-py/             # Python 绑定
└── src/
    └── lib.rs              # 统一导出（可选）
```

**理由**:
- 关注点分离，便于独立测试和维护
- 用户可按需引入，减少依赖体积
- Python 绑定独立 crate，避免 PyO3 影响纯 Rust 用户

**备选方案**: 单 Crate 结构
- 优点：简单，无循环依赖问题
- 缺点：耦合度高，编译慢

### 2. 异步运行时选择

**决策**: 使用 tokio 作为唯一异步运行时

**理由**:
- tokio 生态最完善，reqwest/tokio-tungstenite 原生支持
- 性能优异，支持多核并行
- 社区活跃，文档完善

**备选方案**: async-std 或运行时无关
- 优点：更灵活
- 缺点：增加复杂度，生态支持弱

### 3. HTTP 客户端选择

**决策**: 使用 reqwest

**理由**:
- 功能完善（连接池、代理、TLS）
- API 简洁，易于使用
- 与 tokio 原生集成

**备选方案**: hyper + tower
- 优点：更底层，更灵活
- 缺点：需要更多样板代码

### 4. WebSocket 实现

**决策**: 使用 tokio-tungstenite

**理由**:
- 与 tokio 生态完美集成
- 支持自动 ping/pong
- 社区广泛使用

### 5. Python 绑定方案

**决策**: PyO3 + maturin

**理由**:
- PyO3 是最成熟的 Rust-Python 绑定库
- maturin 简化构建和发布流程
- 支持异步（pyo3-asyncio）
- 可直接 `pip install`

**备选方案**: uniffi
- 优点：多语言绑定
- 缺点：Python 支持相对较弱

### 6. 错误处理策略

**决策**: 使用 thiserror 定义错误类型，对外暴露 Result

```rust
#[derive(Debug, thiserror::Error)]
pub enum OkxError {
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("WebSocket error: {0}")]
    WebSocket(#[from] tokio_tungstenite::tungstenite::Error),
    #[error("API error: code={code}, msg={msg}")]
    Api { code: String, msg: String },
    #[error("Authentication error: {0}")]
    Auth(String),
    #[error("Serialization error: {0}")]
    Serde(#[from] serde_json::Error),
}
```

### 7. API 响应处理

**决策**: 使用泛型 Response 包装器

```rust
#[derive(Debug, Deserialize)]
pub struct OkxResponse<T> {
    pub code: String,
    pub msg: String,
    pub data: Vec<T>,
}
```

**理由**: OKX API 统一返回格式，泛型可复用

### 8. 认证签名实现

**决策**: 遵循 OKX 官方签名规范

```
签名流程:
1. 构造预签名字符串: timestamp + method + requestPath + body
2. 使用 HMAC-SHA256 签名
3. Base64 编码
```

## Architecture Diagram

```
┌─────────────────────────────────────────────────────────────────┐
│                        Python Application                        │
└─────────────────────────────────────────────────────────────────┘
                                  │
                                  ▼
┌─────────────────────────────────────────────────────────────────┐
│                      okx-py (PyO3 Binding)                       │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────────┐  │
│  │ OkxClient   │  │ WsClient    │  │ Types (Account, Order)  │  │
│  │ (sync/async)│  │ (async)     │  │                         │  │
│  └─────────────┘  └─────────────┘  └─────────────────────────┘  │
└─────────────────────────────────────────────────────────────────┘
                                  │
        ┌─────────────────────────┼─────────────────────────┐
        ▼                         ▼                         ▼
┌───────────────┐        ┌───────────────┐        ┌───────────────┐
│   okx-rest    │        │    okx-ws     │        │   okx-core    │
│               │        │               │        │               │
│ - AccountAPI  │        │ - WsPublic    │        │ - Auth        │
│ - TradeAPI    │        │ - WsPrivate   │        │ - Signer      │
│ - FundingAPI  │        │ - Reconnect   │        │ - Error       │
│ - MarketAPI   │        │ - Heartbeat   │        │ - Types       │
│ - PublicAPI   │        │               │        │ - Config      │
└───────────────┘        └───────────────┘        └───────────────┘
        │                         │                         │
        └─────────────────────────┼─────────────────────────┘
                                  ▼
┌─────────────────────────────────────────────────────────────────┐
│                      External Dependencies                       │
│  ┌─────────┐  ┌───────────────────┐  ┌────────┐  ┌───────────┐  │
│  │ reqwest │  │ tokio-tungstenite │  │ serde  │  │   tokio   │  │
│  └─────────┘  └───────────────────┘  └────────┘  └───────────┘  │
└─────────────────────────────────────────────────────────────────┘
                                  │
                                  ▼
┌─────────────────────────────────────────────────────────────────┐
│                         OKX Exchange                             │
│  ┌──────────────────────┐    ┌──────────────────────────────┐   │
│  │ REST API             │    │ WebSocket API                │   │
│  │ https://www.okx.com  │    │ wss://ws.okx.com:8443        │   │
│  └──────────────────────┘    └──────────────────────────────┘   │
└─────────────────────────────────────────────────────────────────┘
```

## Risks / Trade-offs

### Risk 1: PyO3 异步支持复杂度
- **风险**: pyo3-asyncio 桥接 tokio 和 Python asyncio 可能引入复杂性
- **缓解**: 同时提供同步 API，异步作为可选功能

### Risk 2: OKX API 变更
- **风险**: OKX 可能更新 API，导致 SDK 不兼容
- **缓解**: 版本化 API 常量，建立自动化测试

### Risk 3: WebSocket 重连机制
- **风险**: 网络不稳定导致连接中断
- **缓解**: 实现指数退避重连，订阅状态恢复

### Trade-off: 性能 vs 易用性
- 选择泛型会增加编译时间
- 选择 Box<dyn Trait> 会损失性能
- **决策**: 优先使用泛型，关键路径避免动态分发

## Migration Plan

本项目为新建项目，无迁移需求。

### 发布计划
1. v0.1.0: okx-core + okx-rest 基础功能
2. v0.2.0: okx-ws WebSocket 支持
3. v0.3.0: okx-py Python 绑定
4. v1.0.0: 功能完整，API 稳定

## Open Questions

1. **是否支持 Rust 2024 Edition？**
   - 当前选择 2021 Edition，待生态稳定后升级

2. **是否提供同步 API？**
   - 建议提供，便于简单场景使用

3. **Python 绑定是否暴露所有类型？**
   - 建议仅暴露常用类型，减少维护负担

4. **是否支持代理配置？**
   - 建议支持，通过环境变量或配置项
