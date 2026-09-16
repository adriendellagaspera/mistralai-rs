import json
from collections import defaultdict
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
coverage = json.loads((ROOT / "src/generated/coverage.json").read_text())
taxonomy = json.loads((ROOT / "codegen/sdk-taxonomy.json").read_text())
ops = taxonomy.get("operations", {})
by_tag = defaultdict(list)
for item in coverage["operations"]:
    if not item.get("upstream", True):
        continue
    paths = ops.get(item["operation_id"], [])
    for tag in item.get("tags", []):
        if paths:
            by_tag[tag].append((item["operation_id"], paths))

for item in coverage["operations"]:
    if not item.get("upstream", True) or ops.get(item["operation_id"]):
        continue
    print("\n===", item["operation_id"], "===")
    print("method", item.get("method"), "path", item.get("path"), "tags", item.get("tags"))
    for tag in item.get("tags", []):
        print("neighbors", tag)
        for operation_id, paths in sorted(by_tag.get(tag, [])):
            print(" ", operation_id, "=>", paths)
