"""Pinning regressions: a failed or unchanged source must not mask the other SDK."""

from __future__ import annotations

import importlib.util
import json
from pathlib import Path
import tempfile
import unittest
from unittest.mock import patch


HERE = Path(__file__).resolve().parent
SPEC = importlib.util.spec_from_file_location(
    "official_sdk_harvest", HERE / "official-sdks" / "harvest.py"
)
assert SPEC is not None and SPEC.loader is not None
harvest = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(harvest)


class OfficialSourcePinTests(unittest.TestCase):
    def setUp(self) -> None:
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.lock_path = Path(self.temp.name) / "provenance.lock.json"
        self.original = {
            "official_sdks": {
                "python": {"repository": "example/python", "commit": "old-python"},
                "typescript": {"repository": "example/typescript", "commit": "old-typescript"},
            }
        }
        self.lock_path.write_text(json.dumps(self.original, indent=4) + "\n")
        replacement = patch.object(harvest, "LOCK_PATH", self.lock_path)
        replacement.start()
        self.addCleanup(replacement.stop)

    def test_unchanged_source_does_not_reformat_lock(self) -> None:
        with patch.object(harvest, "latest_commit", return_value="old-python"):
            harvest.pin_latest(self.original.copy(), source="python")
        self.assertEqual(self.lock_path.read_text(), json.dumps(self.original, indent=4) + "\n")

    def test_one_sdk_can_advance_independently(self) -> None:
        with patch.object(harvest, "latest_commit", return_value="new-python"):
            harvest.pin_latest(self.original.copy(), source="python")
        changed = json.loads(self.lock_path.read_text())["official_sdks"]
        self.assertEqual(changed["python"]["commit"], "new-python")
        self.assertEqual(changed["typescript"]["commit"], "old-typescript")

    def test_unknown_source_is_rejected(self) -> None:
        with self.assertRaises(ValueError):
            harvest.pin_latest(self.original, source="unknown")


class TypeScriptSurfaceTests(unittest.TestCase):
    def setUp(self) -> None:
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name)
        (self.root / "src/sdk").mkdir(parents=True)
        (self.root / "src/funcs").mkdir()
        (self.root / "src/sdk/sdk.ts").write_text(
            "export class Mistral extends ClientSDK {\n"
            "  get audio(): Audio { return new Audio(this._options); }\n"
            "}\n"
        )
        (self.root / "src/sdk/audio.ts").write_text(
            "export class Audio extends ClientSDK {\n"
            "  get speech(): Speech { return new Speech(this._options); }\n"
            "}\n"
        )
        (self.root / "src/funcs/audioSpeechComplete.ts").write_text(
            'const path = pathToFunc("/v1/audio/speech")();\n'
            'const request = { method: "POST" };\n'
            'const context = { operationID: "speech_v1_audio_speech_post" };\n'
        )

    def test_multiline_multi_symbol_import_includes_speech(self) -> None:
        (self.root / "src/sdk/speech.ts").write_text(
            'import {\n'
            '  audioSpeechComplete,\n'
            '  CompleteAcceptEnum,\n'
            '} from "../funcs/audioSpeechComplete.js";\n'
            'export class Speech extends ClientSDK {\n'
            '  async complete(request: SpeechRequest): Promise<Response> {\n'
            '    return unwrapAsync(audioSpeechComplete(this, request));\n'
            '  }\n'
            '}\n'
        )
        records = harvest.parse_typescript(self.root)
        self.assertEqual(len(records), 1)
        self.assertEqual(records[0]["public_path"], "audio.speech.complete")
        self.assertEqual(records[0]["http_method"], "POST")
        self.assertEqual(records[0]["http_path"], "/v1/audio/speech")

    def test_single_symbol_import_still_supported(self) -> None:
        (self.root / "src/sdk/speech.ts").write_text(
            'import { audioSpeechComplete } from "../funcs/audioSpeechComplete.js";\n'
            'export class Speech extends ClientSDK {\n'
            '  async complete(request: SpeechRequest): Promise<Response> {\n'
            '    return unwrapAsync(audioSpeechComplete(this, request));\n'
            '  }\n'
            '}\n'
        )
        self.assertEqual(
            [item["public_path"] for item in harvest.parse_typescript(self.root)],
            ["audio.speech.complete"],
        )


if __name__ == "__main__":
    unittest.main()
