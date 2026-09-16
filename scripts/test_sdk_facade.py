import json
from pathlib import Path
import sys
import tempfile
import unittest

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT / "codegen"))
import sdk_codegen
from rust_types import parse_type
from sdk_contracts import compare_surface, public_surface


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

    def test_public_type_alias_expands_safe_generated_alias(self):
        source = TYPES + "\npub type MetadataDict = std::collections::BTreeMap<String, serde_json::Value>;\n"
        rust = sdk_codegen.RustIndex(source.encode(), CLIENT.encode())
        model = sdk_codegen.ModelSpec("MetadataValue", "MetadataDict", sdk_codegen.model_policy({"type_alias": True}))
        emitted = sdk_codegen._emit_model(model, sdk_codegen.OpenApiIndex(openapi_document()), rust)
        self.assertEqual(
            "pub type MetadataValue = std::collections::BTreeMap<String, serde_json::Value>;",
            emitted,
        )

    def test_generated_map_wrapper_emits_public_map_newtype(self):
        document = openapi_document()
        document["components"]["schemas"]["CreateThing"] = {
            "type": "object",
            "properties": {"metadata": {"type": "object", "additionalProperties": True}},
        }
        source = TYPES + "\npub struct MetadataMapRaw { pub additional_properties: std::collections::BTreeMap<String, serde_json::Value> }\n"
        rust = sdk_codegen.RustIndex(source.encode(), CLIENT.encode())
        model = sdk_codegen.ModelSpec(
            "MetadataMap", "MetadataMapRaw",
            sdk_codegen.model_policy({"map": {"root": "CreateThing", "path": ["metadata"]}}),
        )
        emitted = sdk_codegen._emit_model(model, sdk_codegen.OpenApiIndex(document), rust)
        self.assertIn("pub struct MetadataMap { values: std::collections::BTreeMap<String, serde_json::Value> }", emitted)
        self.assertIn("impl From<std::collections::BTreeMap<String, serde_json::Value>> for MetadataMap", emitted)
        self.assertIn("additional_properties: value.values", emitted)

    def test_generated_map_wrapper_value_drift_fails_closed(self):
        document = openapi_document()
        document["components"]["schemas"]["CreateThing"] = {
            "type": "object",
            "properties": {"metadata": {"type": "object", "additionalProperties": True}},
        }
        source = TYPES + "\npub struct MetadataMapRaw { pub additional_properties: std::collections::BTreeMap<String, String> }\n"
        rust = sdk_codegen.RustIndex(source.encode(), CLIENT.encode())
        model = sdk_codegen.ModelSpec(
            "MetadataMap", "MetadataMapRaw",
            sdk_codegen.model_policy({"map": {"root": "CreateThing", "path": ["metadata"]}}),
        )
        with self.assertRaisesRegex(sdk_codegen.GenerationError, "map value drift"):
            sdk_codegen._emit_model(model, sdk_codegen.OpenApiIndex(document), rust)

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

    def test_types_are_structural_not_delimiter_slices(self):
        inner = parse_type("Option < Vec < Result<String, Error> > >").unary("Option")
        self.assertEqual(inner.unary("Vec").constructor, "Result")
        self.assertEqual(len(inner.unary("Vec").arguments), 2)
        self.assertIsNone(parse_type("other::Option<String>").unary("Option"))
        with self.assertRaises(ValueError):
            parse_type("Option<String> ; fn injected() {}")

    def test_overlay_rejects_wrong_nested_types(self):
        overlay = manifest()
        overlay["models"]["Adoption"]["constructor"] = "model"
        self.overlay.write_text(json.dumps(overlay))
        with self.assertRaisesRegex(sdk_codegen.GenerationError, "overlay /models/Adoption"):
            self.generate()

    def test_overlay_rejects_unknown_nested_keys(self):
        overlay = manifest()
        overlay["models"]["Receipt"]["accessors"] = {"id": {"kind": "ref", "path": ["id"], "body": "unsafe"}}
        self.overlay.write_text(json.dumps(overlay))
        with self.assertRaisesRegex(sdk_codegen.GenerationError, "overlay /models/Receipt"):
            self.generate()

    def test_overlay_rejects_unknown_version(self):
        overlay = manifest()
        overlay["schema_version"] = 3
        self.overlay.write_text(json.dumps(overlay))
        with self.assertRaisesRegex(sdk_codegen.GenerationError, "overlay /schema_version"):
            self.generate()

    def test_duplicate_methods_and_invalid_identifiers_fail_before_emission(self):
        with self.assertRaisesRegex(ValueError, "duplicate emitted symbol"):
            public_surface({"test.rs": "pub struct A; impl A { pub fn x() {} pub fn x() {} }"})
        with self.assertRaisesRegex(ValueError, "invalid emitted Rust"):
            public_surface({"test.rs": "pub struct 123;"})

    def test_surface_changes_are_classified_conservatively(self):
        before = {"A::new": "pub fn new() -> Self"}
        self.assertEqual(compare_surface(before, before)["classification"], "unchanged")
        self.assertEqual(compare_surface(before, before | {"A::limit": "pub fn limit(i64)"})["classification"], "additive")
        self.assertEqual(compare_surface(before, {})["classification"], "review_required")

    def test_grouped_reexport_addition_is_additive(self):
        before = {
            "mod.rs::reexport::pub use facade_types::{A};":
                "pub use facade_types::{A};",
        }
        after = {
            "mod.rs::reexport::pub use facade_types::{A, B};":
                "pub use facade_types::{A, B};",
        }
        report = compare_surface(before, after)
        self.assertEqual(report["classification"], "additive")
        self.assertEqual(report["removed"], [])
        self.assertEqual(
            report["added"],
            ["mod.rs::reexport::pub use facade_types::B;"],
        )

    def test_nested_resources_emit_hierarchical_accessors(self):
        operation = sdk_codegen.OperationSpec("list", "list", "list", None, "Receipt", (), None)
        root = sdk_codegen.ResourceSpec(("batch",), "batch", "Batch", ())
        child = sdk_codegen.ResourceSpec(("batch", "jobs"), "batch_jobs", "BatchJobs", (operation,))
        rust = sdk_codegen.RustIndex(
            TYPES.encode(),
            b"impl HttpClient { pub async fn list(&self) -> Result<AnimalResponse, Error> { todo!() } }",
        )
        root_source = sdk_codegen._emit_resource(root, rust, (root, child))
        mod_source = sdk_codegen._emit_mod(sdk_codegen.SdkIr("Client", (), (root, child)))
        self.assertIn("pub fn jobs(&self) -> BatchJobs<'a>", root_source)
        self.assertIn("pub fn batch(&self) -> Batch<'_>", mod_source)
        self.assertNotIn("pub fn batch_jobs(&self)", mod_source)

    def test_optional_query_addition_preserves_resource_signature(self):
        operation = sdk_codegen.OperationSpec("list", "list", "list", None, "Receipt", {}, None)
        resource = sdk_codegen.ResourceSpec(("zoo",), "zoo", "Zoo", (operation,))
        old = sdk_codegen.RustIndex(TYPES.encode(), b"impl HttpClient { pub async fn list(&self, provider: Option<impl AsRef<str>>) -> Result<AnimalResponse, Error> { todo!() } }")
        new = sdk_codegen.RustIndex(TYPES.encode(), b"impl HttpClient { pub async fn list(&self, provider: Option<impl AsRef<str>>, limit: Option<i64>) -> Result<AnimalResponse, Error> { todo!() } }")
        before = sdk_codegen._emit_resource(resource, old, (resource,))
        after = sdk_codegen._emit_resource(resource, new, (resource,))
        signature = "pub async fn list_with(&self, request: ListZooRequest)"
        self.assertIn(signature, before)
        self.assertIn(signature, after)
        self.assertIn("pub fn limit(mut self, limit: i64)", after)
        diff = compare_surface(public_surface({"zoo.rs": before}), public_surface({"zoo.rs": after}))
        self.assertEqual(diff["classification"], "additive")

    def test_binary_success_is_validated_and_emitted_as_stream(self):
        document = openapi_document()
        document["paths"]["/animals/{animal_id}/content"] = {
            "get": {
                "operationId": "download_animal",
                "parameters": [{
                    "name": "animal_id", "in": "path", "required": True,
                    "schema": {"type": "string"},
                }],
                "responses": {"200": {
                    "content": {"application/octet-stream": {
                        "schema": {"type": "string", "format": "binary"}
                    }}
                }},
            }
        }
        overlay = manifest()
        overlay["resources"]["zoo"]["operations"]["download"] = {
            "operation_id": "download_animal", "binary_response": True,
        }
        self.openapi.write_text(json.dumps(document))
        self.overlay.write_text(json.dumps(overlay))
        (self.raw / "client.rs").write_text(CLIENT + """
impl HttpClient {
    pub async fn download_animal(&self, animal_id: impl AsRef<str>) -> Result<futures_util::stream::BoxStream<'static, Result<bytes::Bytes, reqwest::Error>>, Error> { todo!() }
}
""")
        resource = (self.generate() / "zoo.rs").read_text()
        self.assertIn(
            "pub async fn download(&self, animal_id: impl AsRef<str>) -> Result<BinaryStream, SdkError>",
            resource,
        )

    def test_empty_success_is_validated_and_emitted_as_unit(self):
        document = openapi_document()
        document["paths"]["/animals/{animal_id}"] = {
            "delete": {
                "operationId": "delete_animal",
                "parameters": [{
                    "name": "animal_id", "in": "path", "required": True,
                    "schema": {"type": "string"},
                }],
                "responses": {"204": {"description": "No Content"}},
            }
        }
        overlay = manifest()
        overlay["resources"]["zoo"]["operations"]["delete"] = {
            "operation_id": "delete_animal", "empty_response": True,
        }
        self.openapi.write_text(json.dumps(document))
        self.overlay.write_text(json.dumps(overlay))
        (self.raw / "client.rs").write_text(CLIENT + """
impl HttpClient {
    pub async fn delete_animal(&self, animal_id: impl AsRef<str>) -> Result<(), Error> { todo!() }
}
""")
        resource = (self.generate() / "zoo.rs").read_text()
        self.assertIn(
            "pub async fn delete(&self, animal_id: impl AsRef<str>) -> Result<(), SdkError>",
            resource,
        )
        self.assertIn("self.raw.delete_animal(animal_id.as_ref()).await.map_err(Into::into)", resource)

    def test_empty_success_raw_drift_fails_closed(self):
        document = openapi_document()
        document["paths"]["/delete"] = {
            "delete": {"operationId": "delete_animal", "responses": {"204": {"description": "No Content"}}}
        }
        overlay = manifest()
        overlay["resources"]["zoo"]["operations"]["delete"] = {
            "operation_id": "delete_animal", "empty_response": True,
        }
        self.openapi.write_text(json.dumps(document))
        self.overlay.write_text(json.dumps(overlay))
        (self.raw / "client.rs").write_text(CLIENT + """
impl HttpClient {
    pub async fn delete_animal(&self) -> Result<AnimalResponse, Error> { todo!() }
}
""")
        with self.assertRaisesRegex(sdk_codegen.GenerationError, "raw empty response drift"):
            self.generate()

    def test_inventory_includes_unmapped_operations(self):
        document = openapi_document()
        document["paths"]["/binary"] = {"get": {"operationId": "download", "responses": {"200": {"content": {"application/octet-stream": {}}}}}}
        self.openapi.write_text(json.dumps(document))
        inventory = json.loads((self.generate() / "coverage.json").read_text())["inventory"]
        self.assertEqual(set(inventory), {"adopt", "download"})
        self.assertEqual(inventory["download"]["status"], "capability_gap")

    def test_generation_is_deterministic(self):
        target = self.generate()
        before = {path.name: path.read_bytes() for path in target.iterdir()}
        self.generate()
        self.assertEqual(before, {path.name: path.read_bytes() for path in target.iterdir()})

    def test_public_symbol_policy_rejects_collisions_and_keywords(self):
        for name in ("Menagerie", "SdkError", "type"):
            with self.subTest(name=name):
                overlay = manifest()
                overlay["resources"]["zoo"]["name"] = name
                self.overlay.write_text(json.dumps(overlay))
                with self.assertRaisesRegex(sdk_codegen.GenerationError, "symbol"):
                    self.generate()

    def test_backend_policies_are_immutable_typed_values(self):
        from dataclasses import FrozenInstanceError
        from sdk_ir import RequestPolicy
        ir = sdk_codegen.build_ir(sdk_codegen.OpenApiIndex(openapi_document()),
                                  sdk_codegen.RustIndex(TYPES.encode(), CLIENT.encode()), manifest())
        request = next(model for model in ir.models if model.name == "Adoption")
        self.assertIsInstance(request.config, RequestPolicy)
        with self.assertRaises(FrozenInstanceError):
            request.config.constructor = ()

    def test_parameter_symbols_resolve_their_raw_module(self):
        index = sdk_codegen.RustIndex(TYPES.encode(), (CLIENT + "\npub enum SortOrder { Asc, Desc }\n").encode())
        self.assertEqual(index.qualified_type("Option<SortOrder>"), "Option<crate::generated::client::SortOrder>")
        self.assertEqual(index.qualified_type("Vec<AnimalRequest>"), "Vec<crate::generated::types::AnimalRequest>")

    def test_body_and_path_parameter_are_composed_from_the_raw_ast(self):
        document = openapi_document()
        operation = document["paths"]["/animals"].pop("post")
        operation["operationId"] = "update"
        operation["parameters"] = [{
            "name": "animal_id", "in": "path", "required": True,
            "schema": {"type": "string"},
        }]
        document["paths"] = {"/animals/{animal_id}": {"patch": operation}}
        overlay = manifest()
        overlay["resources"]["zoo"]["operations"] = {
            "update": {"operation_id": "update", "request": "Adoption", "response": "Receipt"}
        }
        self.openapi.write_text(json.dumps(document))
        self.overlay.write_text(json.dumps(overlay))
        (self.raw / "client.rs").write_text(CLIENT.replace(
            "adopt(&self, request: AnimalRequest)",
            "update(&self, animal_id: impl AsRef<str>, request: AnimalRequest)",
        ))
        resource = (self.generate() / "zoo.rs").read_text()
        self.assertIn("pub async fn update(&self, animal_id: impl AsRef<str>, request: Adoption)", resource)
        self.assertIn("self.raw.update(animal_id.as_ref(), request.into_raw())", resource)

    def test_inline_response_union_is_reconciled_structurally(self):
        document = openapi_document()
        document["paths"]["/animals"]["post"]["responses"]["200"]["content"]["application/json"]["schema"] = {
            "oneOf": [
                {"$ref": "#/components/schemas/Cat"},
                {"$ref": "#/components/schemas/Dog"},
            ],
            "discriminator": {"propertyName": "kind"},
        }
        index = sdk_codegen.RustIndex(TYPES.encode(), CLIENT.encode())
        self.assertTrue(sdk_codegen.OpenApiIndex(document).response_matches("adopt", "AnimalUnion", index))

    def test_overlay_model_references_fail_closed(self):
        overlay = manifest()
        overlay["models"]["Adoption"]["adapters"] = {"animals": "MissingAnimal"}
        self.overlay.write_text(json.dumps(overlay))
        with self.assertRaisesRegex(sdk_codegen.GenerationError, "unknown facade models"):
            self.generate()

    def test_protocol_keyword_fields_are_escaped(self):
        document = openapi_document()
        document["components"]["schemas"]["AnimalRequest"]["properties"]["type"] = {"type": "string"}
        self.openapi.write_text(json.dumps(document))
        (self.raw / "types.rs").write_text(TYPES.replace("pub energy: Option<i64>,", "pub energy: Option<i64>, pub r#type: Option<String>,"))
        output = (self.generate() / "facade_types.rs").read_text()
        self.assertIn("pub fn r#type(mut self, r#type: impl Into<String>)", output)

    def stream_fixture(self):
        document = openapi_document()
        overlay = manifest()
        operation = document["paths"]["/animals"]["post"]
        operation["responses"]["200"]["content"]["text/event-stream"] = {"schema": {"$ref": "#/components/schemas/AnimalResponse"}}
        overlay["resources"]["zoo"]["operations"]["adopt"] = {
            "operation_id": "adopt", "request": "Adoption",
            "stream": {"item": "AnimalResponse", "wrapper": "Receipt", "type": "ReceiptStream"},
        }
        client = CLIENT.replace("Result<AnimalResponse, Error>", "Result<futures_util::stream::BoxStream<'static, Result<bytes::Bytes, reqwest::Error>>, Error>")
        self.openapi.write_text(json.dumps(document))
        self.overlay.write_text(json.dumps(overlay))
        (self.raw / "client.rs").write_text(client)
        return document, overlay, client

    def test_stream_owned_payload_contract_is_validated(self):
        self.stream_fixture()
        self.assertIn("ReceiptStream", (self.generate() / "zoo.rs").read_text())

    def test_borrowed_stream_transport_is_rejected(self):
        _, _, client = self.stream_fixture()
        (self.raw / "client.rs").write_text(client.replace("'static", "'_"))
        with self.assertRaisesRegex(sdk_codegen.GenerationError, "ownership/item drift"):
            self.generate()

    def test_wrong_stream_wire_payload_is_rejected(self):
        document, _, _ = self.stream_fixture()
        document["paths"]["/animals"]["post"]["responses"]["200"]["content"]["text/event-stream"]["schema"] = {"type": "string"}
        self.openapi.write_text(json.dumps(document))
        with self.assertRaisesRegex(sdk_codegen.GenerationError, "stream payload drift"):
            self.generate()

    def test_request_override_type_is_validated_against_both_sources(self):
        overlay = manifest()
        overlay["resources"]["zoo"]["operations"]["adopt"]["request_overrides"] = {"energy": True}
        self.overlay.write_text(json.dumps(overlay))
        with self.assertRaisesRegex(sdk_codegen.GenerationError, "optional Boolean"):
            self.generate()


if __name__ == "__main__":
    unittest.main()
