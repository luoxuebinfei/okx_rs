#!/usr/bin/env python3
"""私有 REST 示例：查询账户余额（需要环境变量提供凭证）。"""

import os
import sys

from okx_py import Config, Credentials, OkxClient


def build_client() -> OkxClient:
    api_key = os.getenv("OKX_API_KEY")
    secret_key = os.getenv("OKX_SECRET_KEY")
    passphrase = os.getenv("OKX_PASSPHRASE")

    if not all([api_key, secret_key, passphrase]):
        raise RuntimeError("请设置 OKX_API_KEY、OKX_SECRET_KEY、OKX_PASSPHRASE 环境变量后再运行。")

    creds = Credentials(api_key, secret_key, passphrase)
    # 默认使用模拟环境，避免误操作真实资金；如需实盘，设置 OKX_SIMULATED=false
    simulated = os.getenv("OKX_SIMULATED", "true").lower() == "true"
    config = Config(creds, simulated=simulated, timeout_secs=10)
    return OkxClient(config)


def main() -> int:
    try:
        client = build_client()
    except Exception as exc:  # noqa: BLE001
        print(f"初始化客户端失败: {exc}")
        return 1

    try:
        balances = client.get_balance()
    except Exception as exc:  # noqa: BLE001
        print(f"查询余额失败: {exc}")
        return 1

    print("账户余额（仅展示非零资产）:")
    for bal in balances:
        for detail in bal.details:
            try:
                if float(detail.eq) > 0:
                    print(f"  {detail.ccy}: {detail.eq}")
            except ValueError:
                # 字段可能为空字符串，跳过转换
                continue
    return 0


if __name__ == "__main__":
    sys.exit(main())
