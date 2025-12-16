# Change: 时间同步与错误处理增强

## Why

当前 SDK 存在以下阻塞性问题：
1. REST/WS 签名固定使用本机时间，无法与服务器时间同步，导致时钟偏移时认证失败
2. HTTP 错误丢失状态码，无法精确识别 429 限速等场景
3. WS 重连后无法复用同一时间源，导致认证不一致
4. 部分 WS 事件（channel-conn-count）未被解析，进入 Unknown 类型

这些问题直接影响 talosbot 等下游项目的稳定性和可维护性。

## What Changes

### M1 阻塞项（必须实现）

1. **REST 签名支持外部时间戳注入**
   - 在 `Signer` 增加 `generate_headers_with_timestamp()` 方法
   - 保留原 `generate_headers()` 不变，保持向后兼容

2. **REST 错误携带 HTTP 状态码** **BREAKING**
   - 新增 `OkxError::HttpStatus { status: u16, body: String }`
   - 非 2xx 响应返回该错误类型（替代原 `OkxError::Http(String)`）

3. **WS 登录支持外部时间戳**
   - 支持注入 `login_timestamp_provider`（Unix 秒）
   - 重连时复用同一 provider

4. **解析 channel-conn-count 消息**
   - 新增 `WsMessage::ChannelConnCount` 变体
   - 新增 `WsMessage::ChannelConnCountError` 变体

### 可选增强（不阻塞）

5. **TimeSync 时间源组件**
   - 基于 `GET /api/v5/public/time` 维护时钟偏移
   - 可配置刷新周期
   - 默认给 REST/WS 登录使用

6. **REST 响应头/限速信息暴露**
   - 暴露响应头供调用方读取
   - 便于实现自适应退避策略

## Impact

- **Affected specs**: okx-core, okx-rest-api, okx-websocket
- **Affected code**:
  - `crates/okx-core/src/signer.rs`
  - `crates/okx-core/src/error.rs`
  - `crates/okx-rest/src/client.rs`
  - `crates/okx-ws/src/client.rs`
  - `crates/okx-ws/src/reconnect.rs`
  - `crates/okx-ws/src/message.rs`
- **Breaking changes**: `OkxError::Http` 变更为 `OkxError::HttpStatus`
