# OKX Rust SDK Justfile

# 说明：
# - 在 CI/沙盒（workspace-write）环境中，通常禁止写入 $HOME（例如 ~/.cache/uv）。
# - 这里将 uv 的缓存/临时目录固定到仓库内，保证 `just py-*` 可复现且不依赖宿主机目录权限。

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
    mkdir -p .uv-cache .cache
    cd crates/okx-py && UV_CACHE_DIR=../../.uv-cache XDG_CACHE_HOME=../../.cache uv run --no-build-isolation maturin develop

# 运行 Python 测试
py-test: py-build
    cd crates/okx-py && UV_CACHE_DIR=../../.uv-cache XDG_CACHE_HOME=../../.cache uv run --no-build-isolation pytest tests/ -v

# 运行 Python 测试（详细模式）
py-test-verbose: py-build
    cd crates/okx-py && UV_CACHE_DIR=../../.uv-cache XDG_CACHE_HOME=../../.cache uv run --no-build-isolation pytest tests/ -vv -s

# 运行特定的 Python 测试文件
py-test-file FILE: py-build
    cd crates/okx-py && UV_CACHE_DIR=../../.uv-cache XDG_CACHE_HOME=../../.cache uv run --no-build-isolation pytest tests/{{FILE}} -v

# 运行 Python 类型检查
py-typecheck:
    cd crates/okx-py && UV_CACHE_DIR=../../.uv-cache XDG_CACHE_HOME=../../.cache uv run --no-build-isolation mypy python/okx_py

# 校验 Python 类型存根与运行时一致性（需要先执行 `just py-setup` 安装 dev 依赖）
py-stubtest: py-build
    cd crates/okx-py && UV_CACHE_DIR=../../.uv-cache XDG_CACHE_HOME=../../.cache MYPYPATH=python uv run --no-build-isolation python -m mypy.stubtest okx_py.okx_py

# 清理构建产物
clean:
    cargo clean
    rm -rf crates/okx-py/target
    rm -rf crates/okx-py/.venv/lib/python*/site-packages/okx_py*

# 完整的 CI 检查
ci: fmt clippy test py-test
    @echo "所有检查通过！"

# 发布 Python 包（本地测试）
py-build-release:
    mkdir -p .uv-cache .cache
    cd crates/okx-py && UV_CACHE_DIR=../../.uv-cache XDG_CACHE_HOME=../../.cache uv run --no-build-isolation --no-sync maturin build --release

# 安装 Python 开发依赖
py-setup:
    mkdir -p .uv-cache .cache
    cd crates/okx-py && UV_CACHE_DIR=../../.uv-cache XDG_CACHE_HOME=../../.cache uv sync --dev

# 生成 HTML 覆盖率（仅 Rust 代码，WSL 友好）
cov-html:
    cargo llvm-cov --workspace --exclude okx-py --html --output-dir target/llvm-cov/html
    @echo "覆盖率报告: target/llvm-cov/html/index.html"

# 覆盖率（完整版，含 okx-py Python 绑定，需要稳定环境）
cov-html-with-py:
    #!/usr/bin/env bash
    set -euo pipefail
    PYTEST_ARGS="crates/okx-py/tests" bash scripts/cov-html.sh
