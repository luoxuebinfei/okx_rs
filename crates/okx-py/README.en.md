# okx-py · Python bindings for OKX (powered by Rust)

> 简体中文见 [README.zh.md](README.zh.md)

High-performance Python bindings built on the Rust `okx-rest` and `okx-ws` crates. All endpoints and parameters come from the official OKX docs (https://www.okx.com/docs-v5/) and mirror the behavior of the official `python-okx`.

## Features
- **Performance**: Rust async core, typically 5–10x faster than pure Python.
- **Type safety**: `.pyi` stubs plus Rust-side validation of request/response shapes.
- **Full coverage**: Account/Trade/Funding/Market/Public REST; WS tickers/books/trades/candles and private account/positions/orders/orders-algo/balance_and_position.
- **Stability**: Auto-reconnect with subscription restore (via `ReconnectingWsClient`).

## Install
PyPI (planned): `pip install okx-py`  
From source (recommended for now):
```bash
just py-setup      # creates .venv and installs dev deps via uv
just py-build      # maturin develop, installs okx_py into venv
```

## Quickstart
Sync REST:
```python
from okx_py import OkxClient, Config, Credentials

cfg = Config(Credentials("api_key", "secret_key", "passphrase"), simulated=True)
client = OkxClient(cfg)

print(client.get_ticker("BTC-USDT"))     # /api/v5/market/ticker
print(client.get_balance())              # /api/v5/account/balance
```

Async REST:
```python
import asyncio
from okx_py import AsyncOkxClient, Config, Credentials

async def main():
    cfg = Config(Credentials("api_key", "secret_key", "passphrase"), simulated=True)
    client = AsyncOkxClient(cfg)
    ticker, balance = await asyncio.gather(
        client.get_ticker("BTC-USDT"),
        client.get_balance(),
    )
    print(ticker, balance)

asyncio.run(main())
```

WebSocket:
```python
import asyncio
from okx_py import WsClient, Config, Credentials

async def main():
    cfg = Config(Credentials("api_key", "secret_key", "passphrase"), simulated=True)
    client = await WsClient.connect_public(cfg)
    await client.subscribe_tickers("BTC-USDT")

    async for msg in client:
        print(msg)
        break

asyncio.run(main())
```

## API surface (from code)
- **Config / Credentials**: mirrors Rust `okx-core::Config`; supports `simulated`, timeout, proxy, and custom endpoints.
- **Sync client `OkxClient`** (`src/client.rs`):
  - Account: `get_balance(ccy=None)`, `get_positions(inst_type=None, inst_id=None)`
  - Trade: `place_order(...)`, `cancel_order(...)`, `get_order(...)`, `get_orders_pending(...)`
  - Market/Public: `get_ticker`, `get_tickers`, `get_instruments`, `get_system_time`
- **Async client `AsyncOkxClient`** (`src/async_client.rs`): same methods as above, returning awaitables.
- **WS client `WsClient`** (`src/ws_client.rs`):
  - Connect: `connect_public` / `connect_private`
  - Subscribe: `subscribe_tickers`, `subscribe_orderbook`, `subscribe_trades`, `subscribe_candles(interval=1m|5m|15m|1H|4H|1D)`, `subscribe_account`, `subscribe_positions`, `subscribe_orders`
  - Receive: `recv()` or `async for msg in client`
  - State: `is_connected()`, `reconnect()`, `close()`, `subscription_count()`

## Examples
- Complete samples in `examples/`: `basic_usage.py`, `async_usage.py`, `websocket_usage.py`.
- Rust-side behavior documented in `docs/en/python.md` and `docs/en/rust-api.md`.

## Development & Tests
- Scripts: `just py-build`, `just py-test`, `just py-test-verbose`, `just py-typecheck`.
- Coverage (with Rust + Py): `just cov-html` (see `scripts/cov-html.sh`).
- Release steps: `docs/en/release.md`.

## License
MIT OR Apache-2.0.
