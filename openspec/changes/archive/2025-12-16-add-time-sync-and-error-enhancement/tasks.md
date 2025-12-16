# Implementation Tasks

## 1. M1-1: REST 签名支持外部时间戳注入

- [x] 1.1 在 `okx-core/src/signer.rs` 添加 `generate_headers_with_timestamp(timestamp_iso, method, request_path, body, simulated)` 方法
- [x] 1.2 保留原 `generate_headers()` 方法不变，内部调用新方法
- [ ] 1.3 在 `okx-rest/src/client.rs` 添加可选的外部时间戳参数支持（延后，调用方可直接使用 Signer）
- [x] 1.4 编写单元测试验证时间戳注入功能

## 2. M1-2: REST 错误携带 HTTP 状态码

- [x] 2.1 在 `okx-core/src/error.rs` 新增 `OkxError::HttpStatus { status: u16, body: String }` 变体
- [x] 2.2 修改 `okx-rest/src/client.rs` 非 2xx 响应返回 `HttpStatus` 错误
- [x] 2.3 添加辅助方法 `http_status()`, `is_http_status()`, `is_rate_limited()`
- [x] 2.4 编写测试验证 429 等状态码正确传递

## 3. M1-3: WS 登录支持外部时间戳

- [x] 3.1 在 `okx-core/src/signer.rs` 添加 `generate_ws_login_params_with_timestamp()` 方法
- [x] 3.2 在 `okx-ws/src/client.rs` 添加 `login_with_timestamp()` 方法
- [x] 3.3 修改 `okx-ws/src/reconnect.rs` 保存并复用 provider
  - 已实现：`timestamp_provider: Option<Arc<dyn TimestampProvider>>` 字段
  - 已实现：`connect_with_timestamp_provider()` 构造方法
  - 已实现：`set_timestamp_provider()` / `clear_timestamp_provider()` 方法
- [x] 3.4 重连恢复订阅前使用同一 provider 重新登录
  - 已实现：`restore_subscriptions()` 中使用 provider 的 `timestamp_unix_secs()` 登录
- [x] 3.5 编写测试验证重连后时间源一致性
  - 已实现：`test_set_and_clear_timestamp_provider` 测试

## 4. M1-4: 解析 channel-conn-count 消息

- [x] 4.1 在 `okx-ws/src/message.rs` 新增 `WsMessage::ChannelConnCount { channel, conn_count, conn_id }` 变体
- [x] 4.2 新增 `WsMessage::ChannelConnCountError { ... }` 变体
- [x] 4.3 在 `parse()` 函数中识别 `channel-conn-count` 和 `channel-conn-count-error` 事件
- [x] 4.4 编写测试验证消息解析正确性
- [x] 4.5 更新 Python 绑定 `ws_message_to_py()` 支持新消息类型

## 5. Python 绑定同步

- [x] 5.1 在 `okx-py/src/lib.rs` 增强 `to_py_err()` 函数，区分 `HttpStatus` 错误类型
  - 已实现：为 429 限速错误返回 `OkxRateLimitError` 异常
  - 已实现：分层异常类型（`OkxError`, `OkxHttpError`, `OkxRateLimitError`, `OkxApiError`, `OkxAuthError`, `OkxWebSocketError`, `OkxTimeoutError`）
- [x] 5.2 在 `okx-py/src/ws_client.rs` 的 `PyWsClient` 添加 `login_with_timestamp()` 方法
  - 已暴露给 Python 用户使用外部时间戳登录的能力
- [x] 5.3 为 Python 绑定添加 `ws_message_to_py` 的单元测试覆盖新消息类型
- [x] 5.4 更新 Python 绑定文档/docstring 说明新功能

## 6. 可选: TimeSync 时间源组件

- [x] 6.1 在 `okx-core` 或 `okx-rest` 新增 `TimeSync` 结构
- [x] 6.2 实现基于 `GET /api/v5/public/time` 的时钟偏移计算
- [x] 6.3 支持配置刷新周期
- [x] 6.4 实现 `TimestampProvider` trait
- [x] 6.5 编写集成测试
- [x] 6.6 在 Python 绑定中暴露 `TimeSync` 组件

## 7. 可选: REST 响应头/限速信息暴露

- [x] 7.1 定义 `ResponseMeta` 结构（包含 headers、status 等）
- [x] 7.2 提供 `request_with_meta()` 方法返回完整响应信息
- [x] 7.3 编写文档说明如何使用限速信息
- [x] 7.4 在 Python 绑定中暴露响应元信息
  - 已实现：`PyResponseMeta` 类
  - 已实现：`get_public_with_meta()`、`get_private_with_meta()`、`post_private_with_meta()` 方法

## 8. 验收与发布

- [x] 8.1 运行 `cargo clippy` 确保无警告
- [x] 8.2 运行 `cargo test` 确保核心 crate 测试通过
- [x] 8.3 更新 CHANGELOG
- [ ] 8.4 打 tag 或固定 commit 供下游项目 pin
