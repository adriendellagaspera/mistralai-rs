import sys
import tempfile
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent))
from patch_generated_sdk import FIXED_STRING_TYPES, MISSING_TYPE_NAMES, patch_generated_sdk


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
            "struct Fixture { file: () }\n"
            "impl Fixture { fn fixture(&self) { let mut form = (); let value = ();\n"
            + "\n".join(
                f'form = form.text("{field}", value.to_string());'
                for field in ("context_bias", "speaker_references", "timestamp_granularities")
            )
            + '\nform = form.text("file", value.to_string());\n} }\n'
        )
        (types / "audio_transcription_request.rs").write_text(multipart_body)
        (types / "stream_request.rs").write_text(multipart_body)
        for filename in ("document_upload.rs", "multi_part_body_params.rs"):
            (types / filename).write_text(
                "struct Fixture { file: () }\n"
                "impl Fixture { fn fixture(&self) { let mut form = (); "
                'form = form.text("file", self.file.to_string()); } }\n'
            )
        (types / "conversation_messages.rs").write_text(
            "pub struct ConversationMessages {\n"
            "    #[serde(default)]\n"
            "    pub messages: MessageEntries,\n"
            "}\n"
        )
        (types / "file.rs").write_text("pub struct File(pub Vec<u8>);\n")

        (src / "config.rs").write_text(
            "pub struct ClientConfig { pub access_token: Option<String>, pub token: Option<String> }\n"
            "pub struct Builder { access_token: Option<String>, token: Option<String> }\n"
            "impl Builder { fn build(self) -> ClientConfig { ClientConfig {\n"
            "            access_token: self.access_token,\n"
            "} } }\n"
        )
        (src / "client.rs").write_text(
            "use std::sync::Arc;\n"
            "use crate::config::ClientConfig;\n"
            "pub struct Mistral;\n"
            "impl Mistral {\n"
            "    pub fn new(config: ClientConfig) -> Result<Self, crate::error::ApiError> {\n"
            "        let http_client = Arc::new(1);\n"
            "        Ok(Self)\n"
            "    }\n"
            "}\n"
        )
        resources = src / "api" / "resources" / "chat"
        resources.mkdir(parents=True)
        auth_block = (
            "                if let Some(token) = &opts.token {\n"
            "                    overridden.access_token = Some(token.clone());\n"
            "                }"
        )
        api_key_block = (
            "                if let Some(api_key) = &opts.api_key {\n"
            "                    overridden.api_key = Some(api_key.clone());\n"
            "                }"
        )
        (resources / "mod.rs").write_text(
            "struct Options { token: Option<String>, api_key: Option<String> }\n"
            "struct Config { access_token: Option<String>, api_key: Option<String> }\n"
            "fn fixture(opts: &Options, mut overridden: Config) {\n"
            + auth_block * 343
            + (auth_block + "\n" + api_key_block) * 6
            + "}\n"
        )
        return src

    def test_applies_all_expected_patches(self):
        with tempfile.TemporaryDirectory() as directory:
            src = self.make_fixture(Path(directory))
            counts = patch_generated_sdk(src)
            self.assertEqual(len([k for k in counts if k.startswith("fixed-enum:")]), 5)
            types = (src / "api" / "types" / "mod.rs").read_text()
            for name, (variant, wire_value) in FIXED_STRING_TYPES.items():
                self.assertIn(f"pub enum {name}", types)
                self.assertIn("#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash,", types)
                self.assertIn(f'#[serde(rename = "{wire_value}")]' , types)
                self.assertIn(f"    {variant},", types)
            messages = (src / "api" / "types" / "conversation_messages.rs").read_text()
            self.assertNotIn("#[serde(default)]", messages)
            for filename in ("audio_transcription_request.rs", "stream_request.rs"):
                source = (src / "api" / "types" / filename).read_text()
                self.assertIn("serde_json::to_string(&value)", source)
                self.assertIn("reqwest::multipart::Part::bytes(value.0)", source)
            config = (src / "config.rs").read_text()
            self.assertIn("self.access_token.or_else(|| self.token.clone())", config)
            client = (src / "client.rs").read_text()
            self.assertIn("config.access_token = config.token.clone();", client)
            resources = (src / "api" / "resources" / "chat" / "mod.rs").read_text()
            self.assertEqual(resources.count("overridden.api_key = Some(api_key.clone());"), 349)
            self.assertEqual(counts["auth:per-call-api-key-overrides"], 343)
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
