## 项目概览
- 目标：构建高性能的 OKX 交易所 Rust SDK，并提供 PyO3 Python 绑定，覆盖 REST/WebSocket API，性能与官方 Python SDK 对齐但更快
- 结构：Cargo workspace，核心 crates：okx-core(配置/签名/错误/通用类型)、okx-rest(REST 客户端)、okx-ws(WebSocket 客户端)、okx-py(Python 绑定)
- 关键约束：严禁凭记忆编写/猜测任何 OKX API；实现前需查官方 Python SDK 与 OKX 文档并核对端点/方法/参数/响应/认证
- 技术栈：Rust 2021 + tokio、reqwest、tokio-tungstenite、serde/serde_json、thiserror、hmac/sha2/base64、chrono、tracing；PyO3+maturin 提供 Python 绑定
- 设计要点：异步优先、自动重连 WebSocket、生产/模拟环境支持、公共 API 必须有文档注释；Workspace 统一 lint/依赖
- OpenSpec：活跃变更 add-okx-rust-sdk（proposal/design/tasks 已存在），大部分任务已完成但 CI/文档等仍待办