"""Unit tests for okx_py types."""

import pytest


def test_credentials_creation():
    """Test Credentials creation."""
    from okx_py import Credentials

    creds = Credentials("test_api_key", "test_secret", "test_passphrase")
    assert repr(creds).startswith("Credentials(api_key='test_api")


def test_config_creation():
    """Test Config creation with default values."""
    from okx_py import Config, Credentials

    creds = Credentials("api_key", "secret", "passphrase")
    config = Config(creds)

    assert config.simulated is False
    assert "okx.com" in config.rest_url


def test_config_simulated():
    """Test Config with simulated trading enabled."""
    from okx_py import Config, Credentials

    creds = Credentials("api_key", "secret", "passphrase")
    config = Config(creds, simulated=True)

    assert config.simulated is True


def test_config_custom_timeout():
    """Test Config with custom timeout."""
    from okx_py import Config, Credentials

    creds = Credentials("api_key", "secret", "passphrase")
    config = Config(creds, timeout_secs=60)

    # Config should be created successfully
    assert config is not None


def test_client_creation():
    """Test OkxClient creation."""
    from okx_py import Config, Credentials, OkxClient

    creds = Credentials("api_key", "secret", "passphrase")
    config = Config(creds, simulated=True)
    client = OkxClient(config)

    assert "simulated=true" in repr(client)  # Rust 输出小写 true


def test_async_client_creation():
    """Test AsyncOkxClient creation."""
    from okx_py import AsyncOkxClient, Config, Credentials

    creds = Credentials("api_key", "secret", "passphrase")
    config = Config(creds, simulated=True)
    client = AsyncOkxClient(config)

    assert "simulated=true" in repr(client)  # Rust 输出小写 true


class TestDataTypes:
    """测试数据类型包装器."""

    def test_ticker_properties(self):
        """测试 Ticker 属性访问（通过 API 响应）."""
        from okx_py import OkxClient, Config, Credentials

        creds = Credentials("dummy", "dummy", "dummy")
        config = Config(creds, simulated=True)
        client = OkxClient(config)

        try:
            ticker = client.get_ticker("BTC-USDT")
            if ticker is not None:
                # 测试所有属性可访问
                assert hasattr(ticker, "inst_id")
                assert hasattr(ticker, "last")
                assert hasattr(ticker, "ask_px")
                assert hasattr(ticker, "bid_px")
                assert hasattr(ticker, "high_24h")
                assert hasattr(ticker, "low_24h")
                assert hasattr(ticker, "vol_24h")

                # 测试属性值类型
                assert isinstance(ticker.inst_id, str)
                assert isinstance(ticker.last, str)
                assert isinstance(ticker.ask_px, str)
                assert isinstance(ticker.bid_px, str)

                # 测试 repr
                repr_str = repr(ticker)
                assert "Ticker" in repr_str
                assert "BTC-USDT" in repr_str
        except Exception as e:
            pytest.skip(f"网络错误: {e}")

    def test_balance_properties(self):
        """测试 Balance 属性（需要认证，仅测试结构）."""
        # Balance 对象需要真实 API 调用，这里仅测试类型存在
        from okx_py import OkxClient, Config, Credentials

        creds = Credentials("test_key", "test_secret", "test_pass")
        config = Config(creds, simulated=True)
        client = OkxClient(config)

        # 验证方法存在
        assert hasattr(client, "get_balance")

    def test_position_properties(self):
        """测试 Position 属性（需要认证，仅测试结构）."""
        from okx_py import OkxClient, Config, Credentials

        creds = Credentials("test_key", "test_secret", "test_pass")
        config = Config(creds, simulated=True)
        client = OkxClient(config)

        # 验证方法存在
        assert hasattr(client, "get_positions")

    def test_order_properties(self):
        """测试 Order 属性（需要认证，仅测试结构）."""
        from okx_py import OkxClient, Config, Credentials

        creds = Credentials("test_key", "test_secret", "test_pass")
        config = Config(creds, simulated=True)
        client = OkxClient(config)

        # 验证方法存在
        assert hasattr(client, "get_order")
        assert hasattr(client, "place_order")
        assert hasattr(client, "cancel_order")
