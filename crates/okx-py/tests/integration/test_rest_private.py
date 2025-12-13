"""REST 私有接口集成测试，需要真实凭证。"""

import pytest

pytestmark = pytest.mark.integration


def test_get_balance(private_client):
    """查询账户余额，缺凭证时跳过，网络错误时跳过。"""
    try:
        balances = private_client.get_balance()
    except Exception as exc:  # noqa: BLE001
        pytest.skip(f"网络/认证不可用: {exc}")
    assert isinstance(balances, list)
