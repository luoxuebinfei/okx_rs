## ADDED Requirements

### Requirement: 频道连接数消息

系统 SHALL 解析 `channel-conn-count` 和 `channel-conn-count-error` 事件为结构化消息。

WsMessage SHALL 新增以下变体：
- `ChannelConnCount`: 频道连接数信息
- `ChannelConnCountError`: 频道连接数错误

#### Scenario: 解析 channel-conn-count 事件
- **WHEN** 收到 `{"event": "channel-conn-count", "channel": "orders", "connCount": "5", "connId": "abc123"}`
- **THEN** 返回 `WsMessage::ChannelConnCount { channel: "orders", conn_count: 5, conn_id: "abc123" }`

#### Scenario: 解析 channel-conn-count-error 事件
- **WHEN** 收到 `{"event": "channel-conn-count-error", "channel": "orders", "code": "60001", "msg": "..."}`
- **THEN** 返回 `WsMessage::ChannelConnCountError { channel: "orders", code: "60001", msg: "..." }`

#### Scenario: 未知事件仍进入 Unknown
- **WHEN** 收到未识别的事件类型
- **THEN** 返回 `WsMessage::Unknown`（保持现有行为）

---

## MODIFIED Requirements

### Requirement: WebSocket 认证

系统 SHALL 实现私有 WebSocket 连接的认证登录。

认证流程 SHALL 遵循：
1. 生成登录参数（timestamp, apiKey, passphrase, sign）
2. 发送 login 操作请求
3. 验证登录响应

系统 SHALL 支持外部时间戳注入：
- 可配置 `TimestampProvider` 用于生成登录时间戳
- 默认使用本机时间（向后兼容）

#### Scenario: 私有频道登录成功
- **WHEN** 发送正确的登录凭证
- **THEN** 系统收到登录成功响应
- **AND** 可以订阅私有频道

#### Scenario: 私有频道登录失败
- **WHEN** 发送无效的登录凭证
- **THEN** 系统收到登录失败响应
- **AND** 返回 `OkxError::Auth`

#### Scenario: 使用外部时间戳登录
- **WHEN** 配置了 `TimestampProvider`
- **THEN** 登录签名使用 provider 提供的 Unix 秒时间戳
- **AND** 不使用本机时间

---

### Requirement: 自动重连

系统 SHALL 实现自动重连机制，保证连接稳定性。

重连策略 SHALL 支持：
- 指数退避重连
- 最大重试次数配置
- 重连间隔配置
- 订阅状态恢复
- 时间戳提供者复用

#### Scenario: 连接断开后自动重连
- **WHEN** WebSocket 连接意外断开
- **THEN** 系统自动尝试重连
- **AND** 使用指数退避策略

#### Scenario: 重连成功后恢复订阅
- **WHEN** 重连成功
- **THEN** 系统自动重新订阅之前的频道
- **AND** 私有连接自动重新登录

#### Scenario: 达到最大重试次数
- **WHEN** 重连次数达到上限
- **THEN** 系统触发错误回调
- **AND** 停止重连尝试

#### Scenario: 重连时复用时间戳提供者
- **WHEN** 配置了 `TimestampProvider` 且发生重连
- **THEN** 重新登录时使用同一 provider
- **AND** 保证时间源一致性
