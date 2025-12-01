#!/usr/bin/env python3
"""WebSocket usage example for okx_py.

This example demonstrates how to use the WsClient for real-time data streaming.
"""

import asyncio
import os

from okx_py import Config, Credentials, WsClient


async def public_websocket_example():
    """Example: Subscribe to public channels."""
    print("=== Public WebSocket Example ===\n")

    # Create config (credentials not needed for public channels)
    creds = Credentials("dummy", "dummy", "dummy")
    config = Config(creds, simulated=True)

    # Connect to public WebSocket
    client = await WsClient.connect_public(config)
    print(f"Connected: {client}")

    # Subscribe to multiple channels
    await client.subscribe_tickers("BTC-USDT")
    await client.subscribe_tickers("ETH-USDT")
    await client.subscribe_trades("BTC-USDT")

    print(f"Subscriptions: {await client.subscription_count()}")
    print("\nReceiving messages (Ctrl+C to stop)...\n")

    # Receive messages
    count = 0
    try:
        async for msg in client:
            if msg is None:
                print("Connection closed")
                break

            msg_type = msg.get("type")
            if msg_type == "data":
                channel = msg.get("channel")
                data = msg.get("data", [])
                print(f"[{channel}] {data[0][:100] if data else 'no data'}...")
            elif msg_type == "event":
                event = msg.get("event")
                print(f"[EVENT] {event}: {msg}")

            count += 1
            if count >= 20:  # Stop after 20 messages for demo
                break

    except KeyboardInterrupt:
        print("\nStopping...")

    await client.close()
    print("Disconnected")


async def private_websocket_example():
    """Example: Subscribe to private channels (requires valid credentials)."""
    print("\n=== Private WebSocket Example ===\n")

    api_key = os.environ.get("OKX_API_KEY")
    secret_key = os.environ.get("OKX_SECRET_KEY")
    passphrase = os.environ.get("OKX_PASSPHRASE")

    if not all([api_key, secret_key, passphrase]):
        print("Skipping private example - set OKX_API_KEY, OKX_SECRET_KEY, OKX_PASSPHRASE")
        return

    creds = Credentials(api_key, secret_key, passphrase)
    config = Config(creds, simulated=True)

    # Connect to private WebSocket
    client = await WsClient.connect_private(config)
    print(f"Connected: {client}")

    # Subscribe to private channels
    await client.subscribe_account()
    await client.subscribe_orders("SPOT")
    await client.subscribe_positions("SWAP")

    print(f"Subscriptions: {await client.subscription_count()}")
    print("\nReceiving messages...\n")

    count = 0
    try:
        async for msg in client:
            if msg is None:
                break

            msg_type = msg.get("type")
            if msg_type == "data":
                channel = msg.get("channel")
                print(f"[{channel}] Update received")
            elif msg_type == "event":
                print(f"[EVENT] {msg}")

            count += 1
            if count >= 10:
                break

    except KeyboardInterrupt:
        print("\nStopping...")

    await client.close()


async def reconnection_example():
    """Example: Automatic reconnection handling."""
    print("\n=== Reconnection Example ===\n")

    creds = Credentials("dummy", "dummy", "dummy")
    config = Config(creds, simulated=True)

    # Connect with max reconnection attempts
    client = await WsClient.connect_public(config, max_reconnect_attempts=5)

    await client.subscribe_tickers("BTC-USDT")

    print("Subscribed. If connection drops, it will auto-reconnect.")
    print("Subscriptions will be restored automatically.\n")

    count = 0
    async for msg in client:
        if msg is None:
            # Connection closed, try to reconnect
            print("Connection lost, attempting reconnect...")
            try:
                await client.reconnect()
                print("Reconnected!")
            except Exception as e:
                print(f"Reconnection failed: {e}")
                break
            continue

        if msg.get("type") == "data":
            print(f"Received: {msg.get('channel')}")

        count += 1
        if count >= 10:
            break

    await client.close()


async def main():
    """Run all examples."""
    await public_websocket_example()
    await private_websocket_example()
    # await reconnection_example()  # Uncomment to test reconnection


if __name__ == "__main__":
    asyncio.run(main())
