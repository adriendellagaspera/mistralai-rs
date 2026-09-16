from pathlib import Path
import subprocess

ROOT = Path(__file__).resolve().parents[1]


def replace(path: str, old: str, new: str) -> None:
    target = ROOT / path
    text = target.read_text()
    if old not in text:
        raise SystemExit(f"patch anchor not found in {path}: {old[:120]!r}")
    target.write_text(text.replace(old, new, 1))


# The only expected textual merge conflict is the compiler test file. Keep the
# current main version, then add the scalar-enum adapter assertion below.
unmerged = subprocess.check_output(
    ["git", "diff", "--name-only", "--diff-filter=U"], text=True
).splitlines()
if unmerged != ["scripts/test_sdk_compiler.py"]:
    raise SystemExit(f"unexpected merge conflicts: {unmerged}")
subprocess.run(
    ["git", "checkout", "--theirs", "scripts/test_sdk_compiler.py"], check=True
)

replace(
    "scripts/test_sdk_compiler.py",
    "    def test_adapter_normalizes_supported_raw_shapes(self):\n",
    '''    def test_adapter_preserves_explicit_enum_wire_names(self):
        raw = OpenApiToRustAdapter.parse(
            b\'''pub enum Visibility {
                #[serde(rename = "shared_global")]
                SharedGlobal,
                #[serde(rename = "private")]
                Private,
            }\''',
            b"impl HttpClient {}",
        )
        self.assertEqual(
            [("SharedGlobal", None, "shared_global"), ("Private", None, "private")],
            [(variant.name, variant.payload, variant.wire_name)
             for variant in raw.variants("Visibility")],
        )

    def test_adapter_normalizes_supported_raw_shapes(self):
''',
)

# #42 makes RawIr serializable. Wire provenance is part of RawIr and therefore
# must survive the sidecar round trip as well.
replace(
    "codegen/sdk_raw_ir.py",
    '''                name: tuple(RawVariant(variant["name"], variant.get("payload"))
                            for variant in variants)
''',
    '''                name: tuple(RawVariant(
                    variant["name"], variant.get("payload"), variant.get("wire_name")
                ) for variant in variants)
''',
)
replace(
    "codegen/sdk_raw_ir.py",
    '''                name: [{"name": variant.name, "payload": variant.payload}
                       for variant in variants]
''',
    '''                name: [{"name": variant.name, "payload": variant.payload,
                        "wire_name": variant.wire_name} for variant in variants]
''',
)
replace(
    "codegen/raw-ir.schema.json",
    '''          "required": ["name", "payload"],
          "properties": {
            "name": {"type": "string"},
            "payload": {"type": ["string", "null"]}
          }
''',
    '''          "required": ["name", "payload", "wire_name"],
          "properties": {
            "name": {"type": "string"},
            "payload": {"type": ["string", "null"]},
            "wire_name": {"type": ["string", "null"]}
          }
''',
)
