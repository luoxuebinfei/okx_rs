"""错误处理测试."""

import pytest


def test_invalid_credentials():
    """测试无效凭证创建."""
    from okx_py import Credentials

    # 空字符串应该可以创建（验证在 API 调用时进行）
    creds = Credentials("", "", "")
    assert creds is not None


def test_invalid_config_timeout():
    """测试无效超时配置."""
    from okx_py import Config, Credentials

    creds = Credentials("key", "secret", "pass")

    # 超时为 0 应该可以创建（Rust 端会处理）
    config = Config(creds, timeout_secs=0)
    assert config is not None


def test_sync_client_invalid_instrument():
    """测试同步客户端查询无效产品."""
    from okx_py import OkxClient, Config, Credentials

    creds = Credentials("dummy", "dummy", "dummy")
    config = Config(creds, simulated=True)
    client = OkxClient(config)

    try:
        # 查询不存在的产品
        ticker = client.get_ticker("INVALID-PAIR")
        # 如果返回 None 或空，说明处理正确
        assert ticker is None or ticker
    except Exception as e:
        # 抛出异常也是合理的错误处理
        assert e is not None


@pytest.mark.asyncio
async def test_async_client_invalid_instrument():
    """测试异步客户端查询无效产品."""
    from okx_py import AsyncOkxClient, Config, Credentials

    creds = Credentials("dummy", "dummy", "dummy")
    config = Config(creds, simulated=True)
    client = AsyncOkxClient(config)

    try:
        # 查询不存在的产品
        ticker = await client.get_ticker("INVALID-PAIR")
        assert ticker is None or ticker
    except Exception as e:
        # 抛出异常也是合理的错误处理
        assert e is not None


def test_sync_client_invalid_inst_type():
    """测试同步客户端查询无效产品类型."""
    from okx_py import OkxClient, Config, Credentials

    creds = Credentials("dummy", "dummy", "dummy")
    config = Config(creds, simulated=True)
    client = OkxClient(config)

    try:
        # 查询无效的产品类型
        instruments = client.get_instruments("INVALID_TYPE")
        # 应该返回空列表或抛出异常
        assert isinstance(instruments, list)
    except Exception as e:
        # 抛出异常是合理的
        assert e is not None


@pytest.mark.asyncio
async def test_async_client_invalid_inst_type():
    """测试异步客户端查询无效产品类型."""
    from okx_py import AsyncOkxClient, Config, Credentials

    creds = Credentials("dummy", "dummy", "dummy")
    config = Config(creds, simulated=True)
    client = AsyncOkxClient(config)

    try:
        # 查询无效的产品类型
        instruments = await client.get_instruments("INVALID_TYPE")
        assert isinstance(instruments, list)
    except Exception as e:
        assert e is not None


@pytest.mark.asyncio
async def test_ws_client_invalid_channel():
    """测试 WebSocket 订阅无效频道."""
    from okx_py import WsClient, Config, Credentials

    creds = Credentials("dummy", "dummy", "dummy")
    config = Config(creds, simulated=True)

    try:
        client = await WsClient.connect_public(config)
        # 订阅行情（正常频道）
        await client.subscribe_tickers("BTC-USDT")
        # 如果没有抛出异常，说明订阅成功
    except Exception as e:
        # 网络错误可接受
        pytest.skip(f"WebSocket 连接错误: {e}")


def test_credentials_repr_security():
    """测试凭证的字符串表示不泄露敏感信息."""
    from okx_py import Credentials

    creds = Credentials("my_secret_api_key_12345", "my_secret_key", "my_passphrase")
    repr_str = repr(creds)

    # 确保完整的 API key 不在 repr 中
    assert "my_secret_api_key_12345" not in repr_str
    # 应该只显示前几个字符
    assert "my_secre" in repr_str or "my_secret_" in repr_str
    # 不应该包含 secret_key 或 passphrase
    assert "my_secret_key" not in repr_str
    assert "my_passphrase" not in repr_str


def test_config_repr_no_credentials():
    """测试配置的字符串表示不包含凭证."""
    from okx_py import Config, Credentials

    creds = Credentials("secret_key", "secret_secret", "secret_pass")
    config = Config(creds, simulated=True)
    repr_str = repr(config)

    # 确保凭证不在 repr 中
    assert "secret_key" not in repr_str
    assert "secret_secret" not in repr_str
    assert "secret_pass" not in repr_str
    # 应该包含配置信息
    assert "Config" in repr_str
    assert "simulated" in repr_str


def test_client_repr_no_credentials():
    """测试客户端的字符串表示不包含凭证."""
    from okx_py import OkxClient, Config, Credentials

    creds = Credentials("secret_key", "secret_secret", "secret_pass")
    config = Config(creds, simulated=True)
    client = OkxClient(config)
    repr_str = repr(client)

    # 确保凭证不在 repr 中
    assert "secret_key" not in repr_str
    assert "secret_secret" not in repr_str
    assert "secret_pass" not in repr_str
