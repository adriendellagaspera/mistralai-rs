"""Orchestrate semantic validation, operation lowering, and Rust emission."""

import argparse
import json
from pathlib import Path

import sdk_codegen as frontend
import sdk_emit
from sdk_autoproject import expand_manifest
from sdk_contracts import coverage_inventory, public_surface
from sdk_ir import BinaryResponse, SseResponse
from sdk_operation_lowering import resolve_operations


GENERATED = frontend.GENERATED
GenerationError = frontend.GenerationError


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
    openapi, rust = frontend.OpenApiIndex.load(openapi_path), frontend.RustIndex.load(raw)
    projection_report = None
    if taxonomy_path is not None:
        taxonomy = json.loads(taxonomy_path.read_text())
        raw_coverage = json.loads((raw / "coverage.json").read_text())
        manifest, projection_report = expand_manifest(openapi, manifest, taxonomy, raw_coverage, rust)
    ir = resolve_operations(frontend.build_ir(openapi, rust, manifest), rust)
    target.mkdir(parents=True, exist_ok=True)

    model_source = GENERATED + "use std::pin::Pin;\nuse futures_util::Stream;\nuse super::SdkError;\nuse crate::generated::types::*;\n\n"
    model_source += "\n\n".join(frontend._emit_model(model, openapi, rust) for model in ir.models)
    aliases = []
    if any(isinstance(operation.response_projection, BinaryResponse)
           for resource in ir.resources for operation in resource.operations):
        aliases.append(
            "pub type BinaryStream = Pin<Box<dyn Stream<Item = Result<bytes::Bytes, SdkError>> + Send + 'static>>;"
        )
    for resource in ir.resources:
        for operation in resource.operations:
            if isinstance(operation.response_projection, SseResponse):
                stream = operation.response_projection.stream
                aliases.append(
                    f"pub type {stream.type} = Pin<Box<dyn Stream<Item = Result<{stream.wrapper}, SdkError>> + Send + 'static>>;"
                )
    if aliases:
        model_source += "\n\n" + "\n".join(aliases) + "\n"
    files = {"facade_types.rs": model_source, "mod.rs": sdk_emit.emit_mod(ir)}
    for resource in ir.resources:
        filename = f"{resource.module}.rs"
        if filename in files or filename == "error.rs":
            raise GenerationError(f"resource collides with reserved output {filename}")
        files[filename] = sdk_emit.emit_resource(resource, ir.resources)
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
