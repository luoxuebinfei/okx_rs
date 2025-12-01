<!-- OPENSPEC:START -->
# OpenSpec Instructions

These instructions are for AI assistants working in this project.

Always open `@/openspec/AGENTS.md` when the request:
- Mentions planning or proposals (words like proposal, spec, change, plan)
- Introduces new capabilities, breaking changes, architecture shifts, or big performance/security work
- Sounds ambiguous and you need the authoritative spec before coding

Use `@/openspec/AGENTS.md` to learn:
- How to create and apply change proposals
- Spec format and conventions
- Project structure and guidelines

Keep this managed block so 'openspec update' can refresh the instructions.

<!-- OPENSPEC:END -->

# OKX Rust SDK 项目指南

## 项目概述

高性能 OKX 交易所 Rust SDK，同时提供 Python 绑定。

## 技术栈

| 组件 | 选型 |
|------|------|
| 异步运行时 | tokio |
| HTTP 客户端 | reqwest |
| WebSocket | tokio-tungstenite |
| 序列化 | serde + serde_json |
| Python 绑定 | PyO3 + maturin |
| 错误处理 | thiserror |

## 项目结构

```
okx_rs/
├── Cargo.toml              # Workspace root
├── crates/
│   ├── okx-core/           # 核心类型、认证、错误
│   ├── okx-rest/           # REST API 客户端
│   ├── okx-ws/             # WebSocket 客户端
│   └── okx-py/             # Python 绑定
└── openspec/               # 规范文档
```

## ⚠️ 最高优先级约束：严禁猜测 API

**所有 OKX API 实现必须经过验证，严禁凭借记忆猜测！**

### 必须使用的验证工具

1. **DeepWiki MCP** - 查询官方 Python SDK
   ```
   mcp__mcp-deepwiki__ask_question
   仓库: okxapi/python-okx
   ```

2. **WebFetch/Exa** - 获取官方 API 文档
   ```
   官方文档: https://www.okx.com/docs-v5/en/
   ```

3. **Context7 MCP** - 查询 Rust 依赖库文档
   ```
   mcp__context7__resolve-library-id  # 先解析库 ID
   mcp__context7__get-library-docs    # 再获取文档
   ```

### 每个 API 实现前必须确认

- [ ] 端点路径正确
- [ ] 请求方法正确 (GET/POST)
- [ ] 请求参数完整且类型正确
- [ ] 响应结构与官方一致
- [ ] 认证要求明确

### 禁止行为

- ❌ 凭记忆编写 API 端点
- ❌ 猜测参数名称或类型
- ❌ 假设响应结构
- ❌ 复制未验证的代码

## 官方参考资源

| 资源 | URL |
|------|-----|
| Python SDK | https://github.com/okxapi/python-okx |
| REST API 文档 | https://www.okx.com/docs-v5/en/ |
| WebSocket 文档 | https://www.okx.com/docs-v5/en/#websocket-api |
| API 变更日志 | https://www.okx.com/docs-v5/log_en/ |

## OKX API 端点

### REST API
- 生产: `https://www.okx.com`
- AWS: `https://aws.okx.com`

### WebSocket
- 公共: `wss://ws.okx.com:8443/ws/v5/public`
- 私有: `wss://ws.okx.com:8443/ws/v5/private`
- 模拟公共: `wss://wspap.okx.com:8443/ws/v5/public?brokerId=9999`
- 模拟私有: `wss://wspap.okx.com:8443/ws/v5/private?brokerId=9999`

## 代码规范

- 使用 `rustfmt` 格式化
- 使用 `clippy` 检查
- Commit 信息: 简体中文，Conventional Commits 格式
- 示例: `feat: 实现 Account API 基础方法`

## API 实现检查模板

实现任何 API 方法前，使用以下模板验证：

```markdown
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
{
  "code": "0",
  "msg": "",
  "data": [...]
}

### 验证状态
- [ ] 端点已验证
- [ ] 参数已验证
- [ ] 响应已验证
```
