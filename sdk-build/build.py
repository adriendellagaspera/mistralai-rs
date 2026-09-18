"""Probe the final Mistral SDK composition boundary using canonical Rust tools only."""

from __future__ import annotations

import argparse
from collections import Counter
import hashlib
import json
from pathlib import Path
import shutil
import subprocess
import tempfile
import tomllib

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
    if any("#stream" in path or "#wav" in path for path in paths):
        raise ValueError("Overlay introduced a synthetic generator-only path")


def write_json(path: Path, value: object) -> None:
    path.write_text(json.dumps(value, indent=2, sort_keys=True) + "\n")


def probe(lock: dict) -> None:
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

        regenerated_raw = snapshot(generated)
        committed_raw = snapshot(ROOT / "src" / "generated", skip={"coverage.json"})
        raw_delta = sorted(
            name for name in regenerated_raw.keys() | committed_raw.keys()
            if regenerated_raw.get(name) != committed_raw.get(name)
        )
        if raw_delta:
            raise SystemExit("canonical raw output drifted: " + ", ".join(raw_delta))

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

        report = derivation["report"]["operations"]
        statuses = Counter(item["status"] for item in report.values())
        reasons = Counter(
            item["reason"]["code"]
            for item in report.values()
            if item["status"] == "rejected"
        )
        api_inventory = json.loads(inventory.read_text())
        operation_slots = sum(
            len(resource["operations"]) for resource in api_inventory["resources"]
        )
        print(json.dumps({
            "derivation_statuses": dict(sorted(statuses.items())),
            "rejection_reasons": dict(sorted(reasons.items())),
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
    args = parser.parse_args()
    lock = json.loads(LOCK.read_text())
    if args.command == "probe":
        probe(lock)


if __name__ == "__main__":
    main()
