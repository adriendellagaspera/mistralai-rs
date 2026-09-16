"""Normalize openapi-to-rust output into rust-sdk-compiler Bindings."""

from .parser import ParseError, parse_bindings
from .reader import read_bindings

__version__ = "0.2.1"

__all__ = ["ParseError", "parse_bindings", "read_bindings"]
