# OKX REST API Specification

## ADDED Requirements

### Requirement: HTTP 客户端基类

系统 SHALL 提供异步 HTTP 客户端基类，处理所有 REST API 请求。

客户端 SHALL 支持：
- 自动签名所有认证请求
- 连接池管理
- 超时配置
- 代理支持
- 自动重试（可配置）

#### Scenario: 发送认证 GET 请求
- **WHEN** 调用需要认证的 GET 端点
- **THEN** 系统自动添加签名头
- **AND** 返回解析后的响应

#### Scenario: 发送 POST 请求
- **WHEN** 调用 POST 端点并传入参数
- **THEN** 系统将参数序列化为 JSON
- **AND** 自动添加签名头
- **AND** 返回解析后的响应

#### Scenario: 配置代理
- **WHEN** 用户配置 HTTP 代理
- **THEN** 所有请求通过代理发送

---

### Requirement: Account API

系统 SHALL 实现账户管理 API，与 OKX `/api/v5/account/*` 端点对应。

#### Scenario: 获取账户余额
- **WHEN** 调用 `get_balance(ccy: Option<String>)`
- **THEN** 系统请求 `/api/v5/account/balance`
- **AND** 返回 `Vec<Balance>`

#### Scenario: 获取持仓信息
- **WHEN** 调用 `get_positions(inst_type: Option<String>, inst_id: Option<String>)`
- **THEN** 系统请求 `/api/v5/account/positions`
- **AND** 返回 `Vec<Position>`

#### Scenario: 设置杠杆
- **WHEN** 调用 `set_leverage(inst_id: String, lever: String, mgn_mode: String)`
- **THEN** 系统 POST 到 `/api/v5/account/set-leverage`
- **AND** 返回设置结果

#### Scenario: 获取账户配置
- **WHEN** 调用 `get_account_config()`
- **THEN** 系统请求 `/api/v5/account/config`
- **AND** 返回 `AccountConfig`

#### Scenario: 获取最大可交易数量
- **WHEN** 调用 `get_max_size(inst_id: String, td_mode: String)`
- **THEN** 系统请求 `/api/v5/account/max-size`
- **AND** 返回最大可交易数量

---

### Requirement: Trade API

系统 SHALL 实现交易 API，与 OKX `/api/v5/trade/*` 端点对应。

#### Scenario: 下单
- **WHEN** 调用 `place_order(order: PlaceOrderRequest)`
- **THEN** 系统 POST 到 `/api/v5/trade/order`
- **AND** 返回订单 ID

#### Scenario: 批量下单
- **WHEN** 调用 `place_batch_orders(orders: Vec<PlaceOrderRequest>)`
- **THEN** 系统 POST 到 `/api/v5/trade/batch-orders`
- **AND** 返回订单 ID 列表

#### Scenario: 撤单
- **WHEN** 调用 `cancel_order(inst_id: String, ord_id: String)`
- **THEN** 系统 POST 到 `/api/v5/trade/cancel-order`
- **AND** 返回撤单结果

#### Scenario: 批量撤单
- **WHEN** 调用 `cancel_batch_orders(orders: Vec<CancelOrderRequest>)`
- **THEN** 系统 POST 到 `/api/v5/trade/cancel-batch-orders`
- **AND** 返回撤单结果列表

#### Scenario: 修改订单
- **WHEN** 调用 `amend_order(request: AmendOrderRequest)`
- **THEN** 系统 POST 到 `/api/v5/trade/amend-order`
- **AND** 返回修改结果

#### Scenario: 查询订单
- **WHEN** 调用 `get_order(inst_id: String, ord_id: String)`
- **THEN** 系统 GET `/api/v5/trade/order`
- **AND** 返回 `Order`

#### Scenario: 查询未成交订单
- **WHEN** 调用 `get_pending_orders(inst_type: Option<String>)`
- **THEN** 系统 GET `/api/v5/trade/orders-pending`
- **AND** 返回 `Vec<Order>`

#### Scenario: 查询历史订单
- **WHEN** 调用 `get_orders_history(inst_type: String)`
- **THEN** 系统 GET `/api/v5/trade/orders-history`
- **AND** 返回 `Vec<Order>`

#### Scenario: 查询成交明细
- **WHEN** 调用 `get_fills(inst_type: Option<String>)`
- **THEN** 系统 GET `/api/v5/trade/fills`
- **AND** 返回 `Vec<Fill>`

#### Scenario: 下算法订单
- **WHEN** 调用 `place_algo_order(request: PlaceAlgoOrderRequest)`
- **THEN** 系统 POST 到 `/api/v5/trade/order-algo`
- **AND** 返回算法订单 ID

#### Scenario: 撤销算法订单
- **WHEN** 调用 `cancel_algo_order(algo_id: String, inst_id: String)`
- **THEN** 系统 POST 到 `/api/v5/trade/cancel-algos`
- **AND** 返回撤销结果

---

### Requirement: Funding API

系统 SHALL 实现资金管理 API，与 OKX `/api/v5/asset/*` 端点对应。

#### Scenario: 获取资金账户余额
- **WHEN** 调用 `get_balances(ccy: Option<String>)`
- **THEN** 系统 GET `/api/v5/asset/balances`
- **AND** 返回 `Vec<FundingBalance>`

#### Scenario: 资金划转
- **WHEN** 调用 `transfer(ccy: String, amt: String, from: String, to: String)`
- **THEN** 系统 POST 到 `/api/v5/asset/transfer`
- **AND** 返回划转 ID

#### Scenario: 提现
- **WHEN** 调用 `withdrawal(request: WithdrawalRequest)`
- **THEN** 系统 POST 到 `/api/v5/asset/withdrawal`
- **AND** 返回提现 ID

#### Scenario: 获取充值地址
- **WHEN** 调用 `get_deposit_address(ccy: String)`
- **THEN** 系统 GET `/api/v5/asset/deposit-address`
- **AND** 返回 `Vec<DepositAddress>`

#### Scenario: 获取充值记录
- **WHEN** 调用 `get_deposit_history(ccy: Option<String>)`
- **THEN** 系统 GET `/api/v5/asset/deposit-history`
- **AND** 返回 `Vec<DepositRecord>`

#### Scenario: 获取提现记录
- **WHEN** 调用 `get_withdrawal_history(ccy: Option<String>)`
- **THEN** 系统 GET `/api/v5/asset/withdrawal-history`
- **AND** 返回 `Vec<WithdrawalRecord>`

---

### Requirement: Market Data API

系统 SHALL 实现行情数据 API，与 OKX `/api/v5/market/*` 端点对应。

此 API 不需要认证。

#### Scenario: 获取所有产品行情
- **WHEN** 调用 `get_tickers(inst_type: String)`
- **THEN** 系统 GET `/api/v5/market/tickers`
- **AND** 返回 `Vec<Ticker>`

#### Scenario: 获取单个产品行情
- **WHEN** 调用 `get_ticker(inst_id: String)`
- **THEN** 系统 GET `/api/v5/market/ticker`
- **AND** 返回 `Ticker`

#### Scenario: 获取深度数据
- **WHEN** 调用 `get_orderbook(inst_id: String, sz: Option<u32>)`
- **THEN** 系统 GET `/api/v5/market/books`
- **AND** 返回 `OrderBook`

#### Scenario: 获取 K 线数据
- **WHEN** 调用 `get_candlesticks(inst_id: String, bar: Option<String>, limit: Option<u32>)`
- **THEN** 系统 GET `/api/v5/market/candles`
- **AND** 返回 `Vec<Candle>`

#### Scenario: 获取历史 K 线
- **WHEN** 调用 `get_history_candlesticks(inst_id: String, bar: Option<String>)`
- **THEN** 系统 GET `/api/v5/market/history-candles`
- **AND** 返回 `Vec<Candle>`

#### Scenario: 获取最近成交
- **WHEN** 调用 `get_trades(inst_id: String, limit: Option<u32>)`
- **THEN** 系统 GET `/api/v5/market/trades`
- **AND** 返回 `Vec<Trade>`

---

### Requirement: Public Data API

系统 SHALL 实现公共数据 API，与 OKX `/api/v5/public/*` 端点对应。

此 API 不需要认证。

#### Scenario: 获取交易产品信息
- **WHEN** 调用 `get_instruments(inst_type: String)`
- **THEN** 系统 GET `/api/v5/public/instruments`
- **AND** 返回 `Vec<Instrument>`

#### Scenario: 获取资金费率
- **WHEN** 调用 `get_funding_rate(inst_id: String)`
- **THEN** 系统 GET `/api/v5/public/funding-rate`
- **AND** 返回资金费率信息

#### Scenario: 获取系统时间
- **WHEN** 调用 `get_time()`
- **THEN** 系统 GET `/api/v5/public/time`
- **AND** 返回服务器时间戳

#### Scenario: 获取限价
- **WHEN** 调用 `get_price_limit(inst_id: String)`
- **THEN** 系统 GET `/api/v5/public/price-limit`
- **AND** 返回价格限制信息

#### Scenario: 获取持仓量
- **WHEN** 调用 `get_open_interest(inst_type: String)`
- **THEN** 系统 GET `/api/v5/public/open-interest`
- **AND** 返回持仓量信息
