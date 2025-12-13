"""测试共享夹具。"""

import pytest

import okx_py


@pytest.fixture
def credentials():
    """提供基础凭证对象。"""
    return okx_py.Credentials("test_api_key", "test_secret_key", "test_passphrase")


@pytest.fixture
def sync_client(credentials):
    """构造同步客户端（模拟环境）。"""
    config = okx_py.Config(credentials, simulated=True, timeout_secs=1)
    return okx_py.OkxClient(config)


@pytest.fixture
def async_client(credentials):
    """构造异步客户端（模拟环境）。"""
    config = okx_py.Config(credentials, simulated=True, timeout_secs=1)
    return okx_py.AsyncOkxClient(config)
