#!/usr/bin/env python3
"""Harvest the public SDK taxonomy from Mistral's generated SDKs."""

from __future__ import annotations

import argparse
import ast
import difflib
import json
import re
import shutil
import subprocess
import tempfile
from collections import defaultdict
from pathlib import Path
from typing import Any

ROOT = Path(__file__).resolve().parents[1]
LOCK_PATH = ROOT / "codegen.lock"
COVERAGE_PATH = ROOT / "src/generated/coverage.json"
OUTPUT_PATH = ROOT / "codegen/sdk-taxonomy.json"


def run(*args: object, cwd: Path = ROOT, capture: bool = False) -> str:
    result = subprocess.run(
        [str(arg) for arg in args],
        cwd=cwd,
        check=True,
        text=True,
        stdout=subprocess.PIPE if capture else None,
    )
    return result.stdout.strip() if capture else ""


def load_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text())


def write_json(path: Path, value: object) -> None:
    path.write_text(json.dumps(value, indent=2, sort_keys=True) + "\n")


def checkout(repository: str, commit: str) -> Path:
    if not re.fullmatch(r"[0-9a-f]{40}", commit):
        raise ValueError(f"Invalid commit for {repository}: {commit}")
    slug = repository.replace("/", "-")
    target = ROOT / ".tools" / "official-sdks" / f"{slug}-{commit}"
    if target.exists():
        actual = run("git", "rev-parse", "HEAD", cwd=target, capture=True)
        if actual != commit:
            raise ValueError(f"Cached checkout mismatch for {repository}")
        return target
    target.parent.mkdir(parents=True, exist_ok=True)
    temporary = Path(tempfile.mkdtemp(prefix=f".{slug}-", dir=target.parent))
    try:
        run("git", "init", temporary)
        run("git", "remote", "add", "origin", f"https://github.com/{repository}.git", cwd=temporary)
        run("git", "fetch", "--depth=1", "origin", commit, cwd=temporary)
        run("git", "checkout", "--detach", "FETCH_HEAD", cwd=temporary)
        actual = run("git", "rev-parse", "HEAD", cwd=temporary, capture=True)
        if actual != commit:
            raise ValueError(f"Fetched {actual}, expected {commit}")
        temporary.rename(target)
    except BaseException:
        shutil.rmtree(temporary, ignore_errors=True)
        raise
    return target


def latest_commit(repository: str) -> str:
    output = run(
        "git",
        "ls-remote",
        f"https://github.com/{repository}.git",
        "refs/heads/main",
        capture=True,
    )
    commit, ref = output.split()
    if ref != "refs/heads/main" or not re.fullmatch(r"[0-9a-f]{40}", commit):
        raise ValueError(f"Unexpected main ref for {repository}: {output}")
    return commit


def normalized_http_path(path: str) -> str:
    return path.split("#", 1)[0].rstrip("/") or "/"


def transport_variant(path: str) -> str:
    if "#" not in path:
        return "default"
    fragment = path.split("#", 1)[1]
    return fragment or "default"


def snake(name: str) -> str:
    first = re.sub(r"(.)([A-Z][a-z]+)", r"\1_\2", name)
    return re.sub(r"([a-z0-9])([A-Z])", r"\1_\2", first).lower()


def normalize_public_path(parts: list[str]) -> str:
    return ".".join(snake(part) for part in parts)


def unique_match(pattern: str, source: str, label: str, *, flags: int = re.MULTILINE) -> str:
    values = sorted(set(re.findall(pattern, source, flags)))
    if len(values) != 1:
        raise ValueError(f"Expected one {label}, found {values}")
    value = values[0]
    if isinstance(value, tuple):
        if len(value) != 1:
            raise ValueError(f"Expected one capture for {label}, found {value}")
        return value[0]
    return value


def parse_ts_function(path: Path) -> dict[str, str]:
    source = path.read_text()
    raw_path = unique_match(
        r"\bpathToFunc\s*\(\s*[\"']([^\"']+)[\"']\s*\)",
        source,
        f"path in {path}",
        flags=re.MULTILINE | re.DOTALL,
    )
    method = unique_match(
        r"\bmethod\s*:\s*[\"']([A-Z]+)[\"']",
        source,
        f"method in {path}",
    )
    operation_id = unique_match(
        r"\boperationID\s*:\s*[\"']([^\"']+)[\"']",
        source,
        f"operation ID in {path}",
    )
    return {
        "http_method": method,
        "http_path": normalized_http_path(raw_path),
        "operation_id": operation_id,
        "transport_variant": transport_variant(raw_path),
    }


def parse_typescript(root: Path) -> list[dict[str, Any]]:
    sdk_dir = root / "src/sdk"
    classes: dict[str, tuple[Path, str]] = {}
    for path in sorted(sdk_dir.glob("*.ts")):
        source = path.read_text()
        match = re.search(r"export class\s+(\w+)\s+extends\s+ClientSDK", source)
        if match:
            classes[match.group(1)] = (path, source)
    if "Mistral" not in classes:
        raise ValueError("TypeScript root SDK class Mistral was not found")

    records: list[dict[str, Any]] = []
    visited: set[tuple[str, tuple[str, ...]]] = set()

    def visit(class_name: str, resource: tuple[str, ...]) -> None:
        marker = (class_name, resource)
        if marker in visited:
            return
        visited.add(marker)
        if class_name not in classes:
            raise ValueError(f"Unknown TypeScript SDK class {class_name}")
        path, source = classes[class_name]
        imports = {
            function: module
            for function, module in re.findall(
                r'import\s*\{\s*(\w+)\s*\}\s*from\s*[\"']\.\./funcs/([^\"']+)\.js[\"'];',
                source,
            )
        }
        method_matches = list(re.finditer(r"^  async\s+(\w+)\s*\(", source, re.MULTILINE))
        for index, match in enumerate(method_matches):
            end = method_matches[index + 1].start() if index + 1 < len(method_matches) else len(source)
            segment = source[match.start():end]
            called = sorted(
                function for function in imports
                if re.search(rf"\b{re.escape(function)}\s*\(", segment)
            )
            if not called:
                continue
            if len(called) != 1:
                raise ValueError(f"Ambiguous TypeScript function call in {path}:{match.group(1)}: {called}")
            function = called[0]
            endpoint = parse_ts_function(root / "src/funcs" / f"{imports[function]}.ts")
            records.append({
                **endpoint,
                "function": function,
                "public_path": ".".join((*resource, match.group(1))),
                "normalized_public_path": normalize_public_path([*resource, match.group(1)]),
            })
        for child, child_class in re.findall(r"^  get\s+(\w+)\(\):\s*(\w+)\s*\{", source, re.MULTILINE):
            visit(child_class, (*resource, child))

    visit("Mistral", ())
    if not records:
        raise ValueError("No TypeScript SDK methods were extracted")
    return sorted(records, key=lambda item: (item["public_path"], item["http_method"], item["http_path"]))


def literal_keyword(call: ast.Call, name: str) -> str | None:
    for keyword in call.keywords:
        if keyword.arg == name and isinstance(keyword.value, ast.Constant) and isinstance(keyword.value.value, str):
            return keyword.value.value
    return None


def call_name(call: ast.Call) -> str | None:
    if isinstance(call.func, ast.Name):
        return call.func.id
    if isinstance(call.func, ast.Attribute):
        return call.func.attr
    return None


def parse_python(root: Path) -> list[dict[str, Any]]:
    client_dir = root / "src/mistralai/client"
    classes: dict[str, tuple[Path, ast.ClassDef]] = {}
    for path in sorted(client_dir.rglob("*.py")):
        tree = ast.parse(path.read_text(), filename=str(path))
        for node in tree.body:
            if isinstance(node, ast.ClassDef) and any(
                isinstance(base, ast.Name) and base.id == "BaseSDK" for base in node.bases
            ):
                classes[node.name] = (path, node)
    if "Mistral" not in classes:
        raise ValueError("Python root SDK class Mistral was not found")

    records: list[dict[str, Any]] = []
    visited: set[tuple[str, tuple[str, ...]]] = set()

    def children(node: ast.ClassDef) -> dict[str, str]:
        result: dict[str, str] = {}
        for statement in node.body:
            if isinstance(statement, ast.Assign):
                for target in statement.targets:
                    if isinstance(target, ast.Name) and target.id == "_sub_sdk_map":
                        value = ast.literal_eval(statement.value)
                        for attr, pair in value.items():
                            result[attr] = pair[1]
            if isinstance(statement, (ast.FunctionDef, ast.AsyncFunctionDef)):
                for inner in ast.walk(statement):
                    if not isinstance(inner, ast.Assign) or not isinstance(inner.value, ast.Call):
                        continue
                    if not isinstance(inner.value.func, ast.Name):
                        continue
                    for target in inner.targets:
                        if (
                            isinstance(target, ast.Attribute)
                            and isinstance(target.value, ast.Name)
                            and target.value.id == "self"
                        ):
                            result[target.attr] = inner.value.func.id
        return result

    def visit(class_name: str, resource: tuple[str, ...]) -> None:
        marker = (class_name, resource)
        if marker in visited:
            return
        visited.add(marker)
        if class_name not in classes:
            raise ValueError(f"Unknown Python SDK class {class_name}")
        path, node = classes[class_name]
        for statement in node.body:
            if not isinstance(statement, ast.FunctionDef):
                continue
            if statement.name.startswith("_") or statement.name.endswith("_async"):
                continue
            http_method = http_path = operation_id = None
            for inner in ast.walk(statement):
                if not isinstance(inner, ast.Call):
                    continue
                name = call_name(inner)
                if name == "_build_request":
                    http_method = literal_keyword(inner, "method")
                    http_path = literal_keyword(inner, "path")
                elif name == "HookContext":
                    operation_id = literal_keyword(inner, "operation_id")
            if http_method and http_path and operation_id:
                records.append({
                    "http_method": http_method,
                    "http_path": normalized_http_path(http_path),
                    "operation_id": operation_id,
                    "transport_variant": transport_variant(http_path),
                    "public_path": ".".join((*resource, statement.name)),
                    "normalized_public_path": normalize_public_path([*resource, statement.name]),
                    "source_file": path.relative_to(root).as_posix(),
                })
        for child, child_class in sorted(children(node).items()):
            visit(child_class, (*resource, child))

    visit("Mistral", ())
    if not records:
        raise ValueError("No Python SDK methods were extracted")
    return sorted(records, key=lambda item: (item["public_path"], item["http_method"], item["http_path"]))


def reconcile(
    source: str,
    records: list[dict[str, Any]],
    raw_operations: list[dict[str, Any]],
) -> tuple[dict[str, list[dict[str, Any]]], list[dict[str, Any]]]:
    by_id = {item["operation_id"]: item for item in raw_operations}
    by_http: dict[tuple[str, str], list[dict[str, Any]]] = defaultdict(list)
    for item in raw_operations:
        by_http[(item["method"], normalized_http_path(item["path"]))].append(item)

    matched: dict[str, list[dict[str, Any]]] = defaultdict(list)
    unresolved: list[dict[str, Any]] = []
    for record in records:
        raw = by_id.get(record["operation_id"])
        match_kind = "operation_id"
        if raw is not None:
            if (raw["method"], normalized_http_path(raw["path"])) != (
                record["http_method"],
                record["http_path"],
            ):
                raise ValueError(
                    f"{source} operation ID {record['operation_id']} disagrees with HTTP route"
                )
        else:
            candidates = by_http[(record["http_method"], record["http_path"])]
            if record["transport_variant"] == "stream":
                stream_candidates = [
                    item for item in candidates
                    if item["operation_id"].endswith("_stream") or not item.get("upstream", True)
                ]
                if len(stream_candidates) == 1:
                    candidates = stream_candidates
            if len(candidates) == 1:
                raw = candidates[0]
                match_kind = "http"
            else:
                unresolved.append({
                    **record,
                    "reason": "no_http_match" if not candidates else "ambiguous_http_match",
                    "candidate_operation_ids": sorted(item["operation_id"] for item in candidates),
                })
                continue
        public = {
            "public_path": record["public_path"],
            "normalized_public_path": record["normalized_public_path"],
            "official_operation_id": record["operation_id"],
            "match_kind": match_kind,
            "transport_variant": record["transport_variant"],
        }
        if "function" in record:
            public["function"] = record["function"]
        matched[raw["operation_id"]].append(public)
    return dict(matched), unresolved


def build_inventory(
    lock: dict[str, Any],
    coverage: dict[str, Any],
    typescript_records: list[dict[str, Any]],
    python_records: list[dict[str, Any]],
) -> dict[str, Any]:
    raw_operations = coverage["operations"]
    ts_matched, ts_unresolved = reconcile("typescript", typescript_records, raw_operations)
    py_matched, py_unresolved = reconcile("python", python_records, raw_operations)
    operations = []
    divergences = []
    for raw in sorted(raw_operations, key=lambda item: item["operation_id"]):
        ts = sorted(ts_matched.get(raw["operation_id"], []), key=lambda item: item["public_path"])
        py = sorted(py_matched.get(raw["operation_id"], []), key=lambda item: item["public_path"])
        if ts and py:
            ts_names = {item["normalized_public_path"] for item in ts}
            py_names = {item["normalized_public_path"] for item in py}
            if ts_names != py_names:
                divergences.append({
                    "operation_id": raw["operation_id"],
                    "typescript": sorted(ts_names),
                    "python": sorted(py_names),
                    "reason": "public_path_mismatch",
                })
        operations.append({
            "http_method": raw["method"],
            "http_path": raw["path"],
            "operation_id": raw["operation_id"],
            "upstream": raw.get("upstream", True),
            "typescript": ts,
            "python": py,
        })
    upstream_ids = {
        item["operation_id"] for item in raw_operations if item.get("upstream", True)
    }
    covered_upstream = {
        item["operation_id"] for item in operations
        if item["upstream"] and (item["typescript"] or item["python"])
    }
    unresolved = {
        "typescript": ts_unresolved,
        "python": py_unresolved,
    }
    return {
        "schema_version": 1,
        "provenance": {
            "protocol": {
                "repository": lock["upstream_repository"],
                "commit": lock["upstream_commit"],
                "artifact": "src/generated/coverage.json",
            },
            "typescript": {
                "repository": lock["official_typescript_sdk_repository"],
                "commit": lock["official_typescript_sdk_commit"],
            },
            "python": {
                "repository": lock["official_python_sdk_repository"],
                "commit": lock["official_python_sdk_commit"],
            },
        },
        "summary": {
            "raw_operations": len(raw_operations),
            "upstream_operations": len(upstream_ids),
            "upstream_with_official_taxonomy": len(covered_upstream),
            "openapi_only": len(upstream_ids - covered_upstream),
            "typescript_methods": len(typescript_records),
            "python_methods": len(python_records),
            "typescript_unresolved": len(ts_unresolved),
            "python_unresolved": len(py_unresolved),
            "cross_sdk_divergences": len(divergences),
        },
        "operations": operations,
        "openapi_only": sorted(upstream_ids - covered_upstream),
        "sdk_only": unresolved,
        "cross_sdk_divergences": divergences,
    }


def generate(lock: dict[str, Any]) -> dict[str, Any]:
    ts_root = checkout(
        lock["official_typescript_sdk_repository"],
        lock["official_typescript_sdk_commit"],
    )
    py_root = checkout(
        lock["official_python_sdk_repository"],
        lock["official_python_sdk_commit"],
    )
    return build_inventory(
        lock,
        load_json(COVERAGE_PATH),
        parse_typescript(ts_root),
        parse_python(py_root),
    )


def pin_latest(lock: dict[str, Any]) -> None:
    lock["official_typescript_sdk_commit"] = latest_commit(
        lock["official_typescript_sdk_repository"]
    )
    lock["official_python_sdk_commit"] = latest_commit(
        lock["official_python_sdk_repository"]
    )
    write_json(LOCK_PATH, lock)


def check(expected: dict[str, Any], actual: dict[str, Any]) -> None:
    expected_text = json.dumps(expected, indent=2, sort_keys=True) + "\n"
    actual_text = json.dumps(actual, indent=2, sort_keys=True) + "\n"
    if expected_text == actual_text:
        print("Official SDK taxonomy matches pinned sources byte-for-byte.")
        return
    print("".join(difflib.unified_diff(
        expected_text.splitlines(keepends=True),
        actual_text.splitlines(keepends=True),
        fromfile="committed/sdk-taxonomy.json",
        tofile="regenerated/sdk-taxonomy.json",
    )))
    raise SystemExit("Official SDK taxonomy is stale")


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("command", choices=["update", "check", "pin-latest"])
    parser.add_argument("--latest", action="store_true", help="advance official SDK pins before updating")
    args = parser.parse_args()
    lock = load_json(LOCK_PATH)
    required = (
        "official_typescript_sdk_repository",
        "official_typescript_sdk_commit",
        "official_python_sdk_repository",
        "official_python_sdk_commit",
    )
    missing = [key for key in required if key not in lock]
    if missing:
        raise ValueError(f"Missing codegen lock keys: {', '.join(missing)}")
    if args.command == "pin-latest":
        pin_latest(lock)
        return
    if args.latest:
        pin_latest(lock)
        lock = load_json(LOCK_PATH)
    actual = generate(lock)
    if args.command == "update":
        write_json(OUTPUT_PATH, actual)
        print(f"Wrote {OUTPUT_PATH.relative_to(ROOT)} from pinned official SDKs.")
    else:
        check(load_json(OUTPUT_PATH), actual)


if __name__ == "__main__":
    main()
