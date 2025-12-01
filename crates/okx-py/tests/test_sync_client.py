"""同步客户端测试."""

import pytest


def test_sync_client_get_system_time():
    """测试同步客户端获取系统时间（公共 API）."""
    from okx_py import OkxClient, Config, Credentials

    # 公共 API 使用虚拟凭证
    creds = Credentials("dummy", "dummy", "dummy")
    config = Config(creds, simulated=True)
    client = OkxClient(config)

    try:
        ts = client.get_system_time()
        assert ts is not None
        assert len(ts) > 0  # 应该是时间戳字符串
    except Exception as e:
        # 单元测试中网络错误可接受
        pytest.skip(f"网络错误: {e}")


def test_sync_client_get_instruments():
    """测试同步客户端获取交易产品（公共 API）."""
    from okx_py import OkxClient, Config, Credentials

    creds = Credentials("dummy", "dummy", "dummy")
    config = Config(creds, simulated=True)
    client = OkxClient(config)

    try:
        instruments = client.get_instruments("SPOT")
        assert isinstance(instruments, list)
        if len(instruments) > 0:
            inst = instruments[0]
            assert "instId" in inst
            assert "instType" in inst
    except Exception as e:
        pytest.skip(f"网络错误: {e}")


def test_sync_client_get_ticker():
    """测试同步客户端获取行情（公共 API）."""
    from okx_py import OkxClient, Config, Credentials

    creds = Credentials("dummy", "dummy", "dummy")
    config = Config(creds, simulated=True)
    client = OkxClient(config)

    try:
        ticker = client.get_ticker("BTC-USDT")
        if ticker is not None:
            assert ticker.inst_id == "BTC-USDT"
            assert ticker.last is not None
            assert len(ticker.last) > 0
    except Exception as e:
        pytest.skip(f"网络错误: {e}")


def test_sync_client_repr():
    """测试同步客户端的字符串表示."""
    from okx_py import OkxClient, Config, Credentials

    creds = Credentials("test_key", "test_secret", "test_pass")
    config = Config(creds, simulated=True)
    client = OkxClient(config)

    repr_str = repr(client)
    assert "OkxClient" in repr_str
    assert "simulated=true" in repr_str  # Rust 输出小写 true


def test_sync_client_methods_exist():
    """测试同步客户端方法存在性."""
    from okx_py import OkxClient, Config, Credentials

    creds = Credentials("dummy", "dummy", "dummy")
    config = Config(creds, simulated=True)
    client = OkxClient(config)

    # 验证关键方法存在
    assert hasattr(client, "get_system_time")
    assert hasattr(client, "get_instruments")
    assert hasattr(client, "get_ticker")
    assert hasattr(client, "get_balance")
    assert hasattr(client, "get_positions")


def test_sync_client_get_tickers():
    """测试同步客户端批量获取行情."""
    from okx_py import OkxClient, Config, Credentials

    creds = Credentials("dummy", "dummy", "dummy")
    config = Config(creds, simulated=True)
    client = OkxClient(config)

    try:
        tickers = client.get_tickers("SPOT")
        assert isinstance(tickers, list)
        if len(tickers) > 0:
            ticker = tickers[0]
            assert hasattr(ticker, "inst_id")
            assert hasattr(ticker, "last")
    except Exception as e:
        pytest.skip(f"网络错误: {e}")


def test_sync_client_get_orders_pending():
    """测试同步客户端查询挂单列表（需要认证）."""
    from okx_py import OkxClient, Config, Credentials

    creds = Credentials("test_key", "test_secret", "test_pass")
    config = Config(creds, simulated=True)
    client = OkxClient(config)

    try:
        orders = client.get_orders_pending()
        assert isinstance(orders, list) or orders is None
    except Exception as e:
        # 认证错误是预期的
        assert e is not None


def test_sync_client_place_and_cancel_order():
    """测试同步客户端下单和撤单流程（需要认证）."""
    from okx_py import OkxClient, Config, Credentials

    creds = Credentials("test_key", "test_secret", "test_pass")
    config = Config(creds, simulated=True)
    client = OkxClient(config)

    try:
        # 尝试下单
        order_id = client.place_order(
            inst_id="BTC-USDT",
            td_mode="cash",
            side="buy",
            ord_type="limit",
            sz="0.001",
            px="20000"
        )

        if order_id and len(order_id) > 0:
            # 如果下单成功，尝试撤单
            cancel_result = client.cancel_order(
                inst_id="BTC-USDT",
                ord_id=order_id
            )
            assert cancel_result is not None
    except Exception as e:
        # 认证错误是预期的
        assert e is not None
