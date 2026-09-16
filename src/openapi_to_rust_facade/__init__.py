"""Public API for the standalone openapi-to-rust facade compiler."""

from __future__ import annotations

import sys as _sys

# The extracted v0.1.0 keeps the already-proven module graph byte-identical.
# Register package-local modules under their historical absolute names while
# consumers migrate to the package API. This compatibility bootstrap disappears
# once imports are made package-relative in a later package-only cleanup.
from . import rust_types as _rust_types
from . import rust_symbols as _rust_symbols

_sys.modules.setdefault("rust_types", _rust_types)
_sys.modules.setdefault("rust_symbols", _rust_symbols)

from . import sdk_ir as _sdk_ir
_sys.modules.setdefault("sdk_ir", _sdk_ir)

from . import sdk_raw_ir as _sdk_raw_ir
_sys.modules.setdefault("sdk_raw_ir", _sdk_raw_ir)

from . import sdk_openapi_to_rust as _sdk_openapi_to_rust
_sys.modules.setdefault("sdk_openapi_to_rust", _sdk_openapi_to_rust)

from . import sdk_frontend as _sdk_frontend
_sys.modules.setdefault("sdk_frontend", _sdk_frontend)

from . import sdk_model_lowering as _sdk_model_lowering
_sys.modules.setdefault("sdk_model_lowering", _sdk_model_lowering)

from . import sdk_operation_lowering as _sdk_operation_lowering
_sys.modules.setdefault("sdk_operation_lowering", _sdk_operation_lowering)

from . import sdk_emit as _sdk_emit
_sys.modules.setdefault("sdk_emit", _sdk_emit)

from . import sdk_compiler as _sdk_compiler
_sys.modules.setdefault("sdk_compiler", _sdk_compiler)

GenerationError = _sdk_compiler.GenerationError
OpenApiIndex = _sdk_compiler.OpenApiIndex
RawIr = _sdk_raw_ir.RawIr
OpenApiToRustAdapter = _sdk_openapi_to_rust.OpenApiToRustAdapter
compile_ir = _sdk_compiler.compile_ir
compile_facade = _sdk_compiler.compile_facade
load_raw_ir = _sdk_compiler.load_raw_ir

__version__ = "0.1.0"

__all__ = [
    "GenerationError",
    "OpenApiIndex",
    "OpenApiToRustAdapter",
    "RawIr",
    "compile_facade",
    "compile_ir",
    "load_raw_ir",
]
