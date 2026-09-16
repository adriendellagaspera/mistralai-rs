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
from .sdk_frontend import GenerationError, OpenApiIndex
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


def _component_ref(ref: Any) -> str | None:
    prefix = "#/components/schemas/"
    if not isinstance(ref, str) or not ref.startswith(prefix):
        return None
    name = ref[len(prefix) :]
    return name or None


def _merge_object_shapes(
    parts: list[dict[str, Any]], *, context: str
) -> dict[str, Any]:
    properties: dict[str, Any] = {}
    required: list[str] = []
    additional_properties: Any = None
    has_additional_properties = False

    for part in parts:
        for name, schema in part.get("properties", {}).items():
            if name in properties and properties[name] != schema:
                raise GenerationError(
                    f"conflicting OpenAPI property {context}.{name} across allOf"
                )
            properties[name] = deepcopy(schema)
        for name in part.get("required", []):
            if name not in required:
                required.append(name)
        if "additionalProperties" in part:
            candidate = part["additionalProperties"]
            if has_additional_properties and candidate != additional_properties:
                raise GenerationError(
                    f"conflicting additionalProperties for OpenAPI object {context}"
                )
            additional_properties = deepcopy(candidate)
            has_additional_properties = True

    missing = [name for name in required if name not in properties]
    if missing:
        raise GenerationError(
            f"OpenAPI object {context} requires undefined properties: {missing}"
        )

    result: dict[str, Any] = {"type": "object", "properties": properties}
    if required:
        result["required"] = required
    if has_additional_properties:
        result["additionalProperties"] = additional_properties
    return result


class OpenApi(OpenApiIndex):
    """Normalized OpenAPI contract consumed by the compiler."""

    def object_schema(self, name: str) -> dict[str, Any]:
        """Resolve one object schema, including local ``allOf`` composition.

        The result contains the effective properties, required fields and
        ``additionalProperties`` constraint. Composition is deliberately
        conservative: incompatible property definitions, incompatible open-
        object constraints, recursive references and incomplete required fields
        fail closed with :class:`GenerationError`.
        """

        def resolve(schema: dict[str, Any], stack: tuple[str, ...]) -> dict[str, Any]:
            reference = _component_ref(schema.get("$ref"))
            parts: list[dict[str, Any]] = []
            if "$ref" in schema:
                if reference is None:
                    raise GenerationError(
                        f"unsupported OpenAPI object reference in {name}: {schema.get('$ref')}"
                    )
                if reference in stack:
                    raise GenerationError(
                        f"recursive OpenAPI object composition: {' -> '.join((*stack, reference))}"
                    )
                parts.append(resolve(self.schema(reference), (*stack, reference)))

            for branch in schema.get("allOf", []):
                if not isinstance(branch, dict):
                    raise GenerationError(f"invalid allOf branch in OpenAPI object {name}")
                parts.append(resolve(branch, stack))

            local = {
                key: deepcopy(schema[key])
                for key in ("type", "properties", "required", "additionalProperties")
                if key in schema
            }
            if local:
                kind = local.get("type")
                if kind not in (None, "object"):
                    raise GenerationError(
                        f"OpenAPI schema {name} is not an object: {kind}"
                    )
                properties = local.get("properties", {})
                required = local.get("required", [])
                if not isinstance(properties, dict) or not isinstance(required, list) or any(
                    not isinstance(field, str) for field in required
                ):
                    raise GenerationError(f"invalid OpenAPI object shape: {name}")
                parts.append(local)

            if not parts:
                raise GenerationError(f"OpenAPI schema {name} is not an object")
            return _merge_object_shapes(parts, context=name)

        return resolve(self.schema(name), (name,))


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
class Compilation:
    """Resolved SDK IR plus deterministic generated files."""

    ir: FacadeIr
    files: Mapping[str, str]


def lower(openapi: OpenApi, bindings: Bindings, policy: Policy) -> FacadeIr:
    """Validate and lower compiler inputs to resolved SDK IR."""
    return sdk_compiler.compile_ir(openapi, bindings, policy.to_dict())


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
    return Compilation(ir, MappingProxyType(dict(files)))