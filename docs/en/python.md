# Python Docs (okx_py)

PyO3 bindings over `okx-rest` and `okx-ws`. Every endpoint maps to the official OKX docs (https://www.okx.com/docs-v5/); all details below come directly from the code.

## Installation
Local dev (Python 3.9+, recommended `uv` + `maturin`):
```bash
just py-setup      # create .venv and install deps
just py-build      # maturin develop installs okx-py
```
See `docs/en/release.md` for PyPI publishing steps.

## Exported core types
- `Credentials(api_key, secret_key, passphrase)` for API keys.
- `Config(credentials, simulated=False, timeout_secs=30, proxy_url=None, rest_url=None, ws_public_url=None, ws_private_url=None)` maps to `okx-core::Config`.
- Domain objects are converted from Rust `okx-core::types` (e.g., `Balance`, `Position`, `Order`, `Ticker`), preserving official camelCase field names.

## Exception types
The SDK provides a hierarchy of exception types for fine-grained error handling:
- `OkxError`: Base class for all OKX-related exceptions
- `OkxHttpError`: HTTP response errors (non-2xx status codes)
- `OkxRateLimitError`: HTTP 429 rate limit errors (inherits from `OkxHttpError`)
- `OkxApiError`: OKX API business errors (code != 0)
- `OkxAuthError`: Authentication errors
- `OkxWebSocketError`: WebSocket connection errors
- `OkxTimeoutError`: Request timeout errors

Example:
```python
from okx_py import OkxClient, OkxRateLimitError, OkxApiError

try:
    client.place_order(...)
except OkxRateLimitError:
    # Handle rate limiting, wait and retry
    time.sleep(1)
except OkxApiError as e:
    # Handle business errors
    print(f"API error: {e}")
```

## Sync REST client `OkxClient`
Source: `crates/okx-py/src/client.rs`. Returns Python objects/dicts and maps to official REST paths:
- `get_balance(ccy=None)` → `/api/v5/account/balance`
- `get_positions(inst_type=None, inst_id=None)` → `/api/v5/account/positions`
- `place_order(inst_id, td_mode, side, ord_type, sz, px=None, cl_ord_id=None)` → `/api/v5/trade/order`
- `cancel_order(inst_id, ord_id=None, cl_ord_id=None)` → `/api/v5/trade/cancel-order`
- `get_order(inst_id, ord_id=None, cl_ord_id=None)` → `/api/v5/trade/order`
- `get_orders_pending(inst_type=None, inst_id=None)` → `/api/v5/trade/orders-pending`
- Public market:
  - `get_ticker(inst_id)` → `/api/v5/market/ticker`
  - `get_tickers(inst_type)` → `/api/v5/market/tickers`
  - `get_instruments(inst_type, inst_id=None)` → `/api/v5/public/instruments`
- Public utility: `get_system_time()` → `/api/v5/public/system-time`

## Async REST client `AsyncOkxClient`
Source: `crates/okx-py/src/async_client.rs`. Method set mirrors `OkxClient`, returning awaitable results:
- `get_balance`, `get_positions`, `place_order`, `cancel_order`, `get_order`, `get_orders_pending`
- Market: `get_ticker`, `get_tickers`, `get_instruments`
- Utility: `get_system_time`

## Coverage and gaps
- Exposed (REST): balance, positions, place/cancel/get order, pending orders, single/all tickers, instruments list, server time.
- Not exposed yet (REST): account config, leverage/margin helpers, fee rates, amend/batch orders, algo orders, funding (transfer/deposit/withdrawal), funding rate/mark price, etc. To use them, mirror the `okx-rest` request structs in the PyO3 bindings.
- WebSocket: public/private subscriptions are exposed with auto-reconnect; add more channel params in `ws_client.rs` if needed.

## REST exposed methods (official paths)
| Purpose | Official path | Python call |
| ------- | ------------- | ----------- |
| Balance | `GET /api/v5/account/balance` | `client.get_balance(ccy=None)` / `await aclient.get_balance()` |
| Positions | `GET /api/v5/account/positions` | `client.get_positions(inst_type=None, inst_id=None)` / async |
| Place order | `POST /api/v5/trade/order` | `client.place_order(inst_id, td_mode, side, ord_type, sz, px=None, cl_ord_id=None)` / async |
| Cancel order | `POST /api/v5/trade/cancel-order` | `client.cancel_order(inst_id, ord_id=None, cl_ord_id=None)` / async |
| Get order | `GET /api/v5/trade/order` | `client.get_order(inst_id, ord_id=None, cl_ord_id=None)` / async |
| Pending orders | `GET /api/v5/trade/orders-pending` | `client.get_orders_pending(inst_type=None, inst_id=None)` / async |
| Single ticker | `GET /api/v5/market/ticker` | `client.get_ticker(inst_id)` / async |
| All tickers by type | `GET /api/v5/market/tickers` | `client.get_tickers(inst_type)` / async |
| Instruments list | `GET /api/v5/public/instruments` | `client.get_instruments(inst_type, inst_id=None)` / async |
| Server time | `GET /api/v5/public/system-time` | `client.get_system_time()` / async |

## REST not yet exposed (implemented in Rust)
- Account: config, leverage info/set, max size/avail size, fee rates, position mode, position risk, etc.
- Trade: batch order/cancel, amend order, order history, fills, full algo order suite (place/cancel/query), close position.
- Funding: balances, transfer, deposit address/history, withdrawal/history, currencies list.
- Public: funding rate/history, mark price, etc.

## WebSocket client `WsClient`
Source: `crates/okx-py/src/ws_client.rs`, built atop `ReconnectingWsClient` (auto-reconnect + subscription restore).
- Connect: `await WsClient.connect_public(config, max_reconnect_attempts=None)` / `connect_private(...)`
- Subscribe public channels:
  - `subscribe_tickers(inst_id)` (tickers)
  - `subscribe_orderbook(inst_id)` (books)
  - `subscribe_trades(inst_id)` (trades)
  - `subscribe_candles(inst_id, interval="1m")` (1m/5m/15m/1H/4H/1D per official candle channels)
- Subscribe private channels:
  - `subscribe_account(ccy=None)` (account)
  - `subscribe_positions(inst_type, inst_id=None)` (positions)
  - `subscribe_orders(inst_type, inst_id=None)` (orders)
- Receive: `await client.recv()` returns a dict (type=data/event/pong/channel_conn_count/channel_conn_count_error/unknown); also supports `async for msg in client`.
- State: `is_connected()`, `reconnect()`, `close()`, `subscription_count()`.
- External timestamp login: `login_with_timestamp(timestamp_unix)` - Login to private WebSocket using a server-synchronized timestamp, useful when there's clock drift between client and server.

## WebSocket exposed subs
| Purpose | Channel | Python call |
| --------| ------- | ----------- |
| Ticker | `tickers` | `await ws.subscribe_tickers("BTC-USDT")` |
| Order book | `books` | `await ws.subscribe_orderbook("BTC-USDT")` |
| Trades | `trades` | `await ws.subscribe_trades("BTC-USDT")` |
| Candles | `candle1m/5m/15m/1H/4H/1D` | `await ws.subscribe_candles("BTC-USDT", interval="1m")` |
| Account (private) | `account` | `await ws.subscribe_account(ccy=None)` |
| Positions (private) | `positions` | `await ws.subscribe_positions(inst_type, inst_id=None)` |
| Orders (private) | `orders` | `await ws.subscribe_orders(inst_type, inst_id=None)` |

## WebSocket not yet exposed (can extend binding)
- Depth variants: `books5`, `books50-l2-tbt`, `books-l2-tbt`
- Index/funding/mark price: `index-tickers`, `funding-rate`, `mark-price`
- Algo orders: `orders-algo`
- Balance & position combined: `balance_and_position`

## Common market usages (official Market Data)
- Spot ticker: `get_ticker("BTC-USDT")` → `GET /api/v5/market/ticker`
- Perpetual ticker: `get_ticker("BTC-USDT-SWAP")` → `GET /api/v5/market/ticker`
- All spot tickers: `get_tickers("SPOT")` → `GET /api/v5/market/tickers`
- All perpetual tickers: `get_tickers("SWAP")` → `GET /api/v5/market/tickers`
- Order book: `get_orderbook("BTC-USDT", depth=5)` → `GET /api/v5/market/books` (if depth isn’t exposed yet, extend the binding to match the Rust params)
- Candles: `get_candles("BTC-USDT", bar="1m")` → `GET /api/v5/market/candles`
- Recent trades: `get_trades("BTC-USDT", limit=50)` → `GET /api/v5/market/trades`
- Index price: `get_index_tickers(quote_ccy="USDT", inst_id="BTC-USD")` → `GET /api/v5/market/index-tickers`

## Examples
- Sync snippet:
  ```python
  from okx_py import OkxClient, Config, Credentials

  cfg = Config(Credentials("api_key", "secret_key", "passphrase"), simulated=True)
  client = OkxClient(cfg)
  print(client.get_ticker("BTC-USDT"))
  ```
- Async snippet:
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
- Full examples live in `crates/okx-py/examples/` (basic_usage/async_usage/websocket_usage), all aligned with the official endpoints and the Rust implementations.
