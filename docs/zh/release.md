# 发布指引（crates.io / PyPI）

本文基于当前代码与官方工具流程整理，帮助在发布前完成准备。所有路径/命令均可在本仓库验证。

## 通用准备
1. 确认元数据：`Cargo.toml` 与 `pyproject.toml` 的 `version`、`repository`、`readme`、`license`、`description` 已更新。
2. 文档：根目录 `README.md`（中文）与 `README.en.md`（英文）已同步；Python 绑定使用 `crates/okx-py/README.en.md` 作为 PyPI 描述。
3. 质量检查：`just ci`（fmt + clippy + test + py-test），必要时 `just py-typecheck`、`cargo doc --all --no-deps`。
4. 变更记录：更新 `CHANGELOG.md`。

## crates.io 发布
发布顺序建议：`okx-core` → `okx-rest` → `okx-ws`（依赖顺序）。
```bash
# 打包验证
cargo package -p okx-core
cargo package -p okx-rest
cargo package -p okx-ws

# 正式发布（需要 crates.io token）
cargo publish -p okx-core
cargo publish -p okx-rest
cargo publish -p okx-ws
```
注意事项：
- 确保 `readme` 指向有效文件（此仓库使用根目录 `README.en.md`）。
- 版本号需递增，避免与已发布版本冲突。
- 如使用代理，请在发布前移除 `with_proxy` 测试配置以免误导用户。

## PyPI 发布（okx-py）
```bash
# 构建 sdist/wheel
cd crates/okx-py
maturin build --release

# 发布（需 PyPI token）
maturin publish --username __token__ --password <pypi_token>
```
注意事项：
- `pyproject.toml` 已将 `readme` 指向 `README.en.md`，确保该文件存在并已翻译。
- Python 版本兼容：3.9+，请在发布前用 `just py-test` 覆盖同步/异步/WS。
- 如果需要测试版发布，可使用 `--repository-url https://test.pypi.org/legacy/` 先行验证。

## 版本与标签建议
- 标准语义化版本：初始对齐 `0.1.x`，稳定后升到 `1.0.0`。
- 发布后打 Tag：`git tag -a v0.1.x -m "Release okx_rs v0.1.x"`，并推送 `git push origin v0.1.x`。

## 参考
- 官方 OKX 文档：https://www.okx.com/docs-v5/
- PyO3 发布指南：https://pyo3.rs/
- maturin 文档：https://www.maturin.rs/
