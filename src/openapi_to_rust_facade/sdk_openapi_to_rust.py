"""Adapter from openapi-to-rust generated source files to :mod:`sdk_raw_ir`."""

from __future__ import annotations

from pathlib import Path
import re

from tree_sitter import Language, Node, Parser
import tree_sitter_rust

from rust_types import parse_type
from sdk_raw_ir import RawField, RawIr, RawOperation, RawParameter, RawVariant


class OpenApiToRustAdapterError(ValueError):
    """Generated Rust cannot be normalized into the supported raw contract."""


def _text(source: bytes, node: Node | None) -> str:
    if node is None:
        raise OpenApiToRustAdapterError("missing Rust AST node")
    return source[node.start_byte : node.end_byte].decode()


def _children(node: Node | None, kind: str) -> list[Node]:
    return [child for child in node.named_children if child.type == kind] if node else []


def _serde_rename(attributes: list[str]) -> str | None:
    matches = []
    for attribute in attributes:
        match = re.fullmatch(r'#\[serde\(rename\s*=\s*"([^"]+)"\)\]', attribute)
        if match:
            matches.append(match.group(1))
    if len(matches) > 1:
        raise OpenApiToRustAdapterError("multiple serde rename attributes on raw enum variant")
    return matches[0] if matches else None


class OpenApiToRustAdapter:
    """Parse the currently supported openapi-to-rust source layout into RawIr."""

    @classmethod
    def load(cls, raw: Path) -> RawIr:
        return cls.parse(
            (raw / "types.rs").read_bytes(),
            (raw / "client.rs").read_bytes(),
        )

    @classmethod
    def parse(cls, types_source: bytes, client_source: bytes) -> RawIr:
        parser = Parser(Language(tree_sitter_rust.language()))
        structs: dict[str, tuple[RawField, ...]] = {}
        enums: dict[str, tuple[RawVariant, ...]] = {}
        aliases = {}
        operations: dict[str, RawOperation] = {}
        symbol_modules: dict[str, str] = {}

        parsed = {}
        for label, source in (("types", types_source), ("client", client_source)):
            root = parser.parse(source).root_node
            if root.has_error:
                raise OpenApiToRustAdapterError(f"invalid raw Rust {label} syntax")
            parsed[label] = root
            for item in root.named_children:
                if item.type in {"struct_item", "enum_item", "type_item"}:
                    name = _text(source, item.child_by_field_name("name"))
                    if name in symbol_modules:
                        raise OpenApiToRustAdapterError(f"ambiguous raw symbol {name}")
                    symbol_modules[name] = label

        for item in parsed["types"].named_children:
            if item.type == "struct_item":
                name = _text(types_source, item.child_by_field_name("name"))
                fields = []
                for field in _children(item.child_by_field_name("body"), "field_declaration"):
                    if not _children(field, "visibility_modifier"):
                        continue
                    fields.append(RawField(
                        _text(types_source, field.child_by_field_name("name")),
                        parse_type(_text(types_source, field.child_by_field_name("type"))),
                    ))
                structs[name] = tuple(fields)
            elif item.type == "enum_item":
                name = _text(types_source, item.child_by_field_name("name"))
                variants = []
                pending_attributes: list[str] = []
                body_node = item.child_by_field_name("body")
                for child in body_node.named_children if body_node else ():
                    if child.type == "attribute_item":
                        pending_attributes.append(_text(types_source, child))
                        continue
                    if child.type != "enum_variant":
                        continue
                    variant = child
                    variant_name = _text(types_source, variant.child_by_field_name("name"))
                    attributes = pending_attributes + [
                        _text(types_source, attribute)
                        for attribute in _children(variant, "attribute_item")
                    ]
                    pending_attributes = []
                    body = variant.child_by_field_name("body")
                    payload = None
                    if body and body.type == "ordered_field_declaration_list":
                        payload_nodes = [node for node in body.named_children if node.type != "attribute_item"]
                        if len(payload_nodes) != 1:
                            raise OpenApiToRustAdapterError(
                                f"raw enum {name}::{variant_name} is not unary"
                            )
                        payload = _text(types_source, payload_nodes[0])
                    variants.append(RawVariant(
                        variant_name, payload, _serde_rename(attributes)
                    ))
                enums[name] = tuple(variants)
            elif item.type == "type_item":
                name = _text(types_source, item.child_by_field_name("name"))
                aliases[name] = parse_type(_text(types_source, item.child_by_field_name("type")))

        for item in parsed["client"].named_children:
            if item.type != "impl_item":
                continue
            if _text(client_source, item.child_by_field_name("type")) != "HttpClient":
                continue
            for function in _children(item.child_by_field_name("body"), "function_item"):
                name = _text(client_source, function.child_by_field_name("name"))
                parameters = []
                parameter_list = function.child_by_field_name("parameters")
                for parameter in parameter_list.named_children if parameter_list else ():
                    if parameter.type != "parameter":
                        continue
                    identifier = next((child for child in parameter.named_children
                                       if child.type in {"identifier", "field_identifier"}), None)
                    parameters.append(RawParameter(
                        _text(client_source, identifier),
                        parse_type(_text(client_source, parameter.child_by_field_name("type"))),
                    ))
                return_node = function.child_by_field_name("return_type")
                success_type = ""
                if return_node and return_node.type == "generic_type":
                    arguments = next((child for child in return_node.named_children
                                      if child.type == "type_arguments"), None)
                    if arguments and arguments.named_children:
                        success_type = _text(client_source, arguments.named_children[0])
                operations[name] = RawOperation(
                    name,
                    tuple(parameters),
                    _text(client_source, return_node),
                    success_type,
                )

        return RawIr.from_parts(
            structs=structs,
            enums=enums,
            aliases=aliases,
            operations=operations,
            symbol_modules=symbol_modules,
        )
