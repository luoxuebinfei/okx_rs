## ADDED Requirements

### Requirement: 官方兼容返回（Raw Response）

系统 SHALL 在 Python 绑定中提供“官方兼容返回”能力，用于返回与官方 `python-okx` 一致的完整响应结构（至少包含 `code`、`msg`、`data`）。

系统 MUST 保持现有类型化返回行为不变，并通过新增接口提供 raw 能力（例如 `*_raw` 方法，或 `raw=True` 参数；以 `tasks.md` 的评审结论为准）。

#### Scenario: 保持既有类型化返回不变
- **WHEN** 用户调用既有方法（未启用 raw）
- **THEN** 返回类型与行为保持不变（仅返回 `data` 映射后的对象列表/对象）

#### Scenario: 获取完整响应结构
- **WHEN** 用户调用 raw 形式的方法（或启用 raw 参数）
- **THEN** 返回包含 `code`、`msg`、`data` 的字典/对象结构
- **AND** 字段命名与官方 `python-okx` 一致

