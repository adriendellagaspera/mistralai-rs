import json
from pathlib import Path
import sys
import tempfile
import unittest

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT / "codegen"))
import sdk_codegen


TYPES = """
pub struct AnimalRequest {
    pub animals: Vec<AnimalUnion>,
    pub model: String,
    pub energy: Option<i64>,
}
pub enum AnimalUnion { Cat(Cat), Dog(Dog) }
pub struct Cat { pub content: CatContent, pub kind: Option<String> }
pub enum CatContent { String(String), Parts(Vec<String>) }
pub struct Dog { pub content: DogContent, pub kind: Option<String> }
pub enum DogContent { String(String), Parts(Vec<String>) }
pub struct AnimalResponse { pub id: String }
"""

CLIENT = """
impl HttpClient {
    pub async fn adopt(&self, request: AnimalRequest) -> Result<AnimalResponse, Error> { todo!() }
}
"""


def openapi_document():
    return {
        "openapi": "3.1.0",
        "paths": {
            "/animals": {
                "post": {
                    "operationId": "adopt",
                    "requestBody": {"content": {"application/json": {"schema": {"$ref": "#/components/schemas/AnimalRequest"}}}},
                    "responses": {"200": {"content": {"application/json": {"schema": {"$ref": "#/components/schemas/AnimalResponse"}}}}},
                }
            }
        },
        "components": {"schemas": {
            "AnimalRequest": {
                "type": "object",
                "required": ["animals", "model"],
                "properties": {
                    "animals": {"type": "array", "items": {
                        "oneOf": [{"$ref": "#/components/schemas/Cat"}, {"$ref": "#/components/schemas/Dog"}],
                        "discriminator": {"propertyName": "kind", "mapping": {
                            "cat": "#/components/schemas/Cat", "dog": "#/components/schemas/Dog"
                        }},
                    }},
                    "model": {"type": "string"},
                    "energy": {"type": "integer"},
                },
            },
            "Cat": {"type": "object", "required": ["content"], "properties": {"content": {"type": "string"}, "kind": {"type": "string", "const": "cat"}}},
            "Dog": {"type": "object", "required": ["content"], "properties": {"content": {"type": "string"}, "kind": {"type": "string", "const": "dog"}}},
            "AnimalResponse": {"type": "object", "required": ["id"], "properties": {"id": {"type": "string"}}},
        }},
    }


def manifest():
    return {
        "schema_version": 2,
        "client": {"name": "Menagerie"},
        "models": {
            "Animal": {"raw": "AnimalUnion", "union": {"root": "AnimalRequest", "path": ["animals", "items"], "payload": "content"}},
            "Adoption": {"raw": "AnimalRequest", "constructor": ["model", "animals"], "adapters": {"animals": "Animal"}},
            "Receipt": {"raw": "AnimalResponse", "borrowed": False, "accessors": {}},
        },
        "resources": {"zoo": {"name": "Zoo", "operations": {
            "adopt": {"operation_id": "adopt", "request": "Adoption", "response": "Receipt"}
        }}},
    }


class GenericSdkCompilerTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name)
        self.raw = self.root / "raw"
        self.raw.mkdir()
        (self.raw / "types.rs").write_text(TYPES)
        (self.raw / "client.rs").write_text(CLIENT)
        self.openapi = self.root / "openapi.json"
        self.overlay = self.root / "overlay.json"
        self.openapi.write_text(json.dumps(openapi_document()))
        self.overlay.write_text(json.dumps(manifest()))

    def generate(self):
        target = self.root / "sdk"
        sdk_codegen.generate(self.raw, target, self.overlay, self.openapi)
        return target

    def test_unrelated_api_uses_the_same_generic_emitters(self):
        target = self.generate()
        types = (target / "facade_types.rs").read_text()
        resource = (target / "zoo.rs").read_text()
        root = (target / "mod.rs").read_text()
        self.assertIn("pub enum Animal", types)
        self.assertIn("pub fn cat(content: impl Into<String>)", types)
        self.assertIn("pub fn energy(mut self, energy: i64)", types)
        self.assertIn("pub async fn adopt(&self, request: Adoption)", resource)
        self.assertIn("pub fn zoo(&self) -> Zoo<'_>", root)

    def test_compatible_union_addition_is_generated_without_engine_changes(self):
        document = openapi_document()
        items = document["components"]["schemas"]["AnimalRequest"]["properties"]["animals"]["items"]
        items["oneOf"].append({"$ref": "#/components/schemas/Bird"})
        items["discriminator"]["mapping"]["bird"] = "#/components/schemas/Bird"
        document["components"]["schemas"]["Bird"] = {
            "type": "object", "required": ["content"],
            "properties": {"content": {"type": "string"}, "kind": {"type": "string", "const": "bird"}},
        }
        self.openapi.write_text(json.dumps(document))
        source = TYPES.replace(
            "pub enum AnimalUnion { Cat(Cat), Dog(Dog) }",
            "pub enum AnimalUnion { Cat(Cat), Dog(Dog), Bird(Bird) }",
        )
        source += "\npub struct Bird { pub content: BirdContent, pub kind: Option<String> }\npub enum BirdContent { String(String) }\n"
        (self.raw / "types.rs").write_text(source)
        self.assertIn("Bird(String)", (self.generate() / "facade_types.rs").read_text())

    def test_openapi_raw_drift_fails_closed(self):
        document = openapi_document()
        document["components"]["schemas"]["AnimalRequest"]["properties"]["mood"] = {"type": "string"}
        self.openapi.write_text(json.dumps(document))
        with self.assertRaisesRegex(sdk_codegen.GenerationError, "OpenAPI/raw field drift"):
            self.generate()

    def test_operation_signature_drift_fails_closed(self):
        (self.raw / "client.rs").write_text(CLIENT.replace("request: AnimalRequest", "request: Other"))
        with self.assertRaisesRegex(sdk_codegen.GenerationError, "raw signature drift"):
            self.generate()

    def test_engine_contains_no_mistral_domain_backend(self):
        source = (ROOT / "codegen/sdk_codegen.py").read_text()
        for forbidden in ("generate_chat", "generate_ocr", "ChatCompletionRequest", "OCRRequest"):
            self.assertNotIn(forbidden, source)


if __name__ == "__main__":
    unittest.main()
