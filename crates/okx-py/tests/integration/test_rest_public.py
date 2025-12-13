"""REST 公共接口集成测试。"""

import pytest

pytestmark = pytest.mark.integration


def test_get_system_time(public_client):
    """获取服务器时间应返回数字字符串；网络不可用则跳过。"""
    try:
        ts = public_client.get_system_time()
    except Exception as exc:  # noqa: BLE001
        pytest.skip(f"公共接口不可用: {exc}")
    assert ts and ts.isdigit()


def test_get_tickers_spot(public_client):
    """获取现货 Ticker 列表，应包含 BTC-USDT；网络不可用则跳过。"""
    try:
        tickers = public_client.get_tickers("SPOT")
    except Exception as exc:  # noqa: BLE001
        pytest.skip(f"公共接口不可用: {exc}")
    assert isinstance(tickers, list)
    assert any(getattr(t, "inst_id", None) == "BTC-USDT" for t in tickers)
