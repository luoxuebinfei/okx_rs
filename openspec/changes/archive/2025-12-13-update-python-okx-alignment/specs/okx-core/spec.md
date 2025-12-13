## ADDED Requirements

### Requirement: 模拟盘请求头规则（x-simulated-trading）

系统 SHALL 按官方文档要求在模拟盘（Demo Trading）请求中携带 `x-simulated-trading` 请求头，并保证默认行为安全（避免在未显式启用模拟盘时发送模拟盘标记）。

#### Scenario: 生产模式不发送模拟盘标记
- **WHEN** 配置 `simulated=false`
- **THEN** 系统不得发送 `x-simulated-trading: 1`

#### Scenario: 模拟盘模式必须发送模拟盘标记
- **WHEN** 配置 `simulated=true`
- **THEN** 系统 MUST 发送 `x-simulated-trading: 1`

