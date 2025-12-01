#!/usr/bin/env python3
"""Basic usage example for okx_py.

This example demonstrates how to use the synchronous OkxClient.
"""

import os

from okx_py import Config, Credentials, OkxClient


def main():
    # Get credentials from environment variables
    api_key = os.environ.get("OKX_API_KEY", "your_api_key")
    secret_key = os.environ.get("OKX_SECRET_KEY", "your_secret_key")
    passphrase = os.environ.get("OKX_PASSPHRASE", "your_passphrase")

    # Create credentials and config
    creds = Credentials(api_key, secret_key, passphrase)
    config = Config(creds, simulated=True)  # Use simulated trading for safety

    # Create client
    client = OkxClient(config)
    print(f"Created client: {client}")

    # Get system time (public API, no auth required)
    print("\n--- System Time ---")
    ts = client.get_system_time()
    print(f"Server time: {ts}")

    # Get instruments (public API)
    print("\n--- SPOT Instruments (first 5) ---")
    instruments = client.get_instruments("SPOT")
    for inst in instruments[:5]:
        print(f"  {inst['instId']}: {inst['baseCcy']}/{inst['quoteCcy']}")

    # Get ticker (public API)
    print("\n--- BTC-USDT Ticker ---")
    ticker = client.get_ticker("BTC-USDT")
    if ticker:
        print(f"  Last: {ticker.last}")
        print(f"  Bid: {ticker.bid_px}")
        print(f"  Ask: {ticker.ask_px}")
        print(f"  24h High: {ticker.high_24h}")
        print(f"  24h Low: {ticker.low_24h}")
        print(f"  24h Volume: {ticker.vol_24h}")

    # The following require valid API credentials:
    if api_key != "your_api_key":
        print("\n--- Account Balance ---")
        try:
            balances = client.get_balance()
            for balance in balances:
                print(f"  Total Equity: {balance.total_eq}")
                for detail in balance.details:
                    if float(detail.eq) > 0:
                        print(f"    {detail.ccy}: {detail.eq}")
        except Exception as e:
            print(f"  Error: {e}")

        print("\n--- Positions ---")
        try:
            positions = client.get_positions()
            if positions:
                for pos in positions:
                    print(f"  {pos.inst_id}: {pos.pos} @ {pos.avg_px}, PnL: {pos.upl}")
            else:
                print("  No open positions")
        except Exception as e:
            print(f"  Error: {e}")


if __name__ == "__main__":
    main()
