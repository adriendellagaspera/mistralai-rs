"""Immutable generator-facing raw binding IR consumed by facade lowering."""

from __future__ import annotations

from dataclasses import dataclass
from types import MappingProxyType
from typing import Any, Mapping

from rust_types import RustType, parse_type


class RawIrError(ValueError):
    """The raw binding contract is missing, ambiguous, or malformed."""


@dataclass(frozen=True)
class RawField:
    name: str
    syntax: RustType

    @property
    def type(self) -> str:
        return self.syntax.spelling


@dataclass(frozen=True)
class RawVariant:
    name: str
    payload: str | None


@dataclass(frozen=True)
class RawParameter:
    name: str
    syntax: RustType

    @property
    def type(self) -> str:
        return self.syntax.spelling


@dataclass(frozen=True)
class RawOperation:
    name: str
    parameters: tuple[RawParameter, ...]
    return_type: str
    success_type: str


@dataclass(frozen=True)
class RawIr:
    """Normalized raw SDK shape, independent from how generated Rust was parsed."""

    _structs: tuple[tuple[str, tuple[RawField, ...]], ...]
    _enums: tuple[tuple[str, tuple[RawVariant, ...]], ...]
    _aliases: tuple[tuple[str, RustType], ...]
    _operations: tuple[tuple[str, RawOperation], ...]
    _symbol_modules: tuple[tuple[str, str], ...]

    @classmethod
    def from_parts(
        cls,
        *,
        structs: Mapping[str, tuple[RawField, ...]],
        enums: Mapping[str, tuple[RawVariant, ...]],
        aliases: Mapping[str, RustType],
        operations: Mapping[str, RawOperation],
        symbol_modules: Mapping[str, str],
    ) -> "RawIr":
        return cls(
            tuple(sorted(structs.items())),
            tuple(sorted(enums.items())),
            tuple(sorted(aliases.items())),
            tuple(sorted(operations.items())),
            tuple(sorted(symbol_modules.items())),
        )

    @classmethod
    def from_dict(cls, value: Mapping[str, Any]) -> "RawIr":
        """Load the candidate machine-readable sidecar representation."""
        if value.get("schema_version") != 1:
            raise RawIrError("unsupported raw IR sidecar version")
        try:
            structs = {
                name: tuple(RawField(field["name"], parse_type(field["type"]))
                            for field in fields)
                for name, fields in value["structs"].items()
            }
            enums = {
                name: tuple(RawVariant(variant["name"], variant.get("payload"))
                            for variant in variants)
                for name, variants in value["enums"].items()
            }
            aliases = {
                name: parse_type(type_name)
                for name, type_name in value["aliases"].items()
            }
            operations = {
                name: RawOperation(
                    operation["name"],
                    tuple(RawParameter(parameter["name"], parse_type(parameter["type"]))
                          for parameter in operation["parameters"]),
                    operation["return_type"],
                    operation["success_type"],
                )
                for name, operation in value["operations"].items()
            }
            symbol_modules = {
                name: module for name, module in value["symbol_modules"].items()
            }
        except (KeyError, TypeError, AttributeError, ValueError) as error:
            raise RawIrError("malformed raw IR sidecar") from error
        return cls.from_parts(
            structs=structs,
            enums=enums,
            aliases=aliases,
            operations=operations,
            symbol_modules=symbol_modules,
        )

    def to_dict(self) -> dict[str, Any]:
        """Return the deterministic candidate sidecar representation."""
        return {
            "schema_version": 1,
            "structs": {
                name: [{"name": field.name, "type": field.type} for field in fields]
                for name, fields in self._structs
            },
            "enums": {
                name: [{"name": variant.name, "payload": variant.payload}
                       for variant in variants]
                for name, variants in self._enums
            },
            "aliases": {name: syntax.spelling for name, syntax in self._aliases},
            "operations": {
                name: {
                    "name": operation.name,
                    "parameters": [
                        {"name": parameter.name, "type": parameter.type}
                        for parameter in operation.parameters
                    ],
                    "return_type": operation.return_type,
                    "success_type": operation.success_type,
                }
                for name, operation in self._operations
            },
            "symbol_modules": dict(self._symbol_modules),
        }

    @property
    def structs(self) -> Mapping[str, tuple[RawField, ...]]:
        return MappingProxyType(dict(self._structs))

    @property
    def enums(self) -> Mapping[str, tuple[RawVariant, ...]]:
        return MappingProxyType(dict(self._enums))

    @property
    def aliases(self) -> Mapping[str, RustType]:
        return MappingProxyType(dict(self._aliases))

    @property
    def operations(self) -> Mapping[str, RawOperation]:
        return MappingProxyType(dict(self._operations))

    @property
    def symbol_modules(self) -> Mapping[str, str]:
        return MappingProxyType(dict(self._symbol_modules))

    def fields(self, name: str) -> tuple[RawField, ...]:
        try:
            return self.structs[name]
        except KeyError as error:
            raise RawIrError(f"raw Rust struct not found: {name}") from error

    def variants(self, name: str) -> tuple[RawVariant, ...]:
        try:
            return self.enums[name]
        except KeyError as error:
            raise RawIrError(f"raw Rust enum not found: {name}") from error

    def operation(self, name: str) -> RawOperation:
        try:
            return self.operations[name]
        except KeyError as error:
            raise RawIrError(f"raw Rust operation not found: {name}") from error

    def qualified_type(self, spelling: str) -> str:
        modules = self.symbol_modules

        def render(syntax: RustType) -> str:
            if syntax.kind == "generic_type":
                return f"{syntax.constructor}<{', '.join(render(argument) for argument in syntax.arguments)}>"
            if syntax.spelling in modules:
                return f"crate::generated::{modules[syntax.spelling]}::{syntax.spelling}"
            return syntax.spelling

        return render(parse_type(spelling))
