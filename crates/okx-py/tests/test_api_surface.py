"""测试公开 API 面向 Python 的表面契约。"""

import asyncio
import inspect

import okx_py


def test_sync_client_has_expected_methods(sync_client):
    """同步客户端应暴露核心 REST 方法，防止接口回退。"""
    expected = [
        "get_ticker",
        "get_balance",
        "get_account_config",
        "get_instruments",
        "get_candles",
        "get_funding_rate",
    ]
    for name in expected:
        assert hasattr(sync_client, name), f"缺少同步方法 {name}"
        assert callable(getattr(sync_client, name))


def test_async_client_has_expected_methods(async_client):
    """异步客户端应暴露核心 REST 方法。"""
    expected = [
        "get_ticker",
        "get_balance",
        "get_account_config",
        "get_instruments",
        "get_candles",
        "get_funding_rate",
    ]
    for name in expected:
        assert hasattr(async_client, name), f"缺少异步方法 {name}"
        assert callable(getattr(async_client, name))


def test_ws_client_has_subscription_methods():
    """WsClient 类应包含主要订阅与控制接口（不建立真实连接）。"""
    cls = okx_py.WsClient
    static_expected = ["connect_public", "connect_private"]
    instance_expected = [
        "subscribe_tickers",
        "subscribe_orderbook",
        "subscribe_trades",
        "subscribe_candles",
        "subscribe_account",
        "subscribe_positions",
        "subscribe_orders",
        "subscribe_orders_algo",
        "subscribe_balance_and_position",
        "recv",
        "is_connected",
        "reconnect",
        "close",
        "subscription_count",
        "__aiter__",
        "__anext__",
    ]

    for name in static_expected:
        assert hasattr(cls, name), f"缺少静态方法 {name}"

    for name in instance_expected:
        assert hasattr(cls, name), f"缺少实例方法 {name}"
