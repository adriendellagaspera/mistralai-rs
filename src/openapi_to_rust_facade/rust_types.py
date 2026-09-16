"""Structural Rust type inspection at the raw-binding boundary.

Unknown syntax is retained as an opaque leaf, never interpreted using string
slicing. Generic consumers must explicitly recognize a constructor and arity.
"""
from dataclasses import dataclass
from functools import lru_cache

from tree_sitter import Language, Parser
import tree_sitter_rust


@dataclass(frozen=True)
class RustType:
    kind: str
    spelling: str
    constructor: str | None = None
    arguments: tuple["RustType", ...] = ()

    def unary(self, constructor: str) -> "RustType | None":
        if self.kind == "generic_type" and self.constructor == constructor and len(self.arguments) == 1:
            return self.arguments[0]
        return None


@lru_cache(maxsize=4096)
def parse_type(spelling: str) -> RustType:
    source = f"type Inspected = {spelling};".encode()
    root = Parser(Language(tree_sitter_rust.language())).parse(source).root_node
    if root.has_error or len(root.named_children) != 1:
        raise ValueError(f"invalid Rust type: {spelling!r}")
    node = root.named_children[0].child_by_field_name("type")
    if node is None:
        raise ValueError(f"missing Rust type: {spelling!r}")

    def convert(node):
        text = source[node.start_byte:node.end_byte].decode()
        if node.type == "generic_type":
            name = node.child_by_field_name("type")
            args = node.child_by_field_name("type_arguments")
            if args is None:
                args = next(child for child in node.named_children if child.type == "type_arguments")
            return RustType(node.type, text, source[name.start_byte:name.end_byte].decode(),
                            tuple(convert(child) for child in args.named_children))
        return RustType(node.type, text)

    return convert(node)
