"""Compile disposable projections for unmapped operation candidates.

This validates the pinned compiler capabilities against Mistral's contracts; it
is repository audit tooling, not part of the generic compiler package.
"""

import hashlib
import json
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile
import tomllib

from openapi_to_rust_bindings import read_bindings
from rust_sdk_generator import GenerationError, OpenApi, Policy, compile, lower

ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT / "tooling" / "pipeline" / "mistral"))
from sdk_autoproject import expand_manifest  # noqa: E402
from sdk_contracts import coverage_inventory, public_surface  # noqa: E402
from sdk_pipeline import RUNTIME  # noqa: E402


def probes(openapi, bindings, configured):
    ir = lower(openapi, bindings, Policy.from_dict(configured))
    inventory = coverage_inventory(openapi, bindings, ir)
    modules, rejected = {}, {}
    for index, (operation_id, entry) in enumerate(inventory.items()):
        if entry["status"] != "candidate_unverified":
            continue
        try:
            request = openapi.request_schema(operation_id)
            wire_operation = openapi.operation(operation_id)
            success = [
                response
                for status, response in wire_operation.get("responses", {}).items()
                if str(status).startswith("2")
            ]
            empty_response = len(success) == 1 and not success[0].get("content")
            models = {}
            operation = {"operation_id": operation_id}
            if empty_response:
                operation["empty_response"] = True
            else:
                response = openapi.response_schema(operation_id)
                if not response:
                    raise GenerationError("response is not a referenced model")
                models["ProbeResponse"] = {
                    "raw": response,
                    "borrowed": False,
                    "accessors": {},
                }
                operation["response"] = "ProbeResponse"
            if request:
                models["ProbeRequest"] = {
                    "raw": request,
                    "constructor": sorted(openapi.schema(request).get("required", [])),
                }
                operation["request"] = "ProbeRequest"
            overlay = {
                "schema_version": 2,
                "client": {"name": "ProbeClient"},
                "models": models,
                "resources": {
                    "resource": {
                        "name": "ProbeResource",
                        "operations": {"invoke": operation},
                    }
                },
            }
            result = compile(
                openapi,
                bindings,
                Policy.from_dict(overlay),
                runtime=RUNTIME,
            )
            files = dict(result.files)
            public_surface(files)
            model_source = files["facade_types.rs"]
            resource_source = files["resource.rs"]
            modules[operation_id] = (
                f"mod probe_{index} {{ use mistralai::SdkError;\n"
                f"mod facade_types {{ {model_source} }}\nuse facade_types::*;\n"
                f"mod resource {{ {resource_source} }}\n}}\n"
            )
        except (GenerationError, ValueError, KeyError) as error:
            rejected[operation_id] = str(error)
    return modules, rejected


def main():
    openapi = OpenApi.load(ROOT / "tooling/sources/openapi/openapi.yaml")
    bindings = read_bindings(ROOT / "src/generated")
    configured = json.loads((ROOT / "tooling/pipeline/semantics.json").read_text())
    configured, _ = expand_manifest(
        openapi,
        configured,
        json.loads((ROOT / "tooling/sources/taxonomy.json").read_text()),
        json.loads((ROOT / "src/generated/coverage.json").read_text()),
        bindings,
    )
    modules, rejected = probes(openapi, bindings, configured)
    report = {
        "generated_candidates": sorted(modules),
        "rejected_candidates": rejected,
    }
    print(json.dumps(report, indent=2), flush=True)
    digest = hashlib.sha256(
        json.dumps(report, sort_keys=True, separators=(",", ":")).encode()
    ).hexdigest()
    baseline = json.loads((ROOT / "tooling/quality/probe-baseline.json").read_text())
    actual = {
        "schema_version": 1,
        "generated_candidates": len(modules),
        "rejected_candidates": len(rejected),
        "report_sha256": digest,
    }
    if actual != baseline:
        raise SystemExit(
            f"coverage probe drift requires review: expected {baseline}, got {actual}"
        )

    dependencies = tomllib.loads((ROOT / "Cargo.toml").read_text())["dependencies"]

    def dependency_version(name):
        configured_dependency = dependencies[name]
        return (
            configured_dependency
            if isinstance(configured_dependency, str)
            else configured_dependency["version"]
        )

    with tempfile.TemporaryDirectory(prefix="sdk-coverage-probe-") as directory:
        path = Path(directory)
        (path / "src").mkdir()
        (path / "Cargo.toml").write_text(
            '[package]\nname = "sdk-coverage-probe"\nversion = "0.0.0"\nedition = "2024"\n'
            f'[dependencies]\nmistralai = {{ package = "mistralai-sdk", path = {json.dumps(str(ROOT))} }}\n'
            + "\n".join(
                f'{name} = {json.dumps(dependency_version(name))}'
                for name in ("url", "uuid", "futures-util")
            )
            + "\n"
        )
        shutil.copyfile(ROOT / "Cargo.lock", path / "Cargo.lock")
        (path / "src/lib.rs").write_text(
            '#![allow(dead_code, unused_imports)]\n'
            'mod generated { pub use mistralai::raw::{client, types}; }\n'
            + "\n".join(modules.values())
        )
        subprocess.run(
            [
                "cargo",
                "check",
                "--offline",
                "--manifest-path",
                str(path / "Cargo.toml"),
            ],
            check=True,
        )
    print(
        f"Compiled {len(modules)} candidate projections; "
        f"{len(rejected)} candidates require additional policies/capabilities."
    )


if __name__ == "__main__":
    main()
