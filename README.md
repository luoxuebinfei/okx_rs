# okx_rs · OKX Rust SDK & Python 绑定

> English version: [README.en.md](README.en.md)

## 项目概览
- Rust 实现的高性能 OKX REST/WebSocket SDK，接口路径与参数完全来自[官方文档](https://www.okx.com/docs-v5/)，并对照官方 `python-okx` 行为实现。
- Workspace 拆分：`okx`（统一入口）、`okx-core`（配置/签名/通用类型）、`okx-rest`（REST 客户端）、`okx-ws`（WebSocket 客户端）、`okx-py`（PyO3 绑定，含同步/异步/WS）。
- 内置签名、超时、代理、WS 心跳与自动重连，支持生产与模拟盘。
- 详细 API/类型说明：`docs/zh/rust-api.md`、`docs/zh/python.md`（英文版在 `docs/en/` 同名）。

## 功能亮点
- **完整覆盖常用 API**：账户、交易、资金、行情、公共数据（REST）；tickers/books/trades/candles/账户/持仓/订单（WS）。
- **稳定性**：Tokio 异步 + reqwest 连接池；WS 自动重连与订阅恢复（`ReconnectingWsClient`）。
- **类型安全**：严格的请求/响应结构（`Balance`、`Order` 等类型与官方字段一致）；统一错误模型 `OkxError`。
- **跨语言**：PyO3 提供 Python 同步/异步客户端与 WS 客户端，附 `.pyi` 类型存根。
- **官方一致的签名**：`timestamp + method + requestPath + body` 的 HMAC-SHA256（详见 `okx-core::signer`，源自官方文档"REST Authentication"）。
- **高级接口拓展**：补齐 Broker 返佣、Spread 交易、Rubik 交易数据、Block/RFQ 全量操作（含 MMP）、WS 高级频道（block/advanced algo/grid/recurring）及对应 Python 同步/异步绑定。

## 模块划分
```
okx_rs/
├── crates/
│   ├── okx/           # 统一入口 crate（推荐依赖）
│   ├── okx-core/      # Config、Credentials、Signer、OkxError、公共常量与类型
│   ├── okx-rest/      # OkxRestClient + AccountApi/TradeApi/FundingApi/MarketApi/PublicApi
│   ├── okx-ws/        # WsClient（基础）与 ReconnectingWsClient（自动重连）
│   └── okx-py/        # Python 客户端（OkxClient、AsyncOkxClient、WsClient）
└── justfile           # 统一脚本（fmt/clippy/test/py-test/typecheck/ci/cov）
```

## API 覆盖（源自官方文档）
- Account：余额 `/api/v5/account/balance`、持仓 `/positions`、杠杆 `/set-leverage`、费率 `/trade-fee`、最大下单量 `/max-size`、最大可用 `/max-avail-size`、风险 `/account-position-risk` 等。
- Trade：下单/撤单/改单、批量接口、当前/历史订单、成交 `/fills`、策略委托 `/algo`、平仓 `/close-position`。
- Funding：资金余额 `/asset/balances`、划转 `/asset/transfer`、提币 `/asset/withdrawal`、充值地址/记录、提币记录、币种列表。
- Market：全部/单个 ticker、orderbook（标准/5档/50档/全量 L2）、K 线、成交、指数行情。
- Public：合约/币对信息、资金费率/历史、系统时间、标记价格。
- WebSocket Channels：tickers/books/books5/books50-l2-tbt/books-l2-tbt/trades/candle(1m/5m/15m/1H/4H/1D)/mark-price/index-tickers/funding-rate；私有 account/positions/orders/orders-algo/balance_and_position。

## 安装

### Rust

**推荐方式：使用统一入口 `okx` crate**

```toml
# Cargo.toml

# 默认：REST + WebSocket
[dependencies]
okx = { git = "https://github.com/luoxuebinfei/okx_rs" }

# 仅 REST API
okx = { git = "https://github.com/luoxuebinfei/okx_rs", default-features = false, features = ["rest"] }

# 仅 WebSocket API
okx = { git = "https://github.com/luoxuebinfei/okx_rs", default-features = false, features = ["ws"] }
```

**直接依赖子 crate（高级用法）**

```toml
okx-rest = { git = "https://github.com/luoxuebinfei/okx_rs" }
okx-ws = { git = "https://github.com/luoxuebinfei/okx_rs" }
```

**本地开发**

```bash
cargo build --all  # 或 just build
```

### Python（建议 `uv` + `maturin`，Python 3.9+）

```bash
just py-setup      # 创建 .venv 并安装依赖
just py-build      # maturin develop 安装 okx-py
```

## 快速开始（Rust REST）

```rust
use okx::prelude::*;

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let creds = Credentials::new("api_key", "secret_key", "passphrase");
    let config = Config::new(creds).simulated(true).with_timeout_secs(10);
    let client = RestClient::new(config);

    // 公共接口
    let ticker = client.market().get_ticker("BTC-USDT").await?;

    // 私有接口（需有效密钥）
    let balance = client.account().get_balance(None).await?;

    println!("ticker={ticker:?} balance={balance:?}");
    Ok(())
}
```

## 快速开始（Rust WebSocket）

```rust
use okx::prelude::*;
use futures_util::StreamExt;

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
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

## 快速开始（自动重连 WebSocket）

```rust
use okx::{ws::ReconnectingWsClient, prelude::*};
use okx::ws::{ConnectionType, ReconnectConfig};
use futures_util::StreamExt;

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let config = Config::new(Credentials::new("key", "secret", "pass"));
    let reconnect_cfg = ReconnectConfig::default().with_max_attempts(10);

    let mut client = ReconnectingWsClient::connect(
        config,
        ConnectionType::Public,
        reconnect_cfg,
    ).await?;

    client.subscribe(vec![Channel::Tickers { inst_id: "BTC-USDT".into() }]).await?;

    while let Some(msg) = client.next().await {
        // 订阅会在重连后自动恢复
        println!("{msg:?}");
    }
    Ok(())
}
```

## Python 示例

同步：
```python
from okx_py import OkxClient, Config, Credentials

cfg = Config(Credentials("api_key", "secret_key", "passphrase"), simulated=True)
client = OkxClient(cfg)

print(client.get_ticker("BTC-USDT"))
print(client.get_balance())
```

异步：
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

更多见 `crates/okx-py/examples/`。

## 配置要点
- `Config::simulated(true)`：切换 WS 到 `wspap.okx.com` 模拟盘；REST 默认 `https://www.okx.com`，可用 `with_rest_url`/`with_ws_public_url`/`with_ws_private_url` 自定义。
- 超时与代理：`with_timeout_secs(30)`，`with_proxy("http://127.0.0.1:7890")`。
- 常量：`API_VERSION = "v5"`，REST 基址 `REST_API_URL` / `REST_API_URL_AWS`，WS `WS_PUBLIC_URL` / `WS_PRIVATE_URL` 与模拟盘对应常量。
- 签名：`Signer::generate_headers` 内部按官方"timestamp + method + requestPath + body"计算 HMAC-SHA256 并 Base64 编码。

## 开发与质量
- 推荐脚本：`just fmt` / `just clippy` / `just test` / `just py-test` / `just py-typecheck` / `just ci`。
- 覆盖率：`just cov-html`（详见 `scripts/cov-html.sh`、`docs/coverage.md`）。
- 生成 Rust 文档：`cargo doc --all --no-deps`（内容见 `docs/rust-api.*.md`）。

## 文档与发布
- Rust API 说明：`docs/zh/rust-api.md`（英文版 `docs/en/rust-api.md`）。
- Python 文档：`docs/zh/python.md`（英文版 `docs/en/python.md`）。
- 发布指南：`docs/zh/release.md` / `docs/en/release.md`（crates.io / PyPI 流程）。
- 变更记录：`CHANGELOG.md`。

## 许可
MIT OR Apache-2.0（双许可证）。
