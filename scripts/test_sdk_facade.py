import json
from pathlib import Path
import sys
import tempfile
import unittest

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT / "codegen"))
import sdk_facade


TYPES = """
pub struct ChatCompletionRequest {
    pub messages: Vec<ChatCompletionRequestMessagesItemUnion>,
    pub model: String,
    pub stream: Option<bool>,
    pub temperature: Option<Option<f64>>,
    pub prompt_cache_key: Option<Option<String>>,
    pub labels: Option<std::collections::BTreeMap<String, String>>,
}
pub struct ChatCompletionResponse {}
pub struct CompletionChunk {}
pub struct OCRRequest {
    pub document: OCRRequestDocument,
    pub include_blocks: Option<bool>,
    pub model: Option<String>,
}
pub struct OCRResponse {}
"""

CLIENT = """
impl HttpClient {
    pub async fn chat_completion_v1_chat_completions_post(
        &self,
        request: ChatCompletionRequest,
    ) -> Result<ChatCompletionResponse, ApiOpError<Error>> { todo!() }
    pub async fn chat_completion_v1_chat_completions_post_stream(
        &self,
        request: ChatCompletionRequest,
    ) -> Result<BoxStream<'static, Result<Bytes, Error>>, ApiOpError<Error>> { todo!() }
    pub async fn ocr_v1_ocr_post(
        &self,
        request: OCRRequest,
    ) -> Result<OCRResponse, ApiOpError<Error>> { todo!() }
}
"""


class FacadeGenerationTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name)
        self.raw = self.root / "raw"
        self.raw.mkdir()
        (self.raw / "types.rs").write_text(TYPES)
        (self.raw / "client.rs").write_text(CLIENT)
        self.manifest = ROOT / "codegen/sdk-semantics.json"

    def test_optional_raw_fields_generate_fluent_setters(self):
        target = self.root / "sdk"
        sdk_facade.generate(self.raw, target, self.manifest)
        chat = (target / "chat.rs").read_text()
        ocr = (target / "ocr.rs").read_text()
        self.assertIn("pub fn temperature(mut self, temperature: f64)", chat)
        self.assertIn("pub fn temperature_null(mut self)", chat)
        self.assertIn("pub fn prompt_cache_key(mut self, prompt_cache_key: impl Into<String>)", chat)
        self.assertIn(
            "pub fn labels(mut self, labels: std::collections::BTreeMap<String, String>)",
            chat,
        )
        self.assertIn("pub fn include_blocks(mut self, include_blocks: bool)", ocr)
        self.assertTrue(chat.startswith(sdk_facade.GENERATED))
        coverage = json.loads((target / "coverage.json").read_text())
        self.assertIn("temperature", coverage["resources"]["chat"]["request_fields"])

    def test_required_field_drift_fails_closed(self):
        (self.raw / "types.rs").write_text(
            TYPES.replace(
                "    pub stream: Option<bool>,",
                "    pub stream: Option<bool>,\n    pub new_required: String,",
            )
        )
        with self.assertRaisesRegex(ValueError, "new required request field"):
            sdk_facade.generate(self.raw, self.root / "sdk", self.manifest)

    def test_operation_signature_drift_fails_closed(self):
        (self.raw / "client.rs").write_text(CLIENT.replace("request: OCRRequest", "request: Other"))
        with self.assertRaisesRegex(ValueError, "semantic review required"):
            sdk_facade.generate(self.raw, self.root / "sdk", self.manifest)


if __name__ == "__main__":
    unittest.main()
