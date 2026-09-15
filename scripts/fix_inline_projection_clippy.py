from pathlib import Path


def replace(path: str, old: str, new: str) -> None:
    p = Path(path)
    text = p.read_text()
    if old not in text:
        raise SystemExit(f"missing expected text in {path}: {old[:120]!r}")
    p.write_text(text.replace(old, new, 1))


# Auto-generated unions may opt into boxing without changing existing public unions.
replace(
    "codegen/sdk-semantics.schema.json",
    '"bidirectional": {"type": "boolean"},\n      "variants": {"type": "object",',
    '"bidirectional": {"type": "boolean"}, "boxed_variants": {"$ref": "#/$defs/names"},\n      "variants": {"type": "object",',
)

replace(
    "codegen/sdk_ir.py",
    'class SimpleUnionPolicy:\n    variants: tuple[tuple[str, str, str | None], ...]\n    bidirectional: bool\n',
    'class SimpleUnionPolicy:\n    variants: tuple[tuple[str, str, str | None], ...]\n    bidirectional: bool\n    boxed_variants: tuple[str, ...] = ()\n',
)
replace(
    "codegen/sdk_ir.py",
    '        ), config["simple_union"].get("bidirectional", False))\n',
    '        ), config["simple_union"].get("bidirectional", False),\n           tuple(config["simple_union"].get("boxed_variants", ())))\n',
)

# Avoid identity String -> String conversions.
replace(
    "codegen/sdk_codegen.py",
    'def _map_response(operation: OperationSpec, expression: str) -> str:\n    if operation.list_response:\n        return f"{expression}.map(|items| items.into_iter().map(Into::into).collect())"\n    return f"{expression}.map(Into::into)"\n',
    'def _map_response(operation: OperationSpec, expression: str) -> str:\n    if operation.list_response:\n        return f"{expression}.map(|items| items.into_iter().map(Into::into).collect())"\n    if operation.scalar_response:\n        return expression\n    return f"{expression}.map(Into::into)"\n',
)

# Box only explicitly configured public union variants; existing manual unions are unchanged.
replace(
    "codegen/sdk_codegen.py",
    '    for raw_name, public_name, adapter in model.config.variants:\n        payload = raw_variants[raw_name].payload\n',
    '    boxed_variants = set(model.config.boxed_variants)\n    for raw_name, public_name, adapter in model.config.variants:\n        payload = raw_variants[raw_name].payload\n',
)
replace(
    "codegen/sdk_codegen.py",
    '        variants.append(f"{public_name}({public_type})")\n        arms.append(f"{model.name}::{public_name}(value) => Self::{raw_name}({raw_value})")\n        reverse_value = adapt_value(raw_syntax, "value") if adapter else "value"\n        reverse_arms.append(f"{model.raw}::{raw_name}(value) => Self::{public_name}({reverse_value})")\n',
    '        boxed = raw_name in boxed_variants\n        variant_type = f"Box<{public_type}>" if boxed else public_type\n        variants.append(f"{public_name}({variant_type})")\n        source = "*value" if boxed else "value"\n        raw_value = adapt_value(raw_syntax, source) if adapter else source\n        arms.append(f"{model.name}::{public_name}(value) => Self::{raw_name}({raw_value})")\n        reverse_value = adapt_value(raw_syntax, "value") if adapter else "value"\n        if boxed:\n            reverse_value = f"Box::new({reverse_value})"\n        reverse_arms.append(f"{model.raw}::{raw_name}(value) => Self::{public_name}({reverse_value})")\n',
)
replace(
    "codegen/sdk_codegen.py",
    '                f"impl From<{public_type}> for {model.name} {{\\n"\n                f"    fn from(value: {public_type}) -> Self {{ Self::{public_name}(value) }}\\n}}"\n',
    '                f"impl From<{public_type}> for {model.name} {{\\n"\n                f"    fn from(value: {public_type}) -> Self {{ Self::{public_name}({\'Box::new(value)\' if boxed else \'value\'}) }}\\n}}"\n',
)

# Auto-generated ref unions are new API, so box their adapted payloads by construction.
replace(
    "codegen/sdk_autoproject.py",
    '        "simple_union": {"bidirectional": True, "variants": configured},\n',
    '        "simple_union": {\n            "bidirectional": True,\n            "boxed_variants": [variant.name for variant in variants],\n            "variants": configured,\n        },\n',
)
