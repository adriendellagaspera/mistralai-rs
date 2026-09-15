from pathlib import Path

path = Path("scripts/probe_sdk_coverage.py")
source = path.read_text()
old = '''            request = openapi.request_schema(operation_id)\n            response = openapi.response_schema(operation_id)\n            if not response:\n                raise compiler.GenerationError("response is not a referenced model")\n            models = {"ProbeResponse": {"raw": response, "borrowed": False, "accessors": {}}}\n            operation = {"operation_id": operation_id, "response": "ProbeResponse"}\n            if request:\n'''
new = '''            request = openapi.request_schema(operation_id)\n            wire_operation = openapi.operation(operation_id)\n            success = [response for status, response in wire_operation.get("responses", {}).items()\n                       if str(status).startswith("2")]\n            empty_response = len(success) == 1 and not success[0].get("content")\n            models = {}\n            operation = {"operation_id": operation_id}\n            if empty_response:\n                operation["empty_response"] = True\n            else:\n                response = openapi.response_schema(operation_id)\n                if not response:\n                    raise compiler.GenerationError("response is not a referenced model")\n                models["ProbeResponse"] = {"raw": response, "borrowed": False, "accessors": {}}\n                operation["response"] = "ProbeResponse"\n            if request:\n'''
if source.count(old) != 1:
    raise SystemExit(f"expected one probe response block, got {source.count(old)}")
source = source.replace(old, new, 1)
source = source.replace(
    '"""Compile disposable projections for unmapped JSON operation candidates.\n',
    '"""Compile disposable projections for unmapped operation candidates.\n',
    1,
)
path.write_text(source)
