"""Syntax/symbol validation and conservative public API inventory.

The inventory is a review aid, not a replacement for rustdoc-based semver checks:
trait resolution, raw dependencies and blanket impls need the Rust compiler.
"""
def public_surface(files: dict[str, str]) -> dict[str, str]:
    from tree_sitter import Language, Parser
    import tree_sitter_rust

    surface = {}
    parser = Parser(Language(tree_sitter_rust.language()))
    for filename, source in sorted(files.items()):
        data = source.encode()
        root = parser.parse(data).root_node
        if root.has_error:
            raise ValueError(f"invalid emitted Rust in {filename}")
        symbols: set[tuple[str, str]] = set()

        def text(node):
            return data[node.start_byte:node.end_byte].decode() if node else ""

        def visit(item, owner=""):
            name = text(item.child_by_field_name("name")).removeprefix("r#")
            body = item.child_by_field_name("body")
            if item.type == "impl_item":
                target = text(item.child_by_field_name("type"))
                trait = text(item.child_by_field_name("trait"))
                signature = text(item)[:body.start_byte - item.start_byte] if body else text(item)
                if trait:
                    surface[f"{filename}::impl::{signature.strip()}"] = signature.strip()
                for child in body.named_children if body else ():
                    visit(child, target + (" as " + trait if trait else ""))
                return
            if not any(child.type == "visibility_modifier" for child in item.named_children):
                return
            if name:
                symbol = (owner, name)
                if symbol in symbols:
                    raise ValueError(f"duplicate emitted symbol {filename}::{owner}::{name}")
                symbols.add(symbol)
                signature = data[item.start_byte:(body.start_byte if body else item.end_byte)].decode().strip()
                # Enum variants and public struct fields are part of the API;
                # private request fields are intentionally excluded.
                if body and item.type in {"enum_item", "struct_item"}:
                    members = [text(child) for child in body.named_children
                               if item.type == "enum_item" or any(c.type == "visibility_modifier" for c in child.named_children)]
                    signature += " { " + ", ".join(members) + " }"
                surface[f"{filename}::{owner}::{name}"] = " ".join(signature.split())
        for item in root.named_children:
            visit(item)
        # Attributes and reexports can change exhaustiveness, derives or public
        # paths without changing an item's declaration text.
        attributes = []
        for item in root.named_children:
            if item.type == "attribute_item":
                attributes.append(text(item))
                continue
            name = text(item.child_by_field_name("name"))
            public = any(child.type == "visibility_modifier" for child in item.named_children)
            if public and name and attributes:
                surface[f"{filename}::attributes::{name}"] = " ".join(attributes)
            if public and item.type == "use_declaration":
                declaration = " ".join(text(item).split())
                surface[f"{filename}::reexport::{declaration}"] = declaration
            attributes = []
    return dict(sorted(surface.items()))


def _expanded_reexports(surface: dict[str, str]) -> dict[str, str]:
    """Normalize legacy flat grouped reexports into one public path per item."""
    expanded = {}
    marker = "::reexport::"
    for key, signature in surface.items():
        if (marker in key and signature.startswith("pub use ")
                and "::{" in signature and signature.endswith("};")):
            head, members = signature[:-2].split("::{", 1)
            if "{" not in members and "}" not in members:
                filename = key.split(marker, 1)[0]
                for member in members.split(","):
                    declaration = f"{head}::{member.strip()};"
                    expanded[f"{filename}{marker}{declaration}"] = declaration
                continue
        expanded[key] = signature
    return expanded


def compare_surface(before: dict[str, str], after: dict[str, str]) -> dict:
    before, after = _expanded_reexports(before), _expanded_reexports(after)
    removed = sorted(before.keys() - after.keys())
    added = sorted(after.keys() - before.keys())
    changed = sorted(key for key in before.keys() & after.keys() if before[key] != after[key])
    return {"classification": "review_required" if removed or changed else "additive" if added else "unchanged",
            "removed": removed, "changed": changed, "added": added}


def coverage_inventory(openapi, rust, definition) -> dict:
    if isinstance(definition, dict):
        mapped = {
            operation["operation_id"]
            for resource in definition.get("resources", {}).values()
            for operation in resource.get("operations", {}).values()
        }
    else:
        mapped = {
            op.operation_id
            for resource in definition.resources
            for op in resource.operations
        }
    inventory = {}
    for operation_id, operation in sorted(openapi.operations.items()):
        reasons = []
        content = operation.get("requestBody", {}).get("content", {})
        if content and set(content) != {"application/json"}:
            reasons.append("request_media_projection")
        if content and not openapi.request_schema(operation_id):
            reasons.append("inline_or_unresolved_request")
        responses = [r for status, r in operation.get("responses", {}).items() if str(status).startswith("2")]
        if len(responses) != 1:
            reasons.append("multiple_success_contracts")
        if (any(response.get("content") for response in responses)
                and not any("application/json" in response.get("content", {}) for response in responses)):
            reasons.append("non_json_success")
        if any(p.get("in") not in {"query", "path"} or "$ref" in p for p in operation.get("parameters", [])):
            reasons.append("parameter_binding_projection")
        if operation_id not in rust.operations:
            reasons.append("raw_symbol_mapping")
        inventory[operation_id] = {
            "method": operation["x-sdk-method"], "path": operation["x-sdk-path"],
            "status": "mapped" if operation_id in mapped else "capability_gap" if reasons else "candidate_unverified",
            "review_reasons": reasons,
        }
    return inventory
