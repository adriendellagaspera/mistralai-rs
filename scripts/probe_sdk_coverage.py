"""Compile disposable projections for unmapped operation candidates.

This validates generic compiler capabilities, not public naming or ergonomics.
No additional resource is committed or exposed by the SDK. Run with the pinned
codegen Python environment after cargo dependencies have been fetched.
"""
import json
import hashlib
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile
import tomllib

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT / "codegen"))
import sdk_compiler as compiler
from sdk_autoproject import expand_manifest
from sdk_contracts import coverage_inventory, public_surface


def probes(openapi, rust, configured):
    ir = compiler.compile_ir(openapi, rust, configured)
    inventory = coverage_inventory(openapi, rust, ir)
    modules, rejected = {}, {}
    for index, (operation_id, entry) in enumerate(inventory.items()):
        if entry["status"] != "candidate_unverified":
            continue
        try:
            request = openapi.request_schema(operation_id)
            wire_operation = openapi.operation(operation_id)
            success = [response for status, response in wire_operation.get("responses", {}).items()
                       if str(status).startswith("2")]
            empty_response = len(success) == 1 and not success[0].get("content")
            models = {}
            operation = {"operation_id": operation_id}
            if empty_response:
                operation["empty_response"] = True
            else:
                response = openapi.response_schema(operation_id)
                if not response:
                    raise compiler.GenerationError("response is not a referenced model")
                models["ProbeResponse"] = {"raw": response, "borrowed": False, "accessors": {}}
                operation["response"] = "ProbeResponse"
            if request:
                models["ProbeRequest"] = {"raw": request, "constructor": sorted(openapi.schema(request).get("required", []))}
                operation["request"] = "ProbeRequest"
            overlay = {"schema_version": 2, "client": {"name": "ProbeClient"}, "models": models,
                       "resources": {"resource": {"name": "ProbeResource", "operations": {"invoke": operation}}}}
            _, files = compiler.compile_facade(openapi, rust, overlay)
            public_surface(files)
            model_source = files["facade_types.rs"]
            resource_source = files["resource.rs"]
            modules[operation_id] = (
                f"mod probe_{index} {{ use mistralai::SdkError;\n"
                f"mod facade_types {{ {model_source} }}\nuse facade_types::*;\n"
                f"mod resource {{ {resource_source} }}\n}}\n")
        except (compiler.GenerationError, ValueError, KeyError) as error:
            rejected[operation_id] = str(error)
    return modules, rejected


def main():
    openapi = compiler.OpenApiIndex.load(ROOT / "spec/openapi.yaml")
    rust = compiler.RustIndex.load(ROOT / "src/generated")
    configured = json.loads((ROOT / "codegen/sdk-semantics.json").read_text())
    configured, _ = expand_manifest(
        openapi, configured,
        json.loads((ROOT / "codegen/sdk-taxonomy.json").read_text()),
        json.loads((ROOT / "src/generated/coverage.json").read_text()),
        rust,
    )
    modules, rejected = probes(openapi, rust, configured)
    report = {"generated_candidates": sorted(modules), "rejected_candidates": rejected}
    print(json.dumps(report, indent=2), flush=True)
    digest = hashlib.sha256(json.dumps(report, sort_keys=True, separators=(",", ":")).encode()).hexdigest()
    baseline = json.loads((ROOT / "codegen/sdk-probe-baseline.json").read_text())
    actual = {"schema_version": 1, "generated_candidates": len(modules),
              "rejected_candidates": len(rejected), "report_sha256": digest}
    if actual != baseline:
        raise SystemExit(f"coverage probe drift requires review: expected {baseline}, got {actual}")
    dependencies = tomllib.loads((ROOT / "Cargo.toml").read_text())["dependencies"]
    def dependency_version(name):
        configured = dependencies[name]
        return configured if isinstance(configured, str) else configured["version"]
    with tempfile.TemporaryDirectory(prefix="sdk-coverage-probe-") as directory:
        path = Path(directory)
        (path / "src").mkdir()
        (path / "Cargo.toml").write_text(
            '[package]\nname = "sdk-coverage-probe"\nversion = "0.0.0"\nedition = "2024"\n'
            f'[dependencies]\nmistralai = {{ package = "mistralai-sdk", path = {json.dumps(str(ROOT))} }}\n'
            + "\n".join(f'{name} = {json.dumps(dependency_version(name))}'
                        for name in ("url", "uuid", "futures-util")) + "\n")
        shutil.copyfile(ROOT / "Cargo.lock", path / "Cargo.lock")
        (path / "src/lib.rs").write_text(
            '#![allow(dead_code, unused_imports)]\nmod generated { pub use mistralai::raw::{client, types}; }\n'
            + "\n".join(modules.values()))
        subprocess.run(["cargo", "check", "--offline", "--manifest-path", str(path / "Cargo.toml")], check=True)
    print(f"Compiled {len(modules)} candidate projections; {len(rejected)} candidates require additional policies/capabilities.")


if __name__ == "__main__":
    main()
