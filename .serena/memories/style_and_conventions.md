## 代码风格与约定
- 语言：Rust 2021，命名规范类型 PascalCase、函数/变量 snake_case、常量 SCREAMING_SNAKE_CASE、模块 snake_case
- 格式/静态检查：统一使用 `rustfmt`、`clippy`（workspace lints: unsafe_code/missing_docs warn，clippy all/pedantic warn）
- 架构原则：SOLID/DRY/SoC/YAGNI，异步优先，复杂配置用 Builder，关键流程可用简洁中文注释
- 错误处理：使用 thiserror，自定义 OkxError + Result；拒绝 TODO/占位，所有代码需可运行
- 最高优先级：实现/变更任何 API 前必须验证官方 Python SDK 与 OKX 文档，禁止猜测端点/参数/响应/认证
- 交流与文档：交流、注释、提交信息均用简体中文，提交遵循 Conventional Commits（例：`feat: 实现 Account API 基础方法`）