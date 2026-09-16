"""Compile an explicit facade policy into resolved IR and Rust sources.

This module is the generic compiler boundary. It deliberately knows nothing
about official-SDK taxonomy discovery, automatic projection, coverage reports,
or public-API audit policy.
"""

from __future__ import annotations

from typing import Any

import sdk_codegen as frontend
import sdk_emit
from sdk_ir import FacadeIr
from sdk_model_lowering import resolve_models
from sdk_operation_lowering import resolve_operations


GenerationError = frontend.GenerationError
OpenApiIndex = frontend.OpenApiIndex
RustIndex = frontend.RustIndex


def compile_ir(openapi: OpenApiIndex, rust: RustIndex, manifest: dict[str, Any]) -> FacadeIr:
    """Validate and lower one explicit semantic facade policy."""
    ir = frontend.build_ir(openapi, rust, manifest)
    try:
        return resolve_models(resolve_operations(ir, rust), openapi, rust)
    except ValueError as error:
        raise GenerationError(str(error)) from error


def compile_facade(
    openapi: OpenApiIndex,
    rust: RustIndex,
    manifest: dict[str, Any],
) -> tuple[FacadeIr, dict[str, str]]:
    """Compile an explicit policy to resolved facade IR and deterministic Rust files."""
    ir = compile_ir(openapi, rust, manifest)
    return ir, sdk_emit.emit(ir)
