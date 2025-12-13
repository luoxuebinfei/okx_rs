"""测试 okx_py 包的导出与客户端基础行为。"""

from pathlib import Path

import pytest
import tomllib

import okx_py


def test_exports_and_version_matches_metadata():
    """验证导出符号与版本号与 Cargo 保持一致。"""
    cargo_toml = Path(__file__).resolve().parents[1] / "Cargo.toml"
    cargo_meta = tomllib.load(cargo_toml.open("rb"))

    assert okx_py.__version__ == cargo_meta["package"]["version"]
    for symbol in [
        "Credentials",
        "Config",
        "OkxClient",
        "AsyncOkxClient",
        "WsClient",
        "Balance",
        "BalanceDetail",
        "Position",
        "Order",
        "Ticker",
    ]:
        assert symbol in okx_py.__all__, f"{symbol} 应包含在 __all__ 中"


def test_sync_client_rejects_invalid_json(sync_client):
    """同步客户端在解析无效 JSON 时应立即报错，不触发网络请求。"""
    with pytest.raises(ValueError) as excinfo:
        sync_client.get_taker_volume("{oops")
    assert "params JSON 解析失败" in str(excinfo.value)

    with pytest.raises(ValueError) as excinfo:
        sync_client.get_simulated_margin("SWAP", None, None, "invalid-json")
    assert "sim_pos JSON 数组解析失败" in str(excinfo.value)


def test_async_client_rejects_invalid_json(async_client):
    """异步客户端也应对无效 JSON 入参进行前置校验。"""
    with pytest.raises(ValueError) as excinfo:
        async_client.get_taker_volume("{oops")
    assert "params JSON 解析失败" in str(excinfo.value)

    with pytest.raises(ValueError) as excinfo:
        async_client.get_simulated_margin("SWAP", None, None, "invalid-json")
    assert "sim_pos JSON 数组解析失败" in str(excinfo.value)
