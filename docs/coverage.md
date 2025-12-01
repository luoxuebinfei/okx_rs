# 测试覆盖率统计

## 工具与准备
- 安装覆盖率工具：`cargo install cargo-llvm-cov`（需同时安装 `rustup component add llvm-tools-preview`）
- 默认排除 `okx-py` crate，避免 Python 头文件依赖阻塞覆盖率流程

## 运行命令
- 执行 `cargo coverage`
- 动作：运行工作区全部单测/集成测试（不含 doctest，兼容 stable toolchain），生成 `target/lcov.info`；控制台输出覆盖率摘要
- 如需包含 doctest，可改用 `cargo +nightly coverage --doctests`
- 现有部分测试会访问 OKX 公共接口，网络不可达时会打印警告但不会导致失败（测试内部已宽容处理网络错误）

## 后续集成
- `target/coverage/lcov.info` 可直接用于 CI 上传或生成 HTML 报告（例如 `genhtml`）
