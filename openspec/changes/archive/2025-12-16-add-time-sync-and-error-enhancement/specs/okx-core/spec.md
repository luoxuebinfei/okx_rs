## ADDED Requirements

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

## MODIFIED Requirements

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
