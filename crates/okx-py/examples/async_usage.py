#!/usr/bin/env python3
"""Async usage example for okx_py.

This example demonstrates how to use the asynchronous AsyncOkxClient.
"""

import asyncio
import os

from okx_py import AsyncOkxClient, Config, Credentials


async def main():
    # Get credentials from environment variables
    api_key = os.environ.get("OKX_API_KEY", "your_api_key")
    secret_key = os.environ.get("OKX_SECRET_KEY", "your_secret_key")
    passphrase = os.environ.get("OKX_PASSPHRASE", "your_passphrase")

    # Create credentials and config
    creds = Credentials(api_key, secret_key, passphrase)
    config = Config(creds, simulated=True)

    # Create async client
    client = AsyncOkxClient(config)
    print(f"Created async client: {client}")

    # Concurrent API calls - much faster than sequential!
    print("\n--- Concurrent API Calls ---")
    results = await asyncio.gather(
        client.get_system_time(),
        client.get_ticker("BTC-USDT"),
        client.get_ticker("ETH-USDT"),
        client.get_tickers("SPOT"),
        return_exceptions=True,
    )

    ts, btc_ticker, eth_ticker, spot_tickers = results

    print(f"Server time: {ts}")

    if isinstance(btc_ticker, Exception):
        print(f"BTC ticker error: {btc_ticker}")
    elif btc_ticker:
        print(f"BTC-USDT: {btc_ticker.last}")

    if isinstance(eth_ticker, Exception):
        print(f"ETH ticker error: {eth_ticker}")
    elif eth_ticker:
        print(f"ETH-USDT: {eth_ticker.last}")

    if isinstance(spot_tickers, Exception):
        print(f"Spot tickers error: {spot_tickers}")
    else:
        print(f"Total SPOT tickers: {len(spot_tickers)}")

    # Sequential calls for comparison
    print("\n--- Sequential API Calls ---")
    instruments = await client.get_instruments("SWAP")
    print(f"SWAP instruments: {len(instruments)}")

    # With valid credentials, you can do trading operations
    if api_key != "your_api_key":
        print("\n--- Account Operations ---")
        try:
            balance = await client.get_balance()
            print(f"Balance: {balance}")

            positions = await client.get_positions()
            print(f"Positions: {len(positions)}")

            pending = await client.get_orders_pending()
            print(f"Pending orders: {len(pending)}")
        except Exception as e:
            print(f"Error: {e}")


if __name__ == "__main__":
    asyncio.run(main())
