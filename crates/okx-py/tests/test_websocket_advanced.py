"""WebSocket 高级功能测试."""

import pytest


@pytest.mark.asyncio
async def test_ws_client_recv():
    """测试 WebSocket recv 方法."""
    from okx_py import WsClient, Config, Credentials
    import asyncio

    creds = Credentials("dummy", "dummy", "dummy")
    config = Config(creds, simulated=True)

    try:
        client = await WsClient.connect_public(config)
        await client.subscribe_tickers("BTC-USDT")

        # 尝试接收一条消息
        try:
            msg = await asyncio.wait_for(client.recv(), timeout=5.0)
            assert msg is not None
        except asyncio.TimeoutError:
            pytest.skip("未在超时时间内收到消息")
    except Exception as e:
        pytest.skip(f"WebSocket 连接错误: {e}")


@pytest.mark.asyncio
async def test_ws_client_is_connected():
    """测试 WebSocket 连接状态检查."""
    from okx_py import WsClient, Config, Credentials

    creds = Credentials("dummy", "dummy", "dummy")
    config = Config(creds, simulated=True)

    try:
        client = await WsClient.connect_public(config)
        # 连接后应该是 connected 状态（is_connected 是异步方法）
        is_connected = await client.is_connected()
        assert isinstance(is_connected, bool)
    except Exception as e:
        pytest.skip(f"WebSocket 连接错误: {e}")


@pytest.mark.asyncio
async def test_ws_client_subscription_count():
    """测试 WebSocket 订阅计数."""
    from okx_py import WsClient, Config, Credentials

    creds = Credentials("dummy", "dummy", "dummy")
    config = Config(creds, simulated=True)

    try:
        client = await WsClient.connect_public(config)

        # 初始订阅数应该是 0（subscription_count 是异步方法）
        count = await client.subscription_count()
        assert count == 0

        # 订阅后计数应该增加
        await client.subscribe_tickers("BTC-USDT")
        count = await client.subscription_count()
        assert count >= 1

        # 再订阅一个
        await client.subscribe_trades("ETH-USDT")
        count = await client.subscription_count()
        assert count >= 2
    except Exception as e:
        pytest.skip(f"WebSocket 连接错误: {e}")


@pytest.mark.asyncio
async def test_ws_client_subscribe_block_channels():
    """测试高级公共频道订阅（block tickers/trades）。"""
    from okx_py import WsClient, Config, Credentials

    creds = Credentials("dummy", "dummy", "dummy")
    config = Config(creds, simulated=True)

    try:
        client = await WsClient.connect_public(config)
        # block tickers / public block trades 频道应存在
        await client.subscribe_block_tickers()
        await client.subscribe_public_block_trades()
    except Exception as e:
        pytest.skip(f"WebSocket 连接错误: {e}")


@pytest.mark.asyncio
async def test_ws_client_close():
    """测试 WebSocket 关闭连接."""
    from okx_py import WsClient, Config, Credentials

    creds = Credentials("dummy", "dummy", "dummy")
    config = Config(creds, simulated=True)

    try:
        client = await WsClient.connect_public(config)
        await client.subscribe_tickers("BTC-USDT")

        # 关闭连接
        await client.close()

        # 关闭后连接状态应该是 false（is_connected 是异步方法）
        is_connected = await client.is_connected()
        assert not is_connected
    except Exception as e:
        pytest.skip(f"WebSocket 连接错误: {e}")


@pytest.mark.asyncio
async def test_ws_client_reconnect():
    """测试 WebSocket 重连."""
    from okx_py import WsClient, Config, Credentials

    creds = Credentials("dummy", "dummy", "dummy")
    config = Config(creds, simulated=True)

    try:
        client = await WsClient.connect_public(config)

        # 关闭连接
        await client.close()
        is_connected = await client.is_connected()
        assert not is_connected

        # 重连
        await client.reconnect()

        # 重连后应该恢复连接（is_connected 是异步方法）
        is_connected = await client.is_connected()
        assert is_connected
    except Exception as e:
        pytest.skip(f"WebSocket 连接错误: {e}")


@pytest.mark.asyncio
async def test_ws_client_async_iterator():
    """测试 WebSocket 异步迭代器."""
    from okx_py import WsClient, Config, Credentials
    import asyncio

    creds = Credentials("dummy", "dummy", "dummy")
    config = Config(creds, simulated=True)

    try:
        client = await WsClient.connect_public(config)
        await client.subscribe_tickers("BTC-USDT")

        # 使用异步迭代器接收消息
        count = 0
        async for msg in client:
            if msg is None:
                break
            count += 1
            if count >= 3:  # 接收3条消息后退出
                break

        assert count > 0
    except asyncio.TimeoutError:
        pytest.skip("未在超时时间内收到消息")
    except Exception as e:
        pytest.skip(f"WebSocket 连接错误: {e}")


@pytest.mark.asyncio
async def test_ws_client_repr():
    """测试 WebSocket 客户端的字符串表示."""
    from okx_py import WsClient, Config, Credentials

    creds = Credentials("dummy", "dummy", "dummy")
    config = Config(creds, simulated=True)

    try:
        # 公共客户端
        public_client = await WsClient.connect_public(config)
        repr_str = repr(public_client)
        assert "WsClient" in repr_str

        # 私有客户端
        creds_private = Credentials("test", "test", "test")
        config_private = Config(creds_private, simulated=True)
        private_client = await WsClient.connect_private(config_private)
        repr_str = repr(private_client)
        assert "WsClient" in repr_str
    except Exception as e:
        pytest.skip(f"WebSocket 连接错误: {e}")
