# okx-core Specification

## Purpose
TBD - created by archiving change add-okx-rust-sdk. Update Purpose after archive.
## Requirements
### Requirement: 配置管理

系统 SHALL 提供配置结构用于管理 OKX API 连接参数。

配置 SHALL 包含以下字段：
- `api_key`: API 密钥
- `secret_key`: 密钥
- `passphrase`: 口令
- `base_url`: REST API 基础 URL（默认 `https://www.okx.com`）
- `ws_public_url`: 公共 WebSocket URL
- `ws_private_url`: 私有 WebSocket URL
- `simulated`: 是否为模拟交易环境

#### Scenario: 创建生产环境配置
- **WHEN** 用户使用 API 凭证创建配置
- **THEN** 系统返回配置实例，`simulated` 默认为 `false`

#### Scenario: 创建模拟交易配置
- **WHEN** 用户创建配置并设置 `simulated` 为 `true`
- **THEN** 系统使用模拟交易环境 URL

---

### Requirement: 请求签名

系统 SHALL 实现 OKX API 签名规范，用于认证请求。

签名流程 SHALL 遵循以下步骤：
1. 构造预签名字符串：`timestamp + method + requestPath + body`
2. 使用 HMAC-SHA256 和 `secret_key` 签名
3. 对结果进行 Base64 编码

系统 SHALL 支持两种签名方式：
- `generate_headers()`: 使用本机时间（向后兼容）
- `generate_headers_with_timestamp()`: 使用外部提供的时间戳

#### Scenario: 签名 GET 请求
- **WHEN** 用户发起 GET 请求到 `/api/v5/account/balance`
- **THEN** 系统生成正确的签名头
- **AND** 签名头包含 `OK-ACCESS-KEY`, `OK-ACCESS-SIGN`, `OK-ACCESS-TIMESTAMP`, `OK-ACCESS-PASSPHRASE`

#### Scenario: 签名 POST 请求
- **WHEN** 用户发起 POST 请求到 `/api/v5/trade/order` 带 JSON body
- **THEN** 系统将 body 纳入签名计算
- **AND** 生成正确的签名

#### Scenario: 使用外部时间戳签名
- **WHEN** 调用 `generate_headers_with_timestamp("2024-01-01T00:00:00.000Z", ...)`
- **THEN** 系统使用提供的时间戳生成签名
- **AND** `OK-ACCESS-TIMESTAMP` 头为提供的时间戳

### Requirement: 错误处理

系统 SHALL 定义统一的错误类型，覆盖所有可能的错误场景。

错误类型 SHALL 包含：
- `Http`: HTTP 通信错误
- `WebSocket`: WebSocket 连接错误
- `Api`: OKX API 返回的业务错误（含 code 和 msg）
- `Auth`: 认证相关错误
- `Serde`: 序列化/反序列化错误
- `Timeout`: 请求超时

#### Scenario: API 返回业务错误
- **WHEN** OKX API 返回 `{"code": "50001", "msg": "Parameter error"}`
- **THEN** 系统返回 `OkxError::Api { code: "50001", msg: "Parameter error" }`

#### Scenario: 网络超时
- **WHEN** 请求超过配置的超时时间
- **THEN** 系统返回 `OkxError::Timeout`

---

### Requirement: 通用响应类型

系统 SHALL 提供泛型响应包装器，解析 OKX API 统一响应格式。

响应结构 SHALL 包含：
- `code`: 状态码（"0" 表示成功）
- `msg`: 错误消息
- `data`: 泛型数据数组

#### Scenario: 解析成功响应
- **WHEN** API 返回 `{"code": "0", "msg": "", "data": [...]}`
- **THEN** 系统正确解析 `data` 字段到目标类型

#### Scenario: 解析错误响应
- **WHEN** API 返回 `{"code": "50001", "msg": "error", "data": []}`
- **THEN** 系统返回 `OkxError::Api`

---

### Requirement: 账户类型定义

系统 SHALL 定义账户相关的数据类型。

类型 SHALL 包含：
- `Balance`: 账户余额（币种、可用、冻结、权益）
- `Position`: 持仓信息（合约、方向、数量、均价、未实现盈亏）
- `AccountConfig`: 账户配置（账户等级、仓位模式等）
- `Leverage`: 杠杆设置

#### Scenario: 反序列化账户余额
- **WHEN** 收到账户余额 JSON 数据
- **THEN** 系统正确解析为 `Balance` 结构

---

### Requirement: 交易类型定义

系统 SHALL 定义交易相关的数据类型。

类型 SHALL 包含：
- `Order`: 订单（订单ID、合约、方向、类型、价格、数量、状态）
- `Fill`: 成交记录（成交ID、订单ID、价格、数量、手续费）
- `AlgoOrder`: 算法订单（触发价、止盈止损等）
- `OrderSide`: 买卖方向枚举
- `OrderType`: 订单类型枚举（limit, market, post_only 等）
- `OrderState`: 订单状态枚举

#### Scenario: 反序列化订单
- **WHEN** 收到订单 JSON 数据
- **THEN** 系统正确解析为 `Order` 结构
- **AND** 枚举字段正确映射

---

### Requirement: 行情类型定义

系统 SHALL 定义行情相关的数据类型。

类型 SHALL 包含：
- `Ticker`: 行情快照（最新价、买一、卖一、24h成交量等）
- `OrderBook`: 深度数据（买盘、卖盘列表）
- `OrderBookLevel`: 深度档位（价格、数量）
- `Candle`: K线数据（开高低收、成交量、时间戳）
- `Trade`: 成交记录（价格、数量、方向、时间）
- `Instrument`: 交易产品信息

#### Scenario: 反序列化深度数据
- **WHEN** 收到深度 JSON 数据
- **THEN** 系统正确解析为 `OrderBook` 结构
- **AND** 买卖盘按价格排序

---

### Requirement: 资金类型定义

系统 SHALL 定义资金相关的数据类型。

类型 SHALL 包含：
- `FundingBalance`: 资金账户余额
- `DepositAddress`: 充值地址
- `DepositRecord`: 充值记录
- `WithdrawalRecord`: 提现记录
- `Transfer`: 划转记录

#### Scenario: 反序列化提现记录
- **WHEN** 收到提现记录 JSON 数据
- **THEN** 系统正确解析为 `WithdrawalRecord` 结构

### Requirement: 模拟盘请求头规则（x-simulated-trading）

系统 SHALL 按官方文档要求在模拟盘（Demo Trading）请求中携带 `x-simulated-trading` 请求头，并保证默认行为安全（避免在未显式启用模拟盘时发送模拟盘标记）。

#### Scenario: 生产模式不发送模拟盘标记
- **WHEN** 配置 `simulated=false`
- **THEN** 系统不得发送 `x-simulated-trading: 1`

#### Scenario: 模拟盘模式必须发送模拟盘标记
- **WHEN** 配置 `simulated=true`
- **THEN** 系统 MUST 发送 `x-simulated-trading: 1`

### Requirement: 时间戳提供者

系统 SHALL 提供 `TimestampProvider` trait，用于抽象时间戳获取逻辑。

TimestampProvider SHALL 支持：
- 返回 Unix 秒时间戳
- 返回 ISO 8601 格式时间戳（用于 REST 签名）
- 线程安全（Send + Sync）

系统 SHALL 提供默认实现 `LocalTimestampProvider`，使用本机时间。

#### Scenario: 获取 Unix 秒时间戳
- **WHEN** 调用 `unix_seconds()`
- **THEN** 返回当前 Unix 秒时间戳

#### Scenario: 获取 ISO 格式时间戳
- **WHEN** 调用 `iso_timestamp()`
- **THEN** 返回格式为 `YYYY-MM-DDTHH:MM:SS.sssZ` 的字符串

---

### Requirement: 时间同步组件

系统 SHALL 提供可选的 `TimeSync` 组件，用于维护与 OKX 服务器的时钟偏移。

TimeSync SHALL 支持：
- 基于 `GET /api/v5/public/time` 计算时钟偏移
- 可配置刷新周期
- 实现 `TimestampProvider` trait

#### Scenario: 初始化时间同步
- **WHEN** 创建 `TimeSync` 实例并调用 `sync()`
- **THEN** 系统请求服务器时间并计算偏移量

#### Scenario: 获取同步后的时间戳
- **WHEN** 调用 `unix_seconds()` 或 `iso_timestamp()`
- **THEN** 返回经过偏移校正的时间戳

#### Scenario: 自动刷新偏移
- **WHEN** 配置了刷新周期
- **THEN** 系统定期重新同步时钟偏移

---

### Requirement: HTTP 状态码错误

系统 SHALL 在 `OkxError` 中提供 `HttpStatus` 变体，携带 HTTP 状态码信息。

`HttpStatus` SHALL 包含：
- `status`: HTTP 状态码（u16）
- `body`: 响应体内容（String）

#### Scenario: 识别限速错误
- **WHEN** HTTP 响应状态码为 429
- **THEN** 返回 `OkxError::HttpStatus { status: 429, body: "..." }`

#### Scenario: 识别服务器错误
- **WHEN** HTTP 响应状态码为 500
- **THEN** 返回 `OkxError::HttpStatus { status: 500, body: "..." }`

#### Scenario: 识别认证错误
- **WHEN** HTTP 响应状态码为 401
- **THEN** 返回 `OkxError::HttpStatus { status: 401, body: "..." }`

---

