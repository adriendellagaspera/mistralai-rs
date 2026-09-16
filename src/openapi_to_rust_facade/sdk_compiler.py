"""Compile an explicit facade policy into resolved IR and Rust sources.

This module is the generic compiler boundary. It deliberately knows nothing
about official-SDK taxonomy discovery, automatic projection, coverage reports,
or public-API audit policy.
"""

from __future__ import annotations

from pathlib import Path
from typing import Any

import sdk_emit
import sdk_frontend as frontend
from sdk_ir import FacadeIr
from sdk_model_lowering import resolve_models
from sdk_openapi_to_rust import OpenApiToRustAdapter
from sdk_operation_lowering import resolve_operations
from sdk_raw_ir import RawIr


GENERATED = frontend.GENERATED
GenerationError = frontend.GenerationError
OpenApiIndex = frontend.OpenApiIndex


def load_raw_ir(raw: Path) -> RawIr:
    """Normalize one openapi-to-rust output directory into the compiler raw IR."""
    try:
        return OpenApiToRustAdapter.load(raw)
    except ValueError as error:
        raise GenerationError(str(error)) from error


def compile_ir(openapi: OpenApiIndex, raw: RawIr, manifest: dict[str, Any]) -> FacadeIr:
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
) -> tuple[FacadeIr, dict[str, str]]:
    """Compile an explicit policy to resolved facade IR and deterministic Rust files."""
    ir = compile_ir(openapi, raw, manifest)
    return ir, sdk_emit.emit(ir)
