"""Immutable generator-facing raw binding IR consumed by facade lowering."""

from __future__ import annotations

from dataclasses import dataclass
from types import MappingProxyType
from typing import Mapping

from rust_types import RustType, parse_type


class RawIrError(ValueError):
    """The raw binding contract is missing or ambiguous."""


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
    wire_name: str | None = None


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
