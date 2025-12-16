# okx-rest-api Specification

## Purpose
TBD - created by archiving change add-okx-rust-sdk. Update Purpose after archive.
## Requirements
### Requirement: HTTP 客户端基类

系统 SHALL 提供异步 HTTP 客户端基类，处理所有 REST API 请求。

客户端 SHALL 支持：
- 自动签名所有认证请求
- 连接池管理
- 超时配置
- 代理支持
- 自动重试（可配置）
- 可选的外部时间戳提供者
- 非 2xx 响应返回 `OkxError::HttpStatus`

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

#### Scenario: 使用外部时间戳签名
- **WHEN** 配置了 `TimestampProvider`
- **THEN** 签名使用 provider 提供的时间戳
- **AND** 不使用本机时间

#### Scenario: 处理限速响应
- **WHEN** 服务器返回 HTTP 429
- **THEN** 返回 `OkxError::HttpStatus { status: 429, body: "..." }`
- **AND** 调用方可根据状态码实现退避策略

#### Scenario: 处理服务器错误
- **WHEN** 服务器返回 HTTP 5xx
- **THEN** 返回 `OkxError::HttpStatus { status: 5xx, body: "..." }`

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

#### Scenario: 获取账户 instruments（对齐官方 python-okx）
- **WHEN** 调用 `get_account_instruments(inst_type: Option<String>, inst_id: Option<String>, uly: Option<String>, inst_family: Option<String>)`
- **THEN** 系统 GET `/api/v5/account/instruments`
- **AND** 返回与官方结构一致的数据列表

#### Scenario: 获取账户风险状态（对齐官方 python-okx）
- **WHEN** 调用 `get_account_risk_state()`
- **THEN** 系统 GET `/api/v5/account/risk-state`
- **AND** 返回与官方结构一致的数据列表

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

#### Scenario: 获取一键还债支持币种列表 v2（对齐官方 python-okx）
- **WHEN** 调用 `get_one_click_repay_currency_list_v2(ccy: Option<String>)`
- **THEN** 系统 GET `/api/v5/trade/one-click-repay-currency-list-v2`
- **AND** 返回与官方结构一致的数据列表

#### Scenario: 发起一键还债 v2（对齐官方 python-okx）
- **WHEN** 调用 `one_click_repay_v2(request: OneClickRepayV2Request)`
- **THEN** 系统 POST `/api/v5/trade/one-click-repay-v2`
- **AND** 返回与官方结构一致的数据列表

#### Scenario: 查询一键还债历史 v2（对齐官方 python-okx）
- **WHEN** 调用 `get_one_click_repay_history_v2(after: Option<String>, before: Option<String>, limit: Option<String>)`
- **THEN** 系统 GET `/api/v5/trade/one-click-repay-history-v2`
- **AND** 返回与官方结构一致的数据列表

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

#### Scenario: 获取兑换汇率（对齐官方 python-okx）
- **WHEN** 调用 `get_exchange_rate(ccy: Option<String>)`
- **THEN** 系统 GET `/api/v5/market/exchange-rate`
- **AND** 返回与官方结构一致的数据列表

#### Scenario: 获取指数成分（对齐官方 python-okx）
- **WHEN** 调用 `get_index_components(index: String)`
- **THEN** 系统 GET `/api/v5/market/index-components`
- **AND** 返回与官方结构一致的数据列表

#### Scenario: 获取平台 24H 交易量（对齐官方 python-okx）
- **WHEN** 调用 `get_platform_24_volume()`
- **THEN** 系统 GET `/api/v5/market/platform-24-volume`
- **AND** 返回与官方结构一致的数据列表

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
- **WHEN** 调用 `get_system_time()`
- **THEN** 系统 GET `/api/v5/public/time`
- **AND** 返回服务器时间戳

#### Scenario: 获取期权最小变动价位（对齐官方 python-okx）
- **WHEN** 调用 `get_instrument_tick_bands(inst_type: String, inst_family: Option<String>)`
- **THEN** 系统 GET `/api/v5/public/instrument-tick-bands`
- **AND** 返回与官方结构一致的数据列表

#### Scenario: 获取期权成交（对齐官方 python-okx）
- **WHEN** 调用 `get_option_trades(inst_id: Option<String>, inst_family: Option<String>, opt_type: Option<String>)`
- **THEN** 系统 GET `/api/v5/public/option-trades`
- **AND** 返回与官方结构一致的数据列表

### Requirement: 响应元信息

系统 SHALL 提供可选的响应元信息获取方式，暴露 HTTP 响应头和状态码。

ResponseMeta SHALL 包含：
- `status`: HTTP 状态码
- `headers`: 响应头集合
- `body`: 响应体

#### Scenario: 获取限速相关响应头
- **WHEN** 调用 `request_with_meta()` 方法
- **THEN** 返回包含完整响应头的 `ResponseMeta`
- **AND** 可读取 `X-RateLimit-*` 等限速相关头

#### Scenario: 实现自适应退避
- **WHEN** 响应头包含限速信息
- **THEN** 调用方可根据头信息调整请求频率

---

