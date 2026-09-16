"""Small public API for the Rust SDK compiler."""

from __future__ import annotations

from copy import deepcopy
from dataclasses import dataclass
import importlib.resources
import json
from types import MappingProxyType
from typing import Any, Mapping

from jsonschema import Draft202012Validator

from . import sdk_compiler
from .sdk_frontend import OpenApiIndex
from .sdk_ir import FacadeIr
from .sdk_raw_ir import RawIr
from .sdk_runtime import RustFacadeRuntime


_BINDINGS_VALIDATOR = Draft202012Validator(
    json.loads(
        importlib.resources.files(__package__)
        .joinpath("rust-bindings.schema.json")
        .read_text()
    )
)


class OpenApi(OpenApiIndex):
    """Normalized OpenAPI contract consumed by the compiler."""


class Bindings(RawIr):
    """Normalized Rust binding surface consumed by the compiler."""

    @classmethod
    def from_dict(cls, value: Mapping[str, Any]) -> "Bindings":
        errors = sorted(
            _BINDINGS_VALIDATOR.iter_errors(value),
            key=lambda error: tuple(str(part) for part in error.absolute_path),
        )
        if errors:
            raise ValueError(f"invalid Bindings: {errors[0].message}")
        return super().from_dict(value)


@dataclass(frozen=True)
class Policy:
    """Explicit public SDK policy."""

    _value: Mapping[str, Any]

    @classmethod
    def from_dict(cls, value: Mapping[str, Any]) -> "Policy":
        return cls(MappingProxyType(deepcopy(dict(value))))

    def to_dict(self) -> dict[str, Any]:
        return deepcopy(dict(self._value))


class Runtime(RustFacadeRuntime):
    """Consumer-owned Rust runtime/support contract."""


@dataclass(frozen=True)
class Ir(FacadeIr):
    """Fully resolved SDK intermediate representation."""


def _public_ir(value: FacadeIr) -> Ir:
    return Ir(value.client_name, value.models, value.resources)


@dataclass(frozen=True)
class Compilation:
    """Resolved SDK IR plus deterministic generated files."""

    ir: Ir
    files: Mapping[str, str]


def lower(openapi: OpenApi, bindings: Bindings, policy: Policy) -> Ir:
    """Validate and lower compiler inputs to resolved SDK IR."""
    return _public_ir(sdk_compiler.compile_ir(openapi, bindings, policy.to_dict()))


def compile(
    openapi: OpenApi,
    bindings: Bindings,
    policy: Policy,
    *,
    runtime: Runtime | None = None,
) -> Compilation:
    """Compile inputs to resolved SDK IR and deterministic Rust files."""
    kwargs = {} if runtime is None else {"runtime": runtime}
    ir, files = sdk_compiler.compile_facade(
        openapi,
        bindings,
        policy.to_dict(),
        **kwargs,
    )
    return Compilation(_public_ir(ir), MappingProxyType(dict(files)))
