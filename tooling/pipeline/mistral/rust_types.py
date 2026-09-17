"""Narrow bridge for structural type inspection used by Mistral projection.

Remove this module once `RustType`/`parse_type` are part of the compiler's public
API, or once projection no longer needs compiler-level Rust syntax inspection.
"""

from rust_sdk_generator.rust_types import RustType, parse_type

__all__ = ["RustType", "parse_type"]
