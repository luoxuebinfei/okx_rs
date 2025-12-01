## 提交前检查
1) 依赖同步/工具就绪（确保安装 llvm-tools + cargo-llvm-cov 如需覆盖率）
2) `cargo fmt --all`
3) `cargo clippy --all-targets --all-features -- -D warnings`
4) `cargo test --all-targets --all-features`（留意可能访问公共 API 的测试日志）
5) 如需覆盖率：`cargo coverage` 生成 target/lcov.info
6) 必要时构建：`cargo build --all-features`
7) 记录关键决策/边界，提交信息用中文 Conventional Commits