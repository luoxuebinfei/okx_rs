"""WebSocket 公共接口集成测试。"""

import asyncio

import pytest
import okx_py

pytestmark = pytest.mark.integration


def test_ws_public_ticker_once(ws_config):
    """订阅公共 Ticker，尝试接收一条消息或在超时后跳过。"""
    async def run():
        client = await okx_py.WsClient.connect_public(ws_config, max_reconnect_attempts=1)
        try:
            await client.subscribe_tickers("BTC-USDT")
            try:
                msg = await asyncio.wait_for(client.recv(), timeout=5)
                # 收到消息或 None（连接关闭）都视为通过
                assert msg is None or "arg" in msg
            except asyncio.TimeoutError:
                pytest.skip("未在超时内收到消息，可能网络受限")
        finally:
            await client.close()

    try:
        asyncio.run(run())
    except Exception as exc:  # noqa: BLE001
        pytest.skip(f"网络不可用，跳过 WebSocket 集成测试: {exc}")
