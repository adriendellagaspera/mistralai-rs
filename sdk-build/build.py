"""Probe the final Mistral SDK composition boundary using canonical Rust tools only."""

from __future__ import annotations

import argparse
from collections import Counter
import difflib
import hashlib
import json
from pathlib import Path
import shutil
import subprocess
import tempfile
import tomllib

from coverage_gate import validate_coverage

ROOT = Path(__file__).resolve().parents[1]
HERE = Path(__file__).resolve().parent
LOCK = HERE / "provenance.lock.json"
PUBLISHED = HERE / "openapi" / "published.yaml"


def run(*args: object, cwd: Path = ROOT) -> None:
    subprocess.run([str(arg) for arg in args], cwd=cwd, check=True)


def output(*args: object, cwd: Path = ROOT) -> str:
    return subprocess.check_output(
        [str(arg) for arg in args], cwd=cwd, text=True
    ).strip()


def sha256(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def verify_published(lock: dict) -> None:
    actual = sha256(PUBLISHED.read_bytes())
    expected = lock["openapi"]["sha256"]
    if actual != expected:
        raise ValueError(
            f"published OpenAPI SHA-256 mismatch: expected {expected}, got {actual}"
        )
    run("python3", HERE / "openapi" / "check_published.py")


def ensure_rust_toolchain(lock: dict) -> None:
    configured = tomllib.loads((ROOT / "rust-toolchain.toml").read_text())["toolchain"]
    if configured["channel"] != lock["rust_toolchain"]:
        raise ValueError("rust-toolchain.toml and provenance lock disagree")
    args = [
        "rustup", "toolchain", "install", configured["channel"],
        "--profile", configured.get("profile", "minimal"),
    ]
    for component in configured.get("components", []):
        args.extend(("--component", component))
    run(*args)


def checkout_tool(tool: dict) -> Path:
    source = ROOT / ".tools" / f"{tool['name']}-{tool['commit']}"
    if not (source / ".git").exists():
        if source.exists():
            shutil.rmtree(source)
        run("git", "init", source)
        run(
            "git", "fetch", "--depth=1",
            f"https://github.com/{tool['repository']}.git",
            tool["commit"],
            cwd=source,
        )
        run("git", "checkout", "--detach", "FETCH_HEAD", cwd=source)
    actual = output("git", "rev-parse", "HEAD", cwd=source)
    if actual != tool["commit"]:
        raise ValueError(f"{tool['name']} commit mismatch: {actual}")
    expected_tree = tool.get("tree_sha")
    if expected_tree:
        actual_tree = output("git", "rev-parse", "HEAD^{tree}", cwd=source)
        if actual_tree != expected_tree:
            raise ValueError(
                f"{tool['name']} tree mismatch: expected {expected_tree}, got {actual_tree}"
            )
    subdirectory = tool.get("subdirectory")
    return source / subdirectory if subdirectory else source


def install_tools(lock: dict) -> tuple[Path, Path, Path]:
    generator = lock["tools"]["openapi_to_rust"]
    generator_source = checkout_tool(generator)
    generator_root = ROOT / ".tools" / f"{generator['name']}-{generator['commit']}-install"
    generator_bin = generator_root / "bin" / "openapi-to-rust"
    if not generator_bin.exists():
        run(
            "cargo", f"+{lock['rust_toolchain']}", "install", "--locked",
            "--path", generator_source, "--root", generator_root,
        )
    actual = output(generator_bin, "--version")
    if actual != f"openapi-to-rust {generator['version']}":
        raise ValueError(f"unexpected raw generator: {actual}")

    compiler = lock["tools"]["rust_sdk_generator"]
    compiler_source = checkout_tool(compiler)
    compiler_root = ROOT / ".tools" / f"{compiler['name']}-{compiler['commit']}-install"
    compiler_bin = compiler_root / "bin" / "rust-sdk-generator"
    if not compiler_bin.exists():
        run(
            "cargo", f"+{lock['rust_toolchain']}", "install", "--locked",
            "--path", compiler_source, "--root", compiler_root,
        )

    bindings = lock["tools"]["openapi_to_rust_bindings"]
    bindings_source = checkout_tool(bindings)
    bindings_root = ROOT / ".tools" / f"{bindings['name']}-{bindings['commit']}-install"
    bindings_bin = bindings_root / "bin" / "openapi-to-rust-bindings"
    if not bindings_bin.exists():
        run(
            "cargo", f"+{lock['rust_toolchain']}", "install", "--locked",
            "--path", bindings_source, "--root", bindings_root,
        )
    return generator_bin, compiler_bin, bindings_bin


def snapshot(directory: Path, *, skip: set[str] | None = None) -> dict[str, bytes]:
    skip = skip or set()
    return {
        path.relative_to(directory).as_posix(): path.read_bytes()
        for path in sorted(directory.rglob("*"))
        if path.is_file() and path.relative_to(directory).as_posix() not in skip
    }


def source_operation_paths(openapi_path: Path) -> dict[str, str]:
    spec = json.loads(openapi_path.read_text())
    result: dict[str, str] = {}
    methods = {"get", "put", "post", "delete", "patch", "head", "options", "trace", "query"}
    for path, item in spec.get("paths", {}).items():
        if not isinstance(item, dict):
            continue
        for method, operation in item.items():
            if method.lower() not in methods or not isinstance(operation, dict):
                continue
            operation_id = operation.get("operationId")
            if not isinstance(operation_id, str) or not operation_id:
                continue
            previous = result.setdefault(operation_id, path)
            if previous != path:
                raise ValueError(
                    f"OpenAPI operationId {operation_id} appears at both {previous} and {path}"
                )
    return result


def normalized_manifest(source: bytes, operation_paths: dict[str, str]) -> dict:
    value = json.loads(source)
    for operation in value.get("operations", []):
        source_operation = operation.get("source_operation")
        if not isinstance(source_operation, dict):
            continue
        operation_id = source_operation.get("operation_id")
        expected = operation_paths.get(operation_id)
        if expected is None:
            raise ValueError(
                f"binding manifest operation {operation_id!r} is absent from overlaid OpenAPI"
            )
        source_operation["path"] = expected
    return value


def verify_raw_baseline(generated: Path, overlaid: Path) -> None:
    regenerated = snapshot(generated)
    committed = snapshot(ROOT / "src" / "generated", skip={"coverage.json"})
    if regenerated.keys() != committed.keys():
        missing = sorted(committed.keys() - regenerated.keys())
        extra = sorted(regenerated.keys() - committed.keys())
        raise ValueError(f"raw file inventory drifted: missing={missing}, extra={extra}")

    operation_paths = source_operation_paths(overlaid)
    old_marker = b"../sources/openapi/openapi.yaml"
    new_marker = b"openapi/published.yaml"
    changed: list[str] = []
    for name in sorted(regenerated):
        before = committed[name]
        after = regenerated[name]
        if name == "binding-manifest.json":
            if normalized_manifest(before, operation_paths) != normalized_manifest(after, operation_paths):
                raise ValueError("binding manifest drifted beyond exact source-operation path repair")
        elif before.replace(old_marker, new_marker) != after:
            normalized_before = before.replace(old_marker, new_marker)
            try:
                before_text = normalized_before.decode()
                after_text = after.decode()
            except UnicodeDecodeError:
                before_text = after_text = ""
            if before_text or after_text:
                print(
                    "".join(
                        difflib.unified_diff(
                            before_text.splitlines(keepends=True),
                            after_text.splitlines(keepends=True),
                            fromfile=f"baseline/{name}",
                            tofile=f"sdk-build/{name}",
                            n=3,
                        )
                    )
                )
            raise ValueError(f"raw output drifted beyond source provenance marker: {name}")
        if before != after:
            changed.append(name)
    print(
        "Raw baseline preserved; accepted reviewed provenance/source-path deltas in: "
        + ", ".join(changed)
    )


def verify_overlaid(path: Path) -> None:
    spec = json.loads(path.read_text())
    schemas = spec["components"]["schemas"]
    chat = schemas["ChatCompletionResponse"]["allOf"][1]
    if "data" in chat.get("required", []):
        raise ValueError("Overlay did not remove ChatCompletionResponse.data")
    if "level" in schemas["SharingDelete"].get("required", []):
        raise ValueError("Overlay did not remove SharingDelete.level")
    required = schemas["WorkflowListResponse"].get("required", [])
    if "beta.workflows" in required or "workflows" not in required:
        raise ValueError("Overlay did not repair WorkflowListResponse.workflows")
    paths = spec["paths"]
    expected_upstream_variants = {
        "/v1/conversations#stream",
        "/v1/conversations/{conversation_id}#stream",
        "/v1/conversations/{conversation_id}/restart#stream",
        "/v1/audio/transcriptions#stream",
    }
    actual_upstream_variants = {
        path for path in paths if "#stream" in path or "#wav" in path
    }
    if actual_upstream_variants != expected_upstream_variants:
        raise ValueError(
            "Overlay changed published representation-specific path inventory: "
            f"{sorted(actual_upstream_variants)}"
        )


def write_json(path: Path, value: object) -> None:
    path.write_text(json.dumps(value, indent=2, sort_keys=True) + "\n")


def require_migration_parity(
    statuses: Counter[str],
    facade_delta: list[str],
) -> None:
    """Reject a tooling/ cutover before closed-world coverage and facade parity."""
    if statuses.get("rejected", 0):
        raise ValueError(
            f"Cannot replace tooling/: {statuses['rejected']} OpenAPI operations remain rejected"
        )
    if facade_delta:
        raise ValueError(
            "Cannot replace tooling/: generated SDK facade differs from the "
            f"committed public baseline in {len(facade_delta)} files: "
            + ", ".join(facade_delta)
        )


def probe(lock: dict, *, require_parity: bool = False) -> None:
    ensure_rust_toolchain(lock)
    verify_published(lock)
    raw_generator, sdk_generator, bindings_adapter = install_tools(lock)

    with tempfile.TemporaryDirectory(prefix=".sdk-build-probe-", dir=ROOT) as temporary:
        work = Path(temporary)
        shutil.copytree(HERE, work / "sdk-build")
        work_build = work / "sdk-build"
        published = work_build / "openapi" / "published.yaml"
        overlaid = work_build / "openapi" / "overlaid.json"
        generated = work / "src" / "generated"

        run(raw_generator, "generate", "--config", work_build / "openapi-to-rust.toml")
        first = overlaid.read_bytes()
        verify_overlaid(overlaid)
        run(raw_generator, "generate", "--config", work_build / "openapi-to-rust.toml")
        if overlaid.read_bytes() != first:
            raise ValueError("overlaid OpenAPI is not byte-for-byte deterministic")
        run(
            raw_generator, "generate", "--config", work_build / "openapi-to-rust.toml",
            "--check",
        )
        run(
            raw_generator, "generate", "--config", work_build / "openapi-to-rust.toml",
            "--dry-run",
        )
        if published.read_bytes() != PUBLISHED.read_bytes():
            raise ValueError("raw generation modified the published OpenAPI")
        for path in sorted(generated.rglob("*.rs")):
            run(
                "rustup", "run", lock["rust_toolchain"], "rustfmt",
                "--edition", "2024", "--config", "skip_children=true", path,
            )

        verify_raw_baseline(generated, overlaid)

        bindings_path = work / "bindings.json"
        bindings_value = json.loads(output(bindings_adapter, generated))
        if bindings_value.get("schema_version") != 3:
            raise ValueError("canonical bindings adapter did not emit Bindings v3")
        write_json(bindings_path, bindings_value)

        derivation = json.loads(output(
            sdk_generator, "derive",
            "--openapi", overlaid,
            "--bindings", bindings_path,
            "--surface", work_build / "official-sdks" / "surface.json",
            "--overrides", work_build / "sdk-overrides.json",
        ))
        write_json(work / "sdk-definition.json", derivation["definition"])
        write_json(work / "derivation-report.json", derivation["report"])

        report = derivation["report"]["operations"]
        coverage = json.loads((work_build / "coverage-baseline.json").read_text())
        validate_coverage(report, coverage, source_operation_paths(overlaid))
        statuses = Counter(item["status"] for item in report.values())
        reasons = Counter(
            item["reason"]["code"]
            for item in report.values()
            if item["status"] == "rejected"
        )
        rejection_operations: dict[str, list[str]] = {}
        for operation_id, item in report.items():
            if item["status"] != "rejected":
                continue
            rejection_operations.setdefault(item["reason"]["code"], []).append(operation_id)
        for operation_ids in rejection_operations.values():
            operation_ids.sort()
        print(json.dumps({
            "derivation_statuses": dict(sorted(statuses.items())),
            "rejection_reasons": dict(sorted(reasons.items())),
            "rejection_operations": dict(sorted(rejection_operations.items())),
        }, indent=2, sort_keys=True), flush=True)

        runtime = {
            "error_type": "SdkError",
            "error_module": "error",
            "error_exports": ["ApiError", "SdkError", "TransportError", "TransportErrorKind"],
            "sse_module": "crate::streaming",
            "sse_function": "json_events",
            "generated_marker": "// @generated by tooling/pipeline/compile_sdk.py; do not edit by hand.\n",
        }
        write_json(work / "runtime.json", runtime)
        facade = work / "src" / "sdk"
        inventory = work / "api-inventory.json"
        run(
            sdk_generator, "generate",
            "--openapi", overlaid,
            "--bindings", bindings_path,
            "--definition", work / "sdk-definition.json",
            "--runtime", work / "runtime.json",
            "--output", facade,
            "--inventory", inventory,
        )
        for path in sorted(facade.rglob("*.rs")):
            run(
                "rustup", "run", lock["rust_toolchain"], "rustfmt",
                "--edition", "2024", "--config", "skip_children=true", path,
            )

        regenerated_facade = snapshot(facade)
        committed_facade = {
            path.name: path.read_bytes()
            for path in (ROOT / "src" / "sdk").glob("*.rs")
            if path.name != "error.rs"
        }
        facade_delta = sorted(
            name for name in regenerated_facade.keys() | committed_facade.keys()
            if regenerated_facade.get(name) != committed_facade.get(name)
        )
        if require_parity:
            require_migration_parity(statuses, facade_delta)

        api_inventory = json.loads(inventory.read_text())
        operation_slots = sum(
            len(resource["operations"]) for resource in api_inventory["resources"]
        )
        print(json.dumps({
            "derivation_statuses": dict(sorted(statuses.items())),
            "rejection_reasons": dict(sorted(reasons.items())),
            "rejection_operations": dict(sorted(rejection_operations.items())),
            "api_inventory": {
                "client": api_inventory["client"],
                "models": len(api_inventory["models"]),
                "resources": len(api_inventory["resources"]),
                "operation_slots": operation_slots,
            },
            "facade_delta_count": len(facade_delta),
            "facade_delta": facade_delta,
        }, indent=2, sort_keys=True))


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("command", choices=["probe"])
    parser.add_argument(
        "--require-parity",
        action="store_true",
        help="fail closed on any rejected operation or committed facade difference",
    )
    args = parser.parse_args()
    lock = json.loads(LOCK.read_text())
    if args.command == "probe":
        probe(lock, require_parity=args.require_parity)


if __name__ == "__main__":
    main()
