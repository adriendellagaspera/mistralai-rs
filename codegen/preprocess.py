"""Minimal deterministic fixes for contradictions in the upstream spec."""

from pathlib import Path


CHAT_RESPONSE_REQUIRED = b"""    ChatCompletionResponse:
      allOf:
      - $ref: '#/components/schemas/ChatCompletionResponseBase'
      - type: object
        title: ChatCompletionResponse
        properties:
          choices:
            type: array
            items:
              $ref: '#/components/schemas/ChatCompletionChoice'
        required:
        - id
        - object
        - data
        - model
        - usage
        - created
        - choices
"""

CHAT_RESPONSE_REQUIRED_FIXED = CHAT_RESPONSE_REQUIRED.replace(b"        - data\n", b"")


def preprocess(source: bytes) -> bytes:
    """Return generator input without modifying the authenticated source spec."""
    count = source.count(CHAT_RESPONSE_REQUIRED)
    if count != 1:
        raise ValueError(
            "Expected exactly one known ChatCompletionResponse contradiction; "
            f"found {count}. Review the upstream schema before regenerating."
        )
    return source.replace(CHAT_RESPONSE_REQUIRED, CHAT_RESPONSE_REQUIRED_FIXED)


def main(source: str, destination: str) -> None:
    Path(destination).write_bytes(preprocess(Path(source).read_bytes()))
