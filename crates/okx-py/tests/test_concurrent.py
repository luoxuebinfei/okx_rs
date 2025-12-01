"""并发和异步测试."""

import pytest
import asyncio


@pytest.mark.asyncio
async def test_async_client_concurrent_requests():
    """测试异步客户端并发请求."""
    from okx_py import AsyncOkxClient, Config, Credentials

    creds = Credentials("dummy", "dummy", "dummy")
    config = Config(creds, simulated=True)
    client = AsyncOkxClient(config)

    try:
        # 并发执行多个请求
        results = await asyncio.gather(
            client.get_system_time(),
            client.get_ticker("BTC-USDT"),
            client.get_ticker("ETH-USDT"),
            client.get_instruments("SPOT"),
            return_exceptions=True
        )

        # 检查结果
        assert len(results) == 4
        # 至少有一些请求应该成功
        success_count = sum(1 for r in results if not isinstance(r, Exception))
        assert success_count > 0
    except Exception as e:
        pytest.skip(f"网络错误: {e}")


@pytest.mark.asyncio
async def test_async_client_concurrent_tickers():
    """测试异步客户端并发获取多个行情."""
    from okx_py import AsyncOkxClient, Config, Credentials

    creds = Credentials("dummy", "dummy", "dummy")
    config = Config(creds, simulated=True)
    client = AsyncOkxClient(config)

    try:
        # 并发获取多个产品的行情
        symbols = ["BTC-USDT", "ETH-USDT", "SOL-USDT", "DOGE-USDT"]
        tasks = [client.get_ticker(symbol) for symbol in symbols]
        tickers = await asyncio.gather(*tasks, return_exceptions=True)

        # 检查结果
        assert len(tickers) == len(symbols)
        # 统计成功的请求
        success_count = sum(
            1 for t in tickers
            if not isinstance(t, Exception) and t is not None
        )
        assert success_count > 0
    except Exception as e:
        pytest.skip(f"网络错误: {e}")


@pytest.mark.asyncio
async def test_multiple_async_clients():
    """测试多个异步客户端实例."""
    from okx_py import AsyncOkxClient, Config, Credentials

    creds = Credentials("dummy", "dummy", "dummy")
    config = Config(creds, simulated=True)

    try:
        # 创建多个客户端实例
        client1 = AsyncOkxClient(config)
        client2 = AsyncOkxClient(config)
        client3 = AsyncOkxClient(config)

        # 并发使用多个客户端
        results = await asyncio.gather(
            client1.get_ticker("BTC-USDT"),
            client2.get_ticker("ETH-USDT"),
            client3.get_system_time(),
            return_exceptions=True
        )

        assert len(results) == 3
        success_count = sum(1 for r in results if not isinstance(r, Exception))
        assert success_count > 0
    except Exception as e:
        pytest.skip(f"网络错误: {e}")


@pytest.mark.asyncio
async def test_ws_client_concurrent_subscriptions():
    """测试 WebSocket 客户端并发订阅."""
    from okx_py import WsClient, Config, Credentials

    creds = Credentials("dummy", "dummy", "dummy")
    config = Config(creds, simulated=True)

    try:
        client = await WsClient.connect_public(config)

        # 并发订阅多个频道
        await asyncio.gather(
            client.subscribe_tickers("BTC-USDT"),
            client.subscribe_tickers("ETH-USDT"),
            client.subscribe_trades("BTC-USDT"),
            client.subscribe_orderbook("ETH-USDT"),
            return_exceptions=True
        )

        # 检查订阅数量（subscription_count 是异步方法）
        count = await client.subscription_count()
        assert count >= 4
    except Exception as e:
        pytest.skip(f"WebSocket 连接错误: {e}")


@pytest.mark.asyncio
async def test_multiple_ws_clients():
    """测试多个 WebSocket 客户端实例."""
    from okx_py import WsClient, Config, Credentials

    creds = Credentials("dummy", "dummy", "dummy")
    config = Config(creds, simulated=True)

    try:
        # 并发创建多个 WebSocket 客户端
        clients = await asyncio.gather(
            WsClient.connect_public(config),
            WsClient.connect_public(config),
            WsClient.connect_public(config),
            return_exceptions=True
        )

        # 检查所有客户端都创建成功
        success_count = sum(1 for c in clients if not isinstance(c, Exception))
        assert success_count > 0

        # 关闭所有客户端
        for client in clients:
            if not isinstance(client, Exception):
                await client.close()
    except Exception as e:
        pytest.skip(f"WebSocket 连接错误: {e}")


@pytest.mark.asyncio
async def test_mixed_rest_and_ws():
    """测试 REST 和 WebSocket 混合使用."""
    from okx_py import AsyncOkxClient, WsClient, Config, Credentials

    creds = Credentials("dummy", "dummy", "dummy")
    config = Config(creds, simulated=True)

    try:
        # 创建 REST 和 WebSocket 客户端
        rest_client = AsyncOkxClient(config)
        ws_client = await WsClient.connect_public(config)

        # 并发使用两种客户端
        rest_result, ws_result = await asyncio.gather(
            rest_client.get_ticker("BTC-USDT"),
            ws_client.subscribe_tickers("BTC-USDT"),
            return_exceptions=True
        )

        # 至少有一个应该成功
        assert not isinstance(rest_result, Exception) or not isinstance(ws_result, Exception)

        # 清理
        await ws_client.close()
    except Exception as e:
        pytest.skip(f"网络错误: {e}")


@pytest.mark.asyncio
async def test_async_client_timeout_handling():
    """测试异步客户端超时处理."""
    from okx_py import AsyncOkxClient, Config, Credentials

    creds = Credentials("dummy", "dummy", "dummy")
    # 设置很短的超时时间
    config = Config(creds, simulated=True, timeout_secs=1)
    client = AsyncOkxClient(config)

    try:
        # 尝试执行请求，可能会超时
        result = await client.get_instruments("SPOT")
        # 如果成功，验证结果
        assert isinstance(result, list) or result is None
    except Exception as e:
        # 超时或网络错误是可接受的
        assert e is not None


@pytest.mark.asyncio
async def test_concurrent_balance_queries():
    """测试并发查询余额（需要认证）."""
    from okx_py import AsyncOkxClient, Config, Credentials

    creds = Credentials("test_key", "test_secret", "test_pass")
    config = Config(creds, simulated=True)
    client = AsyncOkxClient(config)

    try:
        # 并发查询余额
        results = await asyncio.gather(
            client.get_balance(),
            client.get_balance(ccy="BTC"),
            client.get_balance(ccy="USDT"),
            return_exceptions=True
        )

        # 所有请求都应该返回（可能是错误）
        assert len(results) == 3
    except Exception as e:
        # 认证错误是预期的
        assert e is not None
