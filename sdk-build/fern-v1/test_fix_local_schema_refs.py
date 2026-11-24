import sys
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent))
from fix_local_schema_refs import (
    hoist_nested_defs,
    normalize_summary_stream_union,
    validate_local_refs,
)


class NestedSchemaDefinitionTests(unittest.TestCase):
    def test_hoists_fragment_defs_and_rewrites_refs(self):
        document = {
            "openapi": "3.1.0",
            "paths": {
                "/audio": {
                    "post": {
                        "responses": {
                            "200": {
                                "content": {
                                    "application/json": {
                                        "schema": {
                                            "$defs": {"Payload": {"type": "object"}},
                                            "properties": {
                                                "payload": {"$ref": "#/$defs/Payload"}
                                            },
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "components": {"schemas": {}},
        }
        self.assertEqual(hoist_nested_defs(document), 1)
        schema = document["paths"]["/audio"]["post"]["responses"]["200"]["content"]["application/json"]["schema"]
        ref = schema["properties"]["payload"]["$ref"]
        self.assertTrue(ref.startswith("#/components/schemas/FernLocal"))
        self.assertNotIn("$defs", schema)
        validate_local_refs(document)

    def test_normalizes_disjoint_summary_stream_union(self):
        document = {
            "components": {
                "schemas": {
                    "Metadata": {"required": ["status", "translated"]},
                    "Chunk": {"required": ["content"]},
                    "Error": {"required": ["error"]},
                }
            },
            "paths": {
                "/v1/context/search-indexes/{index_name}/summary_field/{language}": {
                    "post": {
                        "responses": {
                            "200": {
                                "content": {
                                    "application/x-ndjson": {
                                        "schema": {
                                            "anyOf": [
                                                {"$ref": "#/components/schemas/Metadata"},
                                                {"$ref": "#/components/schemas/Chunk"},
                                                {"$ref": "#/components/schemas/Error"},
                                            ]
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
        }
        self.assertTrue(normalize_summary_stream_union(document))
        schema = document["paths"]["/v1/context/search-indexes/{index_name}/summary_field/{language}"]["post"]["responses"]["200"]["content"]["application/x-ndjson"]["schema"]
        self.assertIn("oneOf", schema)
        self.assertNotIn("anyOf", schema)
        validate_local_refs(document)

    def test_rejects_overlapping_union_shapes(self):
        document = {
            "components": {
                "schemas": {
                    "A": {"required": ["id"]},
                    "B": {"required": ["id", "content"]},
                    "C": {"required": ["error"]},
                }
            },
            "paths": {
                "/v1/context/search-indexes/{index_name}/summary_field/{language}": {
                    "post": {
                        "responses": {
                            "200": {
                                "content": {
                                    "application/x-ndjson": {
                                        "schema": {
                                            "anyOf": [
                                                {"$ref": "#/components/schemas/A"},
                                                {"$ref": "#/components/schemas/B"},
                                                {"$ref": "#/components/schemas/C"},
                                            ]
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
        }
        with self.assertRaisesRegex(ValueError, "no longer disjoint"):
            normalize_summary_stream_union(document)

    def test_rejects_unresolved_local_refs(self):
        with self.assertRaisesRegex(ValueError, "Unresolved local"):
            validate_local_refs({"$ref": "#/missing"})


if __name__ == "__main__":
    unittest.main()
