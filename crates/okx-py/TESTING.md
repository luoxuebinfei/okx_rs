# okx-py 测试与类型校验

本文件面向开发者，说明如何在本仓库中运行 `okx-py` 的 Python 测试与类型存根一致性校验。

## 测试结构

当前测试文件结构：

```
tests/
├── conftest.py
├── test_api_surface.py        # API 表面一致性（同步/异步/WS 方法存在性）
├── test_clients.py            # 客户端导出、基础错误路径
├── test_new_bindings.py       # 新增绑定方法与签名校验
├── test_rest_coverage.py      # Python 绑定与 Rust trait 覆盖一致性
├── test_types.py              # 基础类型（Credentials/Config/数据类型）校验
├── test_validation.py         # JSON 入参校验（失败路径）
└── integration/
    ├── conftest.py
    ├── test_rest_private.py   # 需要真实凭证/网络（默认跳过）
    ├── test_rest_public.py    # 需要网络（默认跳过）
    └── test_ws_public.py      # 需要网络（默认跳过）
```

## 运行测试（推荐）

项目已在根目录 `justfile` 中提供统一入口，并将 uv 缓存固定到仓库内（避免写入 `$HOME`）：

```bash
# 1) 同步 Python 开发依赖（需要网络；已安装过可跳过）
just py-setup

# 2) 构建并以 editable 方式安装扩展，然后运行测试
just py-test

# 更详细输出
just py-test-verbose
```

说明：
- `just py-test` 会先执行 `maturin develop` 再执行 `pytest`，对 CI/本地行为保持一致。
- 集成测试（`tests/integration/`）在无网络或无真实凭证时会自动跳过，不影响常规回归。
- WSL2/资源受限环境的编译参数已集中在 `.cargo/config.toml`（如 `jobs = 1`、`incremental = true`）。

## 类型存根（.pyi）与一致性校验

类型存根位于：
- `crates/okx-py/python/okx_py/okx_py.pyi`
- `crates/okx-py/python/okx_py/py.typed`

当 Rust 侧新增/调整对外方法（尤其是 PyO3 `#[pymethods]`）后，应确保 `.pyi` 与运行时导出一致。

推荐校验步骤：

```bash
# 需要 mypy（通常来自 dev 依赖）
just py-setup
just py-build

cd crates/okx-py
UV_CACHE_DIR=../../.uv-cache XDG_CACHE_HOME=../../.cache MYPYPATH=python \\
  uv run --no-build-isolation --no-sync python -m mypy.stubtest okx_py.okx_py
```

## 常见问题

### 1) maturin 提示缺少 patchelf

如果看到：
`Failed to set rpath ... did you install it?`

一般不影响本地测试；若需要生成可分发的 wheel，可按 maturin 提示安装对应依赖（建议在隔离环境中处理）。

### 2) 网络受限导致 `uv sync` 失败

`just py-setup` 需要从 PyPI 同步依赖；在网络受限环境中，需要先确保网络可用或预热缓存后再执行。
