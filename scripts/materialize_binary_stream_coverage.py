#!/usr/bin/env python3
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]


def replace_once(path: Path, old: str, new: str) -> None:
    text = path.read_text()
    if text.count(old) != 1:
        raise RuntimeError(f"expected one replacement in {path}, found {text.count(old)}")
    path.write_text(text.replace(old, new, 1))


def append_before_main(path: Path, addition: str) -> None:
    marker = '\n\nif __name__ == "__main__":\n'
    text = path.read_text()
    if addition.strip() in text:
        return
    if marker not in text:
        raise RuntimeError(f"main marker missing in {path}")
    path.write_text(text.replace(marker, "\n\n" + addition.rstrip() + marker, 1))


def update_coverage() -> None:
    path = ROOT / "codegen/coverage.py"
    replace_once(
        path,
        '''def inventory(original, spec, client):
    methods = re.findall(r"pub async fn (\\w+)\\s*\\(", client)
    if len(methods) != len(operations(spec)):
        raise ValueError(f"Expected {len(operations(spec))} generated methods, found {len(methods)}")
''',
        '''def rust_method(operation_id):
    return re.sub(r"_+", "_", operation_id.replace("-", "_")).lower()


def binary_stream_method(op):
    success = [response for status, response in op.get("responses", {}).items()
               if str(status).startswith("2")]
    if len(success) != 1:
        return None
    content = success[0].get("content", {})
    if len(content) != 1:
        return None
    payload = next(iter(content.values()))
    schema = payload.get("schema", {})
    if schema.get("type") == "string" and schema.get("format") == "binary":
        return rust_method(op["operationId"]) + "_stream"
    return None


def inventory(original, spec, client):
    methods = re.findall(r"pub async fn (\\w+)\\s*\\(", client)
    if len(methods) != len(set(methods)):
        raise ValueError("Generated client contains duplicate async method names")
    expected = {rust_method(op["operationId"]) for _, _, op in operations(spec)}
    expected |= {method for _, _, op in operations(spec)
                 if (method := binary_stream_method(op)) is not None}
    actual = set(methods)
    if actual != expected:
        missing = sorted(expected - actual)
        extra = sorted(actual - expected)
        raise ValueError(f"Generated method drift: missing={missing}, extra={extra}")
''',
    )
    replace_once(
        path,
        '''        snake = re.sub(r"_+", "_", op["operationId"].replace("-", "_")).lower()
''',
        '''        snake = rust_method(op["operationId"])
''',
    )
    replace_once(
        path,
        '''        inventory.append({"operation_id": op["operationId"], "method": method.upper(),
                          "path": path.split("#")[0], "rust_method": snake,
                          "upstream": op["operationId"] in upstream,
                          "tags": op.get("tags", []),
                          "success_media": sorted({media for status, response in op.get("responses", {}).items()
                              if str(status).startswith("2") for media in response.get("content", {})})})
''',
        '''        entry = {"operation_id": op["operationId"], "method": method.upper(),
                 "path": path.split("#")[0], "rust_method": snake,
                 "upstream": op["operationId"] in upstream,
                 "tags": op.get("tags", []),
                 "success_media": sorted({media for status, response in op.get("responses", {}).items()
                     if str(status).startswith("2") for media in response.get("content", {})})}
        if stream_method := binary_stream_method(op):
            entry["binary_stream_method"] = stream_method
        inventory.append(entry)
''',
    )


def update_autoproject() -> None:
    path = ROOT / "codegen/sdk_autoproject.py"
    replace_once(
        path,
        '''    raw_methods = {item["operation_id"]: item["rust_method"]
                   for item in raw_operations if item.get("upstream", True)}
''',
        '''    raw_methods = {item["operation_id"]: item["rust_method"]
                   for item in raw_operations if item.get("upstream", True)}
    raw_by_id = {item["operation_id"]: item for item in raw_operations}
''',
    )
    replace_once(
        path,
        '''            if len(content) == 1:
                raw_method = raw_method + "_stream"
''',
        '''            if len(content) == 1:
                stream_method = raw_by_id.get(operation_id, {}).get("binary_stream_method")
                if not stream_method:
                    rejected[operation_id] = "raw_binary_transport_mapping"
                    continue
                raw_method = stream_method
''',
    )
    replace_once(
        path,
        '''                    and item.get("success_media")
                    and set(item["success_media"]).issubset(binary_media)
''',
        '''                    and item.get("success_media")
                    and item.get("binary_stream_method")
                    and set(item["success_media"]).issubset(binary_media)
''',
    )
    replace_once(
        path,
        '''                    raw_method = candidates[0]["rust_method"] + "_stream"
''',
        '''                    raw_method = candidates[0]["binary_stream_method"]
''',
    )


def update_tests() -> None:
    path = ROOT / "scripts/test_coverage.py"
    addition = '''    def test_inventory_tracks_exact_binary_stream_companions(self):
        spec = {"paths": {"/v1/files/{file_id}/content": {"get": {
            "operationId": "download_file",
            "responses": {"200": {"content": {"application/octet-stream": {
                "schema": {"type": "string", "format": "binary"}
            }}}},
        }}}}
        client = """
        pub async fn download_file(&self) {}
        pub async fn download_file_stream(&self) {}
        """
        report = coverage.inventory(spec, spec, client)
        self.assertEqual(2, report["generated_methods"])
        self.assertEqual("download_file_stream", report["operations"][0]["binary_stream_method"])
        with self.assertRaises(ValueError):
            coverage.inventory(spec, spec, client + "pub async fn unexpected(&self) {}")
'''
    append_before_main(path, addition)

    auto = ROOT / "scripts/test_sdk_autoproject.py"
    replace_once(
        auto,
        '''            raw_coverage("download_thing"),
''',
        '''            {"operations": [{
                "operation_id": "download_thing",
                "rust_method": "download_thing",
                "binary_stream_method": "download_thing_stream",
                "upstream": True,
            }]},
''',
    )
    replace_once(
        auto,
        '''            {"operation_id": "sample_audio_wav", "rust_method": "sample_audio_wav", "method": "GET",
             "path": "/v1/audio/voices/{voice_id}/sample", "success_media": ["audio/wav"], "upstream": False},
''',
        '''            {"operation_id": "sample_audio_wav", "rust_method": "sample_audio_wav",
             "binary_stream_method": "sample_audio_wav_stream", "method": "GET",
             "path": "/v1/audio/voices/{voice_id}/sample", "success_media": ["audio/wav"], "upstream": False},
''',
    )


def main() -> None:
    update_coverage()
    update_autoproject()
    update_tests()


if __name__ == "__main__":
    main()
