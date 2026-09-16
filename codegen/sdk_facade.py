"""Compatibility CLI for the semantic SDK compiler.

The pipeline validates source contracts before lowering them to immutable facade
IR. Rust rendering consumes only that resolved IR for operations and resources.
"""

from sdk_pipeline import GENERATED, GenerationError, generate, main

__all__ = ["GENERATED", "GenerationError", "generate", "main"]


if __name__ == "__main__":
    main()
