# Release Guide (crates.io / PyPI)

Based on the current code and official tooling. All paths/commands are verifiable in this repo.

## Common checklist
1. Metadata: `version`, `repository`, `readme`, `license`, `description` in `Cargo.toml` and `pyproject.toml` are up to date.
2. Docs: root `README.md` (Chinese) and `README.en.md` (English) are in sync; PyPI uses `crates/okx-py/README.en.md`.
3. Quality: run `just ci` (fmt + clippy + test + py-test); optionally `just py-typecheck`, `cargo doc --all --no-deps`.
4. Changelog: update `CHANGELOG.md`.

## crates.io
Publish order (dependency safe): `okx-core` → `okx-rest` → `okx-ws`.
```bash
# package validation
cargo package -p okx-core
cargo package -p okx-rest
cargo package -p okx-ws

# publish (requires crates.io token)
cargo publish -p okx-core
cargo publish -p okx-rest
cargo publish -p okx-ws
```
Notes:
- `readme` points to `README.en.md` at repo root; ensure it exists.
- Bump versions to avoid collisions with published artifacts.
- Remove proxy-only settings from examples before release if they might confuse users.

## PyPI (okx-py)
```bash
cd crates/okx-py
maturin build --release        # build sdist/wheel
maturin publish --username __token__ --password <pypi_token>
```
Notes:
- `pyproject.toml` readme is `README.en.md`; keep it translated and current.
- Supported Python: 3.9+. Run `just py-test` to cover sync/async/WS before publishing.
- For dry run, use `--repository-url https://test.pypi.org/legacy/`.

## Versioning & tags
- Semantic versioning: start with `0.1.x`, move to `1.0.0` when stable.
- Tag after release: `git tag -a v0.1.x -m "Release okx_rs v0.1.x"` then `git push origin v0.1.x`.

## References
- OKX official docs: https://www.okx.com/docs-v5/
- PyO3: https://pyo3.rs/
- maturin: https://www.maturin.rs/
