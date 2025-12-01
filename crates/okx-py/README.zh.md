# okx-py · OKX Python 绑定（基于 Rust）

> English version: [README.en.md](README.en.md)

基于 Rust `okx-rest` 与 `okx-ws` 的高性能 Python SDK，接口与参数全部来自官方文档（https://www.okx.com/docs-v5/），行为对齐官方 `python-okx`。

## 特性
- **性能**：Rust 异步核心，相比纯 Python 一般快 5–10 倍。
- **类型安全**：提供 `.pyi` 存根，Rust 端严格校验请求/响应字段。
- **覆盖面**：账户/交易/资金/行情/公共 REST，WS 支持 tickers/books/trades/candles 以及私有账户/持仓/订单/策略订单频道。
- **稳定性**：内置自动重连与订阅恢复（基于 `ReconnectingWsClient`）。

## 安装
PyPI（规划中）：`pip install okx-py`  
源码（当前推荐）：
```bash
just py-setup      # 创建 .venv 并安装开发依赖
just py-build      # maturin develop，安装 okx_py
```

## 快速开始
同步 REST：
```python
from okx_py import OkxClient, Config, Credentials

cfg = Config(Credentials("api_key", "secret_key", "passphrase"), simulated=True)
client = OkxClient(cfg)

print(client.get_ticker("BTC-USDT"))     # /api/v5/market/ticker
print(client.get_balance())              # /api/v5/account/balance
```
异步 REST：
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
WebSocket：
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

## API 范围（以代码为准）
- **Config / Credentials**：对应 Rust `okx-core::Config`，支持 `simulated`、超时、代理、自定义端点。
- **同步客户端 `OkxClient`**（`src/client.rs`）：
  - 账户：`get_balance(ccy=None)`、`get_positions(inst_type=None, inst_id=None)`
  - 交易：`place_order(...)`、`cancel_order(...)`、`get_order(...)`、`get_orders_pending(...)`
  - 行情/公共：`get_ticker`、`get_tickers`、`get_instruments`、`get_system_time`
- **异步客户端 `AsyncOkxClient`**（`src/async_client.rs`）：方法与同步版一致，返回 awaitable。
- **WS 客户端 `WsClient`**（`src/ws_client.rs`）：
  - 连接：`connect_public` / `connect_private`
  - 订阅：`subscribe_tickers`、`subscribe_orderbook`、`subscribe_trades`、`subscribe_candles(interval=1m/5m/15m/1H/4H/1D)`、`subscribe_account`、`subscribe_positions`、`subscribe_orders`
  - 接收：`recv()` 或 `async for msg in client`
  - 状态：`is_connected()`、`reconnect()`、`close()`、`subscription_count()`

## 示例
- 完整示例位于 `examples/`：`basic_usage.py`、`async_usage.py`、`websocket_usage.py`。
- 更多说明见 `docs/zh/python.md` 与 `docs/zh/rust-api.md`。

## 开发与测试
- 常用脚本：`just py-build`、`just py-test`、`just py-test-verbose`、`just py-typecheck`。
- 覆盖率（含 Rust + Python）：`just cov-html`（参见 `scripts/cov-html.sh`）。
- 发布步骤：`docs/zh/release.md`。

## 许可证
MIT OR Apache-2.0。
