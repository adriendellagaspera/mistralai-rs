"""Immutable policies compiled from the validated semantic overlay.

Only the frontend handles JSON dictionaries. Backend model policies form a
closed union of request, discriminated union and response-view strategies.
"""
from dataclasses import dataclass
from enum import StrEnum


class AccessorKind(StrEnum):
    COPY = "copy"
    REF = "ref"
    OPTIONAL_COPY = "optional_copy"
    OPTIONAL_REF = "optional_ref"
    ITER = "iter"
    FIRST_STRING_VARIANT = "first_string_variant"


@dataclass(frozen=True)
class Accessor:
    kind: AccessorKind
    path: tuple[str, ...]
    wrapper: str | None


@dataclass(frozen=True)
class UnionPolicy:
    root: str
    path: tuple[str, ...]
    payload: str
    targets: tuple[str, ...]


@dataclass(frozen=True)
class SimpleUnionPolicy:
    variants: tuple[tuple[str, str, str | None], ...]
    bidirectional: bool


@dataclass(frozen=True)
class UnionFactory:
    field: str
    leading: tuple[str, ...]
    rename: tuple[tuple[str, str], ...]


@dataclass(frozen=True)
class TypeAliasPolicy:
    pass


@dataclass(frozen=True)
class RequestPolicy:
    constructor: tuple[str, ...]
    exclude: tuple[str, ...]
    adapters: tuple[tuple[str, str], ...]
    factory: UnionFactory | None


@dataclass(frozen=True)
class ViewPolicy:
    borrowed: bool
    accessors: tuple[tuple[str, Accessor], ...]


ModelPolicy = UnionPolicy | SimpleUnionPolicy | TypeAliasPolicy | RequestPolicy | ViewPolicy


@dataclass(frozen=True)
class StreamPolicy:
    item: str
    wrapper: str
    type: str


def model_policy(config: dict) -> ModelPolicy:
    if "union" in config:
        union = config["union"]
        return UnionPolicy(union["root"], tuple(union["path"]), union["payload"],
                           tuple(union.get("targets", ())))
    if "simple_union" in config:
        return SimpleUnionPolicy(tuple(
            (raw, value if isinstance(value, str) else value["name"],
             None if isinstance(value, str) else value.get("adapter"))
            for raw, value in config["simple_union"]["variants"].items()
        ), config["simple_union"].get("bidirectional", False))
    if config.get("type_alias"):
        return TypeAliasPolicy()
    if "accessors" in config:
        return ViewPolicy(config.get("borrowed", True), tuple(
            (name, Accessor(AccessorKind(value["kind"]), tuple(value["path"]), value.get("wrapper")))
            for name, value in config["accessors"].items()))
    factory = config.get("union_factory")
    return RequestPolicy(tuple(config.get("constructor", ())), tuple(config.get("exclude", ())),
                         tuple(config.get("adapters", {}).items()),
                         UnionFactory(factory["field"], tuple(factory.get("leading", ())),
                                      tuple(factory.get("rename", {}).items())) if factory else None)


def stream_policy(config: dict | None) -> StreamPolicy | None:
    return StreamPolicy(config["item"], config.get("wrapper", config["item"]), config["type"]) if config else None
