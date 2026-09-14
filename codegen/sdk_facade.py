"""Compatibility CLI for the semantic SDK compiler.

All parsing, validation and emission lives in :mod:`sdk_codegen`; this entry
point intentionally contains no Mistral-specific code or Rust templates.
"""

from sdk_codegen import GENERATED, GenerationError, generate, main

__all__ = ["GENERATED", "GenerationError", "generate", "main"]


if __name__ == "__main__":
    main()
