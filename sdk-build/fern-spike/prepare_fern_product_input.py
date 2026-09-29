#!/usr/bin/env python3
from __future__ import annotations

import sys
from pathlib import Path
import yaml

src = Path(sys.argv[1])
dst = Path(sys.argv[2])
doc = yaml.safe_load(src.read_text())

# Preserve the API's dotted resource taxonomy as an actual nested SDK hierarchy.
for path_item in doc.get("paths", {}).values():
    if not isinstance(path_item, dict):
        continue
    for method, op in path_item.items():
        if method.lower() not in {"get", "post", "put", "patch", "delete", "options", "head", "trace"}:
            continue
        if not isinstance(op, dict):
            continue
        tags = op.get("tags") or []
        if tags and isinstance(tags[0], str):
            op["x-fern-sdk-group-name"] = tags[0].split(".")

# Representative product-level method names. This is deliberately not an
# exhaustive migration map; it proves Fern can own naming without a Rust facade.
method_names = {
    "chat_completion_v1_chat_completions_post": "complete",
    "completionV1ChatCompletionsPost": "complete",
    "fim_completion_v1_fim_completions_post": "complete",
    "completionV1FimCompletionsPost": "complete",
    "embeddings_v1_embeddings_post": "create",
    "files_api_routes_list_files": "list",
    "files_api_routes_upload_file": "upload",
    "files_api_routes_retrieve_file": "retrieve",
    "files_api_routes_delete_file": "delete",
    "files_api_routes_download_file": "download",
    "files_api_routes_get_signed_url": "get_signed_url",
    "ocr_v1_ocr_post": "process",
}
for path_item in doc.get("paths", {}).values():
    if not isinstance(path_item, dict):
        continue
    for method, op in path_item.items():
        if not isinstance(op, dict):
            continue
        op_id = op.get("operationId")
        if op_id in method_names:
            op["x-fern-sdk-method-name"] = method_names[op_id]

# Native Fern conditional streaming: split one OpenAPI operation into unary and
# SSE variants while pinning the request's stream field to false/true.
for path, response in {
    "/v1/chat/completions": "ChatCompletionResponse",
    "/v1/fim/completions": "FIMCompletionResponse",
}.items():
    op = doc["paths"][path]["post"]
    op["x-fern-sdk-method-name"] = "complete"
    op["x-fern-streaming"] = {
        "format": "sse",
        "stream-condition": "$request.stream",
        "response": {"$ref": f"#/components/schemas/{response}"},
        "response-stream": {"$ref": "#/components/schemas/CompletionChunk"},
    }

dst.parent.mkdir(parents=True, exist_ok=True)
dst.write_text(yaml.safe_dump(doc, sort_keys=False))
