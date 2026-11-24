import sys
import tempfile
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent))
from patch_generated_sdk import MISSING_TYPE_NAMES, patch_generated_sdk


class GeneratedSdkCompatibilityPatchTests(unittest.TestCase):
    def make_fixture(self, root: Path) -> Path:
        src = root / "src"
        types = src / "api" / "types"
        types.mkdir(parents=True)
        (types / "mod.rs").write_text("pub mod file;\n")
        for name, count in {
            "GetDeploymentSummariesResponseVespaDeploymentType": 3,
            "AssistantMessageRole": 9,
            "OAuth2ClientCredentialsAuthMethodGrantType": 3,
            "FunctionToolType": 9,
            "OcrCodeBlockType": 6,
        }.items():
            (types / f"{name.lower()}.rs").write_text((name + "\n") * count)
        multipart_body = (
            "struct Fixture { file: () }\\n"
            "impl Fixture { fn fixture(&self) { let mut form = (); let value = ();\\n"
            + "\\n".join(
                f'form = form.text("{field}", value.to_string());'
                for field in ("context_bias", "speaker_references", "timestamp_granularities")
            )
            + '\\nform = form.text("file", value.to_string());\\n} }\\n'
        )
        (types / "audio_transcription_request.rs").write_text(multipart_body)
        (types / "stream_request.rs").write_text(multipart_body)
        for filename in ("document_upload.rs", "multi_part_body_params.rs"):
            (types / filename).write_text(
                "struct Fixture { file: () }\\n"
                "impl Fixture { fn fixture(&self) { let mut form = (); "
                'form = form.text("file", self.file.to_string()); } }\\n'
            )
        (types / "conversation_messages.rs").write_text(
            "    #[serde(default)]\n    pub messages: MessageEntries,\n"
        )
        (types / "file.rs").write_text("pub struct File(pub Vec<u8>);\n")
        return src

    def test_applies_all_expected_patches(self):
        with tempfile.TemporaryDirectory() as directory:
            src = self.make_fixture(Path(directory))
            counts = patch_generated_sdk(src)
            self.assertEqual(len([k for k in counts if k.startswith("type-alias:")]), 5)
            types = (src / "api" / "types" / "mod.rs").read_text()
            for name in MISSING_TYPE_NAMES:
                self.assertIn(f"pub type {name} = String;", types)
            messages = (src / "api" / "types" / "conversation_messages.rs").read_text()
            self.assertNotIn("#[serde(default)]", messages)
            for filename in ("audio_transcription_request.rs", "stream_request.rs"):
                source = (src / "api" / "types" / filename).read_text()
                self.assertIn("serde_json::to_string(&value)", source)
                self.assertIn("reqwest::multipart::Part::bytes(value.0)", source)
            for filename in ("document_upload.rs", "multi_part_body_params.rs"):
                source = (src / "api" / "types" / filename).read_text()
                self.assertIn("reqwest::multipart::Part::bytes(self.file.0)", source)

    def test_fails_on_generator_output_drift(self):
        with tempfile.TemporaryDirectory() as directory:
            src = self.make_fixture(Path(directory))
            path = src / "api" / "types" / "audio_transcription_request.rs"
            path.write_text(path.read_text().replace('"file", value.to_string()', '"upload", value.to_string()'))
            with self.assertRaisesRegex(ValueError, "expected one file text part"):
                patch_generated_sdk(src)


if __name__ == "__main__":
    unittest.main()
