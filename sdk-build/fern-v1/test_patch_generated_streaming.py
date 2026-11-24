import sys
import tempfile
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent))
from patch_generated_streaming import patch


class ConditionalStreamingPatchTests(unittest.TestCase):
    def test_restores_chat_and_fim_stream_methods(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            for resource, client in (("chat", "ChatClient"), ("fim", "FimClient")):
                path = root / "api" / "resources" / resource / "mod.rs"
                path.parent.mkdir(parents=True)
                path.write_text(
                    f"use crate::api::types::*;\n"
                    f"use crate::core::RequestOptions;\n"
                    f"pub struct {client};\n"
                    f"impl {client} {{\n"
                    f"    pub async fn complete(&self) {{}}\n"
                    f"}}\n"
                )
            changed = patch(root)
            self.assertEqual(changed, ["chat/mod.rs", "fim/mod.rs"])
            for resource in ("chat", "fim"):
                source = (root / "api" / "resources" / resource / "mod.rs").read_text()
                self.assertIn("pub async fn complete_stream(", source)
                self.assertIn("body.stream = Some(true);", source)
                self.assertIn("parse_sse_stream(response)", source)

    def test_rejects_duplicate_stream_methods(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            for resource in ("chat", "fim"):
                path = root / "api" / "resources" / resource / "mod.rs"
                path.parent.mkdir(parents=True)
                path.write_text("impl Client { pub async fn complete_stream() {} }\n")
            with self.assertRaisesRegex(ValueError, "already exists"):
                patch(root)


if __name__ == "__main__":
    unittest.main()
