# Python 文档（okx_py）

基于 PyO3 对 `okx-rest` 与 `okx-ws` 的封装，接口与官方 OKX 文档路径一一对应（参见 https://www.okx.com/docs-v5/）。本节列出公开的 Python 类与方法，全部源自代码，无臆测。

## 安装
本地开发（推荐使用 `uv` + `maturin`，Python 3.9+）：
```bash
just py-setup      # 创建 .venv 并安装依赖
just py-build      # maturin develop 安装 okx-py
```
PyPI 发布准备见 `docs/zh/release.md`。

## 导出的核心类型
- `Credentials(api_key, secret_key, passphrase)`：API 密钥。
- `Config(credentials, simulated=False, timeout_secs=30, proxy_url=None, rest_url=None, ws_public_url=None, ws_private_url=None)`：客户端配置，底层对应 `okx-core::Config`。
- 业务类型均来自 Rust `okx-core::types`，经 PyO3 转换后为 Python 对象（如 `Balance`, `Position`, `Order`, `Ticker` 等），字段名保持官方响应命名（camelCase）。

## 异常类型
SDK 提供分层的异常类型，便于精细化错误处理：
- `OkxError`：所有 OKX 相关异常的基类
- `OkxHttpError`：HTTP 响应错误（非 2xx 状态码）
- `OkxRateLimitError`：HTTP 429 限速错误（继承自 `OkxHttpError`）
- `OkxApiError`：OKX API 业务错误（code 非 0）
- `OkxAuthError`：认证错误
- `OkxWebSocketError`：WebSocket 连接错误
- `OkxTimeoutError`：请求超时

示例：
```python
from okx_py import OkxClient, OkxRateLimitError, OkxApiError

try:
    client.place_order(...)
except OkxRateLimitError:
    # 处理限速，等待后重试
    time.sleep(1)
except OkxApiError as e:
    # 处理业务错误
    print(f"API 错误: {e}")
```

## 同步 REST 客户端 `OkxClient`
来源：`crates/okx-py/src/client/mod.rs`（绑定入口）与 `crates/okx-py/src/client/*.rs`（按业务域拆分）。所有方法返回 Python 对象/字典列表，与官方 REST 路径一致：
- `get_balance(ccy=None)` → `/api/v5/account/balance`
- `get_positions(inst_type=None, inst_id=None)` → `/api/v5/account/positions`
- `place_order(inst_id, td_mode, side, ord_type, sz, px=None, cl_ord_id=None)` → `/api/v5/trade/order`
- `cancel_order(inst_id, ord_id=None, cl_ord_id=None)` → `/api/v5/trade/cancel-order`
- `get_order(inst_id, ord_id=None, cl_ord_id=None)` → `/api/v5/trade/order`
- `get_orders_pending(inst_type=None, inst_id=None)` → `/api/v5/trade/orders-pending`
- `get_account_instruments(inst_type, inst_family=None, inst_id=None)` → `/api/v5/account/instruments`
- `get_account_risk_state()` → `/api/v5/account/risk-state`
- 公共行情：
  - `get_ticker(inst_id)` → `/api/v5/market/ticker`
  - `get_tickers(inst_type)` → `/api/v5/market/tickers`
  - `get_platform_24_volume()` → `/api/v5/market/platform-24-volume`
  - `get_index_components(index)` → `/api/v5/market/index-components`
  - `get_exchange_rate()` → `/api/v5/market/exchange-rate`
  - `get_instruments(inst_type, inst_id=None)` → `/api/v5/public/instruments`
- 公共服务：`get_system_time()` → `/api/v5/public/time`
- 公共数据：
  - `get_instrument_tick_bands(inst_type, inst_family=None)` → `/api/v5/public/instrument-tick-bands`
  - `get_option_trades(inst_id=None, inst_family=None, opt_type=None)` → `/api/v5/public/option-trades`
- Finance：
  - `saving_lending_rate_history(params_json=None)` → `/api/v5/finance/savings/lending-rate-history`
  - `flexible_loan_*` → `/api/v5/finance/flexible-loan/*`
  - `staking_defi_eth_*` / `staking_defi_sol_*` → `/api/v5/finance/staking-defi/{eth,sol}/*`
- 一键还债（v2）：
  - `get_one_click_repay_currency_list_v2()` → `/api/v5/trade/one-click-repay-currency-list-v2`
  - `one_click_repay_v2(debt_ccy, repay_ccy_list)` → `/api/v5/trade/one-click-repay-v2`
  - `get_one_click_repay_history_v2(after=None, before=None, limit=None)` → `/api/v5/trade/one-click-repay-history-v2`

## 异步 REST 客户端 `AsyncOkxClient`
来源：`crates/okx-py/src/async_client/mod.rs`（绑定入口）与 `crates/okx-py/src/async_client/*.rs`（按业务域拆分）。方法集合与 `OkxClient` 对齐，返回 `await` 后的结果：
- `get_balance`, `get_positions`, `place_order`, `cancel_order`, `get_order`, `get_orders_pending`
- 公共行情：`get_ticker`, `get_tickers`, `get_instruments`
- 公共服务：`get_system_time`

## 官方兼容返回（Raw Response）

为对齐官方 `python-okx` “返回完整 JSON（code/msg/data）” 的使用习惯，Python 绑定额外提供 raw 形式的通用接口（不影响既有类型化返回）：

- 同步 `OkxClient`：
  - `get_public_raw(path, params_json=None)`：公共 GET，返回完整 JSON 字典
  - `get_private_raw(path, params_json=None)`：私有 GET，返回完整 JSON 字典
  - `post_private_raw(path, body_json)`：私有 POST，返回完整 JSON 字典
- 异步 `AsyncOkxClient`：提供同名方法（返回 awaitable）。

## 覆盖范围与缺失
- 已暴露（REST）：账户余额、持仓、下单/撤单/查单、待成交、单/多 ticker、合约列表、服务器时间。
- 已暴露（高级 REST）：账户配置、杠杆/保证金工具、手续费率、订单改价/批量、策略委托、资金（划转/充值/提现等）、资金费率/标记价格，新增系统状态、调保证金、风险对冲类型、自动借币。
- 未暴露（REST）：官方仍在更新的少量细分端点如部分经纪特性，可按需在绑定中继续扩展。
- WebSocket：公共/私有频道订阅已暴露，支持自动重连；如需补充更多频道参数，可在 `ws_client.rs` 中扩展。

## REST 已暴露方法速查（官方路径）
| 用途 | 官方路径 | Python 调用 | 参数 |
| ---- | -------- | ----------- | ---- |
| 账户余额 | `GET /api/v5/account/balance` | `client.get_balance(ccy=None)` / `await aclient.get_balance()` | `ccy` 可选，逗号分隔 |
| 持仓信息 | `GET /api/v5/account/positions` | `client.get_positions(inst_type=None, inst_id=None)` / 异步同名 | `inst_type`、`inst_id` 可选 |
| 下单 | `POST /api/v5/trade/order` | `client.place_order(inst_id, td_mode, side, ord_type, sz, px=None, cl_ord_id=None, …)` / 异步同名 | 必填 `inst_id, td_mode, side, ord_type, sz`；其余可选 |
| 撤单 | `POST /api/v5/trade/cancel-order` | `client.cancel_order(inst_id, ord_id=None, cl_ord_id=None)` / 异步同名 | `inst_id` 必填，`ord_id` 与 `cl_ord_id` 二选一 |
| 查单 | `GET /api/v5/trade/order` | `client.get_order(inst_id, ord_id=None, cl_ord_id=None)` / 异步同名 | 同上 |
| 待成交列表 | `GET /api/v5/trade/orders-pending` | `client.get_orders_pending(inst_type=None, inst_id=None, …)` / 异步同名 | `inst_type` 必填，`inst_id/uly/inst_family/after/before/limit` 可选 |
| 单一 ticker | `GET /api/v5/market/ticker` | `client.get_ticker(inst_id)` / 异步同名 | `inst_id` 必填 |
| 全部 ticker（按类型） | `GET /api/v5/market/tickers` | `client.get_tickers(inst_type)` / 异步同名 | `inst_type` 必填 |
| 合约/币对列表 | `GET /api/v5/public/instruments` | `client.get_instruments(inst_type, inst_id=None)` / 异步同名 | `inst_type` 必填，`inst_id` 可选 |
| 服务器时间 | `GET /api/v5/public/time` | `client.get_system_time()` / 异步同名 | 无 |
| 系统状态 | `GET /api/v5/system/status` | `client.get_system_status(state=None)` / 异步同名 | `state` 可选（`0` 正常，`1` 维护） |
| 调整持仓保证金 | `POST /api/v5/account/position/margin-balance` | `client.adjustment_margin(inst_id, pos_side, type_, amt, loan_trans=None)` / 异步同名 | `inst_id`、`pos_side`、`type_`(`add/reduce`)、`amt`，`loan_trans` 可选 |
| 设置风险对冲类型 | `POST /api/v5/account/set-riskOffset-type` | `client.set_risk_offset_type(type_)` / 异步同名 | `type_` 必填 |
| 设置自动借币 | `POST /api/v5/account/set-auto-loan` | `client.set_auto_loan(auto_loan=None)` / 异步同名 | `auto_loan` 可选字符串（官方值） |

## REST 未暴露但已在 Rust 实现（可扩展绑定）
- 账户：账户配置、杠杆信息/设置、最大下单量、最大可用、手续费率、持仓模式、风险视图等。
- 交易：批量下单/撤单、改单、订单历史、成交明细、策略委托全套（下单/撤单/查询）、平仓。
- 资金：资金余额、划转、充值地址/记录、提现/记录、币种列表等。
- 公共：资金费率/历史、标记价格等。

## WebSocket 客户端 `WsClient`
来源：`crates/okx-py/src/ws_client.rs`，基于 `ReconnectingWsClient`（自动重连与订阅恢复）。
- 连接：`await WsClient.connect_public(config, max_reconnect_attempts=None)` / `connect_private(...)`
- 订阅公共频道：
  - `subscribe_tickers(inst_id)`（tickers）
  - `subscribe_orderbook(inst_id)`（books）
  - `subscribe_trades(inst_id)`（trades）
  - `subscribe_candles(inst_id, interval="1m")`（1m/5m/15m/1H/4H/1D 对应官方 candle 频道）
- 订阅私有频道：
  - `subscribe_account(ccy=None)`（account）
  - `subscribe_positions(inst_type, inst_id=None)`（positions）
  - `subscribe_orders(inst_type, inst_id=None)`（orders）
- 接收消息：`await client.recv()` 返回 dict（type=data/event/pong/channel_conn_count/channel_conn_count_error/unknown）。实现了 `async for msg in client` 迭代。
- 状态控制：`is_connected()`、`reconnect()`、`close()`、`subscription_count()`。
- 外部时间戳登录：`login_with_timestamp(timestamp_unix)` - 使用服务器同步的时间戳登录私有 WebSocket，解决客户端与服务器时钟偏移问题。

## WebSocket 已暴露订阅
| 用途 | 频道 | Python 调用 |
| ---- | ---- | ----------- |
| Ticker | `tickers` | `await ws.subscribe_tickers("BTC-USDT")` |
| 订单簿 | `books` | `await ws.subscribe_orderbook("BTC-USDT")` |
| 成交 | `trades` | `await ws.subscribe_trades("BTC-USDT")` |
| K 线 | `candle1m/5m/15m/1H/4H/1D` | `await ws.subscribe_candles("BTC-USDT", interval="1m")` |
| 私有账户 | `account` | `await ws.subscribe_account(ccy=None)` |
| 私有持仓 | `positions` | `await ws.subscribe_positions(inst_type, inst_id=None)` |
| 私有订单 | `orders` | `await ws.subscribe_orders(inst_type, inst_id=None)` |

## WebSocket 未暴露频道（可扩展绑定）
已补齐并暴露以下频道方法（见 `crates/okx-py/src/ws_client.rs` 与 `crates/okx-ws` 对应实现）：
- 深度精简频道：`books5`、`books50-l2-tbt`、`books-l2-tbt`
- 指数/资金费率/标记价格：`index-tickers`、`funding-rate`、`mark-price`
- 策略订单：`orders-algo`
- 余额与持仓合并：`balance_and_position`

## 常用行情示例（对齐官方 Market Data）
- 现货单币对价格：`get_ticker("BTC-USDT")` → `GET /api/v5/market/ticker`
- 永续价格：`get_ticker("BTC-USDT-SWAP")` → `GET /api/v5/market/ticker`
- 全部现货报价：`get_tickers("SPOT")` → `GET /api/v5/market/tickers`
- 全部永续报价：`get_tickers("SWAP")` → `GET /api/v5/market/tickers`
- 订单簿：`get_orderbook("BTC-USDT", depth=5)` → `GET /api/v5/market/books`（如绑定未暴露 depth，可在绑定中补充参数再调用）
- K 线：`get_candles("BTC-USDT", bar="1m")` → `GET /api/v5/market/candles`
- 最新成交：`get_trades("BTC-USDT", limit=50)` → `GET /api/v5/market/trades`
- 指数价格：`get_index_tickers(quote_ccy="USDT", inst_id="BTC-USD")` → `GET /api/v5/market/index-tickers`

## 示例
- 快速片段（同步）：
  ```python
  from okx_py import OkxClient, Config, Credentials

  cfg = Config(Credentials("api_key", "secret_key", "passphrase"), simulated=True)
  client = OkxClient(cfg)
  print(client.get_ticker("BTC-USDT"))
  ```
- 快速片段（异步）：
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
- 完整示例：见 `crates/okx-py/examples/`（basic_usage/async_usage/websocket_usage），全部以官方文档路径与代码实现验证。
