import unittest

from fix_local_schema_refs import rewrite_nested_defs, validate_local_refs


class NestedDefsReferenceTests(unittest.TestCase):
    def test_rewrites_fragment_local_refs_to_nearest_defs_scope(self):
        document = {
            "paths": {
                "/audio": {
                    "schema": {
                        "$defs": {
                            "Payload": {"type": "object"},
                        },
                        "properties": {
                            "payload": {"$ref": "#/$defs/Payload"},
                        },
                    }
                }
            }
        }
        self.assertEqual(rewrite_nested_defs(document), 1)
        self.assertEqual(
            document["paths"]["/audio"]["schema"]["properties"]["payload"]["$ref"],
            "#/paths/~1audio/schema/$defs/Payload",
        )
        validate_local_refs(document)

    def test_preserves_root_defs_refs(self):
        document = {
            "$defs": {"Shared": {"type": "string"}},
            "properties": {"value": {"$ref": "#/$defs/Shared"}},
        }
        self.assertEqual(rewrite_nested_defs(document), 1)
        self.assertEqual(
            document["properties"]["value"]["$ref"],
            "#/$defs/Shared",
        )
        validate_local_refs(document)

    def test_rejects_unresolved_refs(self):
        with self.assertRaisesRegex(ValueError, "Unresolved local"):
            validate_local_refs({"$ref": "#/missing"})


if __name__ == "__main__":
    unittest.main()
