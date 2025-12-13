#!/usr/bin/env python3
"""WebSocket 公共 Ticker 最小示例：订阅并读取一条消息。"""

import asyncio
import sys

from okx_py import Config, Credentials, WsClient


async def main() -> int:
    creds = Credentials("dummy", "dummy", "dummy")
    config = Config(creds, simulated=True, timeout_secs=5)

    try:
        client = await WsClient.connect_public(config, max_reconnect_attempts=1)
    except Exception as exc:  # noqa: BLE001
        print(f"连接 WebSocket 失败，可能是网络或 DNS 问题: {exc}")
        return 1

    try:
        await client.subscribe_tickers("BTC-USDT")
        msg = await asyncio.wait_for(client.recv(), timeout=5)
        if msg is None:
            print("连接已关闭，未收到数据")
        else:
            print(f"收到消息: {msg}")
    except asyncio.TimeoutError:
        print("超时未收到消息，可能网络受限")
    finally:
        await client.close()

    return 0


if __name__ == "__main__":
    sys.exit(asyncio.run(main()))
