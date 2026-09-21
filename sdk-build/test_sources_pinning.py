"""Pinning regressions: a failed or unchanged source must not mask the other SDK."""

from __future__ import annotations

import hashlib
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

OPENAPI_SPEC = importlib.util.spec_from_file_location(
    "openapi_source_update", HERE / "openapi" / "update.py"
)
assert OPENAPI_SPEC is not None and OPENAPI_SPEC.loader is not None
openapi_update = importlib.util.module_from_spec(OPENAPI_SPEC)
OPENAPI_SPEC.loader.exec_module(openapi_update)


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


class OpenApiIndependentSourceTests(unittest.TestCase):
    PINNED = "a" * 40
    NEXT = "b" * 40
    OLD = b"openapi: 3.1.0\ninfo: {title: old}\n"
    NEW = b"openapi: 3.1.0\ninfo: {title: new}\n"
    PUBLIC = b"openapi: 3.1.0\ninfo: {title: broader public spec}\n"

    def setUp(self) -> None:
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        root = Path(self.temp.name)
        self.here = root / "openapi"
        self.here.mkdir()
        self.lock_path = root / "provenance.lock.json"
        self.here.joinpath("published.yaml").write_bytes(self.OLD)
        self.initial = {"openapi": {
            "repository": "mistralai/platform-docs-public",
            "path": "openapi.yaml",
            "published_url": "https://docs.mistral.ai/openapi.yaml",
            "commit": self.PINNED,
            "sha256": hashlib.sha256(self.OLD).hexdigest(),
            "published_sha256": hashlib.sha256(self.PUBLIC).hexdigest(),
        }}
        self.lock_path.write_text(json.dumps(self.initial, indent=2) + "\n")
        for name, value in (("LOCK", self.lock_path), ("HERE", self.here)):
            replacement = patch.object(openapi_update, name, value)
            replacement.start()
            self.addCleanup(replacement.stop)
        notice = patch.object(openapi_update, "optional_notice", return_value=None)
        notice.start()
        self.addCleanup(notice.stop)

    def serve(self, latest: str, raw: bytes, public: bytes) -> None:
        def fake_fetch(url: str, *, github_api: bool = False) -> bytes:
            if github_api:
                return json.dumps([{"sha": latest}]).encode()
            if url == self.initial["openapi"]["published_url"]:
                return public
            if url.endswith("/LICENSE"):
                return b"Apache-2.0"
            if url.endswith(f"/{latest}/openapi.yaml"):
                return raw
            raise AssertionError(f"Unexpected network request: {url}")

        replacement = patch.object(openapi_update, "fetch", side_effect=fake_fetch)
        replacement.start()
        self.addCleanup(replacement.stop)

    def test_reviewed_divergent_public_spec_does_not_block_unchanged_pin(self) -> None:
        self.serve(self.PINNED, self.OLD, self.PUBLIC)
        before = self.lock_path.read_bytes()
        openapi_update.main()
        self.assertEqual(self.lock_path.read_bytes(), before)
        self.assertEqual(self.here.joinpath("published.yaml").read_bytes(), self.OLD)

    def test_changed_repository_pin_is_independent_of_known_public_divergence(self) -> None:
        self.serve(self.NEXT, self.NEW, self.PUBLIC)
        openapi_update.main()
        self.assertEqual(self.here.joinpath("published.yaml").read_bytes(), self.NEW)
        after = json.loads(self.lock_path.read_text())["openapi"]
        self.assertEqual(after["commit"], self.NEXT)
        self.assertEqual(after["sha256"], hashlib.sha256(self.NEW).hexdigest())
        self.assertEqual(after["published_sha256"], self.initial["openapi"]["published_sha256"])

    def test_unreviewed_public_change_fails_without_mutating_pin(self) -> None:
        self.serve(self.NEXT, self.NEW, self.PUBLIC + b"# unreviewed")
        before = self.lock_path.read_bytes()
        with self.assertRaisesRegex(ValueError, "Published OpenAPI changed independently"):
            openapi_update.main()
        self.assertEqual(self.lock_path.read_bytes(), before)
        self.assertEqual(self.here.joinpath("published.yaml").read_bytes(), self.OLD)

    def test_missing_reviewed_fingerprint_fails_closed(self) -> None:
        self.initial["openapi"].pop("published_sha256")
        self.lock_path.write_text(json.dumps(self.initial))
        self.serve(self.PINNED, self.OLD, self.PUBLIC)
        with self.assertRaisesRegex(ValueError, "Missing reviewed published"):
            openapi_update.main()

    def test_invalid_new_repository_dialect_still_fails(self) -> None:
        self.serve(self.NEXT, b"openapi: 3.0.0\n", self.PUBLIC)
        with self.assertRaisesRegex(ValueError, "dialect changed"):
            openapi_update.main()
        self.assertEqual(self.here.joinpath("published.yaml").read_bytes(), self.OLD)


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
