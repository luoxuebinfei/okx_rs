#!/usr/bin/env python3
"""公共 REST 快速示例（无需真实凭证，默认使用模拟环境）。"""

import sys

from okx_py import Config, Credentials, OkxClient


def main() -> int:
    # 公共接口不校验凭证，这里使用占位值即可
    creds = Credentials("public_key", "public_secret", "public_pass")
    config = Config(creds, simulated=True, timeout_secs=5)
    client = OkxClient(config)
    print("已创建同步客户端（模拟环境）")

    try:
        ts = client.get_system_time()
        print(f"服务器时间: {ts}")
    except Exception as exc:  # noqa: BLE001
        print(f"获取服务器时间失败，可能是网络问题: {exc}")
        return 1

    try:
        instruments = client.get_instruments("SPOT")[:5]
        print("现货产品示例:")
        for inst in instruments:
            base = inst.get("baseCcy")
            quote = inst.get("quoteCcy")
            inst_id = inst.get("instId")
            print(f"  {inst_id}: {base}/{quote}")
    except Exception as exc:  # noqa: BLE001
        print(f"获取现货产品失败: {exc}")

    try:
        ticker = client.get_ticker("BTC-USDT")
        if ticker:
            print(
                f"BTC-USDT 最新价: {ticker.last}, 买一: {ticker.bid_px}, 卖一: {ticker.ask_px}"
            )
    except Exception as exc:  # noqa: BLE001
        print(f"获取 Ticker 失败: {exc}")

    return 0


if __name__ == "__main__":
    sys.exit(main())
