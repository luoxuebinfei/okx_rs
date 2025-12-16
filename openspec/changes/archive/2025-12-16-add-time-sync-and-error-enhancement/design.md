# Design: 时间同步与错误处理增强

## Context

OKX API 对请求签名有严格的时间戳要求（通常 ±30 秒内有效）。当客户端与服务器存在时钟偏移时，签名会失败。当前实现固定使用本机时间，无法适应时钟不同步的场景。

此外，HTTP 错误处理丢失状态码信息，导致无法精确识别限速（429）等场景，只能通过解析错误消息字符串来判断。

## Goals / Non-Goals

### Goals
- 支持外部注入时间戳，使签名可以使用服务器时间
- 保持向后兼容，原有 API 不变
- HTTP 错误携带状态码，便于精确错误处理
- WS 重连后保持时间源一致性
- 解析更多 WS 事件类型

### Non-Goals
- 不自动同步时间（由调用方决定是否使用 TimeSync）
- 不改变现有成功路径的行为
- 不处理所有可能的 WS 事件（仅处理 channel-conn-count 相关）

## Decisions

### D1: TimestampProvider trait 设计

```rust
/// 时间戳提供者 trait
pub trait TimestampProvider: Send + Sync {
    /// 返回 Unix 秒时间戳
    fn unix_seconds(&self) -> i64;

    /// 返回 ISO 8601 格式时间戳（用于 REST 签名）
    fn iso_timestamp(&self) -> String;
}

/// 默认实现：使用本机时间
pub struct LocalTimestampProvider;

impl TimestampProvider for LocalTimestampProvider {
    fn unix_seconds(&self) -> i64 {
        chrono::Utc::now().timestamp()
    }

    fn iso_timestamp(&self) -> String {
        chrono::Utc::now().format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string()
    }
}
```

**理由**: 使用 trait 而非闭包，便于测试和复用。提供默认实现保持向后兼容。

### D2: Signer API 扩展

```rust
impl Signer {
    /// 原有方法，使用本机时间（保持不变）
    pub fn generate_headers(&self, method: &str, request_path: &str, body: &str, simulated: bool) -> HeaderMap;

    /// 新增方法，支持外部时间戳
    pub fn generate_headers_with_timestamp(
        &self,
        timestamp_iso: &str,
        method: &str,
        request_path: &str,
        body: &str,
        simulated: bool,
    ) -> HeaderMap;
}
```

**理由**: 新增方法而非修改原方法签名，保持向后兼容。

### D3: 错误类型变更（Breaking Change）

```rust
#[derive(Debug, thiserror::Error)]
pub enum OkxError {
    // 原有变体保留
    #[error("HTTP error: {0}")]
    Http(String),  // 保留用于连接级错误

    // 新增变体
    #[error("HTTP status {status}: {body}")]
    HttpStatus { status: u16, body: String },

    // ... 其他变体
}
```

**理由**: 新增 `HttpStatus` 变体而非修改 `Http`，减少破坏性。`Http` 保留用于连接级错误（如 DNS 解析失败）。

### D4: WS 客户端时间戳注入

```rust
pub struct WsClientBuilder {
    // ...
    timestamp_provider: Option<Arc<dyn TimestampProvider>>,
}

impl WsClientBuilder {
    pub fn with_timestamp_provider(mut self, provider: Arc<dyn TimestampProvider>) -> Self {
        self.timestamp_provider = Some(provider);
        self
    }
}
```

**理由**: 使用 Builder 模式注入，保持现有构造方式不变。使用 `Arc` 便于在重连时共享。

### D5: ReconnectingWsClient 时间源复用

```rust
struct ReconnectState {
    subscriptions: Vec<ChannelArg>,
    timestamp_provider: Option<Arc<dyn TimestampProvider>>,
    // ...
}
```

**理由**: 在重连状态中保存 provider 引用，重连时复用同一实例。

### D6: channel-conn-count 消息结构

```rust
pub enum WsMessage {
    // 现有变体...

    /// 频道连接数信息
    ChannelConnCount {
        channel: String,
        conn_count: u32,
        conn_id: String,
    },

    /// 频道连接数错误
    ChannelConnCountError {
        channel: String,
        code: String,
        msg: String,
    },
}
```

**理由**: 与现有消息类型风格一致，字段命名遵循 OKX 官方文档。

## Risks / Trade-offs

| 风险 | 缓解措施 |
|------|----------|
| Breaking change 影响下游 | 新增变体而非修改，提供迁移指南 |
| TimestampProvider 增加复杂度 | 提供默认实现，不使用时无感知 |
| 重连时 provider 可能已失效 | 文档说明 provider 应保持有效 |

## Migration Plan

1. 发布新版本，包含所有新功能
2. 下游项目更新依赖，处理新的 `HttpStatus` 错误变体
3. 可选：使用 `TimeSync` 组件替代本机时间
4. 打 tag 供下游 pin 版本

## Open Questions

1. `TimeSync` 组件是否应该放在 `okx-core` 还是单独的 crate？
   - 建议：放在 `okx-core`，因为它是基础设施组件
2. 是否需要提供同步版本的 `TimestampProvider`？
   - 建议：暂不需要，当前 SDK 全异步设计
