"""Compatibility surface for the extracted facade compiler.

New code should import :mod:`sdk_frontend`, :mod:`sdk_compiler`, or
:mod:`sdk_pipeline` directly. This module keeps historical imports working while
routing all generation and emission through the extracted implementation.
"""

from __future__ import annotations

from pathlib import Path

import sdk_emit
import sdk_frontend as frontend
from sdk_frontend import *  # noqa: F401,F403 - deliberate compatibility re-export
from sdk_ir import (
    BinaryResponse,
    EmptyResponse,
    FacadeIr,
    JsonRequest,
    JsonResponse,
    ModelSpec as ResolvedModelSpec,
    NoRequest,
    OperationSpec as ResolvedOperationSpec,
    ParametersRequest,
    RawParameter,
    RawSignature,
    ResourceSpec as ResolvedResourceSpec,
    SseResponse,
    model_policy,
    stream_policy,
)
from sdk_model_lowering import resolve_models
from sdk_openapi_to_rust import OpenApiToRustAdapter
from sdk_operation_lowering import resolve_operations
from sdk_pipeline import generate, main
from sdk_raw_ir import RawIr


# Explicit aliases document the legacy plan types intentionally retained here.
ModelSpec = frontend.ModelSpec
OperationSpec = frontend.OperationSpec
ResourceSpec = frontend.ResourceSpec
SdkIr = frontend.SdkIr
_resolved_ir = frontend._resolved_ir


class RustIndex:
    """Deprecated raw-index compatibility constructor backed by the canonical adapter."""

    def __new__(cls, types_source: bytes, client_source: bytes) -> RawIr:
        try:
            return OpenApiToRustAdapter.parse(types_source, client_source)
        except ValueError as error:
            raise GenerationError(str(error)) from error

    @classmethod
    def load(cls, raw: Path) -> RawIr:
        try:
            return OpenApiToRustAdapter.load(raw)
        except ValueError as error:
            raise GenerationError(str(error)) from error


def _legacy_request(operation: OperationSpec, raw) -> object:
    if operation.request:
        overrides = operation.request_overrides
        if isinstance(overrides, dict):
            overrides = tuple(overrides.items())
        return JsonRequest(operation.request, operation.request_raw or "", tuple(overrides))
    return ParametersRequest() if raw.parameters else NoRequest()


def _legacy_response(operation: OperationSpec, raw_type: str):
    if operation.stream:
        return SseResponse(operation.stream)
    if operation.empty_response:
        return EmptyResponse()
    if operation.binary_response:
        return BinaryResponse()
    if operation.response:
        return JsonResponse(operation.response, raw_type)
    raise GenerationError(f"operation {operation.operation_id} needs a response projection")


def _resolved_operation(operation: OperationSpec, rust: RawIr) -> ResolvedOperationSpec:
    raw = rust.operation(operation.raw_method)
    signature = RawSignature(
        tuple(RawParameter(parameter.name, parameter.type) for parameter in raw.parameters),
        raw.return_type,
        raw.success_type,
    )
    return ResolvedOperationSpec(
        operation.name,
        operation.operation_id,
        operation.raw_method,
        signature,
        _legacy_request(operation, raw),
        _legacy_response(operation, raw.success_type),
    )


def _resolved_resources(
    resources: tuple[ResourceSpec, ...], rust: RawIr
) -> tuple[ResolvedResourceSpec, ...]:
    return tuple(
        ResolvedResourceSpec(
            resource.path,
            resource.module,
            resource.name,
            tuple(_resolved_operation(operation, rust) for operation in resource.operations),
        )
        for resource in resources
    )


def _emit_model(model: ModelSpec, openapi: OpenApiIndex, rust: RawIr) -> str:
    """Compatibility adapter for historical renderer-focused tests."""
    ir = FacadeIr(
        "Client",
        (ResolvedModelSpec(model.name, model.raw, model.config),),
        (),
    )
    resolved = resolve_models(ir, openapi, rust)
    return sdk_emit.emit_model(resolved.models[0])


def _emit_resource(
    resource: ResourceSpec,
    rust: RawIr,
    resources: tuple[ResourceSpec, ...],
) -> str:
    """Compatibility adapter that still exercises the canonical operation lowering."""
    resolved_resources = _resolved_resources(resources, rust)
    ir = resolve_operations(FacadeIr("Client", (), resolved_resources), rust)
    resolved = next(candidate for candidate in ir.resources if candidate.path == resource.path)
    return sdk_emit.emit_resource(resolved, ir.resources)


def _emit_mod(ir: SdkIr) -> str:
    """Compatibility adapter for resource-hierarchy tests that need no raw lookup."""
    models = tuple(ResolvedModelSpec(model.name, model.raw, model.config) for model in ir.models)
    resources = []
    for resource in ir.resources:
        operations = []
        for operation in resource.operations:
            if operation.stream:
                response = SseResponse(operation.stream)
            elif operation.binary_response:
                response = BinaryResponse()
            elif operation.empty_response:
                response = EmptyResponse()
            elif operation.response:
                response = JsonResponse(operation.response, "")
            else:
                raise GenerationError(
                    f"operation {operation.operation_id} needs a response projection"
                )
            operations.append(
                ResolvedOperationSpec(
                    operation.name,
                    operation.operation_id,
                    operation.raw_method,
                    RawSignature((), "", ""),
                    NoRequest(),
                    response,
                )
            )
        resources.append(
            ResolvedResourceSpec(
                resource.path,
                resource.module,
                resource.name,
                tuple(operations),
            )
        )
    return sdk_emit.emit_mod(FacadeIr(ir.client_name, models, tuple(resources)))


if __name__ == "__main__":
    main()
