# Rust API Notes (okx_rs)

This guide is based on the codebase and the official OKX docs (https://www.okx.com/docs-v5/) so that every path/parameter is sourced, not guessed.

## Core crate `okx-core`
- **Config** (`crates/okx-core/src/config.rs`)
  - Build: `Config::new(credentials)` (production by default, 30s timeout).
  - Switch environment: `simulated(bool)` flips WS endpoints to `wspap.okx.com`.
  - Custom endpoints: `with_rest_url` / `with_ws_public_url` / `with_ws_private_url`.
  - Others: `with_timeout_secs`, `with_proxy`; getters `rest_url`/`ws_public_url`/`ws_private_url`/`is_simulated`/`proxy_url`.
- **Credentials**: `Credentials::new(api_key, secret_key, passphrase)`.
- **Signer** (`okx-core::signer`): implements official signing `timestamp + method + requestPath + body` → HMAC-SHA256 → Base64.
  - `generate_headers(method, request_path, body, simulated)`: REST private headers.
  - `generate_public_headers(simulated)`: REST public headers.
  - `generate_ws_login_params()`: WS login tuple (apiKey/passphrase/timestamp/sign).
- **OkxError / Result**: unified error covering HTTP/WS/API/auth/serialization.
- **Constants**: `API_VERSION = "v5"`, REST bases `REST_API_URL` / `REST_API_URL_AWS`, WS `WS_PUBLIC_URL` / `WS_PRIVATE_URL` plus simulated variants.
- **Common types** (`okx-core::types`): `ApiResponse<T>` (`code`/`msg`/`data`), enums `InstType`/`TdMode`/`Side`/`PosSide`/`OrdType`, etc.
- **Domain types**:
  - Account: `Balance`, `BalanceDetail`, `Position`, `AccountConfig`.
  - Trade: `Order`, `Fill`, `AlgoOrder` and requests/responses `PlaceOrderRequest`, `CancelOrderRequest`, `AmendOrderRequest`, `PlaceAlgoOrderRequest`, etc.
  - Funding: `AssetBalance`, `DepositAddress`, `DepositRecord`, `WithdrawalRecord`, `FundsTransferRequest/Response`, `WithdrawalRequest/Response`, `CurrencyInfo`.
  - Market/Public: `Ticker`, `OrderBook`, `BookLevel`, `Candle`, `Trade`, `Instrument`, `IndexTicker`, `MarkPrice`, `FundingRate`, etc.

## REST client `okx-rest`
- **OkxRestClient** (`crates/okx-rest/src/client.rs`)
  - Build: `OkxRestClient::new(config)`; or `with_http_client` to inject a custom `reqwest::Client`.
  - Public GET: `get_public(path, params)` → builds query + public headers.
  - Private GET: `get(path, params)` → signs with `request_path` including query.
  - Private POST: `post(path, body)` → serializes body, signs, sends.
  - Parsing: expects OKX `ApiResponse<T>`; `code == "0"` returns `Vec<T>`, otherwise `OkxError::Api`.
- **API modules & official paths** (all constants reside in `endpoints` modules and come from the official docs)
  - Account (`account.rs`): `/api/v5/account/balance`, `/positions`, `/config`, `/set-leverage`, `/leverage-info`, `/max-size`, `/max-avail-size`, `/trade-fee`, `/set-position-mode`, `/account-position-risk`. Params: `GetBalanceParams`, `GetPositionsParams`, `SetLeverageRequest`, `GetLeverageInfoParams`, `GetMaxSizeParams`, `GetMaxAvailSizeParams`, `GetFeeRatesParams`, etc.
  - Trade (`trade.rs`): `/api/v5/trade/order`, `/cancel-order`, `/amend-order`, `/orders-pending`, `/orders-history`, `/fills`, `/order-algo`, `/cancel-algos`, `/algo-orders-pending`, `/algo-orders-history`, `/close-position`. Batch variants accept list bodies.
  - Funding (`funding.rs`): `/api/v5/asset/balances`, `/asset/deposit-address`, `/asset/deposit-history`, `/asset/withdrawal-history`, `/asset/transfer`, `/asset/withdrawal`, `/asset/currencies`.
  - Market (`market.rs`): `/api/v5/market/tickers`, `/ticker`, `/books` (and books5/books50/books-l2-tbt), `/candles`, `/trades`, `/index-tickers`.
  - Public (`public.rs`): `/api/v5/public/instruments`, `/funding-rate`, `/funding-rate-history`, `/system-time`, `/mark-price`.
- **Return shape**: all methods return `Result<Vec<T>>` (field names match the official responses).
- **Example**: see `crates/okx-rest/examples/rest_basic.rs`.

### Common market calls (aligned to official Market Data)
| Need | Official path | Key params | Rust (OkxRestClient) | Python (OkxClient / AsyncOkxClient) |
| ---- | ------------- | ---------- | -------------------- | ------------------------------------ |
| Spot ticker | `GET /api/v5/market/ticker` | `instId` (e.g., BTC-USDT) | `client.get_ticker("BTC-USDT").await?` | `client.get_ticker("BTC-USDT")` / async |
| Perpetual ticker | `GET /api/v5/market/ticker` | `instId` (e.g., BTC-USDT-SWAP) | `client.get_ticker("BTC-USDT-SWAP").await?` | `client.get_ticker("BTC-USDT-SWAP")` / async |
| All spot tickers | `GET /api/v5/market/tickers` | `instType=SPOT` | `client.get_tickers(GetTickersParams { inst_type: "SPOT".into(), uly: None, inst_family: None }).await?` | `client.get_tickers("SPOT")` / async |
| All perpetual tickers | `GET /api/v5/market/tickers` | `instType=SWAP`, optional `uly` | `client.get_tickers(GetTickersParams { inst_type: "SWAP".into(), uly: Some("BTC-USDT".into()), inst_family: None }).await?` | `client.get_tickers("SWAP")` / async |
| Order book | `GET /api/v5/market/books` | `instId` required, `sz` optional | `client.get_orderbook("BTC-USDT", Some(5)).await?` | `client.get_orderbook("BTC-USDT", depth=5)` / async |
| Candles | `GET /api/v5/market/candles` | `instId` required, `bar` optional | `client.get_candles(GetCandlesParams { inst_id: "BTC-USDT".into(), bar: Some("1m".into()), after: None, before: None, limit: None }).await?` | `client.get_candles("BTC-USDT", bar="1m")` / async |
| Recent trades | `GET /api/v5/market/trades` | `instId` required, `limit` optional | `client.get_trades("BTC-USDT", Some(50)).await?` | `client.get_trades("BTC-USDT", limit=50)` / async |
| Index price | `GET /api/v5/market/index-tickers` | `quoteCcy`/`instId` optional | `client.get_index_tickers(GetIndexTickersParams { quote_ccy: Some("USDT".into()), inst_id: Some("BTC-USD".into()) }).await?` | `client.get_index_tickers(quote_ccy="USDT", inst_id="BTC-USD")` / async |

> Note: Python examples are based on the current bindings (`crates/okx-py/src/client.rs`, `async_client.rs`). If a parameter is not exposed yet, align the binding signature with the Rust structs first; always keep paths/params identical to the official docs.

### Account (Trading Account)
| Need | Official path | Key params | Rust usage | Python usage |
| ---- | ------------- | ---------- | ---------- | ------------ |
| Balance | `GET /api/v5/account/balance` | `ccy` optional | `client.get_balance(Some("BTC")).await?` | `client.get_balance(ccy="BTC")` / async |
| Positions | `GET /api/v5/account/positions` | optional `instType`/`instId` | `client.get_positions(Some(GetPositionsParams { inst_type: Some("SWAP".into()), inst_id: Some("BTC-USDT-SWAP".into()), pos_id: None })).await?` | `client.get_positions(inst_type="SWAP", inst_id="BTC-USDT-SWAP")` / async |
| Account config | `GET /api/v5/account/config` | none | `client.get_account_config().await?` | Not exposed |
| Set leverage | `POST /api/v5/account/set-leverage` | `instId`/`ccy`, `lever`, `mgnMode` | `client.set_leverage(SetLeverageRequest { inst_id: Some("BTC-USDT-SWAP".into()), ccy: None, lever: "5".into(), mgn_mode: "cross".into(), pos_side: None }).await?` | Not exposed |
| Query leverage | `GET /api/v5/account/leverage-info` | `mgnMode` required; `ccy`/`instId` optional | `client.get_leverage_info(GetLeverageInfoParams { mgn_mode: "cross".into(), ccy: Some("USDT".into()), inst_id: None }).await?` | Not exposed |
| Max size | `GET /api/v5/account/max-size` | `instId`, `tdMode` | `client.get_max_size(GetMaxSizeParams { inst_id: "BTC-USDT".into(), td_mode: "cash".into(), ccy: None, px: None, leverage: None }).await?` | Not exposed |
| Max available | `GET /api/v5/account/max-avail-size` | `instId`, `tdMode` | `client.get_max_avail_size(GetMaxAvailSizeParams { inst_id: "BTC-USDT-SWAP".into(), td_mode: "cross".into(), ccy: None, reduce_only: None, quick_mgn_type: None }).await?` | Not exposed |
| Fee rates | `GET /api/v5/account/trade-fee` | `instType` | `client.get_fee_rates(GetFeeRatesParams { inst_type: "SPOT".into(), inst_id: None, uly: None, inst_family: None }).await?` | Not exposed |
| Position mode | `POST /api/v5/account/set-position-mode` | `posMode` | `client.set_position_mode("long_short_mode").await?` | Not exposed |
| Position risk | `GET /api/v5/account/account-position-risk` | none | `client.get_account_position_risk().await?` | Not exposed |

### Account advanced & system status
| Need | Official path | Key params | Rust usage | Python usage |
| ---- | ------------- | ---------- | ---------- | ------------ |
| Adjust position margin | `POST /api/v5/account/position/margin-balance` | `instId`, `posSide`, `type` (add/reduce), `amt`, optional `loanTrans` | `client.adjustment_margin(AdjustmentMarginRequest { inst_id, pos_side, r#type, amt, loan_trans }).await?` | `client.adjustment_margin(inst_id, pos_side, type_, amt, loan_trans=None)` / async |
| Set risk offset type | `POST /api/v5/account/set-riskOffset-type` | `type` | `client.set_risk_offset_type(SetRiskOffsetTypeRequest { r#type }).await?` | `client.set_risk_offset_type(type_)` / async |
| Set auto-loan | `POST /api/v5/account/set-auto-loan` | optional `autoLoan` string | `client.set_auto_loan(SetAutoLoanRequest { auto_loan }).await?` | `client.set_auto_loan(auto_loan=None)` / async |
| System status | `GET /api/v5/system/status` | optional `state` (0 normal / 1 maintenance) | `client.get_system_status(Some("0")).await?` | `client.get_system_status(state=None)` / async |

### Trade (Order)
| Need | Official path | Key params | Rust usage | Python usage |
| ---- | ------------- | ---------- | ---------- | ------------ |
| Place order | `POST /api/v5/trade/order` | `instId`, `tdMode`, `side`, `ordType`, `sz`, `px` (limit) | `client.place_order(req).await?` | `client.place_order(inst_id, td_mode, side, ord_type, sz, px=..., cl_ord_id=...)` / async |
| Batch order | `POST /api/v5/trade/batch-orders` | list body | `client.place_batch_orders(vec![req1, req2]).await?` | Not exposed |
| Cancel order | `POST /api/v5/trade/cancel-order` | `instId` + (`ordId` or `clOrdId`) | `client.cancel_order(req).await?` | `client.cancel_order(inst_id, ord_id=..., cl_ord_id=...)` / async |
| Batch cancel | `POST /api/v5/trade/cancel-batch-orders` | list body | `client.cancel_batch_orders(...).await?` | Not exposed |
| Amend order | `POST /api/v5/trade/amend-order` | `instId`, `ordId/clOrdId`, `newSz/newPx` | `client.amend_order(req).await?` | Not exposed |
| Order detail | `GET /api/v5/trade/order` | `instId`, `ordId/clOrdId` | `client.get_order(params).await?` | `client.get_order(inst_id, ord_id=..., cl_ord_id=...)` / async |
| Open orders | `GET /api/v5/trade/orders-pending` | optional `instType`/`instId` | `client.get_orders_pending(params).await?` | `client.get_orders_pending(inst_type=..., inst_id=...)` / async |
| Order history | `GET /api/v5/trade/orders-history` | `instType` required | `client.get_orders_history(params).await?` | Not exposed |
| Fills | `GET /api/v5/trade/fills` | optional `instType/instId` | `client.get_fills(None).await?` | Not exposed |
| Algo order | `POST /api/v5/trade/order-algo` | `instId`, `tdMode`, `side`, `ordType`, `sz`, trigger prices | `client.place_algo_order(req).await?` | Not exposed |
| Cancel algo | `POST /api/v5/trade/cancel-algos` | list body | `client.cancel_algo_orders(vec![req]).await?` | Not exposed |
| Algo pending | `GET /api/v5/trade/orders-algo-pending` | `algoId/state` etc. | `client.get_algo_orders_pending(params).await?` | Not exposed |
| Algo history | `GET /api/v5/trade/orders-algo-history` | `state` etc. | `client.get_algo_orders_history(params).await?` | Not exposed |
| Close position | `POST /api/v5/trade/close-position` | `instId`, `mgnMode` | `client.close_position(req).await?` | Not exposed |

### Funding
| Need | Official path | Key params | Rust usage | Python usage |
| ---- | ------------- | ---------- | ---------- | ------------ |
| Asset balances | `GET /api/v5/asset/balances` | `ccy` optional | `client.get_asset_balances(Some("USDT")).await?` | Not exposed |
| Deposit address | `GET /api/v5/asset/deposit-address` | `ccy` | `client.get_deposit_address("USDT").await?` | Not exposed |
| Deposit history | `GET /api/v5/asset/deposit-history` | `ccy` optional | `client.get_deposit_history(Some("USDT")).await?` | Not exposed |
| Withdrawal history | `GET /api/v5/asset/withdrawal-history` | `ccy` optional | `client.get_withdrawal_history(Some("USDT")).await?` | Not exposed |
| Transfer | `POST /api/v5/asset/transfer` | `ccy`, `amt`, `from`, `to` | `client.funds_transfer(request).await?` | Not exposed |
| Withdrawal | `POST /api/v5/asset/withdrawal` | `ccy`, `amt`, `dest`, `toAddr` | `client.withdrawal(request).await?` | Not exposed |
| Currencies list | `GET /api/v5/asset/currencies` | `ccy` optional | `client.get_currencies(Some("USDT")).await?` | Not exposed |

### Public data
| Need | Official path | Key params | Rust usage | Python usage |
| ---- | ------------- | ---------- | ---------- | ------------ |
| Instruments | `GET /api/v5/public/instruments` | `instType` required, optional `uly/instId` | `client.get_instruments(params).await?` | `client.get_instruments("SPOT", inst_id=None)` / async |
| Funding rate | `GET /api/v5/public/funding-rate` | `instId` required | `client.get_funding_rate("BTC-USDT-SWAP").await?` | Not exposed |
| Funding rate history | `GET /api/v5/public/funding-rate-history` | `instId` required, optional `before/after/limit` | `client.get_funding_rate_history(params).await?` | Not exposed |
| Mark price | `GET /api/v5/public/mark-price` | `instType` required, optional `uly/instId` | `client.get_mark_price(params).await?` | Not exposed |
| Server time | `GET /api/v5/public/time` | none | `client.get_system_time().await?` | `client.get_system_time()` / async |

## WebSocket client `okx-ws`
- **Channel enum** (`channel.rs`): mirrors official channel names (tickers/books/books5/books50-l2-tbt/books-l2-tbt/trades/candle1m/5m/15m/1H/4H/1D/mark-price/index-tickers/funding-rate/account/positions/orders/orders-algo/balance_and_position) with `is_private` and `name`.
- **WsClient** (`client.rs`)
  - Connect: `connect_public(config)` / `connect_private(config)`.
  - Private login: `login()` (uses `Signer::generate_ws_login_params`).
  - Subscribe/unsubscribe: `subscribe(Vec<Channel>)` / `unsubscribe(Vec<Channel>)` (auto-login for private channels).
  - Maintenance: `ping()`, `close()`; implements `Stream<Item = Result<WsMessage>>`.
- **WsMessage/WsEvent** (`message.rs`): `Data { channel, arg, data }`, `Event { event, code, msg, ... }`, `Pong`, `Unknown`.
- **Auto-reconnect** (`reconnect.rs`)
  - `ReconnectConfig`: `initial_delay`/`max_delay`/`backoff_multiplier`/`max_attempts`/`restore_subscriptions`.
  - `ReconnectingWsClient::connect(config, ConnectionType, reconnect_config)`: tracks subscriptions and exposes `state()` / `is_connected()` / `subscription_count()`.
- **Examples**: `crates/okx-rest/examples/ws_public.rs` for public subscriptions; Python bindings reference implementation in `crates/okx-py/src/ws_client.rs`.

### WebSocket common subs
| Need | Channel | Params | Rust (WsClient) | Python (WsClient binding) |
| ---- | ------- | ------ | ---------------- | ------------------------- |
| Spot/perp ticker | `tickers` | `instId=BTC-USDT` | `client.subscribe(vec![Channel::Tickers { inst_id: "BTC-USDT".into() }]).await?` | `await ws.subscribe_tickers("BTC-USDT")` |
| Order book (full) | `books` | `instId=BTC-USDT` | `Channel::Books { inst_id: ... }` | `await ws.subscribe_orderbook("BTC-USDT")` |
| Order book 5 | `books5` | `instId=BTC-USDT` | `Channel::Books5 { ... }` | extend binding |
| Order book 50 L2 TBT | `books50-l2-tbt` | `instId=BTC-USDT-SWAP` | `Channel::Books50L2Tbt { ... }` | extend binding |
| Trades | `trades` | `instId=BTC-USDT` | `Channel::Trades { ... }` | `await ws.subscribe_trades("BTC-USDT")` |
| Candles | `candle1m/5m/15m/1H/4H/1D` | `instId=BTC-USDT` | `Channel::Candle1m { ... }` etc. | `await ws.subscribe_candles("BTC-USDT", interval="1m")` |
| Mark price | `mark-price` | `instId=BTC-USDT-SWAP` | `Channel::MarkPrice { ... }` | extend binding |
| Index ticker | `index-tickers` | `instId=BTC-USD` | `Channel::IndexTickers { ... }` | extend binding |
| Funding rate | `funding-rate` | `instId=BTC-USDT-SWAP` | `Channel::FundingRate { ... }` | extend binding |
| Account (private) | `account` | `ccy=USDT` optional | `Channel::Account { ccy: Some("USDT".into()) }` | `await ws.subscribe_account(ccy="USDT")` |
| Positions (private) | `positions` | `instType=SWAP`, optional `instId` | `Channel::Positions { inst_type: "...".into(), inst_family: None, inst_id: Some("BTC-USDT-SWAP".into()) }` | `await ws.subscribe_positions("SWAP", inst_id="BTC-USDT-SWAP")` |
| Orders (private) | `orders` | `instType=SPOT/SWAP...` | `Channel::Orders { inst_type: "...".into(), inst_family: None, inst_id: None }` | `await ws.subscribe_orders("SPOT")` |
| Algo orders (private) | `orders-algo` | `instType=...` | `Channel::OrdersAlgo { ... }` | extend binding |
| Balance & position | `balance_and_position` | none | `Channel::BalanceAndPosition` | extend binding |

## Running & Docs
- Build/test: `just build` / `just test` / `just clippy`.
- Rust docs: `cargo doc --all --no-deps`; hand-written summaries here and in `docs/zh/rust-api.md`.
- All paths/fields map to the official docs sections “Trading Account REST API”, “Market Data”, “WebSocket API”, etc.; see `endpoints` modules for the exact constants in code.

#### Params reference (Market)
- `GET /market/tickers` (GetTickersParams): `instType` (SPOT/SWAP/FUTURES/OPTION), `uly` (optional underlying), `instFamily` (optional).
- `GET /market/ticker` (GetTickerParams): `instId` (required).
- `GET /market/books` (GetOrderBookParams): `instId` (required), `sz` (optional depth, string, <=400).
- `GET /market/candles` (GetCandlesParams): `instId` (required), `bar` (optional 1m/5m/15m/1H/4H/1D etc.), `after`/`before` (optional timestamps), `limit` (optional, <=300).
- `GET /market/trades` (GetTradesParams): `instId` (required), `limit` (optional, <=500).
- `GET /market/index-tickers` (GetIndexTickersParams): `quoteCcy` (optional), `instId` (optional, e.g., BTC-USD).

#### Params reference (Account)
- `GET /account/balance`: `ccy` (optional, up to 20 comma-separated).
- `GET /account/positions` (GetPositionsParams): `instType`, `instId`, `posId` optional.
- `POST /account/set-leverage` (SetLeverageRequest): `instId` (isolated) / `ccy` (cross), `lever`, `mgnMode` (cross/isolated), `posSide` optional.
- `GET /account/leverage-info` (GetLeverageInfoParams): `mgnMode` (required), `ccy` optional, `instId` optional.
- `GET /account/max-size` (GetMaxSizeParams): `instId`, `tdMode` (cash/cross/isolated), optional `ccy`/`px`/`leverage`.
- `GET /account/max-avail-size` (GetMaxAvailSizeParams): `instId`, `tdMode`, optional `ccy`/`reduceOnly`/`quickMgnType`.
- `GET /account/trade-fee` (GetFeeRatesParams): `instType` (required), optional `instId`/`uly`/`instFamily`.
- `POST /account/set-position-mode`: `posMode` (one_way_mode/long_short_mode).

#### Params reference (Trade)
- `POST /trade/order` (PlaceOrderRequest key fields): `instId` (req), `tdMode` (req: cash/cross/isolated), `side` (buy/sell), `ordType` (limit/market/post_only/fok/ioc...), `sz` (req), `px` (req for limit), `clOrdId` (opt), plus opt `posSide/ccy/tag` and TP/SL fields.
- `POST /trade/batch-orders`: array of the above.
- `POST /trade/cancel-order` (CancelOrderRequest): `instId` (req), one of `ordId` or `clOrdId`.
- `POST /trade/cancel-batch-orders`: array of the above.
- `POST /trade/amend-order` (AmendOrderRequest): `instId`, `ordId/clOrdId`, optional `newSz`/`newPx`.
- `GET /trade/order` (GetOrderParams): `instId` (req), `ordId` or `clOrdId`.
- `GET /trade/orders-pending` (GetOrdersPendingParams): optional `instType`/`uly`/`instId`/`after`/`before`/`limit`.
- `GET /trade/orders-history` (GetOrdersHistoryParams): `instType` (req), optional `uly`/`instId`/`after`/`before`/`limit`.
- `GET /trade/fills` (GetFillsParams): optional `instType`/`uly`/`instId`/`after`/`before`/`limit`.
- Algo orders (PlaceAlgoOrderRequest core): `instId`, `tdMode`, `side`, `ordType` (trigger/iceberg/TWAP etc.), `sz`, trigger/exec prices (`tpTriggerPx`/`tpOrdPx`/`slTriggerPx`/`slOrdPx` etc.); cancel/query use algoId/state params.
- `POST /trade/close-position` (ClosePositionRequest): `instId` (req), `mgnMode` (req), optional `posSide`/`ccy`.

#### Params reference (Funding)
- `GET /asset/balances`: `ccy` optional.
- `GET /asset/deposit-address`: `ccy` required.
- `GET /asset/deposit-history` / `withdrawal-history`: `ccy` optional.
- `POST /asset/transfer` (FundsTransferRequest key fields): `ccy`, `amt`, `from`, `to`, optional `subAcct`/`instId`/`toInstId` etc.
- `POST /asset/withdrawal` (WithdrawalRequest key fields): `ccy`, `amt`, `dest`, `toAddr`, optional `chain`/`fee`/`clientId` etc.
- `GET /asset/currencies`: `ccy` optional.

#### Params reference (Public)
- `GET /public/instruments` (GetInstrumentsParams): `instType` (req), optional `uly`/`instId`/`instFamily`/`optType`.
- `GET /public/funding-rate`: `instId` (req).
- `GET /public/funding-rate-history` (GetFundingRateHistoryParams): `instId` (req), optional `before`/`after`/`limit`.
- `GET /public/mark-price` (GetMarkPriceParams): `instType` (req: SWAP/FUTURES/OPTION), optional `uly`/`instId`.
- `GET /public/time`: none.
