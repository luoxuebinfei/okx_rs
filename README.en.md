# okx_rs · OKX Rust SDK & Python bindings

> 简体中文见 [README.md](README.md)

## Overview
- High-performance OKX REST/WebSocket SDK in Rust; all paths/params follow the official docs (https://www.okx.com/docs-v5/) and mirror the official `python-okx` behavior.
- Workspace layout: `okx-core` (config/signing/types), `okx-rest` (REST client), `okx-ws` (WebSocket client), `okx-py` (PyO3 bindings with sync/async/WS).
- Built-in signing, timeout, proxy, WS heartbeat and auto-reconnect; supports production and demo trading.
- Detailed API/type notes live in `docs/en/rust-api.md` and `docs/en/python.md` (Chinese versions in `docs/zh/`).

## Highlights
- **API coverage**: Account, Trade, Funding, Market, Public REST; WS tickers/books/trades/candles/mark-price/index/funding-rate plus private account/positions/orders/orders-algo/balance_and_position.
- **Stability**: Tokio async + reqwest pooling; WS auto-reconnect with subscription restore (`ReconnectingWsClient`).
- **Type safety**: Strict request/response structs (`Balance`, `Order`, etc.) matching official field names; unified `OkxError`.
- **Cross-language**: PyO3 bindings expose sync/async REST and WS clients with `.pyi` stubs.
- **Official signing**: HMAC-SHA256 of `timestamp + method + requestPath + body` (see `okx-core::signer`, sourced from the official “REST Authentication” section).

## Modules
- `crates/okx-core`: `Config`, `Credentials`, `Signer`, `OkxError`, constants, and all shared types.
- `crates/okx-rest`: `OkxRestClient` plus `AccountApi` / `TradeApi` / `FundingApi` / `MarketApi` / `PublicApi`.
- `crates/okx-ws`: `WsClient` (basic) and `ReconnectingWsClient` (auto-reconnect), `Channel`/`WsMessage`.
- `crates/okx-py`: PyO3 bindings (`OkxClient`, `AsyncOkxClient`, `WsClient`) and exported types.
- `justfile`: unified scripts (fmt, clippy, test, py-test, typecheck, ci, coverage).

## API Coverage (from official docs)
- Account: balance `/api/v5/account/balance`, positions `/positions`, leverage `/set-leverage`, fee rates `/trade-fee`, max size `/max-size`, max available `/max-avail-size`, risk `/account-position-risk`, etc.
- Trade: place/cancel/amend (single & batch), current/history orders, fills `/fills`, algo orders `/algo`, close position `/close-position`.
- Funding: asset balances `/asset/balances`, transfer `/asset/transfer`, withdrawal `/asset/withdrawal`, deposit address/history, withdrawal history, currencies list.
- Market: all/single ticker, order book (standard/5/50/L2), candles, trades, index tickers.
- Public: instruments, funding rate/history, system time, mark price.
- WebSocket channels: tickers/books/books5/books50-l2-tbt/books-l2-tbt/trades/candle(1m/5m/15m/1H/4H/1D)/mark-price/index-tickers/funding-rate; private account/positions/orders/orders-algo/balance_and_position.

## Installation
Rust:
- In workspace: `cargo build --all` or `just build`.
- As dependency: not yet on crates.io; use path/git, e.g., `cargo add okx-rest --path crates/okx-rest` or `--git https://github.com/user/okx_rs`.

Python (Python 3.9+, recommended `uv` + `maturin`):
```bash
just py-setup      # create .venv and install deps
just py-build      # maturin develop installs okx-py
```

## Quickstart (Rust REST)
```rust
use okx_rest::{OkxRestClient, AccountApi, MarketApi};
use okx_core::{Config, Credentials};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let creds = Credentials::new("api_key", "secret_key", "passphrase");
    let config = Config::new(creds).simulated(true).with_timeout_secs(10);
    let client = OkxRestClient::new(config);

    // Public API
    let ticker = client.get_ticker("BTC-USDT").await?;

    // Private API (requires valid keys)
    let balance = client.get_balance(None).await?;

    println!("ticker={ticker:?} balance={balance:?}");
    Ok(())
}
```

## Quickstart (Rust WebSocket)
```rust
use okx_ws::{Channel, WsClient, WsMessage};
use okx_core::{Config, Credentials};
use futures_util::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let creds = Credentials::new("api_key", "secret_key", "passphrase");
    let config = Config::new(creds).simulated(true);
    let mut client = WsClient::connect_public(&config).await?;

    client
        .subscribe(vec![Channel::Tickers { inst_id: "BTC-USDT".into() }])
        .await?;

    while let Some(msg) = client.next().await {
        match msg? {
            WsMessage::Data { channel, data, .. } => println!("ch={channel} data={data:?}"),
            WsMessage::Event { event, .. } => println!("event={event:?}"),
            _ => {}
        }
    }
    Ok(())
}
```

## Python Examples
Sync:
```python
from okx_py import OkxClient, Config, Credentials

cfg = Config(Credentials("api_key", "secret_key", "passphrase"), simulated=True)
client = OkxClient(cfg)

print(client.get_ticker("BTC-USDT"))
print(client.get_balance())
```
Async:
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
See `crates/okx-py/examples/` for more.

## Configuration Notes
- `Config::simulated(true)`: switches WS endpoints to `wspap.okx.com`; REST defaults to `https://www.okx.com`, customize via `with_rest_url`/`with_ws_public_url`/`with_ws_private_url`.
- Timeout/proxy: `with_timeout_secs(30)`, `with_proxy("http://127.0.0.1:7890")`.
- Constants: `API_VERSION = "v5"`, REST bases `REST_API_URL` / `REST_API_URL_AWS`, WS `WS_PUBLIC_URL` / `WS_PRIVATE_URL` and simulated variants.
- Signing: `Signer::generate_headers` applies HMAC-SHA256 over `timestamp + method + requestPath + body` per official docs.

## Dev & Quality
- Scripts: `just fmt` / `just clippy` / `just test` / `just py-test` / `just py-typecheck` / `just ci`.
- Coverage: `just cov-html` (see `scripts/cov-html.sh`, `docs/coverage.md`).
- Rust docs: `cargo doc --all --no-deps` (reference `docs/rust-api.*.md`).

## Docs & Release
- Rust API: `docs/en/rust-api.md` (Chinese: `docs/zh/rust-api.md`).
- Python docs: `docs/en/python.md` (Chinese: `docs/zh/python.md`).
- Release guide: `docs/en/release.md` / `docs/zh/release.md` (crates.io / PyPI steps).
- Changelog: `CHANGELOG.md`.

## License
MIT OR Apache-2.0 (dual licensed).
