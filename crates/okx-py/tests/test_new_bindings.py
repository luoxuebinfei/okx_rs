"""测试新增的 Python 绑定"""
import pytest


def test_imports():
    """测试能否正常导入模块"""
    try:
        from okx_py import OkxClient, AsyncOkxClient, Config, Credentials
        assert OkxClient is not None
        assert AsyncOkxClient is not None
        assert Config is not None
        assert Credentials is not None
    except ImportError as e:
        pytest.fail(f"导入失败: {e}")


def test_sync_client_has_new_methods():
    """测试同步客户端是否有新增的方法"""
    from okx_py import OkxClient, Config, Credentials

    # 创建一个测试客户端（不需要真实凭证）
    creds = Credentials("test_key", "test_secret", "test_pass")
    config = Config(creds, simulated=True)
    client = OkxClient(config)

    # 检查 TradeApi 新增方法
    assert hasattr(client, 'amend_batch_orders'), "缺少 amend_batch_orders 方法"
    assert hasattr(client, 'mass_cancel'), "缺少 mass_cancel 方法"
    assert hasattr(client, 'cancel_all_after'), "缺少 cancel_all_after 方法"
    assert hasattr(client, 'order_precheck'), "缺少 order_precheck 方法"

    # 检查 MarketApi 新增方法
    assert hasattr(client, 'get_orderbook_lite'), "缺少 get_orderbook_lite 方法"
    assert hasattr(client, 'get_block_ticker'), "缺少 get_block_ticker 方法"
    assert hasattr(client, 'get_option_family_trades'), "缺少 get_option_family_trades 方法"

    # 检查 GridApi 新增方法
    assert hasattr(client, 'grid_amend_order_algo'), "缺少 grid_amend_order_algo 方法"
    assert hasattr(client, 'grid_stop_order_algo'), "缺少 grid_stop_order_algo 方法"
    assert hasattr(client, 'grid_positions'), "缺少 grid_positions 方法"
    assert hasattr(client, 'grid_ai_param'), "缺少 grid_ai_param 方法"

    # 检查 BlockRfqApi / FinanceApi / CopyTradingApi / BrokerApi 基础方法
    assert hasattr(client, 'get_counterparties'), "缺少 get_counterparties 方法"
    assert hasattr(client, 'defi_get_offers'), "缺少 defi_get_offers 方法"
    assert hasattr(client, 'get_existing_lead_positions'), "缺少 get_existing_lead_positions 方法"
    assert hasattr(client, 'fd_rebate_per_orders'), "缺少 fd_rebate_per_orders 方法"
    assert hasattr(client, 'get_block_rfq_trades'), "缺少 get_block_rfq_trades 方法"


def test_async_client_has_new_methods():
    """测试异步客户端是否有新增的方法"""
    from okx_py import AsyncOkxClient, Config, Credentials

    # 创建一个测试客户端（不需要真实凭证）
    creds = Credentials("test_key", "test_secret", "test_pass")
    config = Config(creds, simulated=True)
    client = AsyncOkxClient(config)

    # 检查 TradeApi 新增方法
    assert hasattr(client, 'amend_batch_orders'), "缺少 amend_batch_orders 方法"
    assert hasattr(client, 'mass_cancel'), "缺少 mass_cancel 方法"
    assert hasattr(client, 'cancel_all_after'), "缺少 cancel_all_after 方法"
    assert hasattr(client, 'order_precheck'), "缺少 order_precheck 方法"

    # 检查 MarketApi 新增方法
    assert hasattr(client, 'get_orderbook_lite'), "缺少 get_orderbook_lite 方法"
    assert hasattr(client, 'get_block_ticker'), "缺少 get_block_ticker 方法"
    assert hasattr(client, 'get_option_family_trades'), "缺少 get_option_family_trades 方法"

    # 检查 GridApi 新增方法
    assert hasattr(client, 'grid_amend_order_algo'), "缺少 grid_amend_order_algo 方法"
    assert hasattr(client, 'grid_stop_order_algo'), "缺少 grid_stop_order_algo 方法"
    assert hasattr(client, 'grid_positions'), "缺少 grid_positions 方法"
    assert hasattr(client, 'grid_ai_param'), "缺少 grid_ai_param 方法"

    # 检查 BlockRfqApi / FinanceApi / CopyTradingApi / BrokerApi 基础方法
    assert hasattr(client, 'get_counterparties'), "缺少 get_counterparties 方法"
    assert hasattr(client, 'defi_get_offers'), "缺少 defi_get_offers 方法"
    assert hasattr(client, 'get_existing_lead_positions'), "缺少 get_existing_lead_positions 方法"
    assert hasattr(client, 'fd_rebate_per_orders'), "缺少 fd_rebate_per_orders 方法"
    assert hasattr(client, 'get_block_rfq_trades'), "缺少 get_block_rfq_trades 方法"
    assert hasattr(client, 'get_asset_balances'), "缺少 get_asset_balances 方法"


def test_method_signatures():
    """测试方法签名是否正确"""
    from okx_py import OkxClient, Config, Credentials
    import inspect

    creds = Credentials("test_key", "test_secret", "test_pass")
    config = Config(creds, simulated=True)
    client = OkxClient(config)

    # 检查方法是否可调用
    assert callable(client.mass_cancel), "mass_cancel 不可调用"
    assert callable(client.get_orderbook_lite), "get_orderbook_lite 不可调用"
    assert callable(client.grid_positions), "grid_positions 不可调用"

    # 检查方法签名（确保参数正确）
    sig = inspect.signature(client.get_orderbook_lite)
    assert 'inst_id' in sig.parameters, "get_orderbook_lite 缺少 inst_id 参数"


if __name__ == '__main__':
    pytest.main([__file__, '-v'])
