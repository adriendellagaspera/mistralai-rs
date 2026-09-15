"""Immutable policy and facade IR for the semantic SDK compiler.

Only the frontend handles JSON dictionaries. Policy objects capture deliberate
product semantics; resolved facade objects capture the complete operation shape
selected during lowering before Rust emission.
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
class RequestPolicy:
    constructor: tuple[str, ...]
    exclude: tuple[str, ...]
    adapters: tuple[tuple[str, str], ...]
    factory: UnionFactory | None


@dataclass(frozen=True)
class ViewPolicy:
    borrowed: bool
    accessors: tuple[tuple[str, Accessor], ...]


ModelPolicy = UnionPolicy | SimpleUnionPolicy | RequestPolicy | ViewPolicy


@dataclass(frozen=True)
class StreamPolicy:
    item: str
    wrapper: str
    type: str


@dataclass(frozen=True)
class RawParameter:
    name: str
    type: str


@dataclass(frozen=True)
class RawSignature:
    parameters: tuple[RawParameter, ...]
    return_type: str
    success_type: str


@dataclass(frozen=True)
class NoRequest:
    """Operation with no request body or explicit HTTP parameters."""


@dataclass(frozen=True)
class ParametersRequest:
    """Operation whose public request is composed only of HTTP parameters."""


@dataclass(frozen=True)
class JsonRequest:
    model: str
    raw: str
    overrides: tuple[tuple[str, bool | None], ...]


RequestProjection = NoRequest | ParametersRequest | JsonRequest


@dataclass(frozen=True)
class JsonResponse:
    model: str
    raw: str


@dataclass(frozen=True)
class EmptyResponse:
    """Successful response with no response body."""


@dataclass(frozen=True)
class BinaryResponse:
    """Successful response exposed as the SDK binary stream primitive."""


@dataclass(frozen=True)
class SseResponse:
    stream: StreamPolicy


ResponseProjection = JsonResponse | EmptyResponse | BinaryResponse | SseResponse


@dataclass(frozen=True)
class ModelSpec:
    name: str
    raw: str
    config: ModelPolicy


@dataclass(frozen=True)
class OperationSpec:
    name: str
    operation_id: str
    raw_method: str
    raw_signature: RawSignature
    request_projection: RequestProjection
    response_projection: ResponseProjection

    @property
    def request(self) -> str | None:
        return self.request_projection.model if isinstance(self.request_projection, JsonRequest) else None

    @property
    def request_raw(self) -> str | None:
        return self.request_projection.raw if isinstance(self.request_projection, JsonRequest) else None

    @property
    def request_overrides(self) -> tuple[tuple[str, bool | None], ...]:
        return self.request_projection.overrides if isinstance(self.request_projection, JsonRequest) else ()

    @property
    def response(self) -> str | None:
        return self.response_projection.model if isinstance(self.response_projection, JsonResponse) else None

    @property
    def stream(self) -> StreamPolicy | None:
        return self.response_projection.stream if isinstance(self.response_projection, SseResponse) else None

    @property
    def empty_response(self) -> bool:
        return isinstance(self.response_projection, EmptyResponse)

    @property
    def binary_response(self) -> bool:
        return isinstance(self.response_projection, BinaryResponse)


@dataclass(frozen=True)
class ResourceSpec:
    path: tuple[str, ...]
    module: str
    name: str
    operations: tuple[OperationSpec, ...]


@dataclass(frozen=True)
class FacadeIr:
    client_name: str
    models: tuple[ModelSpec, ...]
    resources: tuple[ResourceSpec, ...]


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
