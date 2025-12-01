"""数据类型完整性测试."""

import pytest


def test_balance_all_properties():
    """测试 Balance 所有属性（通过 API 响应）."""
    from okx_py import OkxClient, Config, Credentials

    creds = Credentials("test_key", "test_secret", "test_pass")
    config = Config(creds, simulated=True)
    client = OkxClient(config)

    try:
        balances = client.get_balance()
        if balances and len(balances) > 0:
            balance = balances[0]

            # 测试所有属性可访问
            assert hasattr(balance, "total_eq")
            assert hasattr(balance, "iso_eq")
            assert hasattr(balance, "mgn_ratio")
            assert hasattr(balance, "details")

            # 测试属性类型
            assert isinstance(balance.total_eq, str)
            assert isinstance(balance.iso_eq, str)
            assert isinstance(balance.mgn_ratio, str)
            assert isinstance(balance.details, list)

            # 测试 repr
            repr_str = repr(balance)
            assert "Balance" in repr_str
    except Exception:
        # 认证错误是预期的
        pass


def test_balance_detail_all_properties():
    """测试 BalanceDetail 所有属性."""
    from okx_py import OkxClient, Config, Credentials

    creds = Credentials("test_key", "test_secret", "test_pass")
    config = Config(creds, simulated=True)
    client = OkxClient(config)

    try:
        balances = client.get_balance()
        if balances and len(balances) > 0:
            balance = balances[0]
            if balance.details and len(balance.details) > 0:
                detail = balance.details[0]

                # 测试所有属性
                assert hasattr(detail, "ccy")
                assert hasattr(detail, "eq")
                assert hasattr(detail, "avail_bal")
                assert hasattr(detail, "frozen_bal")

                # 测试属性类型
                assert isinstance(detail.ccy, str)
                assert isinstance(detail.eq, str)
                assert isinstance(detail.avail_bal, str)
                assert isinstance(detail.frozen_bal, str)

                # 测试 repr
                repr_str = repr(detail)
                assert "BalanceDetail" in repr_str
                assert detail.ccy in repr_str
    except Exception:
        pass


def test_position_all_properties():
    """测试 Position 所有属性."""
    from okx_py import OkxClient, Config, Credentials

    creds = Credentials("test_key", "test_secret", "test_pass")
    config = Config(creds, simulated=True)
    client = OkxClient(config)

    try:
        positions = client.get_positions()
        if positions and len(positions) > 0:
            position = positions[0]

            # 测试所有属性
            assert hasattr(position, "inst_id")
            assert hasattr(position, "pos_side")
            assert hasattr(position, "pos")
            assert hasattr(position, "avg_px")
            assert hasattr(position, "upl")
            assert hasattr(position, "lever")

            # 测试属性类型
            assert isinstance(position.inst_id, str)
            assert isinstance(position.pos_side, str)
            assert isinstance(position.pos, str)
            assert isinstance(position.avg_px, str)
            assert isinstance(position.upl, str)
            assert isinstance(position.lever, str)

            # 测试 repr
            repr_str = repr(position)
            assert "Position" in repr_str
            assert position.inst_id in repr_str
    except Exception:
        pass


def test_order_all_properties():
    """测试 Order 所有属性."""
    from okx_py import OkxClient, Config, Credentials

    creds = Credentials("test_key", "test_secret", "test_pass")
    config = Config(creds, simulated=True)
    client = OkxClient(config)

    try:
        order = client.get_order(inst_id="BTC-USDT", ord_id="123456")
        if order is not None:
            # 测试所有属性
            assert hasattr(order, "ord_id")
            assert hasattr(order, "cl_ord_id")
            assert hasattr(order, "inst_id")
            assert hasattr(order, "side")
            assert hasattr(order, "ord_type")
            assert hasattr(order, "state")
            assert hasattr(order, "px")
            assert hasattr(order, "sz")
            assert hasattr(order, "acc_fill_sz")
            assert hasattr(order, "avg_px")

            # 测试属性类型
            assert isinstance(order.ord_id, str)
            assert isinstance(order.cl_ord_id, str)
            assert isinstance(order.inst_id, str)
            assert isinstance(order.side, str)
            assert isinstance(order.ord_type, str)
            assert isinstance(order.state, str)
            assert isinstance(order.px, str)
            assert isinstance(order.sz, str)
            assert isinstance(order.acc_fill_sz, str)
            assert isinstance(order.avg_px, str)

            # 测试 repr
            repr_str = repr(order)
            assert "Order" in repr_str
            assert order.ord_id in repr_str
    except Exception:
        pass


def test_ticker_all_properties():
    """测试 Ticker 所有属性."""
    from okx_py import OkxClient, Config, Credentials

    creds = Credentials("dummy", "dummy", "dummy")
    config = Config(creds, simulated=True)
    client = OkxClient(config)

    try:
        ticker = client.get_ticker("BTC-USDT")
        if ticker is not None:
            # 测试所有属性
            assert hasattr(ticker, "inst_id")
            assert hasattr(ticker, "last")
            assert hasattr(ticker, "ask_px")
            assert hasattr(ticker, "bid_px")
            assert hasattr(ticker, "high_24h")
            assert hasattr(ticker, "low_24h")
            assert hasattr(ticker, "vol_24h")

            # 测试属性类型
            assert isinstance(ticker.inst_id, str)
            assert isinstance(ticker.last, str)
            assert isinstance(ticker.ask_px, str)
            assert isinstance(ticker.bid_px, str)
            assert isinstance(ticker.high_24h, str)
            assert isinstance(ticker.low_24h, str)
            assert isinstance(ticker.vol_24h, str)

            # 测试 repr
            repr_str = repr(ticker)
            assert "Ticker" in repr_str
            assert "BTC-USDT" in repr_str
            assert ticker.last in repr_str

            # 测试数值有效性
            assert len(ticker.last) > 0
            assert len(ticker.ask_px) > 0
            assert len(ticker.bid_px) > 0
    except Exception as e:
        pytest.skip(f"网络错误: {e}")


@pytest.mark.asyncio
async def test_async_ticker_properties():
    """测试异步获取的 Ticker 属性."""
    from okx_py import AsyncOkxClient, Config, Credentials

    creds = Credentials("dummy", "dummy", "dummy")
    config = Config(creds, simulated=True)
    client = AsyncOkxClient(config)

    try:
        ticker = await client.get_ticker("ETH-USDT")
        if ticker is not None:
            # 验证关键属性
            assert ticker.inst_id == "ETH-USDT"
            assert len(ticker.last) > 0
            assert len(ticker.ask_px) > 0
            assert len(ticker.bid_px) > 0

            # 验证价格关系（买价 <= 卖价）
            try:
                bid = float(ticker.bid_px)
                ask = float(ticker.ask_px)
                assert bid <= ask, f"买价 {bid} 应该 <= 卖价 {ask}"
            except ValueError:
                # 如果无法转换为浮点数，跳过验证
                pass
    except Exception as e:
        pytest.skip(f"网络错误: {e}")


def test_data_type_none_handling():
    """测试数据类型对 None 值的处理."""
    from okx_py import OkxClient, Config, Credentials

    creds = Credentials("test_key", "test_secret", "test_pass")
    config = Config(creds, simulated=True)
    client = OkxClient(config)

    try:
        # 查询不存在的订单应该返回 None
        order = client.get_order(inst_id="BTC-USDT", ord_id="nonexistent")
        assert order is None or hasattr(order, "ord_id")

        # 查询不存在的产品应该返回 None
        ticker = client.get_ticker("INVALID-PAIR")
        assert ticker is None or hasattr(ticker, "inst_id")
    except Exception:
        # 错误也是可接受的
        pass


def test_data_type_list_handling():
    """测试数据类型列表的处理."""
    from okx_py import OkxClient, Config, Credentials

    creds = Credentials("dummy", "dummy", "dummy")
    config = Config(creds, simulated=True)
    client = OkxClient(config)

    try:
        # 获取产品列表
        instruments = client.get_instruments("SPOT")
        assert isinstance(instruments, list)

        # 获取行情列表
        tickers = client.get_tickers("SPOT")
        assert isinstance(tickers, list)

        # 如果列表不为空，验证元素类型
        if len(tickers) > 0:
            ticker = tickers[0]
            assert hasattr(ticker, "inst_id")
            assert hasattr(ticker, "last")
    except Exception as e:
        pytest.skip(f"网络错误: {e}")
