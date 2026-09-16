"""Compile OpenAPI + normalized Rust bindings + policy into a Rust facade.

This is the generator-agnostic compiler boundary. Concrete generators live in
``openapi_to_rust_facade.adapters`` and only produce :class:`RawIr`.
"""

from __future__ import annotations

from typing import Any

from . import sdk_emit
from . import sdk_frontend as frontend
from .sdk_ir import FacadeIr
from .sdk_model_lowering import resolve_models
from .sdk_operation_lowering import resolve_operations
from .sdk_raw_ir import RawIr
from .sdk_runtime import DEFAULT_RUNTIME, RustFacadeRuntime


GenerationError = frontend.GenerationError
OpenApiIndex = frontend.OpenApiIndex


def compile_ir(
    openapi: OpenApiIndex,
    raw: RawIr,
    manifest: dict[str, Any],
) -> FacadeIr:
    """Validate and lower one explicit semantic facade policy."""
    try:
        ir = frontend.build_ir(openapi, raw, manifest)
        return resolve_models(resolve_operations(ir, raw), openapi, raw)
    except ValueError as error:
        if isinstance(error, GenerationError):
            raise
        raise GenerationError(str(error)) from error


def compile_facade(
    openapi: OpenApiIndex,
    raw: RawIr,
    manifest: dict[str, Any],
    *,
    runtime: RustFacadeRuntime = DEFAULT_RUNTIME,
) -> tuple[FacadeIr, dict[str, str]]:
    """Compile explicit semantics to resolved IR and deterministic Rust files."""
    ir = compile_ir(openapi, raw, manifest)
    return ir, sdk_emit.emit(ir, raw.binding, runtime)
