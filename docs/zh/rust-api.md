# Rust API 说明（okx_rs）

本文基于项目代码与 [OKX 官方文档](https://www.okx.com/docs-v5/) 整理，列出核心类型、可用方法及对应的官方接口路径，避免任何臆测。

## 核心库 `okx-core`
- **Config**（`crates/okx-core/src/config.rs`）
  - 构造：`Config::new(credentials)`，默认生产环境、30s 超时。
  - 环境切换：`simulated(bool)` 切换 WS 到 `wspap.okx.com` 模拟盘。
  - 自定义地址：`with_rest_url` / `with_ws_public_url` / `with_ws_private_url`。
  - 其他：`with_timeout_secs`、`with_proxy`；只读方法 `rest_url`/`ws_public_url`/`ws_private_url`/`is_simulated`/`proxy_url`。
- **Credentials**：`Credentials::new(api_key, secret_key, passphrase)`。
- **Signer**（`okx-core::signer`）：实现官方签名规范 `timestamp + method + requestPath + body` → HMAC-SHA256 → Base64。
  - `generate_headers(method, request_path, body, simulated)`：REST 私有请求头。
  - `generate_public_headers(simulated)`：REST 公共请求头。
  - `generate_ws_login_params()`：WS 登录参数（apiKey/passphrase/timestamp/sign）。
- **OkxError / Result**：统一错误类型，覆盖 HTTP/WS/API/认证/序列化。
- **常量**：`API_VERSION = "v5"`，REST 基址 `REST_API_URL` / `REST_API_URL_AWS`，WS `WS_PUBLIC_URL` / `WS_PRIVATE_URL` 及模拟盘常量。
- **通用类型**（`okx-core::types`）：`ApiResponse<T>`（`code`/`msg`/`data`），枚举 `InstType`/`TdMode`/`Side`/`PosSide`/`OrdType` 等。
- **业务类型**：
  - Account：`Balance`、`BalanceDetail`、`Position`、`AccountConfig`。
  - Trade：`Order`、`Fill`、`AlgoOrder` 及请求/响应 `PlaceOrderRequest`、`CancelOrderRequest`、`AmendOrderRequest`、`PlaceAlgoOrderRequest` 等。
  - Funding：`AssetBalance`、`DepositAddress`、`DepositRecord`、`WithdrawalRecord`、`FundsTransferRequest/Response`、`WithdrawalRequest/Response`、`CurrencyInfo`。
  - Market/Public：`Ticker`、`OrderBook`、`BookLevel`、`Candle`、`Trade`、`Instrument`、`IndexTicker`、`MarkPrice`、`FundingRate` 等。

## REST 客户端 `okx-rest`
- **OkxRestClient**（`crates/okx-rest/src/client.rs`）
  - 构造：`OkxRestClient::new(config)`；也可 `with_http_client` 注入自定义 `reqwest::Client`。
  - 公共 GET：`get_public(path, params)` → 自动拼接查询参数、公共头。
  - 私有 GET：`get(path, params)` → 自动签名，`request_path` 含查询串。
  - 私有 POST：`post(path, body)` → 序列化 body，签名后发送。
  - 解析：OKX 响应 `ApiResponse<T>`，`code == "0"` 返回 `Vec<T>`，否则 `OkxError::Api`。
- **API 模块与官方路径**（常量位于各 `endpoints` 模块，均来源于官方文档）
  - Account（`account.rs`）：`/api/v5/account/balance`、`/positions`、`/config`、`/set-leverage`、`/leverage-info`、`/max-size`、`/max-avail-size`、`/trade-fee`、`/set-position-mode`、`/account-position-risk`。参数结构：`GetBalanceParams`、`GetPositionsParams`、`SetLeverageRequest`、`GetLeverageInfoParams`、`GetMaxSizeParams`、`GetMaxAvailSizeParams`、`GetFeeRatesParams` 等。
  - Trade（`trade.rs`）：`/api/v5/trade/order`、`/cancel-order`、`/amend-order`、`/orders-pending`、`/orders-history`、`/fills`、`/order-algo`、`/cancel-algos`、`/algo-orders-pending`、`/algo-orders-history`、`/close-position`。批量接口使用列表体。
  - Funding（`funding.rs`）：`/api/v5/asset/balances`、`/asset/deposit-address`、`/asset/deposit-history`、`/asset/withdrawal-history`、`/asset/transfer`、`/asset/withdrawal`、`/asset/currencies`。
  - Market（`market.rs`）：`/api/v5/market/tickers`、`/ticker`、`/books`（及 books5/books50/books-l2-tbt）、`/candles`、`/trades`、`/index-tickers`。
  - Public（`public.rs`）：`/api/v5/public/instruments`、`/funding-rate`、`/funding-rate-history`、`/system-time`、`/mark-price`。
- **返回值**：所有方法返回 `Result<Vec<T>>` 或对应列表，字段名与官方响应保持一致。
- **示例**：详见仓库 `crates/okx-rest/examples/rest_basic.rs`。

### 市场行情常用调用（对齐官方 Market Data）
| 需求 | 官方路径 | 关键参数 | Rust 用法 | Python 用法 |
| ---- | -------- | -------- | -------- | ----------- |
| 现货单币对价格 | `GET /api/v5/market/ticker` | `instId` 必填（如 BTC-USDT） | `client.get_ticker("BTC-USDT").await?` | `client.get_ticker("BTC-USDT")` / `await aclient.get_ticker("BTC-USDT")` |
| 永续合约价格 | `GET /api/v5/market/ticker` | `instId` 必填（如 BTC-USDT-SWAP） | `client.get_ticker("BTC-USDT-SWAP").await?` | `client.get_ticker("BTC-USDT-SWAP")` / 异步同名 |
| 全部现货报价 | `GET /api/v5/market/tickers` | `instType=SPOT` | `client.get_tickers(GetTickersParams { inst_type: "SPOT".into(), uly: None, inst_family: None }).await?` | `client.get_tickers("SPOT")` / 异步同名 |
| 全部永续报价 | `GET /api/v5/market/tickers` | `instType=SWAP`，可选 `uly` | `client.get_tickers(GetTickersParams { inst_type: "SWAP".into(), uly: Some("BTC-USDT".into()), inst_family: None }).await?` | `client.get_tickers("SWAP")` / 异步同名 |
| 订单簿 | `GET /api/v5/market/books` | `instId` 必填，`sz` 可选 | `client.get_orderbook("BTC-USDT", Some(5)).await?` | `client.get_orderbook("BTC-USDT", depth=5)` / 异步同名 |
| K 线 | `GET /api/v5/market/candles` | `instId` 必填，`bar` 可选 | `client.get_candles(GetCandlesParams { inst_id: "BTC-USDT".into(), bar: Some("1m".into()), after: None, before: None, limit: None }).await?` | `client.get_candles("BTC-USDT", bar="1m")` / 异步同名 |
| 最新成交 | `GET /api/v5/market/trades` | `instId` 必填，`limit` 可选 | `client.get_trades("BTC-USDT", Some(50)).await?` | `client.get_trades("BTC-USDT", limit=50)` / 异步同名 |
| 指数价格 | `GET /api/v5/market/index-tickers` | `quoteCcy`/`instId` 可选 | `client.get_index_tickers(GetIndexTickersParams { quote_ccy: Some("USDT".into()), inst_id: Some("BTC-USD".into()) }).await?` | `client.get_index_tickers(quote_ccy="USDT", inst_id="BTC-USD")` / 异步同名 |

> 注：表内 Python 示例基于当前绑定公开方法（`crates/okx-py/src/client.rs`、`async_client.rs`）。若个别参数未在绑定公开，可参照 Rust 参数结构扩充绑定后再调用；接口路径与参数需与官方文档保持一致。

#### 参数明细（Market）
- `GET /market/tickers`（GetTickersParams）：`instType`(必填: SPOT/SWAP/FUTURES/OPTION)，`uly`(可选: 标的)，`instFamily`(可选)。
- `GET /market/ticker`（GetTickerParams）：`instId`(必填)。
- `GET /market/books`（GetOrderBookParams）：`instId`(必填)，`sz`(可选深度，字符串，<=400)。
- `GET /market/candles`（GetCandlesParams）：`instId`(必填)，`bar`(可选: 1m/5m/15m/1H/4H/1D 等)，`after`/`before`(可选时间戳)，`limit`(可选，<=300)。
- `GET /market/trades`（GetTradesParams）：`instId`(必填)，`limit`(可选，<=500)。
- `GET /market/index-tickers`（GetIndexTickersParams）：`quoteCcy`(可选)，`instId`(可选，如 BTC-USD)。

### Account（交易账户）常用调用
| 需求 | 官方路径 | 关键参数 | Rust 用法 | Python 用法 |
| ---- | -------- | -------- | --------- | ----------- |
| 账户余额 | `GET /api/v5/account/balance` | `ccy` 可选 | `client.get_balance(Some("BTC")).await?` | `client.get_balance(ccy="BTC")` / 异步同名 |
| 持仓信息 | `GET /api/v5/account/positions` | `instType`/`instId` 可选 | `client.get_positions(Some(GetPositionsParams { inst_type: Some("SWAP".into()), inst_id: Some("BTC-USDT-SWAP".into()), pos_id: None })).await?` | `client.get_positions(inst_type="SWAP", inst_id="BTC-USDT-SWAP")` / 异步同名 |
| 账户配置 | `GET /api/v5/account/config` | 无 | `client.get_account_config().await?` | 未暴露 |
| 设置杠杆 | `POST /api/v5/account/set-leverage` | `instId`/`ccy`、`lever`、`mgnMode` | `client.set_leverage(SetLeverageRequest { inst_id: Some("BTC-USDT-SWAP".into()), ccy: None, lever: "5".into(), mgn_mode: "cross".into(), pos_side: None }).await?` | 未暴露 |
| 查询杠杆 | `GET /api/v5/account/leverage-info` | `mgnMode`，可选 `ccy`/`instId` | `client.get_leverage_info(GetLeverageInfoParams { mgn_mode: "cross".into(), ccy: Some("USDT".into()), inst_id: None }).await?` | 未暴露 |
| 最大下单量 | `GET /api/v5/account/max-size` | `instId`、`tdMode` | `client.get_max_size(GetMaxSizeParams { inst_id: "BTC-USDT".into(), td_mode: "cash".into(), ccy: None, px: None, leverage: None }).await?` | 未暴露 |
| 最大可用 | `GET /api/v5/account/max-avail-size` | `instId`、`tdMode` | `client.get_max_avail_size(GetMaxAvailSizeParams { inst_id: "BTC-USDT-SWAP".into(), td_mode: "cross".into(), ccy: None, reduce_only: None, quick_mgn_type: None }).await?` | 未暴露 |
| 手续费率 | `GET /api/v5/account/trade-fee` | `instType` | `client.get_fee_rates(GetFeeRatesParams { inst_type: "SPOT".into(), inst_id: None, uly: None, inst_family: None }).await?` | 未暴露 |
| 持仓模式 | `POST /api/v5/account/set-position-mode` | `posMode` | `client.set_position_mode("long_short_mode").await?` | 未暴露 |
| 风险视图 | `GET /api/v5/account/account-position-risk` | 无 | `client.get_account_position_risk().await?` | 未暴露 |

#### 参数明细（Account）
- `GET /account/balance`：`ccy`(可选，最多 20 个逗号分隔)。
- `GET /account/positions`（GetPositionsParams）：`instType`，`instId`，`posId` 均可选。
- `POST /account/set-leverage`（SetLeverageRequest）：`instId`(逐仓必填) / `ccy`(全仓必填)，`lever`，`mgnMode`(cross/isolated)，`posSide`(可选)。
- `GET /account/leverage-info`（GetLeverageInfoParams）：`mgnMode`(必填)，`ccy`(可选)，`instId`(可选)。
- `GET /account/max-size`（GetMaxSizeParams）：`instId`，`tdMode`(cash/cross/isolated)，`ccy`/`px`/`leverage`(可选)。
- `GET /account/max-avail-size`（GetMaxAvailSizeParams）：`instId`，`tdMode`，`ccy`/`reduceOnly`/`quickMgnType`(可选)。
- `GET /account/trade-fee`（GetFeeRatesParams）：`instType`(必填)，`instId`/`uly`/`instFamily`(可选)。
- `POST /account/set-position-mode`：`posMode`(one_way_mode/long_short_mode)。

### 账户高级与系统状态补充
| 功能 | 官方路径 | 关键参数 | Rust 用法 | Python 用法 |
| ---- | -------- | -------- | --------- | ----------- |
| 调整持仓保证金 | `POST /api/v5/account/position/margin-balance` | `instId`、`posSide`、`type`(add/reduce)、`amt`、`loanTrans` 可选 | `client.adjustment_margin(AdjustmentMarginRequest { inst_id, pos_side, r#type, amt, loan_trans }).await?` | `client.adjustment_margin(inst_id, pos_side, type_, amt, loan_trans=None)` / 异步同名 |
| 设置风险对冲类型 | `POST /api/v5/account/set-riskOffset-type` | `type` | `client.set_risk_offset_type(SetRiskOffsetTypeRequest { r#type }).await?` | `client.set_risk_offset_type(type_)` / 异步同名 |
| 设置自动借币 | `POST /api/v5/account/set-auto-loan` | `autoLoan` 可选字符串 | `client.set_auto_loan(SetAutoLoanRequest { auto_loan }).await?` | `client.set_auto_loan(auto_loan=None)` / 异步同名 |
| 系统状态 | `GET /api/v5/system/status` | `state` 可选（0 正常 / 1 维护） | `client.get_system_status(Some("0")).await?` | `client.get_system_status(state=None)` / 异步同名 |

### Trade（交易下单）常用调用
| 需求 | 官方路径 | 关键参数 | Rust 用法 | Python 用法 |
| ---- | -------- | -------- | --------- | ----------- |
| 下单 | `POST /api/v5/trade/order` | `instId`、`tdMode`、`side`、`ordType`、`sz`、`px`(限价) | `client.place_order(req).await?` | `client.place_order(inst_id, td_mode, side, ord_type, sz, px=..., cl_ord_id=...)` / 异步 |
| 批量下单 | `POST /api/v5/trade/batch-orders` | 列表体 | `client.place_batch_orders(vec![req1, req2]).await?` | 未暴露 |
| 撤单 | `POST /api/v5/trade/cancel-order` | `instId` + (`ordId` 或 `clOrdId`) | `client.cancel_order(req).await?` | `client.cancel_order(inst_id, ord_id=..., cl_ord_id=...)` / 异步 |
| 批量撤单 | `POST /api/v5/trade/cancel-batch-orders` | 列表体 | `client.cancel_batch_orders(vec![...]).await?` | 未暴露 |
| 改单 | `POST /api/v5/trade/amend-order` | `instId`，`ordId/clOrdId`，`newSz`/`newPx` 可选 | `client.amend_order(req).await?` | 未暴露 |
| 订单详情 | `GET /api/v5/trade/order` | `instId` + (`ordId`/`clOrdId`) | `client.get_order(params).await?` | `client.get_order(inst_id, ord_id=..., cl_ord_id=...)` / 异步 |
| 未成交订单 | `GET /api/v5/trade/orders-pending` | `instType` 等可选 | `client.get_orders_pending(params).await?` | `client.get_orders_pending(inst_type=..., inst_id=...)` / 异步 |
| 订单历史 | `GET /api/v5/trade/orders-history` | `instType` 必填 | `client.get_orders_history(params).await?` | 未暴露 |
| 成交明细 | `GET /api/v5/trade/fills` | `instType/instId` 可选 | `client.get_fills(None).await?` 或带参 | 未暴露 |
| 策略下单 | `POST /api/v5/trade/order-algo` | `instId`、`tdMode`、`side`、`ordType`、`sz`、触发价/委托价等 | `client.place_algo_order(req).await?` | 未暴露 |
| 策略撤单 | `POST /api/v5/trade/cancel-algos` | 列表体 | `client.cancel_algo_orders(vec![req]).await?` | 未暴露 |
| 策略当前 | `GET /api/v5/trade/orders-algo-pending` | `algoId/state` 等 | `client.get_algo_orders_pending(params).await?` | 未暴露 |
| 策略历史 | `GET /api/v5/trade/orders-algo-history` | `state` 等 | `client.get_algo_orders_history(params).await?` | 未暴露 |
| 平仓 | `POST /api/v5/trade/close-position` | `instId`、`mgnMode` | `client.close_position(req).await?` | 未暴露 |

#### 参数明细（Trade）
- `POST /trade/order`（PlaceOrderRequest 关键字段）：`instId`(必填)、`tdMode`(必填: cash/cross/isolated)、`side`(buy/sell)、`ordType`(limit/market/post_only/fok/ioc 等)、`sz`(必填)、`px`(限价必填)、`clOrdId`(可选)、`posSide/ccy/tag` 等可选。
- `POST /trade/batch-orders`：数组体，元素同上。
- `POST /trade/cancel-order`（CancelOrderRequest）：`instId`(必填)，`ordId` 或 `clOrdId` 至少一个。
- `POST /trade/cancel-batch-orders`：数组体，元素同上。
- `POST /trade/amend-order`（AmendOrderRequest）：`instId`，`ordId/clOrdId`，可改 `newSz`/`newPx` 等。
- `GET /trade/order`（GetOrderParams）：`instId`(必填)，`ordId` 或 `clOrdId`。
- `GET /trade/orders-pending`（GetOrdersPendingParams）：可选 `instType`/`uly`/`instId`/`after`/`before`/`limit`。
- `GET /trade/orders-history`（GetOrdersHistoryParams）：`instType`(必填)，可选 `uly`/`instId`/`after`/`before`/`limit`。
- `GET /trade/fills`（GetFillsParams）：可选 `instType`/`uly`/`instId`/`after`/`before`/`limit`。
- 策略委托（PlaceAlgoOrderRequest 核心字段）：`instId`、`tdMode`、`side`、`ordType`（触发/冰山/时间加权等类型），`sz`，触发价/委托价字段（`tpTriggerPx`/`tpOrdPx`/`slTriggerPx`/`slOrdPx` 等）按官方文档要求填写；取消/查询接口使用对应 algoId/状态参数。
- `POST /trade/close-position`（ClosePositionRequest）：`instId`(必填)，`mgnMode`(必填)，可选 `posSide`/`ccy`。

### Funding（资金）常用调用
| 需求 | 官方路径 | 关键参数 | Rust 用法 | Python 用法 |
| ---- | -------- | -------- | --------- | ----------- |
| 资金余额 | `GET /api/v5/asset/balances` | `ccy` 可选 | `client.get_asset_balances(Some("USDT")).await?` | 未暴露 |
| 充值地址 | `GET /api/v5/asset/deposit-address` | `ccy` | `client.get_deposit_address("USDT").await?` | 未暴露 |
| 充值记录 | `GET /api/v5/asset/deposit-history` | `ccy` 可选 | `client.get_deposit_history(Some("USDT")).await?` | 未暴露 |
| 提币记录 | `GET /api/v5/asset/withdrawal-history` | `ccy` 可选 | `client.get_withdrawal_history(Some("USDT")).await?` | 未暴露 |
| 划转 | `POST /api/v5/asset/transfer` | `ccy`、`amt`、`from`、`to` | `client.funds_transfer(request).await?` | 未暴露 |
| 提币 | `POST /api/v5/asset/withdrawal` | `ccy`、`amt`、`dest`、`toAddr` | `client.withdrawal(request).await?` | 未暴露 |
| 币种列表 | `GET /api/v5/asset/currencies` | `ccy` 可选 | `client.get_currencies(Some("USDT")).await?` | 未暴露 |

#### 参数明细（Funding）
- `GET /asset/balances`：`ccy`(可选)。
- `GET /asset/deposit-address`：`ccy`(必填)。
- `GET /asset/deposit-history` / `withdrawal-history`：`ccy`(可选)。
- `POST /asset/transfer`（FundsTransferRequest 核心字段）：`ccy`，`amt`，`from`，`to`，可选 `subAcct`/`instId`/`toInstId` 等。
- `POST /asset/withdrawal`（WithdrawalRequest 核心字段）：`ccy`，`amt`，`dest`，`toAddr`，可选 `chain`/`fee`/`clientId` 等。
- `GET /asset/currencies`：`ccy`(可选)。

### Public（公共数据）常用调用
| 需求 | 官方路径 | 关键参数 | Rust 用法 | Python 用法 |
| ---- | -------- | -------- | --------- | ----------- |
| 合约/币对列表 | `GET /api/v5/public/instruments` | `instType` 必填，`uly/instId` 可选 | `client.get_instruments(params).await?` | `client.get_instruments("SPOT", inst_id=None)` / 异步 |
| 资金费率 | `GET /api/v5/public/funding-rate` | `instId` 必填 | `client.get_funding_rate("BTC-USDT-SWAP").await?` | 未暴露 |
| 资金费率历史 | `GET /api/v5/public/funding-rate-history` | `instId` 必填，`before/after/limit` 可选 | `client.get_funding_rate_history(params).await?` | 未暴露 |
| 标记价格 | `GET /api/v5/public/mark-price` | `instType` 必填，`uly/instId` 可选 | `client.get_mark_price(params).await?` | 未暴露 |
| 服务器时间 | `GET /api/v5/public/time` | 无 | `client.get_system_time().await?` | `client.get_system_time()` / 异步 |

#### 参数明细（Public）
- `GET /public/instruments`（GetInstrumentsParams）：`instType`(必填)，`uly`/`instId`/`instFamily`/`optType`/`uly` 等可选。
- `GET /public/funding-rate`：`instId`(必填)。
- `GET /public/funding-rate-history`（GetFundingRateHistoryParams）：`instId`(必填)，`before`/`after`/`limit` 可选。
- `GET /public/mark-price`（GetMarkPriceParams）：`instType`(必填: SWAP/FUTURES/OPTION)，可选 `uly`/`instId`。
- `GET /public/time`：无参数。

## WebSocket 客户端 `okx-ws`
- 频道枚举 `Channel`（`channel.rs`）：与官方 WS 频道同名（tickers/books/books5/books50-l2-tbt/books-l2-tbt/trades/candle1m/5m/15m/1H/4H/1D/mark-price/index-tickers/funding-rate/account/positions/orders/orders-algo/balance_and_position）。
- 基础客户端 `WsClient`：`connect_public` / `connect_private`；`subscribe`/`unsubscribe`；私有需要 `login`（自动使用 `Signer::generate_ws_login_params`）；实现 `Stream<Item = Result<WsMessage>>`，`WsMessage::Data` / `Event` / `Pong`。
- 自动重连 `ReconnectingWsClient`：`connect(config, ConnectionType, reconnect_config)`，跟踪订阅，支持 `state()`、`is_connected()`、`subscription_count()`，断线恢复订阅。

### WebSocket 常用订阅
| 需求 | 官方频道 | 参数示例 | Rust 用法（WsClient） | Python 用法（WsClient 绑定） |
| ---- | -------- | -------- | --------------------- | --------------------------- |
| 现货/合约 Ticker | `tickers` | `instId=BTC-USDT` | `client.subscribe(vec![Channel::Tickers { inst_id: "BTC-USDT".into() }]).await?` | `await ws.subscribe_tickers("BTC-USDT")` |
| 深度（全量） | `books` | `instId=BTC-USDT` | `Channel::Books { inst_id: ... }` | `await ws.subscribe_orderbook("BTC-USDT")` |
| 深度（5 档） | `books5` | `instId=BTC-USDT` | `Channel::Books5 { ... }` | 需在绑定中补充 |
| 深度（50 档 L2 TBT） | `books50-l2-tbt` | `instId=BTC-USDT-SWAP` | `Channel::Books50L2Tbt { ... }` | 需在绑定中补充 |
| 逐笔成交 | `trades` | `instId=BTC-USDT` | `Channel::Trades { ... }` | `await ws.subscribe_trades("BTC-USDT")` |
| K 线 | `candle1m/5m/15m/1H/4H/1D` | `instId=BTC-USDT` | `Channel::Candle1m { ... }` 等 | `await ws.subscribe_candles("BTC-USDT", interval="1m")` |
| 标记价格 | `mark-price` | `instId=BTC-USDT-SWAP` | `Channel::MarkPrice { ... }` | 需在绑定中补充 |
| 指数 Ticker | `index-tickers` | `instId=BTC-USD` | `Channel::IndexTickers { ... }` | 需在绑定中补充 |
| 资金费率 | `funding-rate` | `instId=BTC-USDT-SWAP` | `Channel::FundingRate { ... }` | 需在绑定中补充 |
| 私有账户 | `account` | `ccy=USDT` 可选 | `Channel::Account { ccy: Some("USDT".into()) }` | `await ws.subscribe_account(ccy="USDT")` |
| 私有持仓 | `positions` | `instType=SWAP`, 可选 `instId` | `Channel::Positions { inst_type: "...".into(), inst_family: None, inst_id: Some("BTC-USDT-SWAP".into()) }` | `await ws.subscribe_positions("SWAP", inst_id="BTC-USDT-SWAP")` |
| 私有订单 | `orders` | `instType=SPOT/SWAP...` | `Channel::Orders { inst_type: "...".into(), inst_family: None, inst_id: None }` | `await ws.subscribe_orders("SPOT")` |
| 私有策略订单 | `orders-algo` | `instType=...` | `Channel::OrdersAlgo { ... }` | 需在绑定中补充 |
| 余额与持仓合并 | `balance_and_position` | 无 | `Channel::BalanceAndPosition` | 需在绑定中补充 |

## WebSocket 客户端 `okx-ws`
- **Channel 枚举**（`channel.rs`）：与官方 WS 频道同名（tickers/books/books5/books50-l2-tbt/books-l2-tbt/trades/candle1m/5m/15m/1H/4H/1D/mark-price/index-tickers/funding-rate/account/positions/orders/orders-algo/balance_and_position），提供 `is_private` 与 `name`。
- **WsClient**（`client.rs`）
  - 连接：`connect_public(config)` / `connect_private(config)`。
  - 私有登录：`login()`（自动使用 `Signer::generate_ws_login_params`）。
  - 订阅/退订：`subscribe(Vec<Channel>)` / `unsubscribe(Vec<Channel>)`（私有频道会自动登录）。
  - 维护：`ping()`、`close()`；实现 `Stream<Item = Result<WsMessage>>`。
- **WsMessage/WsEvent**（`message.rs`）：`Data { channel, arg, data }`、`Event { event, code, msg, ... }`、`Pong`、`Unknown`。
- **自动重连**（`reconnect.rs`）
  - `ReconnectConfig`：`initial_delay`/`max_delay`/`backoff_multiplier`/`max_attempts`/`restore_subscriptions`。
  - `ReconnectingWsClient::connect(config, ConnectionType, reconnect_config)`：内置订阅恢复与状态查询 `state()` / `is_connected()` / `subscription_count()`。
- **示例**：`crates/okx-rest/examples/ws_public.rs` 展示订阅 tickers，`crates/okx-py/src/ws_client.rs` 为 Python 绑定实现示例。

## 运行与文档
- 构建与测试：`just build` / `just test` / `just clippy`。
- Rust 文档生成：`cargo doc --all --no-deps`；手写补充见 `docs/en/rust-api.md` / `docs/zh/rust-api.md`。
- 真实接口与字段可对照官方文档章节 “Trading Account REST API”、“Market Data”、“WebSocket API” 等，当前代码中所有路径均在对应 `endpoints` 模块明确定义。
