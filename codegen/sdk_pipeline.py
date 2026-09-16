"""Orchestrate projection, generic compilation, and repository audit outputs."""

import argparse
import json
from pathlib import Path

import sdk_compiler
from sdk_autoproject import expand_manifest
from sdk_contracts import coverage_inventory, public_surface


GENERATED = sdk_compiler.GENERATED
GenerationError = sdk_compiler.GenerationError


def generate(raw: Path, target: Path, manifest_path: Path, openapi_path: Path | None = None,
             taxonomy_path: Path | None = None) -> None:
    if openapi_path is None:
        candidate = manifest_path.parent.parent / "spec" / "openapi.yaml"
        if not candidate.exists():
            raise GenerationError("--openapi is required outside the repository layout")
        openapi_path = candidate
    if taxonomy_path is None:
        candidate = manifest_path.with_name("sdk-taxonomy.json")
        if candidate.exists():
            taxonomy_path = candidate
    manifest = json.loads(manifest_path.read_text())
    openapi = sdk_compiler.OpenApiIndex.load(openapi_path)
    rust = sdk_compiler.RustIndex.load(raw)
    projection_report = None
    if taxonomy_path is not None:
        taxonomy = json.loads(taxonomy_path.read_text())
        raw_coverage = json.loads((raw / "coverage.json").read_text())
        manifest, projection_report = expand_manifest(
            openapi, manifest, taxonomy, raw_coverage, rust
        )
    ir, files = sdk_compiler.compile_facade(openapi, rust, manifest)
    target.mkdir(parents=True, exist_ok=True)

    try:
        surface = public_surface(files)
    except ValueError as error:
        raise GenerationError(str(error)) from error
    for filename, source in files.items():
        (target / filename).write_text(source)
    (target / "api-surface.json").write_text(json.dumps(surface, indent=2) + "\n")
    coverage = {
        "schema_version": 2,
        "inventory": coverage_inventory(openapi, rust, ir),
        "resources": {resource.module: {"operations": [operation.operation_id for operation in resource.operations]}
                      for resource in ir.resources},
        "models": {model.name: {"raw": model.raw, "fields": sorted(field.name for field in rust.fields(model.raw)) if model.raw in rust.structs else []}
                   for model in ir.models},
    }
    if projection_report is not None:
        coverage["automatic_projection"] = projection_report
    (target / "coverage.json").write_text(json.dumps(coverage, indent=2) + "\n")


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("raw", type=Path)
    parser.add_argument("target", type=Path)
    parser.add_argument("--manifest", type=Path, required=True)
    parser.add_argument("--openapi", type=Path)
    parser.add_argument("--taxonomy", type=Path)
    args = parser.parse_args()
    generate(args.raw, args.target, args.manifest, args.openapi, args.taxonomy)


if __name__ == "__main__":
    main()
