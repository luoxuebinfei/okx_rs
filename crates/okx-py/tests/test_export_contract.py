"""测试 Python 绑定的导出与契约稳定性。

目标：
- 绑定层的构造器/静态工厂方法在参数类型错误时应立即失败（不触发网络）。
- 关键对象的 repr 不泄露敏感信息。
- 只读属性保持只读，避免用户误以为可动态改写内部配置。
"""

from __future__ import annotations

import inspect

import pytest

import okx_py


def test_credentials_repr_does_not_leak_secrets(credentials):
    """Credentials 的 repr 只展示 API key 片段，不应泄露密钥与口令。"""
    text = repr(credentials)
    assert "Credentials" in text
    assert "test_secret_key" not in text
    assert "test_passphrase" not in text


def test_config_repr_does_not_leak_secrets(credentials):
    """Config 的 repr 不应包含任何凭证信息。"""
    config = okx_py.Config(credentials, simulated=True, timeout_secs=1)
    text = repr(config)
    assert "Config" in text
    assert "test_api_key" not in text
    assert "test_secret_key" not in text
    assert "test_passphrase" not in text


def test_python_package_reexports_native_symbols():
    """Python 包 okx_py 应准确重导出原生扩展中的符号，避免 __init__.py 漏改。"""
    from okx_py import okx_py as native

    assert okx_py.__version__ == native.__version__
    for name in [
        "Credentials",
        "Config",
        "OkxClient",
        "AsyncOkxClient",
        "WsClient",
        "Balance",
        "BalanceDetail",
        "Position",
        "Order",
        "Ticker",
    ]:
        assert getattr(okx_py, name) is getattr(native, name)


def test_config_properties_are_read_only(credentials):
    """Config 的 simulated/rest_url 是只读属性（当前设计），禁止赋值以免产生错觉。"""
    config = okx_py.Config(credentials, simulated=True, timeout_secs=1)
    with pytest.raises(AttributeError):
        config.simulated = False
    with pytest.raises(AttributeError):
        config.rest_url = "http://127.0.0.1:9"


def test_clients_require_config():
    """OkxClient/AsyncOkxClient 必须以 Config 构造。"""
    with pytest.raises(TypeError):
        okx_py.OkxClient(None)
    with pytest.raises(TypeError):
        okx_py.AsyncOkxClient(None)


def test_credentials_constructor_rejects_wrong_types():
    """Credentials 构造器应拒绝 None/非字符串类型。"""
    with pytest.raises(TypeError):
        okx_py.Credentials(None, "s", "p")
    with pytest.raises(TypeError):
        okx_py.Credentials("k", None, "p")
    with pytest.raises(TypeError):
        okx_py.Credentials("k", "s", None)


def test_config_constructor_rejects_wrong_types(credentials):
    """Config 构造器应拒绝错误类型的参数。"""
    with pytest.raises(TypeError):
        okx_py.Config(None)
    with pytest.raises(TypeError):
        okx_py.Config(credentials, simulated="true")
    with pytest.raises(TypeError):
        okx_py.Config(credentials, timeout_secs=1.5)

    # u64 不接受负数：应在绑定层直接报错
    with pytest.raises((OverflowError, ValueError, TypeError)):
        okx_py.Config(credentials, timeout_secs=-1)


def test_ws_client_requires_factory_method():
    """WsClient 不应允许直接构造，必须通过 connect_public/connect_private 创建。"""
    with pytest.raises(TypeError):
        okx_py.WsClient()


def test_ws_client_connect_requires_config_type():
    """WsClient.connect_* 在参数类型不匹配时应立即抛出 TypeError。"""
    with pytest.raises(TypeError):
        okx_py.WsClient.connect_public(None)
    with pytest.raises(TypeError):
        okx_py.WsClient.connect_private(None)


def test_ws_client_connect_requires_running_event_loop(credentials):
    """connect_* 依赖运行中的事件循环；在同步上下文调用应明确报错。"""
    config = okx_py.Config(credentials, simulated=True, timeout_secs=1)
    with pytest.raises(RuntimeError) as excinfo:
        okx_py.WsClient.connect_public(config)
    assert "no running event loop" in str(excinfo.value)

    with pytest.raises(RuntimeError) as excinfo:
        okx_py.WsClient.connect_private(config)
    assert "no running event loop" in str(excinfo.value)


@pytest.mark.parametrize("method_name", ["mass_cancel", "cancel_all_after", "order_precheck"])
def test_trade_methods_reject_empty_request(sync_client, method_name):
    """若 required JSON 为空，应本地直接报错并给出清晰字段名。"""
    fn = getattr(sync_client, method_name)
    with pytest.raises(ValueError) as excinfo:
        fn("")
    assert "request 不能为空" in str(excinfo.value)


@pytest.mark.parametrize("method_name", ["mass_cancel", "cancel_all_after", "order_precheck"])
def test_trade_methods_reject_invalid_json(sync_client, method_name):
    """若 required JSON 非法，应本地直接报错（不触发网络请求）。"""
    fn = getattr(sync_client, method_name)
    with pytest.raises(ValueError) as excinfo:
        fn("{oops")
    assert "request JSON 解析失败" in str(excinfo.value)


def test_async_client_validates_request_json_before_await(async_client):
    """异步客户端在返回 awaitable 之前完成 JSON 校验，避免把错误延迟到 await 阶段。"""
    with pytest.raises(ValueError) as excinfo:
        async_client.mass_cancel("{oops")
    assert "request JSON 解析失败" in str(excinfo.value)

    with pytest.raises(ValueError) as excinfo:
        async_client.mass_cancel("")
    assert "request 不能为空" in str(excinfo.value)
