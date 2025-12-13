# Change: 构建 OKX Rust SDK 核心库与 Python 绑定

## Why

当前 OKX 官方仅提供 Python SDK，存在以下局限：
1. **性能瓶颈**：Python 的 GIL 限制并发性能，高频交易场景下延迟较高
2. **类型安全**：动态类型导致运行时错误难以提前发现
3. **资源消耗**：Python 运行时内存占用较大
4. **跨语言受限**：无法在其他高性能语言生态中直接使用

Rust SDK 可解决以上问题，同时通过 PyO3 提供 Python 绑定，实现：
- 高性能异步 HTTP/WebSocket 通信
- 编译期类型检查与零成本抽象
- 原生 Python 模块体验，性能提升 5-10 倍
- 内存安全保证，无 GC 暂停

## What Changes

### 核心模块
- **okx-core**: 基础类型、认证、签名、错误处理
- **okx-rest-api**: REST API 客户端（Account, Trade, Funding, Market, Public 等）
- **okx-websocket**: WebSocket 客户端（公共/私有频道）
- **okx-python**: Python 绑定层（PyO3 + maturin）

### 技术栈
- 异步运行时：tokio
- HTTP 客户端：reqwest
- WebSocket：tokio-tungstenite
- 序列化：serde + serde_json
- Python 绑定：PyO3 + maturin

### API 覆盖范围（与官方 Python SDK 对齐）
- Account API（账户管理、仓位、杠杆）
- Trade API（下单、撤单、订单查询、算法订单）
- Funding API（资金划转、存取款）
- Market Data API（行情、K线、深度）
- Public Data API（公共数据、合约信息）
- WebSocket API（公共/私有实时数据流）

## Impact

- Affected specs: 新增 4 个能力规范
  - `okx-core`: 核心基础设施
  - `okx-rest-api`: REST API 客户端
  - `okx-websocket`: WebSocket 客户端
  - `okx-python-binding`: Python 绑定

- Affected code:
  - `src/` - 新增 Rust 源码
  - `Cargo.toml` - 依赖配置
  - `pyproject.toml` - Python 包配置（后续添加）

- 无破坏性变更（新项目）
