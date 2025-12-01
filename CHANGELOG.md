# Changelog · 变更日志

## 0.1.0（筹备中 / In progress）
### 中文
- 建立多 crate 工作区：`okx-core`（配置/签名/类型）、`okx-rest`（REST 客户端）、`okx-ws`（WebSocket 客户端）、`okx-py`（PyO3 绑定）。
- 覆盖官方 OKX 文档的主要 REST 接口：账户/交易/资金/行情/公共数据，内置签名、超时、代理配置。
- WebSocket 支持公共与私有频道，含心跳与自动重连（订阅恢复）。
- Python 绑定提供同步/异步 REST 客户端与 WS 客户端，附类型存根与示例。
- 新增文档：中英文 README、Rust/Python API 说明、发布指南；补充示例与发布清单。

### English
- Initialize multi-crate workspace: `okx-core` (config/signing/types), `okx-rest` (REST client), `okx-ws` (WebSocket client), `okx-py` (PyO3 bindings).
- Implement main OKX REST endpoints (account/trade/funding/market/public) with signing, timeout, proxy support.
- WebSocket client for public/private channels with heartbeat and auto-reconnect (subscription restore).
- Python bindings exposing sync/async REST and WS clients with type stubs and examples.
- Added docs: separate Chinese/English READMEs, Rust/Python API guides, release guides; added examples and release checklist.
