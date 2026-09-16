"""Stable CLI for the repository-owned Mistral facade generation layer."""

from pathlib import Path
import sys

MISTRAL_CODEGEN = Path(__file__).with_name("mistral")
sys.path.insert(0, str(MISTRAL_CODEGEN))

from sdk_pipeline import GENERATED, GenerationError, generate, main  # noqa: E402

__all__ = ["GENERATED", "GenerationError", "generate", "main"]


if __name__ == "__main__":
    main()
