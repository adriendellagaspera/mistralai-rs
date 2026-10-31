# Pinned Mistral official SDK parity audit

**Evidence only: SDK-observed routes are not approved HTTP contracts.**

## Pinned sources

| Source | Revision |
| --- | --- |
| OpenAPI | `08a585fb75bcfa6ff968773134ee00060e0db283fc6b16f421f544e78ba8db92` |
| Python | `8fdbff0b1bcb0c9c3e5b803080a20ac820c2905c` |
| TypeScript | `50b2bc105cce327d3bd2ccee86240c4b0f6f2d6c` |

## HTTP summary

| Measure | Count |
| --- | ---: |
| Published OpenAPI operations | 321 |
| OpenAPI operations with official SDK taxonomy | 260 |
| Extracted Python SDK methods | 290 |
| Extracted TypeScript SDK methods | 292 |
| Unresolved Python methods | 25 |
| Unresolved TypeScript methods | 25 |
| New HTTP routes observed in both SDKs | 25 |
| New HTTP routes observed in only one SDK | 0 |
| Existing OpenAPI routes with unresolved SDK identity | 0 |
| Public method taxonomy divergences | 1 |

Unresolved methods are not necessarily missing HTTP operations.
Matching uses exact normalized method/path, not inferred request shapes.

## Tier 2 — candidates for contract review

| HTTP route | Official SDKs | Classification |
| --- | --- | --- |
| `DELETE /v1/audio/agents/{agent_id}` | python, typescript | sdk_observed_both |
| `DELETE /v1/audio/sessions/{session_id}` | python, typescript | sdk_observed_both |
| `DELETE /v1/context/search-indexes/{index_name}` | python, typescript | sdk_observed_both |
| `DELETE /v1/context/search-indexes/{index_name}/documents` | python, typescript | sdk_observed_both |
| `GET /v1/audio/agents` | python, typescript | sdk_observed_both |
| `GET /v1/audio/agents/{agent_id}` | python, typescript | sdk_observed_both |
| `GET /v1/audio/agents/{agent_id}/sessions` | python, typescript | sdk_observed_both |
| `GET /v1/audio/agents/{agent_id}/versions` | python, typescript | sdk_observed_both |
| `GET /v1/audio/sessions/{session_id}` | python, typescript | sdk_observed_both |
| `GET /v1/context/search-indexes` | python, typescript | sdk_observed_both |
| `GET /v1/context/search-indexes/{index_name}` | python, typescript | sdk_observed_both |
| `GET /v1/context/search-indexes/{index_name}/chunks/{chunk_id}` | python, typescript | sdk_observed_both |
| `GET /v1/context/search-indexes/{index_name}/summary_field/{language}` | python, typescript | sdk_observed_both |
| `POST /v1/audio/agents` | python, typescript | sdk_observed_both |
| `POST /v1/audio/sessions` | python, typescript | sdk_observed_both |
| `POST /v1/context/search-indexes` | python, typescript | sdk_observed_both |
| `POST /v1/context/search-indexes/{index_name}/documents` | python, typescript | sdk_observed_both |
| `POST /v1/context/search-indexes/{index_name}/grep` | python, typescript | sdk_observed_both |
| `POST /v1/context/search-indexes/{index_name}/navigate` | python, typescript | sdk_observed_both |
| `POST /v1/context/search-indexes/{index_name}/read` | python, typescript | sdk_observed_both |
| `POST /v1/context/search-indexes/{index_name}/search` | python, typescript | sdk_observed_both |
| `POST /v1/context/search-indexes/{index_name}/summary_field/{language}` | python, typescript | sdk_observed_both |
| `PUT /v1/audio/agents/{agent_id}` | python, typescript | sdk_observed_both |
| `PUT /v1/context/search-indexes/{index_name}` | python, typescript | sdk_observed_both |
| `PUT /v1/context/search-indexes/{index_name}/summary_field/{language}` | python, typescript | sdk_observed_both |

Machine-readable evidence contains original operation IDs, public
paths, transport variants, and matching failures.

## Tier 3 — non-HTTP resources (Python parser evidence)

- `audio.realtime` (`RealtimeTranscription`; `src/mistralai/extra/realtime/transcription.py`)
- `audio.realtime_speech` (`RealtimeSpeech`; `src/mistralai/extra/realtime/speech.py`)

**Limitations:** a resource class is not proof of WebSocket or
realtime behavior. The TypeScript parser currently extracts HTTP only;
it cannot prove the absence of non-HTTP TypeScript capabilities.

## Next actions

1. Validate HTTP request/response/authentication/error semantics for each
   SDK-observed route before accepting a separate Tier-2 contract.
2. Review parameter rename and stream alias false positives.
3. Inventory Python and TypeScript non-HTTP protocol methods and classify
   realtime/WebSocket vs local convenience helpers.
