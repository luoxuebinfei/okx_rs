"""测试输入校验与错误消息。"""

import asyncio

import pytest


def test_grid_order_algo_requires_payload(sync_client):
    """网格策略下单的 payload 为空时应立即报错。"""
    with pytest.raises(RuntimeError) as excinfo:
        sync_client.grid_order_algo("")
    assert "payload 不能为空" in str(excinfo.value)


def test_grid_order_algo_rejects_invalid_json(sync_client):
    """网格策略下单遇到非法 JSON 时返回 ValueError。"""
    with pytest.raises(ValueError) as excinfo:
        sync_client.grid_order_algo("{oops")
    assert "payload JSON 解析失败" in str(excinfo.value)


def test_recurring_buy_requires_payload(sync_client):
    """定投策略下单的 payload 为空时应报错。"""
    with pytest.raises(RuntimeError) as excinfo:
        sync_client.place_recurring_buy_order("  ")
    assert "payload 不能为空" in str(excinfo.value)


def test_grid_pending_params_invalid(sync_client):
    """网格委托查询的 params 非法时应抛出 ValueError。"""
    with pytest.raises(ValueError) as excinfo:
        sync_client.grid_orders_algo_pending("not-json")
    assert "params JSON 解析失败" in str(excinfo.value)


def test_position_builder_invalid_json(sync_client):
    """Position Builder 接收的模拟仓位/资产 JSON 非法时应拒绝。"""
    with pytest.raises(ValueError) as excinfo:
        sync_client.position_builder(None, None, None, None, "{oops", None)
    assert "sim_pos JSON 解析失败" in str(excinfo.value)

    with pytest.raises(ValueError) as excinfo:
        sync_client.position_builder(None, None, None, None, None, "{oops")
    assert "sim_asset JSON 解析失败" in str(excinfo.value)


def test_async_grid_order_algo_rejects_invalid_json(async_client):
    """异步网格策略下单的 JSON 校验应提前失败。"""
    with pytest.raises(ValueError) as excinfo:
        asyncio.run(async_client.grid_order_algo("{oops"))
    assert "payload JSON 解析失败" in str(excinfo.value)


def test_async_position_builder_invalid_json(async_client):
    """异步 Position Builder 也应对非法 JSON 提示字段名。"""
    with pytest.raises(ValueError) as excinfo:
        asyncio.run(async_client.position_builder(None, None, None, None, "{oops", None))
    assert "sim_pos JSON 解析失败" in str(excinfo.value)

    with pytest.raises(ValueError) as excinfo:
        asyncio.run(async_client.position_builder(None, None, None, None, None, "{oops"))
    assert "sim_asset JSON 解析失败" in str(excinfo.value)


def test_spread_payload_and_params_validation(sync_client):
    """Spread API 需要合法 JSON，缺失或非法时应直接报错。"""
    with pytest.raises(RuntimeError) as excinfo:
        sync_client.spread_place_order("")
    assert "payload 不能为空" in str(excinfo.value)

    with pytest.raises(ValueError) as excinfo:
        sync_client.spread_place_order("{oops")
    assert "payload JSON 解析失败" in str(excinfo.value)

    with pytest.raises(RuntimeError) as excinfo:
        sync_client.spread_get_order_details("")
    assert "params 不能为空" in str(excinfo.value)

    with pytest.raises(ValueError) as excinfo:
        sync_client.spread_get_order_details("{oops")
    assert "params JSON 解析失败" in str(excinfo.value)


def test_new_domains_require_valid_json(sync_client):
    """新补齐的业务域接口：必填 JSON 参数应在本地提前失败（不触发网络请求）。"""
    # Block RFQ
    with pytest.raises(ValueError) as excinfo:
        sync_client.create_rfq("")
    assert "request_json 不能为空" in str(excinfo.value)

    with pytest.raises(ValueError) as excinfo:
        sync_client.create_rfq("{oops")
    assert "request_json JSON 解析失败" in str(excinfo.value)

    # Finance
    with pytest.raises(ValueError) as excinfo:
        sync_client.defi_purchase("   ")
    assert "request_json 不能为空" in str(excinfo.value)

    with pytest.raises(ValueError) as excinfo:
        sync_client.defi_purchase("{oops")
    assert "request_json JSON 解析失败" in str(excinfo.value)

    # Copy Trading
    with pytest.raises(ValueError) as excinfo:
        sync_client.place_lead_stop_order("")
    assert "request_json 不能为空" in str(excinfo.value)

    with pytest.raises(ValueError) as excinfo:
        sync_client.place_lead_stop_order("{oops")
    assert "request_json JSON 解析失败" in str(excinfo.value)

    # Broker
    with pytest.raises(ValueError) as excinfo:
        sync_client.fd_rebate_per_orders("")
    assert "params_json 不能为空" in str(excinfo.value)

    with pytest.raises(ValueError) as excinfo:
        sync_client.fd_rebate_per_orders("{oops")
    assert "params_json JSON 解析失败" in str(excinfo.value)


def test_async_new_domains_reject_invalid_json(async_client):
    """异步客户端的新域接口也应在本地提前失败（不需要 await）。"""
    with pytest.raises(ValueError) as excinfo:
        asyncio.run(async_client.create_rfq("{oops"))
    assert "request_json JSON 解析失败" in str(excinfo.value)

    with pytest.raises(ValueError) as excinfo:
        asyncio.run(async_client.defi_purchase("{oops"))
    assert "request_json JSON 解析失败" in str(excinfo.value)

    with pytest.raises(ValueError) as excinfo:
        asyncio.run(async_client.place_lead_stop_order("{oops"))
    assert "request_json JSON 解析失败" in str(excinfo.value)

    with pytest.raises(ValueError) as excinfo:
        asyncio.run(async_client.fd_rebate_per_orders("{oops"))
    assert "params_json JSON 解析失败" in str(excinfo.value)
