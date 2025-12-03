# OKX Rust SDK Justfile

# 默认命令：显示帮助
default:
    @just --list

# 构建所有 crate
build:
    cargo build --all

# 运行所有 Rust 测试（包括集成测试，可能需要网络连接）
test:
    cargo test --workspace

# 仅运行单元测试（快速，无网络请求，推荐用于 WSL）
test-unit:
    cargo test --workspace --lib

# 运行集成测试（需要网络连接）
test-integration:
    cargo test --workspace --test '*'

# 检查代码（快速）
check:
    cargo check --all

# 运行 clippy
clippy:
    cargo clippy --all -- -D warnings

# 格式化代码
fmt:
    cargo fmt --all

# 构建 Python 绑定
py-build:
    cd crates/okx-py && maturin develop

# 运行 Python 测试
py-test: py-build
    #!/usr/bin/env bash
    source .venv/bin/activate
    cd crates/okx-py
    pytest tests/ -v

# 运行 Python 测试（详细模式）
py-test-verbose: py-build
    #!/usr/bin/env bash
    source .venv/bin/activate
    cd crates/okx-py
    pytest tests/ -vv -s

# 运行特定的 Python 测试文件
py-test-file FILE: py-build
    #!/usr/bin/env bash
    source .venv/bin/activate
    cd crates/okx-py
    pytest tests/{{FILE}} -v

# 运行 Python 类型检查
py-typecheck:
    #!/usr/bin/env bash
    source .venv/bin/activate
    cd crates/okx-py
    mypy python/okx_py

# 清理构建产物
clean:
    cargo clean
    rm -rf crates/okx-py/target
    rm -rf .venv/lib/python*/site-packages/okx_py*

# 完整的 CI 检查
ci: fmt clippy test py-test
    @echo "所有检查通过！"

# 发布 Python 包（本地测试）
py-build-release:
    cd crates/okx-py && maturin build --release

# 安装 Python 开发依赖
py-setup:
    uv venv --python 3.12
    source .venv/bin/activate && cd crates/okx-py && uv pip install -e ".[dev]"

# 生成 HTML 覆盖率（仅 Rust 代码，WSL 友好）
cov-html:
    cargo llvm-cov --workspace --exclude okx-py --html --output-dir target/llvm-cov/html
    @echo "覆盖率报告: target/llvm-cov/html/index.html"

# 覆盖率（完整版，含 okx-py Python 绑定，需要稳定环境）
cov-html-with-py:
    #!/usr/bin/env bash
    set -euo pipefail
    PYTEST_ARGS="crates/okx-py/tests" bash scripts/cov-html.sh
