# Project Context

## Purpose

构建高性能的 OKX 交易所 Rust SDK，同时提供 Python 绑定，让用户在 Rust 和 Python 生态中都能享受原生性能体验。

## Tech Stack

- **语言**: Rust (2021 Edition)
- **异步运行时**: tokio
- **HTTP 客户端**: reqwest
- **WebSocket**: tokio-tungstenite
- **序列化**: serde + serde_json
- **Python 绑定**: PyO3 + maturin
- **错误处理**: thiserror

## Project Conventions

### Code Style

- 遵循 Rust 官方风格指南
- 使用 `rustfmt` 格式化代码
- 使用 `clippy` 进行静态检查
- 命名规范：
  - 类型: `PascalCase`
  - 函数/变量: `snake_case`
  - 常量: `SCREAMING_SNAKE_CASE`
  - 模块: `snake_case`

### Architecture Patterns

- Workspace 多 Crate 结构
- 异步优先，必要时提供同步包装
- 泛型优于 trait object（关键路径）
- Builder 模式用于复杂配置

### Testing Strategy

- 单元测试：每个模块内部
- 集成测试：`tests/` 目录
- 使用模拟交易环境测试
- 性能基准：criterion

### Git Workflow

- 主分支: `master`
- Commit 信息: 简体中文，遵循 Conventional Commits
- 示例: `feat: 实现 Account API 基础方法`

## Domain Context

OKX 是全球领先的加密货币交易所，提供：
- 现货交易 (SPOT)
- 永续合约 (SWAP)
- 交割合约 (FUTURES)
- 期权 (OPTION)

API 版本: v5

## Important Constraints

### 严格禁止猜测 API

**最高优先级约束**：

所有涉及 OKX API 的实现，**必须**通过以下方式验证，**严禁凭借记忆猜测**：

1. **查询官方 Python SDK 源码**
   - 仓库: https://github.com/okxapi/python-okx
   - 使用 DeepWiki MCP 工具: `mcp__mcp-deepwiki__ask_question`
   - 或直接查看源码文件

2. **查询官方 API 文档**
   - REST API: https://www.okx.com/docs-v5/en/
   - WebSocket: https://www.okx.com/docs-v5/en/#websocket-api
   - 使用 WebFetch 或 Exa 搜索工具获取最新文档

3. **使用 Context7 MCP 查询库文档**
   - 先调用 `mcp__context7__resolve-library-id` 解析库 ID
   - 再调用 `mcp__context7__get-library-docs` 获取文档
   - 适用于查询 Rust 依赖库（tokio, reqwest, PyO3 等）的用法

4. **验证清单**（每个 API 实现前必须确认）
   - [ ] 端点路径正确
   - [ ] 请求方法正确 (GET/POST)
   - [ ] 请求参数完整且类型正确
   - [ ] 响应结构与官方一致
   - [ ] 认证要求明确

4. **禁止行为**
   - 凭记忆编写 API 端点
   - 猜测参数名称或类型
   - 假设响应结构
   - 复制未验证的代码

### 其他约束

- 必须支持生产环境和模拟交易环境
- WebSocket 必须实现自动重连
- Python 绑定必须支持 Python 3.9+
- 所有公开 API 必须有文档注释

## External Dependencies

### OKX API

- **REST API 基础 URL**:
  - 生产: `https://www.okx.com`
  - AWS: `https://aws.okx.com`
- **WebSocket URL**:
  - 公共: `wss://ws.okx.com:8443/ws/v5/public`
  - 私有: `wss://ws.okx.com:8443/ws/v5/private`
  - 模拟公共: `wss://wspap.okx.com:8443/ws/v5/public?brokerId=9999`
  - 模拟私有: `wss://wspap.okx.com:8443/ws/v5/private?brokerId=9999`

### 官方参考资源

- **Python SDK**: https://github.com/okxapi/python-okx
- **API 文档**: https://www.okx.com/docs-v5/en/
- **API 变更日志**: https://www.okx.com/docs-v5/log_en/

## API 实现检查模板

实现任何 API 方法前，使用以下模板验证：

```
## API: [方法名]

### 来源验证
- [ ] 查阅官方 Python SDK: [文件路径]
- [ ] 查阅官方文档: [文档链接]

### 端点信息
- 路径: /api/v5/...
- 方法: GET/POST
- 认证: 是/否

### 请求参数
| 参数名 | 类型 | 必填 | 说明 |
|--------|------|------|------|

### 响应结构
```json
{
  "code": "0",
  "msg": "",
  "data": [...]
}
```

### 验证状态
- [ ] 端点已验证
- [ ] 参数已验证
- [ ] 响应已验证
```
