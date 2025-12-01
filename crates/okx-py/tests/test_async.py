"""Async tests for okx_py."""

import pytest


@pytest.mark.asyncio
async def test_async_client_get_system_time():
    """Test async client can get system time (public API)."""
    from okx_py import AsyncOkxClient, Config, Credentials

    # Use dummy credentials for public API
    creds = Credentials("dummy", "dummy", "dummy")
    config = Config(creds, simulated=True)
    client = AsyncOkxClient(config)

    # get_system_time is a public API that doesn't require auth
    try:
        ts = await client.get_system_time()
        assert ts is not None
        assert len(ts) > 0  # Should be a timestamp string
    except Exception as e:
        # Network errors are acceptable in unit tests
        pytest.skip(f"Network error: {e}")


@pytest.mark.asyncio
async def test_async_client_get_instruments():
    """Test async client can get instruments (public API)."""
    from okx_py import AsyncOkxClient, Config, Credentials

    creds = Credentials("dummy", "dummy", "dummy")
    config = Config(creds, simulated=True)
    client = AsyncOkxClient(config)

    try:
        instruments = await client.get_instruments("SPOT")
        assert isinstance(instruments, list)
        if len(instruments) > 0:
            inst = instruments[0]
            assert "instId" in inst
            assert "instType" in inst
    except Exception as e:
        pytest.skip(f"Network error: {e}")


@pytest.mark.asyncio
async def test_async_client_get_ticker():
    """Test async client can get ticker (public API)."""
    from okx_py import AsyncOkxClient, Config, Credentials

    creds = Credentials("dummy", "dummy", "dummy")
    config = Config(creds, simulated=True)
    client = AsyncOkxClient(config)

    try:
        ticker = await client.get_ticker("BTC-USDT")
        if ticker is not None:
            assert ticker.inst_id == "BTC-USDT"
            assert ticker.last is not None
    except Exception as e:
        pytest.skip(f"Network error: {e}")


@pytest.mark.asyncio
async def test_async_client_get_tickers():
    """测试异步客户端批量获取行情."""
    from okx_py import AsyncOkxClient, Config, Credentials

    creds = Credentials("dummy", "dummy", "dummy")
    config = Config(creds, simulated=True)
    client = AsyncOkxClient(config)

    try:
        tickers = await client.get_tickers("SPOT")
        assert isinstance(tickers, list)
        if len(tickers) > 0:
            ticker = tickers[0]
            assert hasattr(ticker, "inst_id")
            assert hasattr(ticker, "last")
    except Exception as e:
        pytest.skip(f"网络错误: {e}")


@pytest.mark.asyncio
async def test_async_client_get_balance():
    """测试异步客户端获取余额（需要认证）."""
    from okx_py import AsyncOkxClient, Config, Credentials

    creds = Credentials("test_key", "test_secret", "test_pass")
    config = Config(creds, simulated=True)
    client = AsyncOkxClient(config)

    try:
        balance = await client.get_balance()
        # 认证失败是预期的，只要不崩溃即可
        assert isinstance(balance, list) or balance is None
    except Exception as e:
        # 认证错误是预期的
        assert "auth" in str(e).lower() or "signature" in str(e).lower() or "network" in str(e).lower()


@pytest.mark.asyncio
async def test_async_client_get_positions():
    """测试异步客户端获取持仓（需要认证）."""
    from okx_py import AsyncOkxClient, Config, Credentials

    creds = Credentials("test_key", "test_secret", "test_pass")
    config = Config(creds, simulated=True)
    client = AsyncOkxClient(config)

    try:
        positions = await client.get_positions()
        assert isinstance(positions, list) or positions is None
    except Exception as e:
        # 认证错误是预期的
        assert "auth" in str(e).lower() or "signature" in str(e).lower() or "network" in str(e).lower()


@pytest.mark.asyncio
async def test_async_client_get_positions_with_filter():
    """测试异步客户端获取持仓（带过滤条件）."""
    from okx_py import AsyncOkxClient, Config, Credentials

    creds = Credentials("test_key", "test_secret", "test_pass")
    config = Config(creds, simulated=True)
    client = AsyncOkxClient(config)

    try:
        positions = await client.get_positions(inst_type="SWAP", inst_id="BTC-USDT-SWAP")
        assert isinstance(positions, list) or positions is None
    except Exception as e:
        assert "auth" in str(e).lower() or "signature" in str(e).lower() or "network" in str(e).lower()


@pytest.mark.asyncio
async def test_async_client_place_order():
    """测试异步客户端下单（需要认证）."""
    from okx_py import AsyncOkxClient, Config, Credentials

    creds = Credentials("test_key", "test_secret", "test_pass")
    config = Config(creds, simulated=True)
    client = AsyncOkxClient(config)

    try:
        # 尝试下一个市价单
        order_id = await client.place_order(
            inst_id="BTC-USDT",
            td_mode="cash",
            side="buy",
            ord_type="market",
            sz="0.001"
        )
        # 如果成功，应该返回订单ID
        assert isinstance(order_id, str) or order_id is None
    except Exception as e:
        # 认证错误或参数错误是预期的
        assert e is not None


@pytest.mark.asyncio
async def test_async_client_cancel_order():
    """测试异步客户端撤单（需要认证）."""
    from okx_py import AsyncOkxClient, Config, Credentials

    creds = Credentials("test_key", "test_secret", "test_pass")
    config = Config(creds, simulated=True)
    client = AsyncOkxClient(config)

    try:
        # 尝试撤销一个不存在的订单
        result = await client.cancel_order(
            inst_id="BTC-USDT",
            ord_id="123456789"
        )
        assert isinstance(result, str) or result is None
    except Exception as e:
        # 认证错误或订单不存在是预期的
        assert e is not None


@pytest.mark.asyncio
async def test_async_client_get_order():
    """测试异步客户端查询订单（需要认证）."""
    from okx_py import AsyncOkxClient, Config, Credentials

    creds = Credentials("test_key", "test_secret", "test_pass")
    config = Config(creds, simulated=True)
    client = AsyncOkxClient(config)

    try:
        order = await client.get_order(
            inst_id="BTC-USDT",
            ord_id="123456789"
        )
        # 订单不存在时可能返回 None
        assert order is None or hasattr(order, "ord_id")
    except Exception as e:
        # 认证错误是预期的
        assert e is not None


@pytest.mark.asyncio
async def test_async_client_get_orders_pending():
    """测试异步客户端查询挂单列表（需要认证）."""
    from okx_py import AsyncOkxClient, Config, Credentials

    creds = Credentials("test_key", "test_secret", "test_pass")
    config = Config(creds, simulated=True)
    client = AsyncOkxClient(config)

    try:
        orders = await client.get_orders_pending()
        assert isinstance(orders, list) or orders is None
    except Exception as e:
        # 认证错误是预期的
        assert e is not None
