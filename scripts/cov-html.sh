#!/usr/bin/env bash

# 覆盖率一键流程（含 okx-py）
# - 默认目标目录：target/llvm-cov-target
# - 默认 Python 覆盖子集：数据类型/错误处理用例，可通过 PYTEST_ARGS 覆盖为完整 pytest
# - 依赖：已创建 .venv 并安装 maturin、pytest（执行 just py-setup 后满足）

set -euo pipefail

ROOT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)"
TARGET_DIR="${TARGET_DIR:-${ROOT_DIR}/target/llvm-cov-target}"
UV_CACHE_DIR="${UV_CACHE_DIR:-${ROOT_DIR}/.uv-cache}"
PYTEST_ARGS="${PYTEST_ARGS:-crates/okx-py/tests/test_data_types.py crates/okx-py/tests/test_types.py crates/okx-py/tests/test_errors.py}"
RUN_PYTEST="${RUN_PYTEST:-0}"
MATURIN_BIN="${MATURIN:-${ROOT_DIR}/.venv/bin/maturin}"
PYTEST_BIN="${PYTEST:-${ROOT_DIR}/.venv/bin/pytest}"
PYTHON_BIN="${PYTHON:-${ROOT_DIR}/.venv/bin/python}"

# 归一化为绝对路径，避免 profraw 写入失败
TARGET_DIR="$(realpath -m "${TARGET_DIR}")"
UV_CACHE_DIR="$(realpath -m "${UV_CACHE_DIR}")"

if [ ! -x "${MATURIN_BIN}" ]; then
  MATURIN_BIN="$(command -v maturin 2>/dev/null || true)"
fi
if [ -z "${MATURIN_BIN}" ] || [ ! -x "${MATURIN_BIN}" ]; then
  echo "未找到 maturin，请先执行 just py-setup 或设置 MATURIN 路径" >&2
  exit 127
fi

if [ ! -x "${PYTEST_BIN}" ]; then
  PYTEST_BIN="$(command -v pytest 2>/dev/null || true)"
fi
if [ -z "${PYTEST_BIN}" ] || [ ! -x "${PYTEST_BIN}" ]; then
  echo "未找到 pytest，请先执行 just py-setup 或设置 PYTEST 路径" >&2
  exit 127
fi

if [ ! -x "${PYTHON_BIN}" ]; then
  PYTHON_BIN="$(command -v python 2>/dev/null || true)"
fi
if [ -z "${PYTHON_BIN}" ] || [ ! -x "${PYTHON_BIN}" ]; then
  echo "未找到 python，请确保 .venv 已创建或设置 PYTHON 路径" >&2
  exit 127
fi

mkdir -p "${TARGET_DIR}" "${UV_CACHE_DIR}"
cd "${ROOT_DIR}"

echo "==> 清理旧覆盖数据"
CARGO_TARGET_DIR="${TARGET_DIR}" cargo llvm-cov clean --workspace

echo "==> 运行工作区 Rust 测试（插桩，不生成报告）"
CARGO_TARGET_DIR="${TARGET_DIR}" cargo llvm-cov --workspace --no-report

echo "==> 构建带覆盖率的 PyO3 扩展 (maturin develop)"
LLVM_PROFILE_FILE="${TARGET_DIR}/okx-py-build-%p-%m.profraw" \
CARGO_TARGET_DIR="${TARGET_DIR}" \
RUSTFLAGS="-Cinstrument-coverage" \
UV_CACHE_DIR="${UV_CACHE_DIR}" \
"${MATURIN_BIN}" develop -m crates/okx-py/Cargo.toml

if [ "${RUN_PYTEST}" = "1" ]; then
  echo "==> 运行 Python 覆盖用例 (${PYTEST_ARGS})"
  LLVM_PROFILE_FILE="${TARGET_DIR}/okx-py-pytest-%p-%m.profraw" \
  CARGO_TARGET_DIR="${TARGET_DIR}" \
  RUSTFLAGS="-Cinstrument-coverage" \
  "${PYTEST_BIN}" ${PYTEST_ARGS}
else
  echo "==> 运行 Python 覆盖 smoke（设置 RUN_PYTEST=1 可改为 pytest）"
  LLVM_PROFILE_FILE="${TARGET_DIR}/okx-py-smoke-%p-%m.profraw" \
  CARGO_TARGET_DIR="${TARGET_DIR}" \
  RUSTFLAGS="-Cinstrument-coverage" \
  "${PYTHON_BIN}" - <<'PY'
import asyncio
from okx_py import Config, Credentials, OkxClient, AsyncOkxClient

creds = Credentials("k", "s", "p")
cfg = Config(creds, simulated=True, timeout_secs=1)
try:
    cfg.rest_url = "http://127.0.0.1:9"
except Exception:
    pass

cli = OkxClient(cfg)
for fn, kwargs in [
    (cli.get_balance, {}),
    (cli.get_positions, {}),
    (cli.get_order, {"inst_id": "BTC-USDT", "ord_id": "1"}),
]:
    try:
        fn(**kwargs)
    except Exception:
        pass

async def main():
    acli = AsyncOkxClient(cfg)
    for coro in [
        acli.get_system_time(),
        acli.get_instruments("SPOT"),
        acli.get_ticker("BTC-USDT"),
    ]:
        try:
            await asyncio.wait_for(coro, timeout=2.0)
        except Exception:
            pass

asyncio.run(main())
PY
fi

echo "==> 生成 HTML 覆盖报告"
CARGO_TARGET_DIR="${TARGET_DIR}" cargo llvm-cov report --html --output-dir "${TARGET_DIR}/html"

echo "完成：${TARGET_DIR}/html/html/index.html"
