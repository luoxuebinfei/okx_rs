"""集成测试公共夹具与开关。"""

import os
import socket

import pytest

import okx_py


SIMULATED_DEFAULT = os.getenv("OKX_SIMULATED", "true").lower() == "true"


def _ensure_okx_reachable():
    """快速探测 OKX 域名是否可连，不可连则跳过。"""
    try:
        with socket.create_connection(("www.okx.com", 443), timeout=2):
            return
    except OSError as exc:
        pytest.skip(f"网络不可用，跳过集成测试: {exc}")


def _build_config(api_key: str, secret_key: str, passphrase: str) -> okx_py.Config:
    creds = okx_py.Credentials(api_key, secret_key, passphrase)
    return okx_py.Config(creds, simulated=SIMULATED_DEFAULT, timeout_secs=5)


@pytest.fixture
def public_client():
    """公共 REST 客户端（默认模拟环境，使用占位凭证）。"""
    _ensure_okx_reachable()
    return okx_py.OkxClient(_build_config("public_key", "public_secret", "public_pass"))


@pytest.fixture
def private_client():
    """需要真实凭证的私有 REST 客户端，凭证缺失即跳过。"""
    _ensure_okx_reachable()
    required = {k: os.getenv(k) for k in ["OKX_API_KEY", "OKX_SECRET_KEY", "OKX_PASSPHRASE"]}
    if not all(required.values()):
        pytest.skip("缺少 OKX_API_KEY/OKX_SECRET_KEY/OKX_PASSPHRASE，跳过私有集成测试")
    return okx_py.OkxClient(
        _build_config(required["OKX_API_KEY"], required["OKX_SECRET_KEY"], required["OKX_PASSPHRASE"])
    )


@pytest.fixture
def ws_config():
    """WebSocket 使用的基础配置。"""
    _ensure_okx_reachable()
    return _build_config("ws_key", "ws_secret", "ws_pass")
