"""测试 okx_py 核心类型。

这些测试验证 Credentials、Config 等基础类型的行为。
"""

import pytest

from okx_py import Credentials, Config


class TestCredentials:
    """测试 Credentials 类。"""

    def test_create_credentials(self):
        """测试创建凭证对象。"""
        creds = Credentials("test_api_key", "test_secret_key", "test_passphrase")
        assert creds is not None

    def test_credentials_repr(self):
        """测试凭证的字符串表示（应该隐藏部分 API key）。"""
        creds = Credentials("apikey123456789", "secret", "pass")
        repr_str = repr(creds)
        assert "Credentials" in repr_str
        # API key 应该被部分隐藏
        assert "apikey123456789" not in repr_str
        assert "apikey12" in repr_str

    def test_credentials_with_short_api_key(self):
        """测试短 API key 的处理。"""
        creds = Credentials("short", "secret", "pass")
        repr_str = repr(creds)
        assert "Credentials" in repr_str


class TestConfig:
    """测试 Config 类。"""

    @pytest.fixture
    def credentials(self):
        """创建测试用凭证。"""
        return Credentials("test_api_key", "test_secret_key", "test_passphrase")

    def test_create_config_default(self, credentials):
        """测试使用默认参数创建配置。"""
        config = Config(credentials)
        assert config is not None
        assert config.simulated is False
        assert "okx.com" in config.rest_url

    def test_create_config_simulated(self, credentials):
        """测试创建模拟交易配置。"""
        config = Config(credentials, simulated=True)
        assert config.simulated is True

    def test_create_config_with_timeout(self, credentials):
        """测试创建带超时的配置。"""
        config = Config(credentials, simulated=False, timeout_secs=60)
        assert config is not None

    def test_config_repr(self, credentials):
        """测试配置的字符串表示。"""
        config = Config(credentials, simulated=True)
        repr_str = repr(config)
        assert "Config" in repr_str
        assert "simulated=true" in repr_str
        assert "rest_url" in repr_str
