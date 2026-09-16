from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
auto_path = ROOT / "codegen/sdk_autoproject.py"
test_path = ROOT / "scripts/test_sdk_autoproject.py"

auto = auto_path.read_text()
old = '''def _canonical_public_path(paths: list[str]) -> tuple[str | None, str | None]:
    if not paths:
        return None, "missing_official_taxonomy"
    # Prefer the most structured official path. Multiple equal-depth paths are
    # true aliases or transport conveniences and need an explicit semantic call.
    depth = max(path.count(".") for path in paths)
    candidates = sorted(path for path in paths if path.count(".") == depth)
    if len(candidates) != 1:
        return None, "taxonomy_alias_ambiguity"
    return candidates[0], None
'''
new = '''def _success_media(operation: dict[str, Any]) -> set[str]:
    return {
        media
        for status, response in operation.get("responses", {}).items()
        if str(status).startswith("2")
        for media in response.get("content", {})
    }


def _stream_public_path(path: str) -> bool:
    method = path.rsplit(".", 1)[-1]
    return method == "stream" or method.endswith("_stream")


def _canonical_public_path(
    paths: list[str], operation: dict[str, Any] | None = None
) -> tuple[str | None, str | None]:
    if not paths:
        return None, "missing_official_taxonomy"
    # Prefer the most structured official path. At equal depth, use transport
    # evidence only when it identifies one stream/non-stream convenience alias.
    depth = max(path.count(".") for path in paths)
    candidates = sorted(path for path in paths if path.count(".") == depth)
    if len(candidates) == 1:
        return candidates[0], None
    if operation is not None:
        media = _success_media(operation)
        stream = [path for path in candidates if _stream_public_path(path)]
        ordinary = [path for path in candidates if not _stream_public_path(path)]
        if media == {"text/event-stream"} and len(stream) == 1:
            return stream[0], None
        if "application/json" in media and "text/event-stream" not in media and len(ordinary) == 1:
            return ordinary[0], None
    return None, "taxonomy_alias_ambiguity"


def _identifier(value: str) -> str:
    return re.sub(r"_+", "_", re.sub(r"[^A-Za-z0-9_]", "_", value)).strip("_").lower()


def _operation_stem(operation_id: str) -> str:
    version = re.search(r"_v[0-9]+(?:_|$)", operation_id)
    stem = operation_id[:version.start()] if version and version.start() else operation_id
    return _identifier(stem)


def _common_operation_prefix(operation_ids: list[str]) -> tuple[str, ...]:
    if len(operation_ids) < 2:
        return ()
    split = [_operation_stem(operation_id).split("_") for operation_id in operation_ids]
    prefix: list[str] = []
    for values in zip(*split):
        if len(set(values)) != 1:
            break
        prefix.append(values[0])
    # Never strip a semantic action merely because every operation in a small
    # tag family happens to share that verb.
    if prefix and prefix[0] in {
        "get", "list", "create", "update", "delete", "post", "put", "patch",
        "start", "cancel", "judge", "execute", "archive", "export", "import",
    }:
        return ()
    return tuple(prefix)


def _fallback_context(openapi: Any, taxonomy: dict[str, Any]) -> tuple[
    dict[str, set[tuple[str, ...]]], dict[str, tuple[str, ...]]
]:
    resources: dict[str, set[tuple[str, ...]]] = {}
    operation_ids_by_tag: dict[str, list[str]] = {}
    for operation_id, operation in sorted(openapi.operations.items()):
        tags = operation.get("tags", [])
        for tag in tags:
            operation_ids_by_tag.setdefault(tag, []).append(operation_id)
        paths = taxonomy.get("operations", {}).get(operation_id, [])
        public_path, reason = _canonical_public_path(paths, operation)
        if reason or public_path is None:
            continue
        parts = tuple(public_path.split("."))
        if len(parts) < 2:
            continue
        for tag in tags:
            resources.setdefault(tag, set()).add(parts[:-1])
    prefixes = {
        tag: _common_operation_prefix(operation_ids)
        for tag, operation_ids in operation_ids_by_tag.items()
    }
    return resources, prefixes


def _fallback_resource_path(
    tag: str, resources: dict[str, set[tuple[str, ...]]]
) -> tuple[tuple[str, ...] | None, str | None]:
    tag_parts = tag.split(".")
    for depth in range(len(tag_parts), 0, -1):
        parent = ".".join(tag_parts[:depth])
        candidates = resources.get(parent, set())
        if len(candidates) > 1:
            return None, "taxonomy_fallback_ambiguity"
        if len(candidates) == 1:
            base = next(iter(candidates))
            suffix = tuple(_identifier(part) for part in tag_parts[depth:])
            if any(not part for part in suffix):
                return None, "invalid_fallback_taxonomy"
            return (*base, *suffix), None
    normalized = tuple(_identifier(part) for part in tag_parts)
    if not normalized or any(not part for part in normalized):
        return None, "invalid_fallback_taxonomy"
    return normalized, None


def _fallback_public_path(
    operation_id: str,
    operation: dict[str, Any],
    resources: dict[str, set[tuple[str, ...]]],
    prefixes: dict[str, tuple[str, ...]],
) -> tuple[str | None, str | None]:
    candidates: set[str] = set()
    for tag in operation.get("tags", []):
        resource, reason = _fallback_resource_path(tag, resources)
        if reason or resource is None:
            return None, reason or "missing_official_taxonomy"
        method_tokens = _operation_stem(operation_id).split("_")
        prefix = prefixes.get(tag, ())
        if prefix and tuple(method_tokens[:len(prefix)]) == prefix and len(method_tokens) > len(prefix):
            method_tokens = method_tokens[len(prefix):]
        method = "_".join(method_tokens)
        if not method:
            return None, "invalid_fallback_taxonomy"
        candidates.add(".".join((*resource, method)))
    if len(candidates) != 1:
        return None, "taxonomy_fallback_ambiguity" if candidates else "missing_official_taxonomy"
    return next(iter(candidates)), None
'''
if old not in auto:
    raise SystemExit("canonical taxonomy anchor not found")
auto = auto.replace(old, new)
old_loop = '''    added: list[str] = []
    rejected: dict[str, str] = {}

    for operation_id, operation in sorted(openapi.operations.items()):
        if operation_id in mapped:
            continue
        paths = taxonomy.get("operations", {}).get(operation_id, [])
        public_path, reason = _canonical_public_path(paths)
        if reason:
            rejected[operation_id] = reason
            continue
'''
new_loop = '''    added: list[str] = []
    rejected: dict[str, str] = {}
    fallback_resources, fallback_prefixes = _fallback_context(openapi, taxonomy)

    for operation_id, operation in sorted(openapi.operations.items()):
        if operation_id in mapped:
            continue
        paths = taxonomy.get("operations", {}).get(operation_id, [])
        if paths:
            public_path, reason = _canonical_public_path(paths, operation)
        else:
            public_path, reason = _fallback_public_path(
                operation_id, operation, fallback_resources, fallback_prefixes
            )
        if reason:
            rejected[operation_id] = reason
            continue
'''
if old_loop not in auto:
    raise SystemExit("expand loop anchor not found")
auto = auto.replace(old_loop, new_loop)
auto_path.write_text(auto)

tests = test_path.read_text()
anchor = '''    def test_equal_depth_aliases_require_review(self):
        _, report = sdk_autoproject.expand_manifest(
            FakeOpenApi(), manifest(),
            {"operations": {"list_things": ["things.list", "items.list"]}},
            raw_coverage("list_things"),
        )
        self.assertEqual("taxonomy_alias_ambiguity", report["rejected"]["list_things"])
'''
addition = anchor + '''
    def test_json_transport_resolves_stream_convenience_alias(self):
        expanded, report = sdk_autoproject.expand_manifest(
            FakeOpenApi(), manifest(),
            {"operations": {"list_things": ["things.complete", "things.stream"]}},
            raw_coverage("list_things"),
        )
        self.assertEqual(1, report["added_count"])
        self.assertIn("complete", expanded["resources"]["things"]["operations"])

    def test_missing_official_taxonomy_uses_openapi_tag_fallback(self):
        api = FakeOpenApi()
        api.operations["list_things"]["tags"] = ["beta.things"]
        expanded, report = sdk_autoproject.expand_manifest(
            api, manifest(), {"operations": {}}, raw_coverage("list_things"),
        )
        self.assertEqual(1, report["added_count"])
        self.assertIn("list_things", expanded["resources"]["beta_things"]["operations"])

    def test_fallback_inherits_known_parent_resource_mapping(self):
        api = FakeOpenApi()
        api.operations["list_things"]["tags"] = ["beta.workflows"]
        api.operations["worker_info"] = {
            "tags": ["beta.workflows.workers"],
            "responses": {"200": {"content": {"application/json": {"schema": {"$ref": "#/components/schemas/Thing"}}}}},
            "parameters": [],
        }
        expanded, report = sdk_autoproject.expand_manifest(
            api, manifest(),
            {"operations": {"list_things": ["workflows.list"]}},
            raw_coverage("list_things", "worker_info"),
        )
        self.assertEqual(2, report["added_count"])
        self.assertIn("worker_info", expanded["resources"]["workflows_workers"]["operations"])

    def test_fallback_strips_only_nonsemantic_common_operation_prefix(self):
        api = FakeOpenApi()
        api.operations = {
            "connector_list_v1": {
                "tags": ["beta.connectors"],
                "responses": {"200": {"content": {"application/json": {"schema": {"$ref": "#/components/schemas/Thing"}}}}},
                "parameters": [],
            },
            "connector_create_or_update_credentials_v1": {
                "tags": ["beta.connectors"],
                "responses": {"200": {"content": {"application/json": {"schema": {"$ref": "#/components/schemas/Thing"}}}}},
                "parameters": [],
            },
        }
        expanded, report = sdk_autoproject.expand_manifest(
            api, manifest(),
            {"operations": {"connector_list_v1": ["beta.connectors.list"]}},
            raw_coverage("connector_list_v1", "connector_create_or_update_credentials_v1"),
        )
        self.assertEqual(2, report["added_count"])
        self.assertIn(
            "create_or_update_credentials",
            expanded["resources"]["beta_connectors"]["operations"],
        )
'''
if anchor not in tests:
    raise SystemExit("test anchor not found")
tests = tests.replace(anchor, addition)
test_path.write_text(tests)
