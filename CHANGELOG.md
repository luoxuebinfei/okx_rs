# Changelog · 变更日志

## 0.1.0（筹备中 / In progress）
### 中文
- 建立多 crate 工作区：`okx-core`（配置/签名/类型）、`okx-rest`（REST 客户端）、`okx-ws`（WebSocket 客户端）、`okx-py`（PyO3 绑定）。
- 覆盖官方 OKX 文档的主要 REST 接口：账户/交易/资金/行情/公共数据，内置签名、超时、代理配置。
- WebSocket 支持公共与私有频道，含心跳与自动重连（订阅恢复）。
- Python 绑定提供同步/异步 REST 客户端与 WS 客户端，附类型存根与示例。
- 新增高级 API 覆盖：Broker 返佣、Spread 交易、Rubik 交易数据、Block/RFQ 批量操作与 MMP，以及对应 Python 绑定与 WS 高级频道订阅。
- 新增文档：中英文 README、Rust/Python API 说明、发布指南；补充示例与发布清单。
- **时间同步与错误增强**：
  - REST/WS 签名支持外部时间戳注入，解决客户端与服务器时钟偏移问题
  - REST 错误携带 HTTP 状态码，新增 `HttpStatus` 错误变体及辅助方法（`http_status()`、`is_rate_limited()`）
  - WS 新增 `login_with_timestamp()` 方法
  - WS 新增 `channel-conn-count` 消息解析
  - WS 重连支持时间源复用（`connect_with_timestamp_provider()`）
  - Python 绑定新增分层异常类型（`OkxError`、`OkxHttpError`、`OkxRateLimitError`、`OkxApiError` 等）
  - 新增 `TimeSync` 时间同步组件，自动维护与 OKX 服务器的时钟偏移
  - 新增 `ResponseMeta` 响应元数据，暴露 HTTP 状态码和响应头（含限速信息）
  - 新增 `get_public_with_meta()`、`get_with_meta()`、`post_with_meta()` 方法

### English
- Initialize multi-crate workspace: `okx-core` (config/signing/types), `okx-rest` (REST client), `okx-ws` (WebSocket client), `okx-py` (PyO3 bindings).
- Implement main OKX REST endpoints (account/trade/funding/market/public) with signing, timeout, proxy support.
- WebSocket client for public/private channels with heartbeat and auto-reconnect (subscription restore).
- Python bindings exposing sync/async REST and WS clients with type stubs and examples.
- Added docs: separate Chinese/English READMEs, Rust/Python API guides, release guides; added examples and release checklist.
- **Time sync and error enhancements**:
  - REST/WS signing supports external timestamp injection to handle client-server clock drift
  - REST errors now carry HTTP status codes with new `HttpStatus` error variant and helper methods (`http_status()`, `is_rate_limited()`)
  - WS adds `login_with_timestamp()` method
  - WS adds `channel-conn-count` message parsing
  - WS reconnection supports timestamp provider reuse (`connect_with_timestamp_provider()`)
  - Python bindings add hierarchical exception types (`OkxError`, `OkxHttpError`, `OkxRateLimitError`, `OkxApiError`, etc.)
  - Add `TimeSync` component for automatic clock offset synchronization with OKX server
  - Add `ResponseMeta` for exposing HTTP status codes and response headers (including rate limit info)
  - Add `get_public_with_meta()`, `get_with_meta()`, `post_with_meta()` methods
