"""回归测试：确保 Python 绑定覆盖 okx-rest 的全部 REST trait 方法（按映射规则判定）。"""

from __future__ import annotations

import re
from dataclasses import dataclass
from pathlib import Path


@dataclass(frozen=True)
class TraitMethod:
    """用于将 Rust trait 方法与 Python 方法名做映射。"""

    trait: str
    method: str


# 显式映射：用于解决方法名冲突或保留更清晰的既有命名
METHOD_NAME_OVERRIDES: dict[TraitMethod, str] = {
    # Rust `BlockRfqApi.get_trades` 与 `MarketApi.get_trades` 同名冲突，Python 侧用别名暴露
    TraitMethod("BlockRfqApi", "get_trades"): "get_block_rfq_trades",
    # Rust `SubaccountApi.get_funding_balance` 在 Python 侧使用更清晰的命名
    TraitMethod("SubaccountApi", "get_funding_balance"): "get_subaccount_funding_balance",
}


def _find_repo_root(start: Path) -> Path:
    """向上查找仓库根目录（以存在 Cargo.toml 与 crates/ 目录为准）。"""
    p = start.resolve()
    for parent in [p, *p.parents]:
        if (parent / "Cargo.toml").is_file() and (parent / "crates").is_dir():
            return parent
    raise RuntimeError("未找到仓库根目录（缺少 Cargo.toml/crates）")


def _extract_trait_blocks(text: str) -> list[tuple[str, str]]:
    """提取 `pub trait Xxx { ... }` 的 (trait_name, body_text)。"""
    out: list[tuple[str, str]] = []
    for m in re.finditer(r"pub\s+trait\s+([A-Za-z0-9_]+)\s*\{", text):
        trait = m.group(1)
        i = m.end()
        depth = 1
        j = i
        while j < len(text) and depth:
            c = text[j]
            if c == "{":
                depth += 1
            elif c == "}":
                depth -= 1
            j += 1
        out.append((trait, text[i : j - 1]))
    return out


def _extract_trait_methods(body: str) -> set[str]:
    """从 trait body 中提取 `fn method_name(`。"""
    return set(re.findall(r"\n\s*fn\s+([a-zA-Z0-9_]+)\s*\(", body))


def _load_okx_rest_trait_methods() -> dict[str, set[str]]:
    """加载 okx-rest 所有 trait 方法：{trait_name -> {method_name}}。"""
    repo_root = _find_repo_root(Path(__file__))
    api_dir = repo_root / "crates" / "okx-rest" / "src" / "api"
    if not api_dir.is_dir():
        raise RuntimeError(f"未找到 okx-rest api 目录: {api_dir}")

    traits: dict[str, set[str]] = {}
    for path in sorted(api_dir.glob("*.rs")):
        text = path.read_text(encoding="utf-8")
        for trait, body in _extract_trait_blocks(text):
            methods = _extract_trait_methods(body)
            if methods:
                traits.setdefault(trait, set()).update(methods)
    if not traits:
        raise RuntimeError("未解析到任何 okx-rest trait 方法，测试口径可能失效")
    return traits


def _target_python_name(trait: str, method: str) -> str:
    return METHOD_NAME_OVERRIDES.get(TraitMethod(trait, method), method)


def test_method_mapping_has_no_collisions():
    """映射后的 Python 方法名必须唯一，否则一定会丢失某个 trait 的能力。"""
    traits = _load_okx_rest_trait_methods()

    owners: dict[str, list[str]] = {}
    for trait, methods in sorted(traits.items()):
        for method in sorted(methods):
            py_name = _target_python_name(trait, method)
            owners.setdefault(py_name, []).append(f"{trait}.{method}")

    collisions = {name: refs for name, refs in owners.items() if len(refs) > 1}
    assert not collisions, "发现映射冲突（同一 Python 方法名对应多个 Rust trait 方法）:\n" + "\n".join(
        f"- {name}: {refs}" for name, refs in sorted(collisions.items())
    )


def test_python_rest_methods_cover_okx_rest_traits(sync_client, async_client):
    """OkxClient/AsyncOkxClient 应覆盖 okx-rest trait 方法（含显式映射）。"""
    traits = _load_okx_rest_trait_methods()

    for client, client_name in [(sync_client, "OkxClient"), (async_client, "AsyncOkxClient")]:
        missing: list[str] = []
        for trait, methods in sorted(traits.items()):
            for method in sorted(methods):
                py_name = _target_python_name(trait, method)
                if not hasattr(client, py_name):
                    missing.append(f"{client_name}: {trait}.{method} -> {py_name}")
                    continue
                assert callable(getattr(client, py_name)), f"{client_name}.{py_name} 不可调用"

        assert not missing, "发现未覆盖的方法:\n" + "\n".join(missing)


def test_sync_async_surface_consistency(sync_client, async_client):
    """同步/异步客户端的“目标方法集合”应一致（允许异步存在兼容别名）。"""
    traits = _load_okx_rest_trait_methods()
    target_names: set[str] = set()
    for trait, methods in traits.items():
        for method in methods:
            target_names.add(_target_python_name(trait, method))

    sync_has = {name for name in target_names if hasattr(sync_client, name)}
    async_has = {name for name in target_names if hasattr(async_client, name)}

    assert sync_has == target_names, f"同步客户端缺少目标方法: {sorted(target_names - sync_has)}"
    assert async_has == target_names, f"异步客户端缺少目标方法: {sorted(target_names - async_has)}"

    # 允许异步客户端保留历史别名（不强制同步也提供）
    allowed_async_only = {"get_funding_balance"}
    extra_async = {name for name in dir(async_client) if not name.startswith("_")} - {
        name for name in dir(sync_client) if not name.startswith("_")
    }
    unexpected = extra_async - allowed_async_only
    assert not unexpected, f"异步客户端新增了未声明的额外方法: {sorted(unexpected)}"
