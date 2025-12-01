## 常用命令（保持与 CI 一致）
- 依赖安装：按需安装 `cargo-llvm-cov`（覆盖率）与 `rustup component add llvm-tools-preview`
- 格式化：`cargo fmt --all`
- Lint：`cargo clippy --all-targets --all-features -- -D warnings`
- 测试：`cargo test --all-targets --all-features`（部分测试会访问 OKX 公共接口，网络不可达时仅打印警告且测试已容错）
- 覆盖率：`cargo coverage`（alias -> llvm-cov；排除 okx-py，生成 target/lcov.info）；需 doctest 时用 `cargo +nightly coverage --doctests`
- 构建：`cargo build --all-features`
- Python 绑定（如涉及）：`cd crates/okx-py && maturin develop`，测试 `pytest tests/`