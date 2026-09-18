"""Probe the canonical Rust SDK derivation path against pinned Mistral inputs."""

import argparse
import hashlib
import json
from pathlib import Path
import shutil
import subprocess
import tempfile
import tomllib

ROOT = Path(__file__).resolve().parents[1]
HERE = ROOT / "sdk-build"


def run(*args, cwd=ROOT):
    subprocess.run([str(arg) for arg in args], cwd=cwd, check=True)


def output(*args, cwd=ROOT):
    return subprocess.check_output([str(arg) for arg in args], cwd=cwd, text=True).strip()


def verify_sha256(path, expected):
    actual = hashlib.sha256(path.read_bytes()).hexdigest()
    if actual != expected:
        raise ValueError(f"{path}: expected SHA-256 {expected}, got {actual}")


def checkout(repository, commit, destination):
    if (destination / ".git").exists():
        actual = output("git", "rev-parse", "HEAD", cwd=destination)
        if actual == commit:
            return destination
        shutil.rmtree(destination)
    destination.parent.mkdir(parents=True, exist_ok=True)
    run("git", "init", destination)
    run(
        "git",
        "fetch",
        "--depth=1",
        f"https://github.com/{repository}.git",
        commit,
        cwd=destination,
    )
    run("git", "checkout", "--detach", "FETCH_HEAD", cwd=destination)
    actual = output("git", "rev-parse", "HEAD", cwd=destination)
    if actual != commit:
        raise ValueError(f"{repository}: expected {commit}, got {actual}")
    return destination


def tools(lock):
    raw_source = checkout(
        lock["generator_repository"],
        lock["generator_commit"],
        ROOT / ".tools" / f"openapi-to-rust-source-{lock['generator_commit']}",
    )
    sdk_source = checkout(
        lock["compiler_tool_repository"],
        lock["compiler_tool_commit"],
        ROOT / ".tools" / f"rust-sdk-generator-source-{lock['compiler_tool_commit']}",
    )
    raw_root = ROOT / ".tools" / f"openapi-to-rust-{lock['generator_commit']}"
    sdk_root = ROOT / ".tools" / f"rust-sdk-tools-{lock['compiler_tool_commit']}"
    raw = raw_root / "bin" / "openapi-to-rust"
    generator = sdk_root / "bin" / "rust-sdk-generator"
    bindings = sdk_root / "bin" / "openapi-to-rust-bindings"
    toolchain = lock["rust_toolchain"]
    if not raw.exists():
        run("cargo", f"+{toolchain}", "install", "--locked", "--path", raw_source, "--root", raw_root)
    if not generator.exists():
        run("cargo", f"+{toolchain}", "install", "--locked", "--path", sdk_source, "--root", sdk_root)
    if not bindings.exists():
        run(
            "cargo",
            f"+{toolchain}",
            "install",
            "--locked",
            "--path",
            sdk_source / lock["bindings_tool_subdirectory"],
            "--root",
            sdk_root,
        )
    return raw, generator, bindings


def write_json(path, value):
    path.write_text(json.dumps(value, indent=2) + "\n")


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("command", choices=["probe"])
    parser.parse_args()

    lock = json.loads((HERE / "provenance.lock.json").read_text())
    toolchain = tomllib.loads((ROOT / "rust-toolchain.toml").read_text())["toolchain"]["channel"]
    if toolchain != lock["rust_toolchain"]:
        raise ValueError("rust-toolchain.toml and provenance.lock.json disagree")
    published = HERE / "openapi" / "published.yaml"
    verify_sha256(published, lock["spec_sha256"])
    raw_generator, sdk_generator, bindings_adapter = tools(lock)

    with tempfile.TemporaryDirectory(prefix=".sdk-build-", dir=ROOT) as temporary:
        work = Path(temporary)
        shutil.copytree(HERE, work / "sdk-build")
        (work / "src").mkdir()
        config = work / "sdk-build" / "openapi-to-rust.toml"
        run(raw_generator, "generate", "--config", config)
        overlaid = work / "sdk-build" / "openapi" / "overlaid.json"
        generated = work / "src" / "generated"
        first_overlay = overlaid.read_bytes()
        run(raw_generator, "generate", "--config", config)
        if overlaid.read_bytes() != first_overlay:
            raise ValueError("Overlay materialization is not byte-for-byte deterministic")

        bindings_path = work / "rust-bindings.json"
        bindings_json = output(bindings_adapter, generated)
        bindings_value = json.loads(bindings_json)
        if bindings_value.get("schema_version") != 3:
            raise ValueError("openapi-to-rust-bindings did not emit canonical Bindings v3")
        bindings_path.write_text(bindings_json + "\n")

        derivation = json.loads(
            output(
                sdk_generator,
                "derive",
                "--openapi",
                overlaid,
                "--bindings",
                bindings_path,
                "--surface",
                work / "sdk-build" / "official-sdks" / "surface.json",
                "--overrides",
                work / "sdk-build" / "sdk-overrides.json",
            )
        )
        definition_path = work / "sdk-definition.json"
        write_json(definition_path, derivation["definition"])
        report = derivation["report"]
        statuses = {}
        reasons = {}
        for operation_id, item in report["operations"].items():
            status = item["status"]
            statuses[status] = statuses.get(status, 0) + 1
            if status == "rejected":
                code = item["reason"]["code"]
                reasons.setdefault(code, []).append(operation_id)

        runtime_path = work / "sdk-runtime.json"
        write_json(
            runtime_path,
            {
                "error_type": "SdkError",
                "error_module": "error",
                "error_exports": ["ApiError", "SdkError", "TransportError", "TransportErrorKind"],
                "sse_module": "crate::streaming",
                "sse_function": "json_events",
                "generated_marker": "// @generated by sdk-build/build.py; do not edit by hand.\n",
            },
        )
        facade = work / "src" / "sdk"
        inventory_path = work / "api-inventory.json"
        run(
            sdk_generator,
            "generate",
            "--openapi",
            overlaid,
            "--bindings",
            bindings_path,
            "--definition",
            definition_path,
            "--runtime",
            runtime_path,
            "--output",
            facade,
            "--inventory",
            inventory_path,
        )
        inventory = json.loads(inventory_path.read_text())
        print(json.dumps({
            "derivation_statuses": statuses,
            "rejections": reasons,
            "api_inventory": {
                "client": inventory["client"],
                "models": len(inventory["models"]),
                "resources": len(inventory["resources"]),
                "public_operations": sum(len(resource["operations"]) for resource in inventory["resources"]),
            },
        }, indent=2))


if __name__ == "__main__":
    main()
