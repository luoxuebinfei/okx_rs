# OKX WebSocket Specification

## ADDED Requirements

### Requirement: WebSocket 连接管理

系统 SHALL 提供 WebSocket 连接管理器，处理与 OKX 的实时数据通信。

连接管理器 SHALL 支持：
- TLS 安全连接
- 自动心跳维护（ping/pong）
- 连接状态监控
- 优雅关闭

#### Scenario: 建立公共 WebSocket 连接
- **WHEN** 调用 `connect()` 到公共 WebSocket URL
- **THEN** 系统建立 TLS 连接
- **AND** 开始心跳维护

#### Scenario: 建立私有 WebSocket 连接
- **WHEN** 调用 `connect()` 到私有 WebSocket URL
- **THEN** 系统建立 TLS 连接
- **AND** 自动发送登录请求
- **AND** 开始心跳维护

#### Scenario: 心跳维护
- **WHEN** 连接建立后
- **THEN** 系统定期发送 ping 消息（默认 25 秒）
- **AND** 监控 pong 响应

---

### Requirement: 自动重连

系统 SHALL 实现自动重连机制，保证连接稳定性。

重连策略 SHALL 支持：
- 指数退避重连
- 最大重试次数配置
- 重连间隔配置
- 订阅状态恢复

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

---

### Requirement: WebSocket 认证

系统 SHALL 实现私有 WebSocket 连接的认证登录。

认证流程 SHALL 遵循：
1. 生成登录参数（timestamp, apiKey, passphrase, sign）
2. 发送 login 操作请求
3. 验证登录响应

#### Scenario: 私有频道登录成功
- **WHEN** 发送正确的登录凭证
- **THEN** 系统收到登录成功响应
- **AND** 可以订阅私有频道

#### Scenario: 私有频道登录失败
- **WHEN** 发送无效的登录凭证
- **THEN** 系统收到登录失败响应
- **AND** 返回 `OkxError::Auth`

---

### Requirement: 公共 WebSocket 频道

系统 SHALL 支持订阅以下公共 WebSocket 频道：

- `tickers`: 行情数据
- `trades`: 成交数据
- `books`: 深度数据（可配置深度）
- `books5`: 5 档深度
- `books50-l2-tbt`: 50 档深度（tick by tick）
- `candle{period}`: K 线数据（1m, 5m, 1H, 1D 等）
- `index-tickers`: 指数行情
- `mark-price`: 标记价格
- `funding-rate`: 资金费率

#### Scenario: 订阅行情频道
- **WHEN** 调用 `subscribe(channel: "tickers", inst_id: "BTC-USDT")`
- **THEN** 系统发送订阅请求
- **AND** 收到实时行情推送

#### Scenario: 订阅深度频道
- **WHEN** 调用 `subscribe(channel: "books", inst_id: "BTC-USDT")`
- **THEN** 系统发送订阅请求
- **AND** 收到全量深度快照
- **AND** 后续收到增量更新

#### Scenario: 订阅 K 线频道
- **WHEN** 调用 `subscribe(channel: "candle1m", inst_id: "BTC-USDT")`
- **THEN** 系统发送订阅请求
- **AND** 收到实时 K 线推送

#### Scenario: 批量订阅多个频道
- **WHEN** 调用 `subscribe_batch(channels: Vec<ChannelArg>)`
- **THEN** 系统一次性发送多个订阅请求

#### Scenario: 取消订阅
- **WHEN** 调用 `unsubscribe(channel: "tickers", inst_id: "BTC-USDT")`
- **THEN** 系统发送取消订阅请求
- **AND** 停止接收该频道数据

---

### Requirement: 私有 WebSocket 频道

系统 SHALL 支持订阅以下私有 WebSocket 频道（需要登录）：

- `account`: 账户信息变更
- `positions`: 持仓变更
- `balance_and_position`: 账户余额和持仓
- `orders`: 订单状态变更
- `orders-algo`: 算法订单状态变更
- `liquidation-warning`: 强平预警

#### Scenario: 订阅账户频道
- **WHEN** 登录成功后调用 `subscribe(channel: "account")`
- **THEN** 系统发送订阅请求
- **AND** 收到账户变更推送

#### Scenario: 订阅订单频道
- **WHEN** 登录成功后调用 `subscribe(channel: "orders", inst_type: "SPOT")`
- **THEN** 系统发送订阅请求
- **AND** 收到订单状态变更推送

#### Scenario: 订阅持仓频道
- **WHEN** 登录成功后调用 `subscribe(channel: "positions", inst_type: "SWAP")`
- **THEN** 系统发送订阅请求
- **AND** 收到持仓变更推送

#### Scenario: 未登录订阅私有频道
- **WHEN** 未登录状态下订阅私有频道
- **THEN** 系统返回 `OkxError::Auth` 错误

---

### Requirement: 消息处理

系统 SHALL 提供灵活的消息处理机制。

消息处理 SHALL 支持：
- 回调函数模式
- async Stream 模式
- 消息类型自动解析
- 错误消息处理

#### Scenario: 回调函数处理消息
- **WHEN** 设置消息回调函数
- **THEN** 收到消息时自动调用回调
- **AND** 消息已解析为对应类型

#### Scenario: Stream 模式处理消息
- **WHEN** 使用 `messages()` 获取 Stream
- **THEN** 返回 `impl Stream<Item = WsMessage>`
- **AND** 可使用 async/await 迭代

#### Scenario: 处理推送消息
- **WHEN** 收到数据推送
- **THEN** 系统解析消息类型
- **AND** 自动反序列化为对应数据结构

#### Scenario: 处理错误消息
- **WHEN** 收到错误推送
- **THEN** 系统解析错误码和错误消息
- **AND** 触发错误处理逻辑
