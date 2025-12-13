# okx-python-binding Specification

## Purpose
TBD - created by archiving change add-okx-rust-sdk. Update Purpose after archive.
## Requirements
### Requirement: Python 模块结构

系统 SHALL 提供 Python 模块 `okx_rs`，可通过 `pip install` 安装。

模块结构 SHALL 包含：
- `okx_rs.OkxClient`: 同步 REST 客户端
- `okx_rs.OkxAsyncClient`: 异步 REST 客户端
- `okx_rs.WsClient`: WebSocket 客户端
- `okx_rs.types`: 数据类型模块

#### Scenario: 安装模块
- **WHEN** 用户执行 `pip install okx-rs`
- **THEN** 模块安装成功
- **AND** 可以 `import okx_rs`

#### Scenario: 导入类型
- **WHEN** 用户执行 `from okx_rs import OkxClient, types`
- **THEN** 导入成功
- **AND** 可以使用 `OkxClient` 和 `types` 模块

---

### Requirement: 同步客户端

系统 SHALL 提供同步 Python 客户端 `OkxClient`，封装 REST API。

#### Scenario: 创建同步客户端
- **WHEN** 用户创建 `OkxClient(api_key, secret_key, passphrase)`
- **THEN** 返回客户端实例
- **AND** 默认为生产环境

#### Scenario: 创建模拟交易客户端
- **WHEN** 用户创建 `OkxClient(..., simulated=True)`
- **THEN** 返回客户端实例
- **AND** 使用模拟交易环境

#### Scenario: 同步获取账户余额
- **WHEN** 调用 `client.get_balance()`
- **THEN** 阻塞等待结果
- **AND** 返回 `List[Balance]`

#### Scenario: 同步下单
- **WHEN** 调用 `client.place_order(inst_id="BTC-USDT", ...)`
- **THEN** 阻塞等待结果
- **AND** 返回订单 ID

---

### Requirement: 异步客户端

系统 SHALL 提供异步 Python 客户端 `OkxAsyncClient`，支持 asyncio。

#### Scenario: 创建异步客户端
- **WHEN** 用户创建 `OkxAsyncClient(api_key, secret_key, passphrase)`
- **THEN** 返回客户端实例

#### Scenario: 异步获取账户余额
- **WHEN** 调用 `await client.get_balance()`
- **THEN** 异步返回 `List[Balance]`
- **AND** 不阻塞事件循环

#### Scenario: 异步批量下单
- **WHEN** 调用 `await client.place_batch_orders([...])`
- **THEN** 异步返回订单 ID 列表

#### Scenario: 与 asyncio 集成
- **WHEN** 在 `async def` 函数中使用客户端
- **THEN** 可正常与其他 asyncio 代码协作

---

### Requirement: WebSocket 客户端

系统 SHALL 提供 Python WebSocket 客户端 `WsClient`。

#### Scenario: 创建公共 WebSocket 客户端
- **WHEN** 调用 `WsClient.public()`
- **THEN** 返回公共 WebSocket 客户端
- **AND** 无需认证

#### Scenario: 创建私有 WebSocket 客户端
- **WHEN** 调用 `WsClient.private(api_key, secret_key, passphrase)`
- **THEN** 返回私有 WebSocket 客户端
- **AND** 自动登录

#### Scenario: 订阅频道（回调模式）
- **WHEN** 调用 `client.subscribe("tickers", "BTC-USDT", callback=handler)`
- **THEN** 订阅成功
- **AND** 收到数据时调用 `handler(data)`

#### Scenario: 订阅频道（async for 模式）
- **WHEN** 使用 `async for msg in client.messages():`
- **THEN** 异步迭代接收消息

#### Scenario: 取消订阅
- **WHEN** 调用 `client.unsubscribe("tickers", "BTC-USDT")`
- **THEN** 取消订阅成功
- **AND** 停止接收该频道数据

#### Scenario: 关闭连接
- **WHEN** 调用 `await client.close()`
- **THEN** 优雅关闭 WebSocket 连接

---

### Requirement: 类型导出

系统 SHALL 将 Rust 数据类型导出为 Python 类。

导出的类型 SHALL 包含：
- `Balance`: 账户余额
- `Position`: 持仓信息
- `Order`: 订单
- `Fill`: 成交记录
- `Ticker`: 行情
- `OrderBook`: 深度
- `Candle`: K 线
- `Trade`: 成交

#### Scenario: 访问 Balance 属性
- **WHEN** 获得 `Balance` 实例
- **THEN** 可访问 `balance.ccy`, `balance.available`, `balance.frozen`

#### Scenario: 访问 Order 属性
- **WHEN** 获得 `Order` 实例
- **THEN** 可访问 `order.ord_id`, `order.inst_id`, `order.side`, `order.state`

#### Scenario: 类型支持 repr
- **WHEN** 打印类型实例
- **THEN** 显示可读的字符串表示

#### Scenario: 类型支持字典转换
- **WHEN** 调用 `dict(balance)` 或 `balance.to_dict()`
- **THEN** 返回 Python 字典

---

### Requirement: 错误处理

系统 SHALL 将 Rust 错误转换为 Python 异常。

异常类型 SHALL 包含：
- `OkxError`: 基础异常类
- `OkxApiError`: API 业务错误
- `OkxAuthError`: 认证错误
- `OkxNetworkError`: 网络错误

#### Scenario: API 返回业务错误
- **WHEN** API 返回 `{"code": "50001", "msg": "Parameter error"}`
- **THEN** 抛出 `OkxApiError`
- **AND** 异常包含 `code` 和 `msg` 属性

#### Scenario: 认证失败
- **WHEN** 使用无效凭证
- **THEN** 抛出 `OkxAuthError`

#### Scenario: 网络错误
- **WHEN** 网络连接失败
- **THEN** 抛出 `OkxNetworkError`

---

### Requirement: 类型存根文件

系统 SHALL 提供 `.pyi` 类型存根文件，支持 IDE 智能提示。

#### Scenario: IDE 类型提示
- **WHEN** 用户在支持类型检查的 IDE 中使用模块
- **THEN** IDE 显示正确的类型提示

#### Scenario: mypy 类型检查
- **WHEN** 用户使用 mypy 检查代码
- **THEN** 类型检查通过

---

### Requirement: 性能优化

系统 SHALL 优化 Python 绑定性能，减少开销。

#### Scenario: 避免不必要的复制
- **WHEN** 传递大型数据结构
- **THEN** 尽可能使用零复制或引用

#### Scenario: GIL 释放
- **WHEN** 执行网络 I/O 操作
- **THEN** 释放 GIL 以允许其他 Python 线程执行

#### Scenario: 异步执行不阻塞
- **WHEN** 使用异步客户端
- **THEN** I/O 操作在 Rust 异步运行时执行
- **AND** Python 事件循环不被阻塞

