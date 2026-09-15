import hashlib
import json
from pathlib import Path

root = Path(__file__).resolve().parents[1]
patch = root / "codegen/patches/generator.patch"
lock_path = root / "codegen.lock"
lock = json.loads(lock_path.read_text())
lock["generator_patch_sha256"] = hashlib.sha256(patch.read_bytes()).hexdigest()
lock_path.write_text(json.dumps(lock, indent=2) + "\n")
