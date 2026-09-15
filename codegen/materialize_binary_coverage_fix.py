from pathlib import Path

path = Path("codegen/coverage.py")
source = path.read_text()
old = '''def inventory(original, spec, client):
    methods = re.findall(r"pub async fn (\\w+)\\s*\\(", client)
    if len(methods) != len(operations(spec)):
        raise ValueError(f"Expected {len(operations(spec))} generated methods, found {len(methods)}")
    if re.search(r'HttpError::Config\\s*\\(\\s*"', client) or "unimplemented!" in client or "todo!" in client:
        raise ValueError("Generated client contains a runtime configuration stub")
    inventory = []
    upstream = {op["operationId"] for _, _, op in operations(original)}
    for path, method, op in operations(spec):
        # Error enum docs retain exact upstream IDs, independent of Rust casing.
        # Untyped-error endpoints still carry their IDs in the generated method.
        snake = re.sub(r"_+", "_", op["operationId"].replace("-", "_")).lower()
        if snake not in methods:
            raise ValueError(f"Missing generated operation: {op['operationId']} ({snake})")
        inventory.append({"operation_id": op["operationId"], "method": method.upper(),
                          "path": path.split("#")[0], "rust_method": snake,
                          "upstream": op["operationId"] in upstream,
                          "tags": op.get("tags", []),
                          "success_media": sorted({media for status, response in op.get("responses", {}).items()
                              if str(status).startswith("2") for media in response.get("content", {})})})
    return {
        "upstream_operations": len(upstream), "generated_methods": len(methods),
        "operations": sorted(inventory, key=lambda op: op["operation_id"])
    }
'''
new = '''def _rust_method(operation_id):
    return re.sub(r"_+", "_", operation_id.replace("-", "_")).lower()


def _has_binary_success(op):
    return any(
        media.get("schema", {}).get("type") == "string"
        and media.get("schema", {}).get("format") == "binary"
        for status, response in op.get("responses", {}).items()
        if str(status).startswith("2")
        for media in response.get("content", {}).values()
    )


def inventory(original, spec, client):
    methods = re.findall(r"pub async fn (\\w+)\\s*\\(", client)
    operation_methods = {_rust_method(op["operationId"]) for _, _, op in operations(spec)}
    companions = {
        f"{_rust_method(op['operationId'])}_stream"
        for _, _, op in operations(spec)
        if _has_binary_success(op)
    }
    collisions = sorted(operation_methods & companions)
    if collisions:
        raise ValueError(f"Generated transport companion collides with an operation: {collisions}")
    expected = operation_methods | companions
    actual = set(methods)
    if len(methods) != len(actual):
        duplicates = sorted(name for name in actual if methods.count(name) > 1)
        raise ValueError(f"Generated client contains duplicate methods: {duplicates}")
    if actual != expected:
        raise ValueError(
            f"Generated method inventory drift: missing={sorted(expected - actual)}, "
            f"extra={sorted(actual - expected)}"
        )
    if re.search(r'HttpError::Config\\s*\\(\\s*"', client) or "unimplemented!" in client or "todo!" in client:
        raise ValueError("Generated client contains a runtime configuration stub")
    inventory = []
    upstream = {op["operationId"] for _, _, op in operations(original)}
    for path, method, op in operations(spec):
        # Error enum docs retain exact upstream IDs, independent of Rust casing.
        # Untyped-error endpoints still carry their IDs in the generated method.
        snake = _rust_method(op["operationId"])
        inventory.append({"operation_id": op["operationId"], "method": method.upper(),
                          "path": path.split("#")[0], "rust_method": snake,
                          "upstream": op["operationId"] in upstream,
                          "tags": op.get("tags", []),
                          "success_media": sorted({media for status, response in op.get("responses", {}).items()
                              if str(status).startswith("2") for media in response.get("content", {})})})
    return {
        "upstream_operations": len(upstream), "generated_methods": len(methods),
        "transport_companions": sorted(companions),
        "operations": sorted(inventory, key=lambda op: op["operation_id"])
    }
'''
if source.count(old) != 1:
    raise SystemExit("coverage inventory block not found exactly once")
path.write_text(source.replace(old, new, 1))
