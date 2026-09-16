"""Public API for the standalone Rust facade compiler."""

from .sdk_compiler import GenerationError, OpenApiIndex, compile_facade, compile_ir
from .sdk_raw_ir import (
    RawBindingLayout,
    RawClientBinding,
    RawIr,
    RustBindingsIr,
)
from .sdk_runtime import DEFAULT_RUNTIME, RustFacadeRuntime

__version__ = "0.2.0"

__all__ = [
    "DEFAULT_RUNTIME",
    "GenerationError",
    "OpenApiIndex",
    "RawBindingLayout",
    "RawClientBinding",
    "RawIr",
    "RustBindingsIr",
    "RustFacadeRuntime",
    "compile_facade",
    "compile_ir",
]
