"""Public API for the standalone Rust facade compiler."""

from __future__ import annotations

import sys as _sys

# Transitional aliases for the extracted internal module graph. They deliberately
# exclude generator adapters: importing the compiler must not import tree-sitter.
from . import rust_types as _rust_types
from . import rust_symbols as _rust_symbols

_sys.modules.setdefault("rust_types", _rust_types)
_sys.modules.setdefault("rust_symbols", _rust_symbols)

from . import sdk_ir as _sdk_ir
_sys.modules.setdefault("sdk_ir", _sdk_ir)

from . import sdk_raw_ir as _sdk_raw_ir
_sys.modules.setdefault("sdk_raw_ir", _sdk_raw_ir)

from . import sdk_frontend as _sdk_frontend
_sys.modules.setdefault("sdk_frontend", _sdk_frontend)

from . import sdk_model_lowering as _sdk_model_lowering
_sys.modules.setdefault("sdk_model_lowering", _sdk_model_lowering)

from . import sdk_operation_lowering as _sdk_operation_lowering
_sys.modules.setdefault("sdk_operation_lowering", _sdk_operation_lowering)

from . import sdk_emit as _sdk_emit
_sys.modules.setdefault("sdk_emit", _sdk_emit)

from . import sdk_runtime as _sdk_runtime
from . import sdk_compiler as _sdk_compiler

GenerationError = _sdk_compiler.GenerationError
OpenApiIndex = _sdk_compiler.OpenApiIndex
RawIr = _sdk_raw_ir.RawIr
RustBindingsIr = _sdk_raw_ir.RustBindingsIr
RawBindingLayout = _sdk_raw_ir.RawBindingLayout
RawClientBinding = _sdk_raw_ir.RawClientBinding
RustFacadeRuntime = _sdk_runtime.RustFacadeRuntime
DEFAULT_RUNTIME = _sdk_runtime.DEFAULT_RUNTIME
compile_ir = _sdk_compiler.compile_ir
compile_facade = _sdk_compiler.compile_facade

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
