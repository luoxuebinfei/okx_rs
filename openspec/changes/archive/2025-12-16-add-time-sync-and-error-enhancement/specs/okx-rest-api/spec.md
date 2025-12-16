## ADDED Requirements

### Requirement: 响应元信息

系统 SHALL 提供可选的响应元信息获取方式，暴露 HTTP 响应头和状态码。

ResponseMeta SHALL 包含：
- `status`: HTTP 状态码
- `headers`: 响应头集合
- `body`: 响应体

#### Scenario: 获取限速相关响应头
- **WHEN** 调用 `request_with_meta()` 方法
- **THEN** 返回包含完整响应头的 `ResponseMeta`
- **AND** 可读取 `X-RateLimit-*` 等限速相关头

#### Scenario: 实现自适应退避
- **WHEN** 响应头包含限速信息
- **THEN** 调用方可根据头信息调整请求频率

---

## MODIFIED Requirements

### Requirement: HTTP 客户端基类

系统 SHALL 提供异步 HTTP 客户端基类，处理所有 REST API 请求。

客户端 SHALL 支持：
- 自动签名所有认证请求
- 连接池管理
- 超时配置
- 代理支持
- 自动重试（可配置）
- 可选的外部时间戳提供者
- 非 2xx 响应返回 `OkxError::HttpStatus`

#### Scenario: 发送认证 GET 请求
- **WHEN** 调用需要认证的 GET 端点
- **THEN** 系统自动添加签名头
- **AND** 返回解析后的响应

#### Scenario: 发送 POST 请求
- **WHEN** 调用 POST 端点并传入参数
- **THEN** 系统将参数序列化为 JSON
- **AND** 自动添加签名头
- **AND** 返回解析后的响应

#### Scenario: 配置代理
- **WHEN** 用户配置 HTTP 代理
- **THEN** 所有请求通过代理发送

#### Scenario: 使用外部时间戳签名
- **WHEN** 配置了 `TimestampProvider`
- **THEN** 签名使用 provider 提供的时间戳
- **AND** 不使用本机时间

#### Scenario: 处理限速响应
- **WHEN** 服务器返回 HTTP 429
- **THEN** 返回 `OkxError::HttpStatus { status: 429, body: "..." }`
- **AND** 调用方可根据状态码实现退避策略

#### Scenario: 处理服务器错误
- **WHEN** 服务器返回 HTTP 5xx
- **THEN** 返回 `OkxError::HttpStatus { status: 5xx, body: "..." }`
