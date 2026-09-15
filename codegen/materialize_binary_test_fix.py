from pathlib import Path

path = Path("scripts/test_sdk_facade.py")
source = path.read_text()
marker = "    def test_typed_binary_success_is_a_candidate_not_non_json_gap(self):\n"
start = source.index(marker)
insertion_point = source.index("        self.openapi.write_text(json.dumps(document))\n", start)
insert = '''        (self.raw / "client.rs").write_text(CLIENT + """\nimpl HttpClient {\n    pub async fn download(&self) -> Result<bytes::Bytes, Error> { todo!() }\n}\n""")\n'''
source = source[:insertion_point] + insert + source[insertion_point:]
path.write_text(source)
