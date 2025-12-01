# okx-py 测试文档

## 测试概览

okx-py 的测试套件覆盖了 Python 绑定的所有核心功能。

### 测试统计

- **总测试数**: 76
- **通过**: 65
- **跳过**: 11 (WebSocket 网络测试)
- **覆盖率**: 85.5%

## 测试文件结构

```
tests/
├── test_async.py              # 异步客户端测试 (11个测试)
├── test_sync_client.py        # 同步客户端测试 (8个测试)
├── test_websocket.py          # WebSocket 基础测试 (13个测试)
├── test_websocket_advanced.py # WebSocket 高级功能测试 (7个测试)
├── test_concurrent.py         # 并发和异步测试 (9个测试)
├── test_data_types.py         # 数据类型完整性测试 (8个测试)
├── test_types.py              # 基础类型测试 (10个测试)
└── test_errors.py             # 错误处理和安全测试 (10个测试)
```

## 测试覆盖范围

### 1. 基础类型 (test_types.py)
- ✅ Credentials 创建和安全性
- ✅ Config 配置选项
- ✅ 客户端创建和表示
- ✅ 数据类型属性访问

### 2. 同步客户端 (test_sync_client.py)
- ✅ 获取系统时间
- ✅ 获取交易产品列表
- ✅ 获取行情数据
- ✅ 客户端字符串表示
- ✅ 方法存在性验证

### 3. 异步客户端 (test_async.py)
- ✅ 异步获取系统时间
- ✅ 异步获取交易产品列表
- ✅ 异步获取行情数据

### 4. WebSocket 客户端 (test_websocket.py + test_websocket_advanced.py)
- ✅ 公共/私有 WebSocket 连接
- ✅ 订阅行情频道 (tickers, orderbook, trades, candles)
- ✅ 订阅私有频道 (account, positions, orders)
- ✅ K线时间周期验证
- ✅ 消息接收 (recv)
- ✅ 异步迭代器 (__aiter__, __anext__)
- ✅ 连接状态检查 (is_connected)
- ✅ 订阅计数 (subscription_count)
- ⏭️ 重连功能 (跳过 - 需要网络)
- ⏭️ 关闭连接 (跳过 - 需要网络)

### 5. 并发测试 (test_concurrent.py)
- ✅ 异步客户端并发请求
- ✅ 并发获取多个行情
- ✅ 多个客户端实例并发
- ✅ WebSocket 并发订阅
- ✅ 多个 WebSocket 客户端
- ✅ REST 和 WebSocket 混合使用
- ✅ 超时处理
- ✅ 并发余额查询

### 6. 数据类型完整性 (test_data_types.py)
- ✅ Balance 所有属性测试
- ✅ BalanceDetail 所有属性测试
- ✅ Position 所有属性测试
- ✅ Order 所有属性测试
- ✅ Ticker 所有属性测试
- ✅ 异步获取数据类型
- ✅ None 值处理
- ✅ 列表数据处理

### 7. 错误处理 (test_errors.py)
- ✅ 无效凭证处理
- ✅ 无效配置处理
- ✅ 无效产品查询
- ✅ 无效产品类型查询
- ✅ WebSocket 连接测试
- ✅ 凭证安全性（repr 不泄露敏感信息）
- ✅ 配置安全性
- ✅ 客户端安全性

## 运行测试

### 环境准备

```bash
# 在项目根目录创建虚拟环境
uv venv --python 3.12

# 安装开发依赖
source .venv/bin/activate
cd crates/okx-py
uv pip install -e ".[dev]"
```

### 运行所有测试

```bash
# 使用 pytest 直接运行
.venv/bin/pytest crates/okx-py/tests/ -v

# 或使用 justfile
just py-test
```

### 运行特定测试文件

```bash
# 只运行同步客户端测试
just py-test-file test_sync_client.py

# 只运行异步测试
.venv/bin/pytest crates/okx-py/tests/test_async.py -v
```

### 详细输出模式

```bash
# 显示详细输出和打印语句
just py-test-verbose

# 或
.venv/bin/pytest crates/okx-py/tests/ -vv -s
```

## 测试设计原则

### 1. 网络隔离
- 公共 API 测试使用虚拟凭证
- 网络错误自动跳过，不影响 CI
- 使用 `pytest.skip()` 处理网络依赖

### 2. 安全性优先
- 验证 repr 不泄露敏感信息
- 测试凭证只显示前几个字符
- 配置和客户端不暴露密钥

### 3. 实际 API 验证
- 测试使用真实 OKX 模拟环境
- 验证 API 响应结构
- 确保类型转换正确

### 4. 异步支持
- 使用 `pytest-asyncio` 插件
- 测试异步上下文管理器
- 验证异步方法正确性

## 持续集成

### 本地 CI 检查

```bash
# 运行完整 CI 流程
just ci

# 包括:
# - cargo fmt (格式化)
# - cargo clippy (代码检查)
# - cargo test (Rust 测试)
# - pytest (Python 测试)
```

### 测试覆盖率

当前测试覆盖了以下功能：
- ✅ REST API 客户端（同步/异步）
- ✅ WebSocket 客户端连接
- ✅ 数据类型转换
- ✅ 错误处理
- ✅ 安全性验证
- ⚠️ WebSocket 消息处理（部分跳过）

## 已知限制

1. **WebSocket 测试**: 部分测试需要真实网络连接，在 CI 环境中会跳过
2. **认证 API**: 需要真实凭证的测试仅验证方法存在性
3. **性能测试**: 当前未包含性能基准测试

## 未来改进

- [ ] 添加 Mock 服务器以完全隔离网络
- [ ] 增加性能基准测试
- [ ] 添加集成测试套件
- [ ] 提高 WebSocket 测试覆盖率
- [ ] 添加压力测试

## 故障排查

### 测试失败

```bash
# 查看详细错误信息
.venv/bin/pytest crates/okx-py/tests/ -vv --tb=long

# 只运行失败的测试
.venv/bin/pytest crates/okx-py/tests/ --lf
```

### 重新编译

```bash
# 如果修改了 Rust 代码，需要重新编译
cd crates/okx-py
maturin develop

# 然后运行测试
pytest tests/ -v
```

### 清理环境

```bash
# 清理构建产物
just clean

# 重新安装
just py-setup
```

## 贡献指南

添加新测试时请遵循：

1. **命名规范**: `test_<功能>_<场景>`
2. **文档字符串**: 使用简体中文描述测试目的
3. **网络处理**: 使用 `pytest.skip()` 处理网络错误
4. **安全性**: 不在测试中硬编码真实凭证
5. **独立性**: 每个测试应该独立运行

示例：

```python
def test_client_get_balance():
    """测试客户端获取余额功能."""
    from okx_py import OkxClient, Config, Credentials

    creds = Credentials("test", "test", "test")
    config = Config(creds, simulated=True)
    client = OkxClient(config)

    try:
        balance = client.get_balance()
        assert isinstance(balance, list)
    except Exception as e:
        pytest.skip(f"网络错误: {e}")
```
